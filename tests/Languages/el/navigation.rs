//! Greek navigation output for localized command phrases, NavigationParts, and character movement.
use crate::common::*;
use anyhow::Result;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn assert_navigation(mathml: &str, command: &str, expected: &str) -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_nav("el", mathml)?;
        let speech = do_navigate_command(command)?;
        let trimmed_speech = speech.trim_end_matches([' ', ',', ';']).to_string();
        assert_eq!(expected, trimmed_speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn zoom_in_fraction_uses_greek_direction_and_part_names() -> Result<()> {
    let expr = r#"
      <math>
        <mfrac id="fraction">
          <mn id="numerator">1</mn>
          <mn id="denominator">2</mn>
        </mfrac>
      </math>
    "#;
    assert_navigation(
        expr,
        "ZoomIn",
        "μεγέθυνση προς τα μέσα; μέσα σε αριθμητής; 1",
    )
}

#[test]
fn move_next_reads_the_next_digit_in_a_number() -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        let expr = r#"<math><mn id="digits">1234</mn></math>"#;
        init_nav("el", expr)?;
        set_navigation_node("digits", 1)?;
        let speech = do_navigate_command("MoveNext")?;
        let trimmed_speech = speech.trim_end_matches([' ', ',', ';']).to_string();
        assert_eq!("μετακίνηση δεξιά; 2", trimmed_speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn fraction_navigation_preserves_position_and_localizes_zoom_out() -> Result<()> {
    // Navigation must identify the actual operand as well as speaking Greek command names.
    let expr = r#"<math><mfrac id="fraction"><mn id="num">1</mn><mn id="den">2</mn></mfrac></math>"#;
    init_nav("el", expr)?;
    set_preference("SpeechStyle", "GreekMathReader")?;
    for (command, expected, id) in [
        ("ZoomIn", "μεγέθυνση προς τα μέσα; μέσα σε αριθμητής; 1", "num"),
        ("MoveNext", "μετακίνηση δεξιά; μέσα σε παρονομαστής; 2", "den"),
        ("MovePrevious", "μετακίνηση αριστερά; μέσα σε αριθμητής; 1", "num"),
        ("ZoomOut", "σμίκρυνση προς τα έξω; έξω από αριθμητής; 1 διά 2", "fraction"),
        ("ZoomInAll", "μεγέθυνση μέχρι τέλους; μέσα σε αριθμητής; 1", "num"),
        ("ZoomOutAll", "σμίκρυνση μέχρι τέλους; έξω από αριθμητής; 1 διά 2", "fraction"),
        ("ReadCurrent", "ανάγνωση τρέχον; 1 διά 2", "fraction"),
    ] {
        assert_eq!(expected, do_navigate_command(command)?, "{command}");
        assert_eq!((id.to_string(), 0), get_navigation_mathml_id()?, "{command}");
    }
    Ok(())
}

#[test]
fn matrix_navigation_announces_rows_columns_and_reaches_each_cell() -> Result<()> {
    // Test both horizontal and vertical moves; reading the right text alone is insufficient.
    let expr = r#"<math><mo>(</mo><mtable><mtr><mtd><mn id="a">1</mn></mtd><mtd><mn id="b">2</mn></mtd></mtr><mtr><mtd><mn id="c">3</mn></mtd><mtd><mn id="d">4</mn></mtd></mtr></mtable><mo>)</mo></math>"#;
    init_nav("el", expr)?;
    set_navigation_node("a", 0)?;
    for (command, expected, id) in [
        ("MoveCellNext", "μετακίνηση δεξιά, στήλη 2; 2", "b"),
        ("MoveCellDown", "μετακίνηση προς τα κάτω, γραμμή 2, στήλη 2; 4", "d"),
        ("MoveCellPrevious", "μετακίνηση αριστερά, στήλη 1; 3", "c"),
        ("MoveCellUp", "μετακίνηση επάνω, γραμμή 1, στήλη 1; 1", "a"),
    ] {
        assert_eq!(expected, do_navigate_command(command)?, "{command}");
        assert_eq!((id.to_string(), 0), get_navigation_mathml_id()?, "{command}");
    }
    Ok(())
}

#[test]
fn moving_back_a_digit_updates_the_character_offset() -> Result<()> {
    // The backward character rule must move the cursor, not just repeat the current digit.
    init_nav("el", r#"<math><mn id="digits">1234</mn></math>"#)?;
    set_navigation_node("digits", 3)?;
    assert_eq!("μετακίνηση αριστερά; 2", do_navigate_command("MovePrevious")?);
    assert_eq!(("digits".to_string(), 2), get_navigation_mathml_id()?);
    Ok(())
}
