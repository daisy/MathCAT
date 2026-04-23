/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;


// AI generated
#[test]
fn special_alphabet_chars() -> Result<()> {
  let expr = "<math> <mi>ℌ</mi><mo>,</mo><mi>ℭ</mi></math>";
  test("fr", "SimpleSpeak", expr, "g ronde minuscule majuscule h, comma; g ronde minuscule majuscule c")?;
  let expr = "<math> <mi>ℍ</mi><mo>,</mo><mi>ℿ</mi></math>";
  test("fr", "SimpleSpeak", expr, "g ronde minuscule majuscule h, comma; g ronde minuscule majuscule pi")?;
  let expr = "<math> <mi>ℐ</mi><mo>,</mo><mi>ℳ</mi></math>";
  test("fr", "SimpleSpeak", expr, "constante de planck sur deux pi majuscule i; comma; constante de planck sur deux pi majuscule m")?;
  Ok(())
}

// AI generated
#[test]
fn greek() -> Result<()> {
    let expr = "<math> <mi>Α</mi><mo>,</mo><mi>Ω</mi></math>";
    test("fr", "SimpleSpeak", expr, "majuscule alpha comma, majuscule oméga")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha comma, oméga")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "double frappé majuscule delta, comma; double frappé majuscule upsilon")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha comma, oméga")?;
    Ok(())
}

// AI generated
#[test]
fn cap_cyrillic() -> Result<()> {
    let expr = "<math> <mi>А</mi><mo>,</mo><mi>Я</mi></math>";
    test("fr", "SimpleSpeak", expr, "majuscule a comma, majuscule ya")?;
    Ok(())
}

// AI generated
#[test]
fn parenthesized() -> Result<()> {
    let expr = "<math> <mi>⒜</mi><mo>,</mo><mi>⒵</mi></math>";
    test("fr", "SimpleSpeak", expr, "entre parenthèses a, comma, entre parenthèses z")?;
    Ok(())
}

// AI generated
#[test]
fn circled() -> Result<()> {
    let expr = "<math> <mi>Ⓐ</mi><mo>,</mo><mi>Ⓩ</mi></math>";
    test("fr", "SimpleSpeak", expr, "encerclé majuscule a, comma, encerclé majuscule z")?;
    let expr = "<math> <mi>🅐</mi><mo>,</mo><mi>🅩</mi></math>";
    test("fr", "SimpleSpeak", expr, "encerclé de noir majuscule a, comma; encerclé de noir majuscule z")?;
    let expr = "<math> <mi>ⓐ</mi><mo>,</mo><mi>ⓩ</mi></math>";
    test("fr", "SimpleSpeak", expr, "encerclé a comma, encerclé z")?;
    Ok(())
}

// AI generated
#[test]
fn fraktur() -> Result<()> {
    let expr = "<math> <mi>𝔄</mi><mo>,</mo><mi>𝔜</mi></math>";
    test("fr", "SimpleSpeak", expr, "fraktur majuscule a, comma, fraktur majuscule y")?;
    let expr = "<math> <mi>𝔞</mi><mo>,</mo><mi>𝔷</mi></math>";
    test("fr", "SimpleSpeak", expr, "fraktur a comma, fraktur z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "fraktur majuscule a, comma, fraktur majuscule y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "fraktur a comma, fraktur z")?;
    Ok(())
}

// AI generated
#[test]
fn bold_fraktur() -> Result<()> {
    let expr = "<math> <mi>𝕬</mi><mo>,</mo><mi>𝖅</mi></math>";
    test("fr", "SimpleSpeak", expr, "fraktur gras majuscule a, comma, fraktur gras majuscule z")?;
    let expr = "<math> <mi>𝖆</mi><mo>,</mo><mi>𝖟</mi></math>";
    test("fr", "SimpleSpeak", expr, "fraktur gras a comma, fraktur gras z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "fraktur gras majuscule a, comma, fraktur gras majuscule z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "fraktur gras a comma, fraktur gras z")?;
    Ok(())
}

// AI generated
#[test]
fn double_struck() -> Result<()> {
    let expr = "<math> <mi>𝔸</mi><mo>,</mo><mi>𝕐</mi></math>";
    test("fr", "SimpleSpeak", expr, "double frappé majuscule a, comma, double frappé majuscule y")?;
    let expr = "<math> <mi>𝕒</mi><mo>,</mo><mi>𝕫</mi></math>";
    test("fr", "SimpleSpeak", expr, "double frappé a comma, double frappé z")?;
    let expr = "<math> <mi>𝟘</mi><mo>,</mo><mi>𝟡</mi></math>";
    test("fr", "SimpleSpeak", expr, "double frappé 0 comma, double frappé 9")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "double frappé majuscule a, comma, double frappé majuscule y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "double frappé a comma, double frappé z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "double frappé 0 comma, double frappé 9")?;
    Ok(())
}

