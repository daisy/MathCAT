//! Regional rule selection and isolation, including packaged rules.

use crate::common::*;
use anyhow::{ensure, Result};
use libmathcat::speech::{RulesFor, SPEECH_RULES};
use libmathcat::shim_filesystem::find_files_in_dir_that_ends_with_shim;
use std::path::Path;

fn check_regional_files(region: &str, style: &str) -> Result<()> {
    set_mathml("<math><msqrt><mi>x</mi></msqrt></math>")?;
    get_spoken_text()?;
    return SPEECH_RULES.with(|rules| {
        let rules = rules.borrow();
        let prefs = rules.pref_manager.borrow();
        let directory = Path::new("Languages/zh").join(region);
        ensure!(prefs.get_rule_file(&RulesFor::Speech).ends_with(directory.join(format!("{style}_Rules.yaml"))),
            "Expected {region}/{style}, got {}", prefs.get_rule_file(&RulesFor::Speech).display());
        ensure!(prefs.get_rule_file(&RulesFor::Navigation).ends_with(directory.join("navigate.yaml")));
        ensure!(prefs.get_rule_file(&RulesFor::OverView).ends_with(directory.join("overview.yaml")));
        ensure!(prefs.get_definitions_file(true).ends_with(directory.join("definitions.yaml")));
        let (unicode, unicode_full) = prefs.get_speech_unicode_file();
        ensure!(unicode.ends_with(directory.join("unicode.yaml")));
        ensure!(unicode_full.ends_with(directory.join("unicode-full.yaml")));
        return Ok(());
    });
}

#[test]
fn simplified_chinese_loads_all_files_from_cn() -> Result<()> {
    // Both speech styles must use the cn definitions, symbols, overview, and navigation.
    set_rules_dir(abs_rules_dir_path())?;
    set_preference("Language", "zh-cn")?;
    for style in ["SimpleSpeak", "ClearSpeak"] {
        set_preference("SpeechStyle", style)?;
        check_regional_files("cn", style)?;
    }
    return Ok(());
}

#[test]
fn switching_regions_keeps_traditional_chinese_isolated() -> Result<()> {
    // Taiwan's missing ClearSpeak style must fall back to its own SimpleSpeak,
    // and switching back to cn must reload Simplified Chinese files.
    set_rules_dir(abs_rules_dir_path())?;
    set_preference("Language", "zh-cn")?;
    set_preference("SpeechStyle", "ClearSpeak")?;
    check_regional_files("cn", "ClearSpeak")?;
    set_preference("Language", "zh-tw")?;
    check_regional_files("tw", "SimpleSpeak")?;
    set_preference("Language", "zh-cn")?;
    set_preference("SpeechStyle", "ClearSpeak")?;
    check_regional_files("cn", "ClearSpeak")?;
    return Ok(());
}

#[test]
fn regional_styles_are_not_listed_as_base_chinese_styles() -> Result<()> {
    // Recursive listings would let packaged-rule fallback select a sibling
    // region's style instead of searching the requested region first.
    set_rules_dir(abs_rules_dir_path())?;
    set_preference("Language", "zh-cn")?;
    let directory = Path::new(&abs_rules_dir_path()).join("Languages/zh");
    let styles = find_files_in_dir_that_ends_with_shim(&directory, "_Rules.yaml");
    ensure!(styles.is_empty(), "Base zh unexpectedly lists regional styles: {styles:?}");
    let mut styles = find_files_in_dir_that_ends_with_shim(&directory.join("cn"), "_Rules.yaml");
    styles.sort();
    ensure!(styles == ["ClearSpeak_Rules.yaml", "SimpleSpeak_Rules.yaml"]);
    return Ok(());
}
