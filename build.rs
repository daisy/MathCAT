//! The build.rs file is necessary to generate rules.zip.
//! rules.zip are needed so there is a way to get the rules dir into the build since you can't get from the crate.
//! The expectation is that most builds (with the exception of WASM builds) will need a build.rs file to extract the rules.
#![allow(clippy::needless_return)]
#![allow(dead_code)] // include! of rules_archive.rs pulls in the downloadable packager too

use std::collections::HashSet;

include!("src/rules_archive.rs");

/// Shared `src/parser/*.rs` modules that are not notation parsers.
const PARSER_SHARED_MODULES: &[&str] = &["expr", "mod", "symbols"];
/// `from-braille-*` Cargo features that are not notation parsers.
const FROM_BRAILLE_INTERNAL: &[&str] = &["impl"];

/// Body of the `[features]` section in `Cargo.toml`.
fn features_section(manifest: &str) -> &str {
    return manifest
        .split("\n[features]")
        .nth(1)
        .and_then(|rest| rest.split("\n[").next())
        .unwrap_or_else(|| panic!("Cargo.toml has no [features] section"));
}

/// Members of `feature = [ "a", "b", ... ]` (multi-line lists allowed after `cargo publish`).
fn feature_list_members(features: &str, feature: &str) -> Vec<String> {
    // Ensure a leading newline so the first feature in the section matches the same pattern.
    let marked = format!("\n{features}");
    let unquoted = format!("\n{feature} = [");
    let quoted = format!("\n\"{feature}\" = [");
    let after = marked
        .split(&unquoted)
        .nth(1)
        .or_else(|| marked.split(&quoted).nth(1))
        .unwrap_or_else(|| panic!("Cargo.toml has no `{feature}` feature list"));
    let list_body = after
        .split(']')
        .next()
        .unwrap_or_else(|| panic!("Cargo.toml `{feature}` feature list is not closed with `]`"));
    return list_body
        .split(',')
        .filter_map(|member| {
            let member = member.trim().trim_matches('"').trim();
            if member.is_empty() {
                return None;
            }
            return Some(member.to_string());
        })
        .collect();
}

/// Suffixes of feature keys that start with `prefix` (e.g. `from-braille-` → `ueb`).
fn feature_key_suffixes(features: &str, prefix: &str) -> Vec<String> {
    let mut suffixes = Vec::new();
    for line in features.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, _)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim().trim_matches('"');
        if let Some(suffix) = key.strip_prefix(prefix) &&
            !suffix.is_empty()
        {
            suffixes.push(suffix.to_string());
        }
    }
    suffixes.sort();
    return suffixes;
}

/// The `Rules/<kind>` directories whose `<prefix>-<name>` feature is on, or `None` when all of them
/// are, so a default build stages exactly what it always has.
///
/// Panics if a directory is missing from `<prefix>-all` in Cargo.toml. Otherwise a newly added
/// language or braille code would quietly drop out of every build, since nothing would turn it on.
fn subset_from_features(rules_dir: &Path, kind: &str, prefix: &str, manifest: &str) -> Option<Vec<String>> {
    let all_feature = format!("{prefix}-all");
    let all_members: HashSet<String> = feature_list_members(features_section(manifest), &all_feature)
        .into_iter()
        .collect();
    let mut kept = Vec::new();
    let mut everything = true;
    for entry in read_dir(rules_dir.join(kind)).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !path.is_dir() || (kind == LANGUAGES_DIR && name == SKIP_LANGUAGE_DIR) {
            continue;
        }
        let feature = format!("{prefix}-{}", name.to_ascii_lowercase());
        if !all_members.contains(&feature) {
            panic!(
                "Rules/{kind}/{name} has no feature: add `{feature} = []` to Cargo.toml and list it in `{all_feature}`"
            );
        }
        let env_var = format!("CARGO_FEATURE_{}", feature.to_ascii_uppercase().replace('-', "_"));
        if std::env::var_os(env_var).is_some() {
            kept.push(name);
        } else {
            everything = false;
        }
    }
    kept.sort();
    return if everything { None } else { Some(kept) };
}