// AI generated
#[test]
fn script() -> Result<()> {
    let expr = "<math> <mi>𝒜</mi><mo>,</mo><mi>𝒵</mi></math>";
    test("fr", "SimpleSpeak", expr, "script majuscule a, comma, script majuscule z")?;
    let expr = "<math> <mi>𝒶</mi><mo>,</mo><mi>𝓏</mi></math>";
    test("fr", "SimpleSpeak", expr, "script a comma, script z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "script majuscule a, comma, script majuscule z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "script a comma, script z")?;
    Ok(())
}

// AI generated
#[test]
fn bold_script() -> Result<()> {
    let expr = "<math> <mi>𝓐</mi><mo>,</mo><mi>𝓩</mi></math>";
    test("fr", "SimpleSpeak", expr, "script gras majuscule a, comma, script gras majuscule z")?;
    let expr = "<math> <mi>𝓪</mi><mo>,</mo><mi>𝔃</mi></math>";
    test("fr", "SimpleSpeak", expr, "script gras a comma, script gras z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "script gras majuscule a, comma, script gras majuscule z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "script gras a comma, script gras z")?;
    Ok(())
}

// AI generated
#[test]
fn bold() -> Result<()> {
    let expr = "<math> <mi>𝐀</mi><mo>,</mo><mi>𝐙</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras majuscule a comma, gras majuscule z")?;
    let expr = "<math> <mi>𝐚</mi><mo>,</mo><mi>𝐳</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras a comma, gras z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras majuscule a comma, gras majuscule z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras a comma, gras z")?;
    Ok(())
}

// AI generated
#[test]
fn italic() -> Result<()> {
    let expr = "<math> <mi>𝐴</mi><mo>,</mo><mi>𝑍</mi></math>";
    test("fr", "SimpleSpeak", expr, "majuscule a comma, majuscule z")?;
    let expr = "<math> <mi>𝑎</mi><mo>,</mo><mi>𝑧</mi></math>";
    test("fr", "SimpleSpeak", expr, "a comma, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "majuscule a comma, majuscule z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a comma, z")?;
    Ok(())
}

// AI generated
#[test]
fn sans_serif() -> Result<()> {
  let expr = "<math> <mi>𝖠</mi><mo>,</mo><mi>𝖹</mi></math>";
  test("fr", "SimpleSpeak", expr, "majuscule a comma, majuscule z")?;
  let expr = "<math> <mi>𝖺</mi><mo>,</mo><mi>𝗓</mi></math>";
  test("fr", "SimpleSpeak", expr, "a comma, z")?;
  // MathType private space versions
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("fr", "SimpleSpeak", expr, "majuscule a comma, majuscule z")?;
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("fr", "SimpleSpeak", expr, "a comma, z")?;
  Ok(())
}

// AI generated
#[test]
fn sans_serif_bold() -> Result<()> {
    let expr = "<math> <mi>𝗔</mi><mo>,</mo><mi>𝗭</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras majuscule a comma, gras majuscule z")?;
    let expr = "<math> <mi>𝗮</mi><mo>,</mo><mi>𝘇</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras a comma, gras z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras majuscule a comma, gras majuscule z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras a comma, gras z")?;
    Ok(())
}

// AI generated
#[test]
fn sans_serif_italic() -> Result<()> {
    let expr = "<math> <mi>𝘈</mi><mo>,</mo><mi>𝘡</mi></math>";
    test("fr", "SimpleSpeak", expr, "majuscule a comma, majuscule z")?;
    let expr = "<math> <mi>𝘢</mi><mo>,</mo><mi>𝘻</mi></math>";
    test("fr", "SimpleSpeak", expr, "a comma, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "majuscule a comma, majuscule z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a comma, z")?;
    Ok(())
}

// AI generated
#[test]
fn sans_serif_bold_italic() -> Result<()> {
    let expr = "<math> <mi>𝘼</mi><mo>,</mo><mi>𝙕</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras majuscule a comma, gras majuscule z")?;
    let expr = "<math> <mi>𝙖</mi><mo>,</mo><mi>𝙯</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras a comma, gras z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras majuscule a comma, gras majuscule z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras a comma, gras z")?;
    Ok(())
}

// AI generated
#[test]
fn monospace() -> Result<()> {
    let expr = "<math> <mi>𝙰</mi><mo>,</mo><mi>𝚉</mi></math>";
    test("fr", "SimpleSpeak", expr, "majuscule a comma, majuscule z")?;
    let expr = "<math> <mi>𝚊</mi><mo>,</mo><mi>𝚣</mi></math>";
    test("fr", "SimpleSpeak", expr, "a comma, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "majuscule a comma, majuscule z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a comma, z")?;
    Ok(())
}


// AI generated
#[test]
fn bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝚨</mi><mo>,</mo><mi>𝛀</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras majuscule alpha, comma, gras majuscule oméga")?;
    let expr = "<math> <mi>𝛂</mi><mo>,</mo><mi>𝛚</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras alpha comma, gras oméga")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras majuscule alpha, comma, gras majuscule oméga")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras alpha comma, gras oméga")?;
    Ok(())
}

