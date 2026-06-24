/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;


// AI generated
#[test]
fn special_alphabet_chars() -> Result<()> {
  let expr = "<math> <mi>ℌ</mi><mo>,</mo><mi>ℭ</mi></math>";
  test("hu", "SimpleSpeak", expr, "fraktur nagy h vessző, fraktur nagy c")?;
  let expr = "<math> <mi>ℍ</mi><mo>,</mo><mi>ℿ</mi></math>";
  test("hu", "SimpleSpeak", expr, "dupla leütésű nagy h, vessző; dupla leütésű nagy pí")?;
  let expr = "<math> <mi>ℐ</mi><mo>,</mo><mi>ℳ</mi></math>";
  test("hu", "SimpleSpeak", expr, "script nagy i vessző, script nagy m")?;
  return Ok(());

}

// AI generated
#[test]
fn greek() -> Result<()> {
    let expr = "<math> <mi>Α</mi><mo>,</mo><mi>Ω</mi></math>";
    test("hu", "SimpleSpeak", expr, "nagy alfa vessző, nagy omega")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("hu", "SimpleSpeak", expr, "alfa vessző, omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "dupla leütésű nagy delta, vessző; dupla leütésű nagy üpszilon")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("hu", "SimpleSpeak", expr, "alfa vessző, omega")?;
    return Ok(());

}

// AI generated
#[test]
fn cap_cyrillic() -> Result<()> {
    let expr = "<math> <mi>А</mi><mo>,</mo><mi>Я</mi></math>";
    test("hu", "SimpleSpeak", expr, "nagybetű а vessző, nagybetű ya")?;
    return Ok(());

}

// AI generated
#[test]
fn parenthesized() -> Result<()> {
    let expr = "<math> <mi>⒜</mi><mo>,</mo><mi>⒵</mi></math>";
    test("hu", "SimpleSpeak", expr, "zárójeles a vessző, zárójeles z")?;
    return Ok(());

}

// AI generated
#[test]
fn circled() -> Result<()> {
    let expr = "<math> <mi>Ⓐ</mi><mo>,</mo><mi>Ⓩ</mi></math>";
    test("hu", "SimpleSpeak", expr, "bekarikázott nagy a, vessző, bekarikázott nagy z")?;
    let expr = "<math> <mi>🅐</mi><mo>,</mo><mi>🅩</mi></math>";
    test("hu", "SimpleSpeak", expr, "fekete körrel jelölt nagy a, vessző; fekete körrel jelölt nagy z")?;
    let expr = "<math> <mi>ⓐ</mi><mo>,</mo><mi>ⓩ</mi></math>";
    test("hu", "SimpleSpeak", expr, "bekarikázott a vessző, bekarikázott z")?;
    return Ok(());

}

// AI generated
#[test]
fn fraktur() -> Result<()> {
    let expr = "<math> <mi>𝔄</mi><mo>,</mo><mi>𝔜</mi></math>";
    test("hu", "SimpleSpeak", expr, "fraktur nagy a vessző, fraktur nagy y")?;
    let expr = "<math> <mi>𝔞</mi><mo>,</mo><mi>𝔷</mi></math>";
    test("hu", "SimpleSpeak", expr, "fraktur a vessző, fraktur z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "fraktur nagy a vessző, fraktur nagy y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "fraktur a vessző, fraktur z")?;
    return Ok(());

}

// AI generated
#[test]
fn bold_fraktur() -> Result<()> {
    let expr = "<math> <mi>𝕬</mi><mo>,</mo><mi>𝖅</mi></math>";
    test("hu", "SimpleSpeak", expr, "fraktur félkövér nagy a, vessző; fraktur félkövér nagy z")?;
    let expr = "<math> <mi>𝖆</mi><mo>,</mo><mi>𝖟</mi></math>";
    test("hu", "SimpleSpeak", expr, "fraktur félkövér a, vessző, fraktur félkövér z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "fraktur félkövér nagy a, vessző; fraktur félkövér nagy z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "fraktur félkövér a, vessző, fraktur félkövér z")?;
    return Ok(());

}

// AI generated
#[test]
fn double_struck() -> Result<()> {
    let expr = "<math> <mi>𝔸</mi><mo>,</mo><mi>𝕐</mi></math>";
    test("hu", "SimpleSpeak", expr, "dupla leütésű nagy a, vessző; dupla leütésű nagy y")?;
    let expr = "<math> <mi>𝕒</mi><mo>,</mo><mi>𝕫</mi></math>";
    test("hu", "SimpleSpeak", expr, "dupla leütésű a, vessző, dupla leütésű z")?;
    let expr = "<math> <mi>𝟘</mi><mo>,</mo><mi>𝟡</mi></math>";
    test("hu", "SimpleSpeak", expr, "dupla leütésű 0, vessző, dupla leütésű 9")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "dupla leütésű nagy a, vessző; dupla leütésű nagy y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "dupla leütésű a, vessző, dupla leütésű z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "dupla leütésű 0, vessző, dupla leütésű 9")?;
    return Ok(());

}

