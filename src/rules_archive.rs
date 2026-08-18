// Shared Rules archive writer used by `build.rs` (include-zip) and the `package-rules` binary.
//
// Layout matches native MathCAT extraction:
// - `Rules/*.yaml` and `Rules/Intent/**` stay loose in the outer archive
// - each `Rules/Languages/<lang>/` and `Rules/Braille/<code>/` directory is stored as `<name>/<name>.zip`
// - `Rules/Languages/zz` (synthetic test language) is omitted
//
// Inner language/braille zips always use BZIP2. Downloadable `Rules.zip` uses DEFLATE for the
// outer archive so Windows Explorer / `unzip` can open it. The include-zip embed uses BZIP2
// for both layers (MathCAT's `zip` crate reads it; zip 8.x bzip2 is pure Rust and works on wasm).

use std::fs::{self, File, read_dir};
use std::io::{self, Cursor, Read, Seek, Write};
use std::path::{Path, PathBuf};

use yaml_rust::{YamlEmitter, YamlLoader};
use zip::write::{SimpleFileOptions, ZipWriter};
use zip::CompressionMethod;

const YAML_SUFFIXES: [&str; 2] = ["yaml", "yml"];
const MINIMIZE_NAMES: [&str; 2] = ["unicode.yaml", "unicode-full.yaml"];
const SKIP_LANGUAGE_DIR: &str = "zz";
const ARCHIVE_ROOT: &str = "Rules";

/// Compression methods for nested language zips vs the outer archive.
#[derive(Clone, Copy)]
pub struct ArchiveCompression {
    pub inner: CompressionMethod,
    pub outer: CompressionMethod,
}

/// Inner BZIP2 / outer BZIP2 — used for the include-zip embed (only MathCAT reads it).
pub fn include_zip_compression() -> ArchiveCompression {
    return ArchiveCompression {
        inner: CompressionMethod::BZIP2,
        outer: CompressionMethod::BZIP2,
    };
}

/// Inner BZIP2 / outer DEFLATE — used for downloadable `Rules.zip` / `Rules-minimized.zip`.
pub fn downloadable_compression() -> ArchiveCompression {
    return ArchiveCompression {
        inner: CompressionMethod::BZIP2,
        outer: CompressionMethod::DEFLATE,
    };
}

fn zip_options(method: CompressionMethod) -> SimpleFileOptions {
    return SimpleFileOptions::default()
        .compression_method(method)
        .compression_level(Some(9));
}

fn is_yaml_file(path: &Path) -> bool {
    return path
        .extension()
        .map(|ext| {
            let ext = ext.to_ascii_lowercase();
            YAML_SUFFIXES.iter().any(|s| ext == *s)
        })
        .unwrap_or(false);
}

fn is_zip_file(path: &Path) -> bool {
    return path.extension().map(|ext| ext.eq_ignore_ascii_case("zip")).unwrap_or(false);
}

fn posix_path(parts: &[&str]) -> String {
    return parts.join("/");
}

fn posix_join(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        return name.to_string();
    }
    return format!("{}/{}", prefix, name);
}

fn file_name_str(path: &Path) -> io::Result<String> {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| io::Error::other(format!("non-UTF-8 file name: {}", path.display())))?;
    return Ok(name.to_string());
}

/// Speech unicode YAML only — braille `unicode.yaml` stays verbatim.
fn should_minimize(path: &Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if !MINIMIZE_NAMES.contains(&name) {
        return false;
    }
    return path.components().any(|c| c.as_os_str() == "Languages");
}

/// Load YAML, drop comments, re-emit. Returns `Err` if yaml-rust cannot parse the file.
pub fn minimize_yaml_text(text: &str) -> Result<String, String> {
    let normalized = text.replace('\t', "    ");
    let docs = YamlLoader::load_from_str(&normalized).map_err(|e| e.to_string())?;
    if docs.is_empty() {
        return Ok(String::new());
    }
    let mut out = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out);
        emitter.compact(true);
        for doc in &docs {
            emitter.dump(doc).map_err(|e| e.to_string())?;
        }
    }
    if !out.ends_with('\n') {
        out.push('\n');
    }
    // yaml-rust quotes scalars that parse as f64 (including "infinity"); keep a guard in case
    // a future emitter change stops doing that — unquoted `infinity` becomes a float on load.
    if looks_like_unquoted_infinity(&out) {
        return Err("emitted unquoted 'infinity' (yaml-rust would treat it as a float)".to_string());
    }
    return Ok(out);
}

