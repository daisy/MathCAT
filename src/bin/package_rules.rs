//! Package `Rules/` into `Rules.zip` or `Rules-minimized.zip`.
//!
//! Usage:
//!   cargo run --bin package-rules -- Rules Rules.zip
//!   cargo run --bin package-rules -- Rules Rules-minimized.zip --minimize

#![allow(clippy::needless_return)]

include!("../rules_archive.rs");

use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "package-rules",
    about = "Package MathCAT Rules/ into a zip archive. \
             Use --minimize to strip comments from Languages unicode YAML."
)]
struct Options {
    /// Path to the Rules directory
    source: PathBuf,
    /// Output zip file path (e.g. Rules.zip)
    output: PathBuf,
    /// Emit compact unicode.yaml / unicode-full.yaml without comments (speech languages only)
    #[arg(long)]
    minimize: bool,
}

fn main() -> ExitCode {
    let cli = Options::parse();
    if cli.source.file_name().and_then(|n| n.to_str()) != Some("Rules") {
        eprintln!(
            "error: expected a directory named 'Rules', got '{}'",
            cli.source.display()
        );
        return ExitCode::from(1);
    }

    match package_rules(
        &cli.source,
        &cli.output,
        cli.minimize,
        downloadable_compression(),
    ) {
        Ok(minimized) => {
            let label = if cli.minimize { "minimized" } else { "standard" };
            println!("Created {} archive: {}", label, cli.output.display());
            if cli.minimize {
                println!("  minimized YAML files: {}", minimized);
            }
            return ExitCode::SUCCESS;
        }
        Err(e) => {
            eprintln!("error: {}", e);
            return ExitCode::from(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{Cursor, Read};
    use tempfile::tempdir;
    use zip::ZipArchive;

    fn read_nested_zip_text(outer: &[u8], outer_path: &str, inner_path: &str) -> String {
        let mut archive = ZipArchive::new(Cursor::new(outer)).unwrap();
        let mut inner_bytes = Vec::new();
        archive
            .by_name(outer_path)
            .unwrap()
            .read_to_end(&mut inner_bytes)
            .unwrap();
        let mut inner = ZipArchive::new(Cursor::new(inner_bytes)).unwrap();
        let mut text = String::new();
        inner
            .by_name(inner_path)
            .unwrap()
            .read_to_string(&mut text)
            .unwrap();
        return text;
    }

    fn outer_names(bytes: &[u8]) -> Vec<String> {
        let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
        let mut names: Vec<String> = (0..archive.len())
            .map(|i| archive.by_index(i).unwrap().name().replace('\\', "/"))
            .collect();
        names.sort();
        return names;
    }

    fn inner_method(bytes: &[u8], outer_path: &str) -> zip::CompressionMethod {
        let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
        let mut inner_bytes = Vec::new();
        archive
            .by_name(outer_path)
            .unwrap()
            .read_to_end(&mut inner_bytes)
            .unwrap();
        let mut inner = ZipArchive::new(Cursor::new(inner_bytes)).unwrap();
        return inner.by_index(0).unwrap().compression();
    }

    fn outer_method(bytes: &[u8], name: &str) -> zip::CompressionMethod {
        let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
        return archive.by_name(name).unwrap().compression();
    }

    #[test]
    fn minimize_yaml_strips_comments() {
        let source = "---\n# comment\n - \"+\": [t: \"plus\"]  # inline\n";
        let minimized = minimize_yaml_text(source).unwrap();
        assert!(!minimized.contains('#'));
        let docs = yaml_rust::YamlLoader::load_from_str(&minimized).unwrap();
        assert_eq!(docs.len(), 1);
    }

    #[test]
    fn minimize_quotes_infinity() {
        let source = "---\n - \"∞\": [t: \"infinity\"]\n";
        let minimized = minimize_yaml_text(source).unwrap();
        assert!(
            minimized.contains("\"infinity\"") || minimized.contains("'infinity'"),
            "infinity must be quoted, got: {minimized}"
        );
        assert!(!looks_like_unquoted_infinity(&minimized));
        let docs = yaml_rust::YamlLoader::load_from_str(&minimized).unwrap();
        assert_eq!(docs.len(), 1);
    }

    #[test]
    fn package_rules_standard_zip_layout() {
        let tmp = tempdir().unwrap();
        let rules = tmp.path().join("Rules");
        let lang = rules.join("Languages").join("en");
        fs::create_dir_all(&lang).unwrap();
        fs::write(rules.join("prefs.yaml"), "- SpeechStyle: [t: \"ClearSpeak\"]\n").unwrap();
        fs::write(lang.join("unicode.yaml"), "- \"+\": [t: \"plus\"]\n").unwrap();

        let output = tmp.path().join("Rules.zip");
        package_rules(&rules, &output, false, downloadable_compression()).unwrap();

        let bytes = fs::read(&output).unwrap();
        let names = outer_names(&bytes);
        assert!(names.contains(&"Rules/prefs.yaml".to_string()));
        assert!(names.contains(&"Rules/Languages/en/en.zip".to_string()));
        assert!(!names.iter().any(|n| n.ends_with("unicode.yaml")));
        assert_eq!(
            inner_method(&bytes, "Rules/Languages/en/en.zip"),
            zip::CompressionMethod::BZIP2
        );
        assert_eq!(
            outer_method(&bytes, "Rules/prefs.yaml"),
            zip::CompressionMethod::DEFLATE
        );
        let unicode = read_nested_zip_text(&bytes, "Rules/Languages/en/en.zip", "unicode.yaml");
        assert_eq!(unicode.replace("\r\n", "\n"), "- \"+\": [t: \"plus\"]\n");
    }

    #[test]
    fn should_not_minimize_braille_unicode() {
        let tmp = tempdir().unwrap();
        let rules = tmp.path().join("Rules");
        let braille = rules.join("Braille").join("Nemeth");
        fs::create_dir_all(&braille).unwrap();
        let source = "---\n# keep this comment\n - \"1\": [t: \"N⠂\"]\n";
        fs::write(braille.join("unicode.yaml"), source).unwrap();

        let output = tmp.path().join("Rules-minimized.zip");
        let minimized = package_rules(&rules, &output, true, downloadable_compression()).unwrap();
        assert_eq!(minimized, 0);

        let bytes = fs::read(&output).unwrap();
        let text = read_nested_zip_text(&bytes, "Rules/Braille/Nemeth/Nemeth.zip", "unicode.yaml");
        assert_eq!(text.replace("\r\n", "\n"), source.replace("\r\n", "\n"));
    }

    #[test]
    fn package_rules_minimized_matches_release_layout() {
        let tmp = tempdir().unwrap();
        let rules = tmp.path().join("Rules");
        let lang = rules.join("Languages").join("en");
        fs::create_dir_all(&lang).unwrap();
        fs::write(
            lang.join("unicode.yaml"),
            "---\n# header comment\n - \"+\": [t: \"plus\"]  # trailing\n - \"-\": [t: \"minus\"]\n",
        )
        .unwrap();
        fs::write(
            lang.join("definitions.yaml"),
            "---\n - AdditionalFunctionNames:\n    - real=inf\n",
        )
        .unwrap();

        let output = tmp.path().join("Rules-minimized.zip");
        let minimized = package_rules(&rules, &output, true, downloadable_compression()).unwrap();
        assert_eq!(minimized, 1);

        let bytes = fs::read(&output).unwrap();
        let names = outer_names(&bytes);
        assert!(names.contains(&"Rules/Languages/en/en.zip".to_string()));
        assert!(!names.iter().any(|n| n.ends_with("unicode.yaml")));

        let unicode = read_nested_zip_text(&bytes, "Rules/Languages/en/en.zip", "unicode.yaml");
        let defs = read_nested_zip_text(&bytes, "Rules/Languages/en/en.zip", "definitions.yaml");
        assert!(!unicode.contains('#'));
        assert!(defs.contains("real=inf"));
        assert_eq!(yaml_rust::YamlLoader::load_from_str(&unicode).unwrap().len(), 1);
    }

    #[test]
    fn skips_zz_test_language() {
        let tmp = tempdir().unwrap();
        let rules = tmp.path().join("Rules");
        fs::create_dir_all(rules.join("Languages").join("zz")).unwrap();
        fs::write(rules.join("Languages").join("zz").join("unicode.yaml"), "- a: 1\n").unwrap();
        fs::create_dir_all(rules.join("Languages").join("en")).unwrap();
        fs::write(rules.join("Languages").join("en").join("unicode.yaml"), "- b: 2\n").unwrap();

        let output = tmp.path().join("Rules.zip");
        package_rules(&rules, &output, false, downloadable_compression()).unwrap();
        let names = outer_names(&fs::read(&output).unwrap());
        assert!(names.contains(&"Rules/Languages/en/en.zip".to_string()));
        assert!(!names.iter().any(|n| n.contains("/zz")));
    }
}
