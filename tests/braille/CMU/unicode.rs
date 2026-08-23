// CMU braille Unicode full-range coverage tests.
use crate::common::*;
use anyhow::Result;

#[test]
fn unicode_full_special_constants() -> Result<()> {
    let expr = r#"<math><mi>ⅆⅇⅈ</mi></math>"#;
    test_braille("CMU", expr, "⠙⠑⠊")?;
    return Ok(())
}
