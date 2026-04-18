/// AI generated
use crate::common::*;
use anyhow::Result;

#[test]
fn greek_letters() -> Result<()> {
    // AI generated
    let expr = "<math><mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha comma, oméga")?;
    Ok(())
}
