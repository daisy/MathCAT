//! Regression tests for named HTML entities in `set_mathml()` (issue #896).
//!
//! `set_mathml()` resolves named entities before handing the string to the XML
//! parser. The regex used to bound the name at 10 letters, so the 2127 names in
//! `src/entities.in` that are longer (up to `CounterClockwiseContourIntegral`,
//! 31 letters) were never substituted, reached the parser unresolved and made
//! `set_mathml()` fail outright.

#![allow(non_snake_case)]
#![allow(clippy::needless_return)]

mod common;

use libmathcat::interface::*;

fn init() {
    init_panic_handler();
    set_rules_dir(common::abs_rules_dir_path()).unwrap();
}

/// Panics with MathCAT's own error message if `mathml` is rejected. That is what
/// an unresolved entity used to cause.
fn parse_ok(mathml: &str) {
    if let Err(e) = set_mathml(mathml) {
        panic!("set_mathml() rejected {mathml}: {}", errors_to_string(&e));
    }
}

/// Speech for `mathml`, which additionally requires a successful parse.
fn spoken_text(mathml: &str) -> String {
    parse_ok(mathml);
    match get_spoken_text() {
        Ok(speech) => speech,
        Err(e) => panic!("get_spoken_text() failed for {mathml}: {}", errors_to_string(&e)),
    }
}

#[test]
fn entity_longer_than_ten_letters() {
    init();
    // 14 letters: the case reported by the JAWS team in issue #896.
    let speech = spoken_text("<math><mi>x</mi><mo>&InvisibleTimes;</mo><mi>y</mi></math>");
    assert!(!speech.trim().is_empty(), "speech was empty");
}

#[test]
fn entity_one_letter_over_the_old_limit() {
    init();
    // 11 letters: the shortest name the old `{2,10}` bound rejected.
    let speech = spoken_text("<math><mo>&CircleTimes;</mo></math>");
    assert!(!speech.trim().is_empty(), "speech was empty");
}

#[test]
fn longest_entity_name() {
    init();
    // 31 letters: the longest name in src/entities.in.
    let speech = spoken_text("<math><mo>&CounterClockwiseContourIntegral;</mo></math>");
    assert!(!speech.trim().is_empty(), "speech was empty");
}

#[test]
fn short_entity_names_still_work() {
    init();
    // Regression: names the old bound never affected must keep being resolved.
    // `it` and `af` expand to invisible operators and are spoken as nothing, so
    // only the parse is asserted here.
    for mathml in [
        "<math><mi>&it;</mi></math>",                     // 2 letters
        "<math><mi>&af;</mi></math>",                     // 2 letters
        "<math><mi>&pi;</mi></math>",                     // 2 letters
        "<math><mo>&RightArrow;</mo></math>",             // 10 letters: old limit
        "<math><mi>&ImaginaryI;</mi></math>",             // 10 letters: old limit
    ] {
        parse_ok(mathml);
    }
    assert!(!spoken_text("<math><mi>&pi;</mi></math>").trim().is_empty());
    assert!(!spoken_text("<math><mo>&RightArrow;</mo></math>").trim().is_empty());
}

#[test]
fn numeric_character_reference_is_unaffected() {
    init();
    let speech = spoken_text("<math><mi>x</mi><mo>&#x2062;</mo><mi>y</mi></math>");
    assert!(!speech.trim().is_empty(), "speech was empty");
}

#[test]
fn unknown_entity_name_is_still_rejected() {
    init();
    // Widening the bound must not turn unknown names into text that silently
    // reaches the parser.
    assert!(
        set_mathml("<math><mi>&NotAnEntityNameAtAll;</mi></math>").is_err(),
        "an unknown entity name must still be an error"
    );
}