fn looks_like_unquoted_infinity(yaml: &str) -> bool {
    // Match a YAML value `infinity` that is not inside quotes.
    for line in yaml.lines() {
        let trimmed = line.trim();
        if trimmed == "infinity" || trimmed.ends_with(": infinity") || trimmed.ends_with("- infinity") {
            return true;
        }
    }
    return false;
}

fn minimize_yaml_file(src: &Path, dst: &Path) -> Result<(), String> {
    let text = fs::read_to_string(src).map_err(|e| e.to_string())?;
    let minimized = minimize_yaml_text(&text)?;
    // One document after a successful load+emit — MathCAT requires docs.len() == 1.
    let docs = YamlLoader::load_from_str(&minimized).map_err(|e| e.to_string())?;
    if docs.len() != 1 {
        return Err(format!("minimized YAML is {} documents, expected 1", docs.len()));
    }
    let _ = docs;
    fs::write(dst, minimized).map_err(|e| e.to_string())?;
    return Ok(());
}

/// Copy a Rules tree. When `minify` is true, rewrite `Languages/**/unicode.yaml` and
/// `unicode-full.yaml` (comments stripped). `Languages/zz` is omitted. Returns the number of
/// files successfully minimized.
pub fn copy_rules_tree(src: &Path, dst: &Path, minify: bool) -> io::Result<usize> {
    fs::create_dir_all(dst)?;
    return copy_dir(src, dst, minify, false);
}

fn copy_dir(src: &Path, dst: &Path, minify: bool, parent_is_languages: bool) -> io::Result<usize> {
    let mut minimized = 0usize;
    for entry in read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let name = file_name_str(&src_path)?;
        if parent_is_languages && name == SKIP_LANGUAGE_DIR {
            continue;
        }
        if is_zip_file(&src_path) {
            continue;
        }
        let dst_path = dst.join(&name);
        if src_path.is_dir() {
            fs::create_dir_all(&dst_path)?;
            minimized += copy_dir(&src_path, &dst_path, minify, name == "Languages")?;
        } else if minify && should_minimize(&src_path) {
            match minimize_yaml_file(&src_path, &dst_path) {
                Ok(()) => minimized += 1,
                Err(e) => {
                    eprintln!(
                        "warning: could not minimize {}; copied verbatim ({})",
                        src_path.display(),
                        e
                    );
                    fs::copy(&src_path, &dst_path)?;
                }
            }
        } else {
            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&src_path, &dst_path)?;
        }
    }
    return Ok(minimized);
}

fn add_file_to_zip<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    in_path: &Path,
    zip_name: &str,
    options: SimpleFileOptions,
) -> io::Result<()> {
    zip.start_file(zip_name, options)
        .map_err(io::Error::other)?;
    let mut file = File::open(in_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    zip.write_all(&buffer)?;
    return Ok(());
}

fn add_bytes_to_zip<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    zip_name: &str,
    bytes: &[u8],
    options: SimpleFileOptions,
) -> io::Result<()> {
    zip.start_file(zip_name, options)
        .map_err(io::Error::other)?;
    zip.write_all(bytes)?;
    return Ok(());
}

fn zip_entry<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    full_path: &Path,
    relative_path: &str,
    options: SimpleFileOptions,
) -> io::Result<usize> {
    let mut n_files_in_zip = 0;
    if full_path.is_dir() {
        for entry in read_dir(full_path)? {
            let entry_path = entry?.path();
            if is_zip_file(&entry_path) {
                continue;
            }
            let entry_name = file_name_str(&entry_path)?;
            let child_rel = posix_join(relative_path, &entry_name);
            n_files_in_zip += zip_entry(zip, &entry_path, &child_rel, options)?;
        }
    } else if is_yaml_file(full_path) {
        add_file_to_zip(zip, full_path, relative_path, options)?;
        n_files_in_zip += 1;
    }
    return Ok(n_files_in_zip);
}

fn write_inner_zip(source_dir: &Path, inner: SimpleFileOptions) -> io::Result<Option<Vec<u8>>> {
    let cursor = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(cursor);
    let n_files = zip_entry(&mut zip, source_dir, "", inner)?;
    let cursor = zip.finish().map_err(io::Error::other)?;
    if n_files == 0 {
        return Ok(None);
    }
    return Ok(Some(cursor.into_inner()));
}

