/// AI generated
use crate::common::*;
use anyhow::Result;

#[test]
fn trig_names() -> Result<()> {
    // AI generated
    let expr = "<math><mrow><mi>sin</mi><mi>x</mi><mo>+</mo><mi>cos</mi><mi>y</mi></mrow></math>";
    test("fr", "SimpleSpeak", expr, "sinus de x plus cosinus de y")?;
    Ok(())
}

#[test]
fn simple_fraction() -> Result<()> {
    // AI generated
    let expr = "<math><mfrac><mn>21</mn><mn>34</mn></mfrac></math>";
    test("fr", "SimpleSpeak", expr, "21 sur 34")?;
    Ok(())
}

#[test]
fn square_root() -> Result<()> {
    // AI generated
    let expr = "<math><msqrt><mi>x</mi></msqrt></math>";
    test("fr", "SimpleSpeak", expr, "la racine carrée de x")?;
    Ok(())
}
