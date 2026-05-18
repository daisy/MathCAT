mod common;

use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn mathml2text_command() -> Command { Command::new(env!("CARGO_BIN_EXE_mathml2text")) }

fn rules_dir() -> String { common::abs_rules_dir_path() }

fn unique_temp_file(name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after UNIX_EPOCH")
        .as_nanos();
    std::env::temp_dir().join(format!("mathml2text-{name}-{timestamp}.mml"))
}

/// Verifies that `mathml2text` uses an explicitly provided `--rules-dir`.
/// This protects the CLI override path instead of silently falling back to the default rules location.
#[test]
fn accepts_explicit_rules_dir() {
    let input_file = unique_temp_file("explicit-rules");
    std::fs::write(
        &input_file,
        "<math><mn>4</mn></math>",
    )
    .expect("should write temp input file");

    let output = mathml2text_command()
        .args([
            "--rules-dir",
            rules_dir().as_str(),
        ])
        .arg(&input_file)
        .output()
        .expect("mathml2text should run");

    let _ = std::fs::remove_file(&input_file);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!("4\n", String::from_utf8_lossy(&output.stdout));
}

/// Verifies that the positional input-file argument still works on its own.
/// This keeps the default CLI file-input path covered while testing the rules-dir changes separately.
#[test]
fn still_accepts_input_file() {
    let input_file = unique_temp_file("input-file");
    std::fs::write(
        &input_file,
        "<math><mn>2</mn></math>",
    )
    .expect("should write temp input file");

    let output = mathml2text_command()
        .arg(&input_file)
        .output()
        .expect("mathml2text should run");

    let _ = std::fs::remove_file(&input_file);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!("2\n", String::from_utf8_lossy(&output.stdout));
}

/// Verifies that an invalid explicit `--rules-dir` causes the CLI to fail.
/// This protects against regressions where the flag is parsed but then ignored at runtime.
#[test]
fn rejects_invalid_explicit_rules_dir() {
    let missing_rules_dir = std::env::temp_dir().join(format!(
        "mathml2text-missing-rules-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after UNIX_EPOCH")
            .as_nanos()
    ));
    let input_file = unique_temp_file("invalid-rules");
    std::fs::write(
        &input_file,
        "<math><mn>5</mn></math>",
    )
    .expect("should write temp input file");

    let output = mathml2text_command()
        .args([
            "--rules-dir",
            missing_rules_dir.to_str().expect("temp path should be valid UTF-8"),
        ])
        .arg(&input_file)
        .output()
        .expect("mathml2text should run");

    let _ = std::fs::remove_file(&input_file);

    assert!(!output.status.success(), "stdout: {}", String::from_utf8_lossy(&output.stdout));
}