/// Keep the opt-in from-braille feature list aligned with the parser modules.
fn check_from_braille_features(parser_dir: &Path, manifest: &str) {
    let mut parser_names = Vec::new();
    for entry in read_dir(parser_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
            continue;
        }
        let name = path.file_stem().unwrap().to_string_lossy().to_ascii_lowercase();
        if PARSER_SHARED_MODULES.contains(&name.as_str()) {
            continue;
        }
        parser_names.push(name);
    }
    parser_names.sort();

    let features = features_section(manifest);
    let mut declared_names: Vec<String> = feature_key_suffixes(features, "from-braille-")
        .into_iter()
        .filter(|suffix| !FROM_BRAILLE_INTERNAL.contains(&suffix.as_str()))
        .collect();
    declared_names.sort();

    let mut aggregate_members: Vec<String> = feature_list_members(features, "from-braille")
        .into_iter()
        .filter_map(|member| member.strip_prefix("from-braille-").map(str::to_string))
        .collect();
    aggregate_members.sort();

    if parser_names != declared_names || parser_names != aggregate_members {
        panic!(
            "from-braille features do not match src/parser/*.rs notations.\n\
             parsers on disk: {parser_names:?}\n\
             declared from-braille-* features: {declared_names:?}\n\
             from-braille = [...]: {aggregate_members:?}\n\
             Add `from-braille-<name> = [\"from-braille-impl\"]` and list `from-braille-<name>` in `from-braille`."
        );
    }
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/rules_archive.rs");
    println!("cargo::rerun-if-changed=src/parser");
    println!("cargo::rerun-if-changed=Rules");
    println!("cargo::rerun-if-changed=Cargo.toml");

    let rules_dir = std::env::current_dir().unwrap().join("Rules");
    let manifest = fs::read_to_string("Cargo.toml").unwrap();
    check_from_braille_features(&PathBuf::from("src/parser"), &manifest);
    // Checked on every build, not only include-zip ones, so a rules directory added without a
    // feature is caught by whoever adds it.
    let languages = subset_from_features(&rules_dir, LANGUAGES_DIR, "lang", &manifest);
    let braille_codes = subset_from_features(&rules_dir, BRAILLE_DIR, "braille", &manifest);

    // This doesn't work because the build claims OUT_DIR is not defined(?)
    // let archive = PathBuf::from(concat!(env!("OUT_DIR"),"/rules.zip"));
    if std::env::var("CARGO_FEATURE_INCLUDE_ZIP").is_ok() {
        let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
        let staging = out_dir.join("rules_src");
        if staging.exists() {
            let _ = std::fs::remove_dir_all(&staging);
        }
        let subset = RulesSubset { languages, braille_codes };
        if let Some(langs) = &subset.languages {
            let mut langs = langs.clone();
            if !langs.iter().any(|l| l.eq_ignore_ascii_case("en")) {
                langs.push("en (the fallback)".to_string());
            }
            println!("cargo::warning=MathCAT rules limited to languages: {}", langs.join(", "));
        }
        if let Some(codes) = &subset.braille_codes {
            let codes = if codes.is_empty() { "UEB (the fallback)".to_string() } else { codes.join(", ") };
            println!("cargo::warning=MathCAT rules limited to braille codes: {}", codes);
        }
        if let Err(e) = copy_rules_subset(&rules_dir, &staging, true, &subset) {
            panic!("build.rs failed to stage minimized Rules: {}", e);
        }
        let archive_path = out_dir.join("rules.zip");
        if let Err(e) = write_rules_archive(&staging, &archive_path, include_zip_compression()) {
            panic!("build.rs failed to write {}: {}", archive_path.display(), e);
        }
    }
}
