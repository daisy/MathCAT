#![allow(non_snake_case)]

mod general;
mod calculus;
mod linear_algebra;
mod tables;
// Reviewed core concept speech, shared across language and verbosity variants.
fn test_core_speech(mathml: &str, english: &str, russian: &str) -> anyhow::Result<()> {
    for (language, expected) in [("en", english), ("ru", russian)] {
        for style in ["ClearSpeak", "SimpleSpeak"] {
            for verbosity in ["Terse", "Medium", "Verbose"] {
                crate::common::test_prefs(language, style, vec![("Verbosity", verbosity), ("IntentErrorRecovery", "Error")], mathml, expected)?;
            }
        }
    }
    Ok(())
}

