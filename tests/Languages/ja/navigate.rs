//! What the Japanese navigation commands announce.
//!
//! These cover the command prefix only (the part say-command produces); the
//! description that follows it comes from NavigationParts and is asserted
//! elsewhere, so each check compares everything up to the first pause.

use crate::common::*;
use anyhow::Result;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn init_nav(mathml: &str) -> Result<()> {
    set_rules_dir(abs_rules_dir_path())?;
    set_preference("Language", "ja")?;
    set_preference("SpeechStyle", "SimpleSpeak")?;
    set_preference("Verbosity", "Medium")?;
    set_preference("NavMode", "Enhanced")?;
    set_preference("NavVerbosity", "Verbose")?;
    set_preference("AutoZoomOut", "False")?;
    set_preference("Overview", "False")?;
    set_mathml(mathml)?;
    Ok(())
}

fn assert_command_prefix(mathml: &str, commands: &[&str], expected: &str) -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_nav(mathml)?;
        let mut speech = String::new();
        for command in commands {
            speech = do_navigate_command(command)?;
        }
        let prefix = speech.split(';').next().unwrap_or("").trim().to_string();
        assert_eq!(prefix, expected, "full speech was {speech:?}");
        Ok(())
    }));
    report_any_panic(result)
}

const EXPR: &str = r#"<math><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>1</mn></mrow></math>"#;

/// The prefix used to be the English word "zoom", spoken as-is by a Japanese
/// synthesiser. ズーム + イン also reads as the ordinary loanword.
#[test]
fn zoom_prefix_is_japanese() -> Result<()> {
    assert_command_prefix(EXPR, &["ZoomIn"], "ズーム イン")
}

#[test]
fn zoom_out_prefix_is_japanese() -> Result<()> {
    assert_command_prefix(EXPR, &["ZoomIn", "ZoomOut"], "ズーム アウト")
}

/// Japanese puts the target before the verb, with the particle the verb takes:
/// 右 に 移動, not 移動 右.
#[test]
fn move_says_target_before_verb() -> Result<()> {
    assert_command_prefix(EXPR, &["ZoomIn", "MoveNext"], "右 に 移動")
}

/// Read and describe take を, not に, on the same direction word.
#[test]
fn read_takes_its_own_particle() -> Result<()> {
    assert_command_prefix(EXPR, &["ZoomIn", "ReadNext"], "右 を 読み上げ")
}

/// The U+F8FE concatenation joins the suffix onto the prefix with no space,
/// so this has to come out as one word.
#[test]
fn zoom_in_all_is_one_phrase() -> Result<()> {
    assert_command_prefix(EXPR, &["ZoomInAll"], "ズームインを最大にしました")
}

/// ReadCurrent is announced by its own rule, which has to use the same order.
#[test]
fn read_current_says_target_first() -> Result<()> {
    assert_command_prefix(EXPR, &["ZoomIn", "ReadCurrent"], "現在 を読み上げ")
}
