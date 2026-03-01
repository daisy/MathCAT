/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;


/*
#[test]
fn special_alphabet_chars() -> Result<()> {
  let expr = "<math> <mi>ℌ</mi><mo>,</mo><mi>ℭ</mi></math>";
  test("de", "SimpleSpeak", expr, "fraktur cap h comma, fraktur cap c")?;
  let expr = "<math> <mi>ℍ</mi><mo>,</mo><mi>ℿ</mi></math>";
  test("de", "SimpleSpeak", expr, "double struck cap h, comma, double struck cap pi")?;
  let expr = "<math> <mi>ℐ</mi><mo>,</mo><mi>ℳ</mi></math>";
  test("de", "SimpleSpeak", expr, "script cap i comma, script cap m")?;
  return Ok(());

}

#[test]
fn greek() -> Result<()> {
    let expr = "<math> <mi>Α</mi><mo>,</mo><mi>Ω</mi></math>";
    test("de", "SimpleSpeak", expr, "cap alpha comma, cap omega")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("de", "SimpleSpeak", expr, "alpha comma, omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "double struck cap delta, comma, double struck cap upsilon")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("de", "SimpleSpeak", expr, "alpha comma, omega")?;
    return Ok(());

}

#[test]
fn cap_cyrillic() -> Result<()> {
    let expr = "<math> <mi>А</mi><mo>,</mo><mi>Я</mi></math>";
    test("de", "SimpleSpeak", expr, "cap a comma, cap ya")?;
    return Ok(());

}

#[test]
fn parenthesized() -> Result<()> {
    let expr = "<math> <mi>⒜</mi><mo>,</mo><mi>⒵</mi></math>";
    test("de", "SimpleSpeak", expr, "parenthesized eigh comma, parenthesized z")?;
    return Ok(());

}

#[test]
fn circled() -> Result<()> {
    let expr = "<math> <mi>Ⓐ</mi><mo>,</mo><mi>Ⓩ</mi></math>";
    test("de", "SimpleSpeak", expr, "circled cap eigh comma, circled cap z")?;
    let expr = "<math> <mi>ⓐ</mi><mo>,</mo><mi>ⓩ</mi></math>";
    test("de", "SimpleSpeak", expr, "circled eigh comma, circled z")?;
    return Ok(());

}

#[test]
fn fraktur() -> Result<()> {
    let expr = "<math> <mi>𝔄</mi><mo>,</mo><mi>𝔜</mi></math>";
    test("de", "SimpleSpeak", expr, "fraktur cap eigh comma, fraktur cap y")?;
    let expr = "<math> <mi>𝔞</mi><mo>,</mo><mi>𝔷</mi></math>";
    test("de", "SimpleSpeak", expr, "fraktur eigh comma, fraktur z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "fraktur cap eigh comma, fraktur cap y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "fraktur eigh comma, fraktur z")?;
    return Ok(());

}

#[test]
fn bold_fraktur() -> Result<()> {
    let expr = "<math> <mi>𝕬</mi><mo>,</mo><mi>𝖅</mi></math>";
    test("de", "SimpleSpeak", expr, "fraktur bold cap eigh, comma, fraktur bold cap z")?;
    let expr = "<math> <mi>𝖆</mi><mo>,</mo><mi>𝖟</mi></math>";
    test("de", "SimpleSpeak", expr, "fraktur bold eigh comma, fraktur bold z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "fraktur bold cap eigh, comma, fraktur bold cap z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "fraktur bold eigh comma, fraktur bold z")?;
    return Ok(());

}

#[test]
fn double_struck() -> Result<()> {
    let expr = "<math> <mi>𝔸</mi><mo>,</mo><mi>𝕐</mi></math>";
    test("de", "SimpleSpeak", expr, "double struck cap eigh, comma, double struck cap y")?;
    let expr = "<math> <mi>𝕒</mi><mo>,</mo><mi>𝕫</mi></math>";
    test("de", "SimpleSpeak", expr, "double struck eigh comma, double struck z")?;
    let expr = "<math> <mi>𝟘</mi><mo>,</mo><mi>𝟡</mi></math>";
    test("de", "SimpleSpeak", expr, "double struck 0 comma, double struck 9")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "double struck cap eigh, comma, double struck cap y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "double struck eigh comma, double struck z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "double struck 0 comma, double struck 9")?;
    return Ok(());

}

#[test]
fn script() -> Result<()> {
    let expr = "<math> <mi>𝒜</mi><mo>,</mo><mi>𝒵</mi></math>";
    test("de", "SimpleSpeak", expr, "script cap eigh comma, script cap z")?;
    let expr = "<math> <mi>𝒶</mi><mo>,</mo><mi>𝓏</mi></math>";
    test("de", "SimpleSpeak", expr, "script eigh comma, script z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "script cap eigh comma, script cap z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "script eigh comma, script z")?;
    return Ok(());

}

#[test]
fn bold_script() -> Result<()> {
    let expr = "<math> <mi>𝓐</mi><mo>,</mo><mi>𝓩</mi></math>";
    test("de", "SimpleSpeak", expr, "eingekreist groß a komma eingekreist groß z")?;
    let expr = "<math> <mi>𝓪</mi><mo>,</mo><mi>𝔃</mi></math>";
    test("de", "SimpleSpeak", expr, "script bold eigh comma, script bold z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "script bold cap eigh, comma, script bold cap z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "script bold eigh comma, script bold z")?;
    return Ok(());

}

#[test]
fn bold() -> Result<()> {
    let expr = "<math> <mi>𝐀</mi><mo>,</mo><mi>𝐙</mi></math>";
    test("de", "SimpleSpeak", expr, "bold cap eigh comma, bold cap z")?;
    let expr = "<math> <mi>𝐚</mi><mo>,</mo><mi>𝐳</mi></math>";
    test("de", "SimpleSpeak", expr, "bold eigh comma, bold z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold cap eigh comma, bold cap z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold eigh comma, bold z")?;
    return Ok(());

}

#[test]
fn italic() -> Result<()> {
    let expr = "<math> <mi>𝐴</mi><mo>,</mo><mi>𝑍</mi></math>";
    test("de", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
    let expr = "<math> <mi>𝑎</mi><mo>,</mo><mi>𝑧</mi></math>";
    test("de", "SimpleSpeak", expr, "eigh comma, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "eigh comma, z")?;
    return Ok(());

}

#[test]
fn sans_serif() -> Result<()> {
  let expr = "<math> <mi>𝖠</mi><mo>,</mo><mi>𝖹</mi></math>";
  test("de", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
  let expr = "<math> <mi>𝖺</mi><mo>,</mo><mi>𝗓</mi></math>";
  test("de", "SimpleSpeak", expr, "eigh comma, z")?;
  // MathType private space versions
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("de", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("de", "SimpleSpeak", expr, "eigh comma, z")?;
  return Ok(());

}

#[test]
fn sans_serif_bold() -> Result<()> {
    let expr = "<math> <mi>𝗔</mi><mo>,</mo><mi>𝗭</mi></math>";
    test("de", "SimpleSpeak", expr, "bold cap eigh comma, bold cap z")?;
    let expr = "<math> <mi>𝗮</mi><mo>,</mo><mi>𝘇</mi></math>";
    test("de", "SimpleSpeak", expr, "bold eigh comma, bold z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold cap eigh comma, bold cap z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold eigh comma, bold z")?;
    return Ok(());

}

#[test]
fn sans_serif_italic() -> Result<()> {
    let expr = "<math> <mi>𝘈</mi><mo>,</mo><mi>𝘡</mi></math>";
    test("de", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
    let expr = "<math> <mi>𝘢</mi><mo>,</mo><mi>𝘻</mi></math>";
    test("de", "SimpleSpeak", expr, "eigh comma, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "eigh comma, z")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic() -> Result<()> {
    let expr = "<math> <mi>𝘼</mi><mo>,</mo><mi>𝙕</mi></math>";
    test("de", "SimpleSpeak", expr, "bold cap eigh comma, bold cap z")?;
    let expr = "<math> <mi>𝙖</mi><mo>,</mo><mi>𝙯</mi></math>";
    test("de", "SimpleSpeak", expr, "bold eigh comma, bold z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold cap eigh comma, bold cap z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold eigh comma, bold z")?;
    return Ok(());

}

#[test]
fn monospace() -> Result<()> {
    let expr = "<math> <mi>𝙰</mi><mo>,</mo><mi>𝚉</mi></math>";
    test("de", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
    let expr = "<math> <mi>𝚊</mi><mo>,</mo><mi>𝚣</mi></math>";
    test("de", "SimpleSpeak", expr, "eigh comma, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "eigh comma, z")?;
    return Ok(());

}


#[test]
fn bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝚨</mi><mo>,</mo><mi>𝛀</mi></math>";
    test("de", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi>𝛂</mi><mo>,</mo><mi>𝛚</mi></math>";
    test("de", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    return Ok(());

}

#[test]
fn bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝛛</mi><mo>,</mo><mi>𝛡</mi></math>";
    test("de", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    return Ok(());

}


#[test]
fn italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝛢</mi><mo>,</mo><mi>𝛺</mi></math>";
    test("de", "SimpleSpeak", expr, "cap alpha comma, cap omega")?;
    let expr = "<math> <mi>𝛼</mi><mo>,</mo><mi>𝜔</mi></math>";
    test("de", "SimpleSpeak", expr, "alpha comma, omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "cap alpha comma, cap omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "alpha comma, omega")?;
    return Ok(());

}

#[test]
fn italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝜕</mi><mo>,</mo><mi>𝜛</mi></math>";
    test("de", "SimpleSpeak", expr, "partial derivative comma, pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "partial derivative comma, pi")?;
    return Ok(());

}

#[test]
fn bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝜜</mi><mo>,</mo><mi>𝜴</mi></math>";
    test("de", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi>𝜶</mi><mo>,</mo><mi>𝝎</mi></math>";
    test("de", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    return Ok(());

}

#[test]
fn bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝝏</mi><mo>,</mo><mi>𝝕</mi></math>";
    test("de", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝝖</mi><mo>,</mo><mi>𝝮</mi></math>";
    test("de", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi>𝝰</mi><mo>,</mo><mi>𝞈</mi></math>";
    test("de", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝞉</mi><mo>,</mo><mi>𝞏</mi></math>";
    test("de", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝞐</mi><mo>,</mo><mi>𝞨</mi></math>";
    test("de", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi>𝞪</mi><mo>,</mo><mi>𝟂</mi></math>";
    test("de", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝟃</mi><mo>,</mo><mi>𝟉</mi></math>";
    test("de", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("de", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    return Ok(());

}

#[test]
fn pua_regular() -> Result<()> {
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("de", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
  return Ok(());

}

#[test]
fn turned() -> Result<()> {
    let expr = "<math> <mi>Ⅎ</mi><mo>,</mo><mi>⅄</mi></math>";
    test("de", "SimpleSpeak", expr, "turned cap f comma, turned sans-serif cap y")?;
    return Ok(());

  }
*/
#[test]
fn enclosed_numbers() -> Result<()> {
  let expr = "<math> <mi>①</mi><mo>,</mo><mi>⑨</mi></math>";
  test("de", "SimpleSpeak", expr, "umkreiste 1 komma umkreiste 9")?;
  let expr = "<math> <mi>⑴</mi><mo>,</mo><mi>⑼</mi></math>";
  test("de", "SimpleSpeak", expr, "klammern 1 komma klammern 9")?;
  let expr = "<math> <mi>⒈</mi><mo>,</mo><mi>⒐</mi></math>";
  test("de", "SimpleSpeak", expr, "1 mit periode komma 9 mit periode")?;
  let expr = "<math> <mi>⓵</mi><mo>,</mo><mi>⓽</mi></math>";
  test("de", "SimpleSpeak", expr, "doppelt umkreist 1 komma doppelt umkreist 9")?;
  return Ok(());

}