// AI generated
#[test]
fn script() -> Result<()> {
    let expr = "<math> <mi>𝒜</mi><mo>,</mo><mi>𝒵</mi></math>";
    test("hu", "SimpleSpeak", expr, "script nagy a vessző, script nagy z")?;
    let expr = "<math> <mi>𝒶</mi><mo>,</mo><mi>𝓏</mi></math>";
    test("hu", "SimpleSpeak", expr, "script a vessző, script z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "script nagy a vessző, script nagy z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "script a vessző, script z")?;
    return Ok(());

}

// AI generated
#[test]
fn bold_script() -> Result<()> {
    let expr = "<math> <mi>𝓐</mi><mo>,</mo><mi>𝓩</mi></math>";
    test("hu", "SimpleSpeak", expr, "script félkövér nagy a, vessző; script félkövér nagy z")?;
    let expr = "<math> <mi>𝓪</mi><mo>,</mo><mi>𝔃</mi></math>";
    test("hu", "SimpleSpeak", expr, "script félkövér a, vessző, script félkövér z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "script félkövér nagy a, vessző; script félkövér nagy z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "script félkövér a, vessző, script félkövér z")?;
    return Ok(());

}

// AI generated
#[test]
fn bold() -> Result<()> {
    let expr = "<math> <mi>𝐀</mi><mo>,</mo><mi>𝐙</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér nagy a, vessző, félkövér nagy z")?;
    let expr = "<math> <mi>𝐚</mi><mo>,</mo><mi>𝐳</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér a vessző, félkövér z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér nagy a, vessző, félkövér nagy z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér a vessző, félkövér z")?;
    return Ok(());

}

// AI generated
#[test]
fn italic() -> Result<()> {
    let expr = "<math> <mi>𝐴</mi><mo>,</mo><mi>𝑍</mi></math>";
    test("hu", "SimpleSpeak", expr, "nagy a vessző, nagy z")?;
    let expr = "<math> <mi>𝑎</mi><mo>,</mo><mi>𝑧</mi></math>";
    test("hu", "SimpleSpeak", expr, "a vessző, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "nagy a vessző, nagy z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "a vessző, z")?;
    return Ok(());

}

// AI generated
#[test]
fn sans_serif() -> Result<()> {
  let expr = "<math> <mi>𝖠</mi><mo>,</mo><mi>𝖹</mi></math>";
  test("hu", "SimpleSpeak", expr, "nagy a vessző, nagy z")?;
  let expr = "<math> <mi>𝖺</mi><mo>,</mo><mi>𝗓</mi></math>";
  test("hu", "SimpleSpeak", expr, "a vessző, z")?;
  // MathType private space versions
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("hu", "SimpleSpeak", expr, "nagy a vessző, nagy z")?;
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("hu", "SimpleSpeak", expr, "a vessző, z")?;
  return Ok(());

}

// AI generated
#[test]
fn sans_serif_bold() -> Result<()> {
    let expr = "<math> <mi>𝗔</mi><mo>,</mo><mi>𝗭</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér nagy a, vessző, félkövér nagy z")?;
    let expr = "<math> <mi>𝗮</mi><mo>,</mo><mi>𝘇</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér a vessző, félkövér z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér nagy a, vessző, félkövér nagy z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér a vessző, félkövér z")?;
    return Ok(());

}

// AI generated
#[test]
fn sans_serif_italic() -> Result<()> {
    let expr = "<math> <mi>𝘈</mi><mo>,</mo><mi>𝘡</mi></math>";
    test("hu", "SimpleSpeak", expr, "nagy a vessző, nagy z")?;
    let expr = "<math> <mi>𝘢</mi><mo>,</mo><mi>𝘻</mi></math>";
    test("hu", "SimpleSpeak", expr, "a vessző, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "nagy a vessző, nagy z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "a vessző, z")?;
    return Ok(());

}

// AI generated
#[test]
fn sans_serif_bold_italic() -> Result<()> {
    let expr = "<math> <mi>𝘼</mi><mo>,</mo><mi>𝙕</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér nagy a, vessző, félkövér nagy z")?;
    let expr = "<math> <mi>𝙖</mi><mo>,</mo><mi>𝙯</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér a vessző, félkövér z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér nagy a, vessző, félkövér nagy z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér a vessző, félkövér z")?;
    return Ok(());

}

// AI generated
#[test]
fn monospace() -> Result<()> {
    let expr = "<math> <mi>𝙰</mi><mo>,</mo><mi>𝚉</mi></math>";
    test("hu", "SimpleSpeak", expr, "nagy a vessző, nagy z")?;
    let expr = "<math> <mi>𝚊</mi><mo>,</mo><mi>𝚣</mi></math>";
    test("hu", "SimpleSpeak", expr, "a vessző, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "nagy a vessző, nagy z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "a vessző, z")?;
    return Ok(());

}


// AI generated
#[test]
fn bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝚨</mi><mo>,</mo><mi>𝛀</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér nagy alfa, vessző, félkövér nagy omega")?;
    let expr = "<math> <mi>𝛂</mi><mo>,</mo><mi>𝛚</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér alfa vessző, félkövér omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér nagy alfa, vessző, félkövér nagy omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér alfa vessző, félkövér omega")?;
    return Ok(());

}

