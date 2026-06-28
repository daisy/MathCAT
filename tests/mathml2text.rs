mod common;

use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

struct Mathml2TextFixture {
    input_file: PathBuf,
}

impl Mathml2TextFixture {
    fn new(name: &str, mathml: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after UNIX_EPOCH")
            .as_nanos();
        let input_file = std::env::temp_dir().join(format!("mathml2text-{name}-{timestamp}.mml"));
        std::fs::write(&input_file, mathml).expect("should write temp input file");
        Self { input_file }
    }

    fn run_with_rules_dir(&self, rules_dir: impl AsRef<OsStr>) -> Output {
        Command::new(env!("CARGO_BIN_EXE_mathml2text"))
            .arg("--rules-dir")
            .arg(rules_dir)
            .arg(&self.input_file)
            .output()
            .expect("mathml2text should run")
    }

    fn missing_rules_dir(&self) -> PathBuf {
        self.input_file.with_extension("missing-rules")
    }
}

impl Drop for Mathml2TextFixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.input_file);
    }
}

/// Verifies that `mathml2text` uses an explicitly provided `--rules-dir`.
/// This protects the CLI override path instead of silently falling back to the default rules location.
#[test]
fn accepts_explicit_rules_dir() {
    let fixture = Mathml2TextFixture::new("explicit-rules", "<math><mn>4</mn></math>");
    let output = fixture.run_with_rules_dir(common::abs_rules_dir_path());

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!("4\n", String::from_utf8_lossy(&output.stdout));
}

/// Verifies that an invalid explicit `--rules-dir` causes the CLI to fail.
/// This protects against regressions where the flag is parsed but then ignored at runtime.
#[test]
fn rejects_invalid_explicit_rules_dir() {
    let fixture = Mathml2TextFixture::new("invalid-rules", "<math><mn>5</mn></math>");
    let output = fixture.run_with_rules_dir(fixture.missing_rules_dir());

    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}
