/// AI generated
use crate::common::*;
use anyhow::Result;

#[test]
fn modified_vars() -> Result<()> {
    // AI generated
    let expr = "<math><mrow><mover><mi>x</mi><mo>^</mo></mover><mo>+</mo><mover><mi>t</mi><mo>→</mo></mover></mrow></math>";
    test("fr", "SimpleSpeak", expr, "x circonflexe plus vecteur t")?;
    Ok(())
}

#[test]
fn subscript_literal() -> Result<()> {
    // AI generated
    let expr = "<math><msub><mi>x</mi><mn>1</mn></msub></math>";
    test("fr", "SimpleSpeak", expr, "x indice 1")?;
    Ok(())
}