/// Zip each immediate subdirectory of Languages/ or Braille/ into `<name>/<name>.zip`.
fn zip_dir<W: Write + Seek>(
    rules_section: &Path,
    archive_zip: &mut ZipWriter<W>,
    inner: SimpleFileOptions,
    outer: SimpleFileOptions,
    archive_prefix: &str,
) -> io::Result<()> {
    if !rules_section.is_dir() {
        return Ok(());
    }
    for entry in read_dir(rules_section)? {
        let entry_path = entry?.path();
        if is_zip_file(&entry_path) {
            continue;
        }
        if entry_path.is_dir() {
            let dir_name = file_name_str(&entry_path)?;
            if dir_name == SKIP_LANGUAGE_DIR {
                continue;
            }
            if let Some(bytes) = write_inner_zip(&entry_path, inner)? {
                let zip_name = posix_path(&[archive_prefix, &dir_name, &format!("{}.zip", dir_name)]);
                add_bytes_to_zip(archive_zip, &zip_name, &bytes, outer)?;
            }
        } else if is_yaml_file(&entry_path) {
            // e.g. Rules/Braille/definitions.yaml
            let name = file_name_str(&entry_path)?;
            let zip_name = posix_path(&[archive_prefix, &name]);
            add_file_to_zip(archive_zip, &entry_path, &zip_name, outer)?;
        }
    }
    return Ok(());
}

/// Loose YAML at Rules/ and Rules/Intent/ (not zipped per-directory).
fn zip_other_files<W: Write + Seek>(
    rules_dir: &Path,
    archive_zip: &mut ZipWriter<W>,
    outer: SimpleFileOptions,
    archive_prefix: &str,
) -> io::Result<()> {
    for entry in read_dir(rules_dir)? {
        let entry_path = entry?.path();
        let entry_name = file_name_str(&entry_path)?;
        if entry_path.is_dir() {
            if entry_name == "Intent" {
                zip_other_files(
                    &rules_dir.join("Intent"),
                    archive_zip,
                    outer,
                    &posix_join(archive_prefix, "Intent"),
                )?;
            }
        } else if is_yaml_file(&entry_path) {
            let zip_name = posix_join(archive_prefix, &entry_name);
            add_file_to_zip(archive_zip, &entry_path, &zip_name, outer)?;
        }
    }
    return Ok(());
}

/// Write a MathCAT Rules archive from an already-staged tree (`rules_dir` is the `Rules` folder).
pub fn write_rules_archive(
    rules_dir: &Path,
    output: &Path,
    compression: ArchiveCompression,
) -> io::Result<()> {
    if let Some(parent) = output.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    let inner = zip_options(compression.inner);
    let outer = zip_options(compression.outer);
    let archive_zip_file = File::create(output)?;
    let mut archive_zip = ZipWriter::new(archive_zip_file);

    zip_other_files(rules_dir, &mut archive_zip, outer, ARCHIVE_ROOT)?;
    zip_dir(
        &rules_dir.join("Languages"),
        &mut archive_zip,
        inner,
        outer,
        &posix_join(ARCHIVE_ROOT, "Languages"),
    )?;
    zip_dir(
        &rules_dir.join("Braille"),
        &mut archive_zip,
        inner,
        outer,
        &posix_join(ARCHIVE_ROOT, "Braille"),
    )?;

    archive_zip.finish().map_err(io::Error::other)?;
    return Ok(());
}

/// Copy (optionally minify) `source` and write a downloadable or embeddable Rules archive.
pub fn package_rules(
    source: &Path,
    output: &Path,
    minify: bool,
    compression: ArchiveCompression,
) -> io::Result<usize> {
    if !source.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Rules source directory not found: {}", source.display()),
        ));
    }

    let staging_root = tempfile_staging_dir()?;
    let staged_rules = staging_root.join(ARCHIVE_ROOT);
    let minimized = copy_rules_tree(source, &staged_rules, minify)?;
    write_rules_archive(&staged_rules, output, compression)?;
    let _ = fs::remove_dir_all(&staging_root);
    return Ok(minimized);
}

fn tempfile_staging_dir() -> io::Result<PathBuf> {
    let base = std::env::temp_dir().join(format!(
        "mathcat-rules-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&base)?;
    return Ok(base);
}
