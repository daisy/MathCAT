/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;


#[test]
fn silent_intent() -> Result<()> {
    let expr = "<math> <mrow intent='testing:silent($arg1, $arg2)'><mn arg='arg1'>2</mn> <mi arg='arg2'>x</mi></mrow> </math>";
    test("fr", "SimpleSpeak", expr, "2 x")?;
    test("fr", "LiteralSpeak", expr, "2 x")?;
    return Ok(());

}

#[test]
fn prefix_intent() -> Result<()> {
    let expr = r#"<math><msup intent='teste:prefix($x)'> <mi arg='x'>x</mi> <mi>T</mi> </msup> </math>"#;
    test("fr", "SimpleSpeak", expr, "teste x")?;
    return Ok(());

}

#[test]
fn postfix_intent() -> Result<()> {
    let expr = r#"<math><msup intent='teste:postfix($x)'> <mi arg='x'>x</mi> <mi>T</mi> </msup> </math>"#;
    test("fr", "SimpleSpeak", expr, "x teste")?;
    return Ok(());

}

#[test]
fn infix_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='teste:infix($x, $y, $z, 2)'>
        <mi arg='x'>x</mi>
        <mi arg='y'>y</mi>
        <mi arg='z'>z</mi>
    </mrow> </math>"#;
    test("fr", "SimpleSpeak", expr, "x teste y teste z teste 2")?;
    return Ok(());

}

#[test]
fn infix_intent_no_args() -> Result<()> {
    // this is illegal intent, so it is just an mrow with one child
    let expr = r#"<math><mrow intent='testing:infix()'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("fr", "SimpleSpeak", expr, "x")?;
    return Ok(());

}

#[test]
fn infix_intent_one_arg() -> Result<()> {
    let expr = r#"<math><mrow intent='teste:infix($x)'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    // Note: we say the intent name because there are infix plus/minus with a single arg due to continued rows or combined columns
    test("fr", "SimpleSpeak", expr, "teste x")?;
    return Ok(());

}

#[test]
fn function_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='teste:function($x, $y, $z, 2)'>
        <mi arg='x'>x</mi>
        <mi arg='y'>y</mi>
        <mi arg='z'>z</mi>
    </mrow> </math>"#;
    test("fr", "SimpleSpeak", expr, "teste de x virgule, y virgule, z virgule, 2")?;
    return Ok(());

}

#[test]
fn function_no_args_intent() -> Result<()> {
    // this is illegal intent, so it is just an mrow with one child
    let expr = r#"<math><mrow intent='teste:function()'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("fr", "SimpleSpeak", expr, "x")?;
    return Ok(());

}

#[test]
fn function_one_arg_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='teste:function($x)'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("fr", "SimpleSpeak", expr, "teste de x")?;
    return Ok(());

}

#[test]
fn silent_intent_mi() -> Result<()> {
    let expr = "<math> <mn>2</mn> <mi intent=':silent'>x</mi></math>";
    test("fr", "SimpleSpeak", expr, "2")?;
    test("fr", "ClearSpeak", expr, "2")?;
    return Ok(());

}

#[test]
fn silent_intent_msup() -> Result<()> {
    let expr = "<math>
        <msup intent='index:silent($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("fr", "SimpleSpeak", expr, "h majuscule 2")?;
    test("fr", "ClearSpeak", expr, "h majuscule 2")?;
    return Ok(());

}

#[test]
fn silent_intent_underscore() -> Result<()> {
    let expr = "<math>
        <msup intent='_($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("fr", "SimpleSpeak", expr, "h majuscule 2")?;
    test("fr", "ClearSpeak", expr, "h majuscule 2")?;
    return Ok(());

}

#[test]
fn intent_prob_x() -> Result<()> {
    let expr = "<math>
    <msup intent='$op($arg)'>
        <mi arg='arg'>x</mi>
        <mi arg='op' intent='probabilité' mathvariant='normal'>P</mi>
    </msup></math>";
    test("fr", "ClearSpeak", expr, "probabilité de x")?;
    return Ok(());

}

#[test]
fn do_some_stuff_71() -> Result<()> {
    let expr = "<math>
    <mrow>
        <mn>41</mn>
        <mo>+</mo>
        <mn>71</mn>
    </mrow>
    </math>";
    test("fr", "SimpleSpeak", expr, "quarante et un plus soixante et onze")

}