// AI generated
#[test]
fn bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝛛</mi><mo>,</mo><mi>𝛡</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras dérivée partielle, comma, gras pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras dérivée partielle, comma, gras pi")?;
    Ok(())
}


// AI generated
#[test]
fn italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝛢</mi><mo>,</mo><mi>𝛺</mi></math>";
    test("fr", "SimpleSpeak", expr, "majuscule alpha comma, majuscule oméga")?;
    let expr = "<math> <mi>𝛼</mi><mo>,</mo><mi>𝜔</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha comma, oméga")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "majuscule alpha comma, majuscule oméga")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha comma, oméga")?;
    Ok(())
}

// AI generated
#[test]
fn italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝜕</mi><mo>,</mo><mi>𝜛</mi></math>";
    test("fr", "SimpleSpeak", expr, "dérivée partielle, comma, pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "dérivée partielle, comma, pi")?;
    Ok(())
}

// AI generated
#[test]
fn bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝜜</mi><mo>,</mo><mi>𝜴</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras majuscule alpha, comma, gras majuscule oméga")?;
    let expr = "<math> <mi>𝜶</mi><mo>,</mo><mi>𝝎</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras alpha comma, gras oméga")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras majuscule alpha, comma, gras majuscule oméga")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras alpha comma, gras oméga")?;
    Ok(())
}

// AI generated
#[test]
fn bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝝏</mi><mo>,</mo><mi>𝝕</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras dérivée partielle, comma, gras pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras dérivée partielle, comma, gras pi")?;
    Ok(())
}

// AI generated
#[test]
fn sans_serif_bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝝖</mi><mo>,</mo><mi>𝝮</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras majuscule alpha, comma, gras majuscule oméga")?;
    let expr = "<math> <mi>𝝰</mi><mo>,</mo><mi>𝞈</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras alpha comma, gras oméga")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras majuscule alpha, comma, gras majuscule oméga")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras alpha comma, gras oméga")?;
    Ok(())
}

// AI generated
#[test]
fn sans_serif_bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝞉</mi><mo>,</mo><mi>𝞏</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras dérivée partielle, comma, gras pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras dérivée partielle, comma, gras pi")?;
    Ok(())
}

// AI generated
#[test]
fn sans_serif_bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝞐</mi><mo>,</mo><mi>𝞨</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras majuscule alpha, comma, gras majuscule oméga")?;
    let expr = "<math> <mi>𝞪</mi><mo>,</mo><mi>𝟂</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras alpha comma, gras oméga")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras majuscule alpha, comma, gras majuscule oméga")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras alpha comma, gras oméga")?;
    Ok(())
}

// AI generated
#[test]
fn sans_serif_bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝟃</mi><mo>,</mo><mi>𝟉</mi></math>";
    test("fr", "SimpleSpeak", expr, "gras dérivée partielle, comma, gras pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "gras dérivée partielle, comma, gras pi")?;
    Ok(())
}

// AI generated
#[test]
fn pua_regular() -> Result<()> {
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("fr", "SimpleSpeak", expr, "majuscule a comma, majuscule z")?;
  Ok(())
}

// AI generated
#[test]
fn turned() -> Result<()> {
    let expr = "<math> <mi>Ⅎ</mi><mo>,</mo><mi>⅄</mi></math>";
    test("fr", "SimpleSpeak", expr, "tourné majuscule f, comma; e ronde minuscule majuscule y")?;
    Ok(())
  }

// AI generated
#[test]
fn unicode_typo_regressions() -> Result<()> {
  test("fr", "SimpleSpeak", "<math><mi>ⁱ</mi></math>", "exposant i")?;
  test("fr", "SimpleSpeak", "<math><mi>☌</mi></math>", "conjonction")?;
  Ok(())
}

// AI generated
#[test]
fn enclosed_numbers() -> Result<()> {
  let expr = "<math> <mi>①</mi><mo>,</mo><mi>⑨</mi></math>";
  test("fr", "SimpleSpeak", expr, "encerclé 1 comma, encerclé 9")?;
  let expr = "<math> <mi>❶</mi><mo>,</mo><mi>㊿</mi></math>";
  test("fr", "SimpleSpeak", expr, "noir encerclé un comma, numéro cinquante encerclé")?;
  let expr = "<math> <mi>⑴</mi><mo>,</mo><mi>⑼</mi></math>";
  test("fr", "SimpleSpeak", expr, "entre parenthèses 1, comma, entre parenthèses 9")?;
  let expr = "<math> <mi>⒈</mi><mo>,</mo><mi>⒐</mi></math>";
  test("fr", "SimpleSpeak", expr, "1 avec point comma, 9 avec point")?;
  let expr = "<math> <mi>⓵</mi><mo>,</mo><mi>⓽</mi></math>";
  test("fr", "SimpleSpeak", expr, "double encerclé 1 comma, double encerclé 9")?;
  Ok(())
}
