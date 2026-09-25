//! The build.rs file is necessary to generate rules.zip.
//! rules.zip are needed so there is a way to get the rules dir into the build since you can't get from the crate.
//! The expectation is that most builds (with the exception of WASM builds) will need a build.rs file to extract the rules.
#![allow(clippy::needless_return)]
#![allow(dead_code)] // include! of rules_archive.rs pulls in the downloadable packager too

include!("src/rules_archive.rs");

/// A comma separated list from the environment, or `None` for "keep everything".
fn subset_from_env(var: &str) -> Option<Vec<String>> {
    println!("cargo::rerun-if-env-changed={}", var);
    let value = std::env::var(var).ok()?;
    let names: Vec<String> = value.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    return if names.is_empty() { None } else { Some(names) };
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/rules_archive.rs");
    println!("cargo::rerun-if-changed=Rules");

    // This doesn't work because the build claims OUT_DIR is not defined(?)
    // let archive = PathBuf::from(concat!(env!("OUT_DIR"),"/rules.zip"));
    if std::env::var("CARGO_FEATURE_INCLUDE_ZIP").is_ok() {
        let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
        let rules_dir = std::env::current_dir().unwrap().join("Rules");
        let staging = out_dir.join("rules_src");
        if staging.exists() {
            let _ = std::fs::remove_dir_all(&staging);
        }
        // A caller that asks for one language and one braille code should not carry the other
        // fifteen and the other nine. Unset means keep everything, as before.
        let subset = RulesSubset {
            languages: subset_from_env("MATHCAT_INCLUDE_LANGUAGES"),
            braille_codes: subset_from_env("MATHCAT_INCLUDE_BRAILLE"),
        };
        if let Some(codes) = &subset.braille_codes {
            println!("cargo::warning=MathCAT rules limited to braille codes: {}", codes.join(", "));
        }
        if let Some(langs) = &subset.languages {
            let mut langs = langs.clone();
            if !langs.iter().any(|l| l.eq_ignore_ascii_case("en")) {
                langs.push("en (the fallback)".to_string());
            }
            println!("cargo::warning=MathCAT rules limited to languages: {}", langs.join(", "));
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