// AI generated
#[test]
fn bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝛛</mi><mo>,</mo><mi>𝛡</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér parciális derivált, vessző, félkövér pí")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér parciális derivált, vessző, félkövér pí")?;
    return Ok(());

}


// AI generated
#[test]
fn italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝛢</mi><mo>,</mo><mi>𝛺</mi></math>";
    test("hu", "SimpleSpeak", expr, "nagy alfa vessző, nagy omega")?;
    let expr = "<math> <mi>𝛼</mi><mo>,</mo><mi>𝜔</mi></math>";
    test("hu", "SimpleSpeak", expr, "alfa vessző, omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "nagy alfa vessző, nagy omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "alfa vessző, omega")?;
    return Ok(());

}

// AI generated
#[test]
fn italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝜕</mi><mo>,</mo><mi>𝜛</mi></math>";
    test("hu", "SimpleSpeak", expr, "parciális derivált, vessző, pí")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "parciális derivált, vessző, pí")?;
    return Ok(());

}

// AI generated
#[test]
fn bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝜜</mi><mo>,</mo><mi>𝜴</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér nagy alfa, vessző, félkövér nagy omega")?;
    let expr = "<math> <mi>𝜶</mi><mo>,</mo><mi>𝝎</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér alfa vessző, félkövér omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér nagy alfa, vessző, félkövér nagy omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér alfa vessző, félkövér omega")?;
    return Ok(());

}

// AI generated
#[test]
fn bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝝏</mi><mo>,</mo><mi>𝝕</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér parciális derivált, vessző, félkövér pí")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér parciális derivált, vessző, félkövér pí")?;
    return Ok(());

}

// AI generated
#[test]
fn sans_serif_bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝝖</mi><mo>,</mo><mi>𝝮</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér nagy alfa, vessző, félkövér nagy omega")?;
    let expr = "<math> <mi>𝝰</mi><mo>,</mo><mi>𝞈</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér alfa vessző, félkövér omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér nagy alfa, vessző, félkövér nagy omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér alfa vessző, félkövér omega")?;
    return Ok(());

}

// AI generated
#[test]
fn sans_serif_bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝞉</mi><mo>,</mo><mi>𝞏</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér parciális derivált, vessző, félkövér pí")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér parciális derivált, vessző, félkövér pí")?;
    return Ok(());

}

// AI generated
#[test]
fn sans_serif_bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝞐</mi><mo>,</mo><mi>𝞨</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér nagy alfa, vessző, félkövér nagy omega")?;
    let expr = "<math> <mi>𝞪</mi><mo>,</mo><mi>𝟂</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér alfa vessző, félkövér omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér nagy alfa, vessző, félkövér nagy omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér alfa vessző, félkövér omega")?;
    return Ok(());

}

// AI generated
#[test]
fn sans_serif_bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝟃</mi><mo>,</mo><mi>𝟉</mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér parciális derivált, vessző, félkövér pí")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("hu", "SimpleSpeak", expr, "félkövér parciális derivált, vessző, félkövér pí")?;
    return Ok(());

}

// AI generated
#[test]
fn pua_regular() -> Result<()> {
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("hu", "SimpleSpeak", expr, "nagy a vessző, nagy z")?;
  return Ok(());

}

// AI generated
#[test]
fn turned() -> Result<()> {
    let expr = "<math> <mi>Ⅎ</mi><mo>,</mo><mi>⅄</mi></math>";
    test("hu", "SimpleSpeak", expr, "fordított nagy f, vessző; fordított talpatlan nagy y")?;
    return Ok(());

  }

// AI generated
#[test]
fn unicode_typo_regressions() -> Result<()> {
  test("hu", "SimpleSpeak", "<math><mi>ⁱ</mi></math>", "i-edik hatványig")?;
  test("hu", "SimpleSpeak", "<math><mi>☌</mi></math>", "összekapcsolás")?;
  Ok(())
}

// AI generated
#[test]
fn enclosed_numbers() -> Result<()> {
  let expr = "<math> <mi>①</mi><mo>,</mo><mi>⑨</mi></math>";
  test("hu", "SimpleSpeak", expr, "bekarikázott 1 vessző, bekarikázott 9")?;
  let expr = "<math> <mi>❶</mi><mo>,</mo><mi>㊿</mi></math>";
  test("hu", "SimpleSpeak", expr, "fekete bekarikázott egy, vessző; bekarikázott szám ötven")?;
  let expr = "<math> <mi>⑴</mi><mo>,</mo><mi>⑼</mi></math>";
  test("hu", "SimpleSpeak", expr, "zárójeles 1 vessző, zárójeles 9")?;
  let expr = "<math> <mi>⒈</mi><mo>,</mo><mi>⒐</mi></math>";
  test("hu", "SimpleSpeak", expr, "1 ponttal vessző, 9 ponttal")?;
  let expr = "<math> <mi>⓵</mi><mo>,</mo><mi>⓽</mi></math>";
  test("hu", "SimpleSpeak", expr, "dupla bekarikázott 1, vessző, dupla bekarikázott 9")?;
  return Ok(());

}
