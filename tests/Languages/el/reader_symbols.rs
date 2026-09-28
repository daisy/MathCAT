//! Latin letters use Greek names only in GreekMathReader; other styles retain their conventions.
use crate::common::*;
use anyhow::Result;

#[test]
fn greek_math_reader_speaks_all_latin_letters_phonetically() -> Result<()> {
    let letters = [
        ("a", "έι"), ("b", "μπί"), ("c", "σί"), ("d", "ντί"), ("e", "ί"),
        ("f", "εφ"), ("g", "τζί"), ("h", "έιτς"), ("i", "άι"), ("j", "τζέι"),
        ("k", "κάπα"), ("l", "έλ"), ("m", "μι"), ("n", "νι"), ("o", "όμικρον"),
        ("p", "πι"), ("q", "κιού"), ("r", "άρ"), ("s", "ες"), ("t", "ταυ"),
        ("u", "γιού"), ("v", "βί"), ("w", "ντάμπλιου"), ("x", "χι"),
        ("y", "γουάι"), ("z", "ζήτα"),
    ];
    for (letter, name) in letters {
        let expr = format!("<math><mi>{letter}</mi></math>");
        test("el", "GreekMathReader", &expr, name)?;
    }
    Ok(())
}

#[test]
fn greek_math_reader_capitals_keep_capital_word_preferences() -> Result<()> {
    let letters = [
        ("A", "έι"), ("B", "μπί"), ("C", "σί"), ("D", "ντί"), ("E", "ί"),
        ("F", "εφ"), ("G", "τζί"), ("H", "έιτς"), ("I", "άι"), ("J", "τζέι"),
        ("K", "κάπα"), ("L", "έλ"), ("M", "μι"), ("N", "νι"), ("O", "όμικρον"),
        ("P", "πι"), ("Q", "κιού"), ("R", "άρ"), ("S", "ες"), ("T", "ταυ"),
        ("U", "γιού"), ("V", "βί"), ("W", "ντάμπλιου"), ("X", "χι"),
        ("Y", "γουάι"), ("Z", "ζήτα"),
    ];
    for (letter, name) in letters {
        let expr = format!("<math><mi>{letter}</mi></math>");
        test("el", "GreekMathReader", &expr, &format!("κεφαλαίο {name}"))?;
    }
    test_prefs(
        "el",
        "GreekMathReader",
        vec![("CapitalLetters_UseWord", "false"), ("CapitalLetters_Pitch", "20")],
        "<math><mi>A</mi></math>",
        "έι",
    )?;
    Ok(())
}

#[test]
fn greek_math_reader_latin_vowels_remain_distinct_from_greek_letters() -> Result<()> {
    let expr = "<math><mi>a</mi><mi>e</mi><mi>i</mi><mi>α</mi><mi>ε</mi><mi>ι</mi></math>";
    test(
        "el",
        "GreekMathReader",
        expr,
        "έι ί άι άλφα έψιλον γιώτα",
    )?;
    Ok(())
}

#[test]
fn existing_speech_styles_keep_latin_letter_behavior() -> Result<()> {
    // Test separate tokens: adjacent mi elements may be canonicalized into a word.
    for (letter, expected) in [("a", "a"), ("b", "b"), ("A", "κεφαλαίο A"), ("B", "κεφαλαίο b")] {
        let expr = format!("<math><mi>{letter}</mi></math>");
        test("el", "SimpleSpeak", &expr, expected)?;
        test("el", "ClearSpeak", &expr, expected)?;
    }
    Ok(())
}

#[test]
fn quote_punctuation_rules_remain_loadable() -> Result<()> {
    test("el", "GreekMathReader", r#"<math><mo>"</mo></math>"#, "εισαγωγικά")?;
    test("el", "GreekMathReader", "<math><mo>'</mo></math>", "τόνος")?;
    Ok(())
}

#[test]
fn extended_unicode_and_split_ranges_remain_greek() -> Result<()> {
    // Fullwidth operators retain their mathematical operation; audit range aliases have real coverage.
    for (symbol, expected) in [
        ("＋", "συν"), ("＜", "μικρότερο από"), ("＝", "ίσον"),
        ("＞", "μεγαλύτερο από"), ("￫", "βέλος προς τα δεξιά"),
        ("Ⓐ", "κυκλωμένο κεφαλαίο έι"), ("ⓩ", "κυκλωμένο ζήτα"),
        ("⟂", "είναι κάθετο στο"), ("⊥", "είναι κάθετο"),
    ] {
        test("el", "GreekMathReader", &format!("<math><mo>{symbol}</mo></math>"), expected)?;
    }
    Ok(())
}
