/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;


#[test]
fn silent_intent() -> Result<()> {
    let expr = "<math> <mrow intent='prueba:silent($arg1, $arg2)'><mn arg='arg1'>2</mn> <mi arg='arg2'>x</mi></mrow> </math>";
    test("es", "SimpleSpeak", expr, "2 x")?;
    test("es", "LiteralSpeak", expr, "2 x")?;
    return Ok(());

}

#[test]
fn prefix_intent() -> Result<()> {
    let expr = r#"<math><msup intent='prueba:prefix($x)'> <mi arg='x'>x</mi> <mi>T</mi> </msup> </math>"#;
    test("es", "SimpleSpeak", expr, "prueba x")?;
    return Ok(());

}

#[test]
fn postfix_intent() -> Result<()> {
    let expr = r#"<math><msup intent='prueba:postfix($x)'> <mi arg='x'>x</mi> <mi>T</mi> </msup> </math>"#;
    test("es", "SimpleSpeak", expr, "x prueba")?;
    return Ok(());

}

#[test]
fn infix_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='prueba:infix($x, $y, $z, 2)'>
        <mi arg='x'>x</mi>
        <mi arg='y'>y</mi>
        <mi arg='z'>z</mi>
    </mrow> </math>"#;
    test("es", "SimpleSpeak", expr, "x prueba y prueba z prueba 2")?;
    return Ok(());

}

#[test]
fn infix_intent_no_args() -> Result<()> {
    // this is illegal intent, so it is just an mrow with one child
    let expr = r#"<math><mrow intent='prueba:infix()'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("es", "SimpleSpeak", expr, "x")?;
    return Ok(());

}

#[test]
fn infix_intent_one_arg() -> Result<()> {
    let expr = r#"<math><mrow intent='prueba:infix($x)'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    // Note: we say the intent name because there are infix plus/minus with a single arg due to continued rows or combined columns
    test("es", "SimpleSpeak", expr, "prueba x")?;
    return Ok(());

}

#[test]
fn function_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='prueba:function($x, $y, $z, 2)'>
        <mi arg='x'>x</mi>
        <mi arg='y'>y</mi>
        <mi arg='z'>z</mi>
    </mrow> </math>"#;
    test("es", "SimpleSpeak", expr, "prueba de x coma, y coma, z coma, 2")?;
    return Ok(());

}

#[test]
fn function_no_args_intent() -> Result<()> {
    // this is illegal intent, so it is just an mrow with one child
    let expr = r#"<math><mrow intent='prueba:function()'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("es", "SimpleSpeak", expr, "x")?;
    return Ok(());

}

#[test]
fn function_one_arg_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='prueba:function($x)'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("es", "SimpleSpeak", expr, "prueba de x")?;
    return Ok(());

}

#[test]
fn silent_intent_mi() -> Result<()> {
    let expr = "<math> <mn>2</mn> <mi intent=':silent'>x</mi></math>";
    test("es", "SimpleSpeak", expr, "2")?;
    test("es", "ClearSpeak", expr, "2")?;
    return Ok(());

}

#[test]
fn silent_intent_msup() -> Result<()> {
    let expr = "<math>
        <msup intent='index:silent($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("es", "SimpleSpeak", expr, "mayúscula h 2")?;
    test("es", "ClearSpeak", expr, "mayúscula h 2")?;
    return Ok(());

}

#[test]
fn silent_intent_underscore() -> Result<()> {
    let expr = "<math>
        <msup intent='_($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("es", "SimpleSpeak", expr, "mayúscula h 2")?;
    test("es", "ClearSpeak", expr, "mayúscula h 2")?;
    return Ok(());

}

#[test]
fn intent_prob_x() -> Result<()> {
    let expr = "<math>
    <msup intent='$op($arg)'>
        <mi arg='arg'>x</mi>
        <mi arg='op' intent='probability' mathvariant='normal'>P</mi>
    </msup></math>";
    test("es", "ClearSpeak", expr, "probabilidad de x")?;
    return Ok(());

}
