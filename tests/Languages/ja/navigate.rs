//! What the Japanese navigation commands announce.
//!
//! These cover the command prefix only (the part say-command produces); the
//! description that follows it comes from NavigationParts and is asserted
//! elsewhere, so those checks compare everything up to the first pause.
//! The announcements that are not a command prefix -- moving into a notation,
//! undo, the placemarkers -- are compared whole.

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

fn assert_speech(mathml: &str, commands: &[&str], expected: &str) -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_nav(mathml)?;
        let mut speech = String::new();
        for command in commands {
            speech = do_navigate_command(command)?;
        }
        assert_eq!(speech.trim(), expected);
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

/// Moving into or out of a 2D notation spoke the English literal that $Move2D
/// carries -- "in", "out of", "end of", "start of" -- because the rule was
/// copied from en, where that variable is already English. Japanese also wants
/// it after the part it applies to, not in front of it.
#[test]
fn moving_into_a_notation_is_japanese() -> Result<()> {
    assert_speech(EXPR, &["ZoomIn", "ZoomIn"], "ズーム イン; 底 に入る; x")
}

/// Undo listed the verb first (元に戻す ズームイン = "undo zoom in"), which is the
/// English order; Japanese names what is undone and then 元に戻す.
#[test]
fn undo_names_what_is_undone_first() -> Result<()> {
    assert_speech(EXPR, &["ZoomIn", "MoveLastLocation"], "ズームイン を 元に戻す; x の 2 乗 プラス 1")
}

/// The placemarker announcements had the same shape: 読み上げ プレースホルダー 3
/// is "read placeholder 3" word for word. The particle each verb takes is the
/// one the move commands already use.
#[test]
fn placemarker_names_the_placeholder_first() -> Result<()> {
    assert_speech(EXPR, &["SetPlacemarker3"], "プレースホルダー 3 を 設定; x の 2 乗 プラス 1")
}

#[test]
fn reading_a_placemarker_names_it_first() -> Result<()> {
    assert_speech(EXPR, &["SetPlacemarker3", "Read3"], "プレースホルダー 3 を 読み上げ; x の 2 乗 プラス 1")
}
