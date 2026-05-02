/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;


#[test]
fn special_alphabet_chars() -> Result<()> {
  let expr = "<math> <mi>ℌ</mi><mo>,</mo><mi>ℭ</mi></math>";
  test("nb", "SimpleSpeak", expr, "fraktur stor h komma, fraktur stor c")?;
  let expr = "<math> <mi>ℍ</mi><mo>,</mo><mi>ℿ</mi></math>";
  test("nb", "SimpleSpeak", expr, "dobbeltstreket stor h, komma, dobbeltstreket stor pi")?;
  let expr = "<math> <mi>ℐ</mi><mo>,</mo><mi>ℳ</mi></math>";
  test("nb", "SimpleSpeak", expr, "script font stor i, komma, script font stor m")?;
  return Ok(());

}

#[test]
fn greek() -> Result<()> {
    let expr = "<math> <mi>Α</mi><mo>,</mo><mi>Ω</mi></math>";
    test("nb", "SimpleSpeak", expr, "stor alfa komma, stor omega")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("nb", "SimpleSpeak", expr, "alfa komma, omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "dobbeltstreket stor delta, komma; dobbeltstreket stor ypsilon")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("nb", "SimpleSpeak", expr, "alfa komma, omega")?;
    return Ok(());

}

#[test]
fn cap_cyrillic() -> Result<()> {
    let expr = "<math> <mi>А</mi><mo>,</mo><mi>Я</mi></math>";
    test("nb", "SimpleSpeak", expr, "stor a, komma, stor ja")?;
    return Ok(());

}

#[test]
fn parenthesized() -> Result<()> {
    let expr = "<math> <mi>⒜</mi><mo>,</mo><mi>⒵</mi></math>";
    test("nb", "SimpleSpeak", expr, "a i parentes komma, z i parentes")?;
    return Ok(());

}

#[test]
fn circled() -> Result<()> {
    let expr = "<math> <mi>Ⓐ</mi><mo>,</mo><mi>Ⓩ</mi></math>";
    test("nb", "SimpleSpeak", expr, "stor a i sirkel komma, stor z i sirkel")?;
    let expr = "<math> <mi>ⓐ</mi><mo>,</mo><mi>ⓩ</mi></math>";
    test("nb", "SimpleSpeak", expr, "a i sirkel komma, z i sirkel")?;
    return Ok(());

}

#[test]
fn fraktur() -> Result<()> {
    let expr = "<math> <mi>𝔄</mi><mo>,</mo><mi>𝔜</mi></math>";
    test("nb", "SimpleSpeak", expr, "fraktur stor a komma, fraktur stor y")?;
    let expr = "<math> <mi>𝔞</mi><mo>,</mo><mi>𝔷</mi></math>";
    test("nb", "SimpleSpeak", expr, "fraktur a komma, fraktur z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fraktur stor a komma, fraktur stor y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fraktur a komma, fraktur z")?;
    return Ok(());

}

#[test]
fn bold_fraktur() -> Result<()> {
    let expr = "<math> <mi>𝕬</mi><mo>,</mo><mi>𝖅</mi></math>";
    test("nb", "SimpleSpeak", expr, "fraktur fet stor a, komma, fraktur fet stor z")?;
    let expr = "<math> <mi>𝖆</mi><mo>,</mo><mi>𝖟</mi></math>";
    test("nb", "SimpleSpeak", expr, "fraktur fet a komma, fraktur fet z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fraktur fet stor a, komma, fraktur fet stor z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fraktur fet a komma, fraktur fet z")?;
    return Ok(());

}

#[test]
fn double_struck() -> Result<()> {
    let expr = "<math> <mi>𝔸</mi><mo>,</mo><mi>𝕐</mi></math>";
    test("nb", "SimpleSpeak", expr, "dobbeltstreket stor a, komma, dobbeltstreket stor y")?;
    let expr = "<math> <mi>𝕒</mi><mo>,</mo><mi>𝕫</mi></math>";
    test("nb", "SimpleSpeak", expr, "dobbeltstreket a komma, dobbeltstreket z")?;
    let expr = "<math> <mi>𝟘</mi><mo>,</mo><mi>𝟡</mi></math>";
    test("nb", "SimpleSpeak", expr, "dobbeltstreket 0 komma, dobbeltstreket 9")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "dobbeltstreket stor a, komma, dobbeltstreket stor y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "dobbeltstreket a komma, dobbeltstreket z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "dobbeltstreket 0 komma, dobbeltstreket 9")?;
    return Ok(());

}

#[test]
fn script() -> Result<()> {
    let expr = "<math> <mi>𝒜</mi><mo>,</mo><mi>𝒵</mi></math>";
    test("nb", "SimpleSpeak", expr, "script font stor a, komma, script font stor z")?;
    let expr = "<math> <mi>𝒶</mi><mo>,</mo><mi>𝓏</mi></math>";
    test("nb", "SimpleSpeak", expr, "script font a komma, script font z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "script font stor a, komma, script font stor z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "script font a komma, script font z")?;
    return Ok(());

}

#[test]
fn bold_script() -> Result<()> {
    let expr = "<math> <mi>𝓐</mi><mo>,</mo><mi>𝓩</mi></math>";
    test("nb", "SimpleSpeak", expr, "script font fet stor a, komma, script font fet stor z")?;
    let expr = "<math> <mi>𝓪</mi><mo>,</mo><mi>𝔃</mi></math>";
    test("nb", "SimpleSpeak", expr, "script font fet a komma, script font fet z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "script font fet stor a, komma, script font fet stor z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "script font fet a komma, script font fet z")?;
    return Ok(());

}

#[test]
fn bold() -> Result<()> {
    let expr = "<math> <mi>𝐀</mi><mo>,</mo><mi>𝐙</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet stor a komma, fet stor z")?;
    let expr = "<math> <mi>𝐚</mi><mo>,</mo><mi>𝐳</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet a komma, fet z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet stor a komma, fet stor z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet a komma, fet z")?;
    return Ok(());

}

#[test]
fn italic() -> Result<()> {
    let expr = "<math> <mi>𝐴</mi><mo>,</mo><mi>𝑍</mi></math>";
    test("nb", "SimpleSpeak", expr, "stor a komma, stor z")?;
    let expr = "<math> <mi>𝑎</mi><mo>,</mo><mi>𝑧</mi></math>";
    test("nb", "SimpleSpeak", expr, "a komma, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "stor a komma, stor z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "a komma, z")?;
    return Ok(());

}

#[test]
fn sans_serif() -> Result<()> {
  let expr = "<math> <mi>𝖠</mi><mo>,</mo><mi>𝖹</mi></math>";
  test("nb", "SimpleSpeak", expr, "stor a komma, stor z")?;
  let expr = "<math> <mi>𝖺</mi><mo>,</mo><mi>𝗓</mi></math>";
  test("nb", "SimpleSpeak", expr, "a komma, z")?;
  // MathType private space versions
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("nb", "SimpleSpeak", expr, "stor a komma, stor z")?;
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("nb", "SimpleSpeak", expr, "a komma, z")?;
  return Ok(());

}

#[test]
fn sans_serif_bold() -> Result<()> {
    let expr = "<math> <mi>𝗔</mi><mo>,</mo><mi>𝗭</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet stor a komma, fet stor z")?;
    let expr = "<math> <mi>𝗮</mi><mo>,</mo><mi>𝘇</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet a komma, fet z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet stor a komma, fet stor z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet a komma, fet z")?;
    return Ok(());

}

#[test]
fn sans_serif_italic() -> Result<()> {
    let expr = "<math> <mi>𝘈</mi><mo>,</mo><mi>𝘡</mi></math>";
    test("nb", "SimpleSpeak", expr, "stor a komma, stor z")?;
    let expr = "<math> <mi>𝘢</mi><mo>,</mo><mi>𝘻</mi></math>";
    test("nb", "SimpleSpeak", expr, "a komma, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "stor a komma, stor z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "a komma, z")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic() -> Result<()> {
    let expr = "<math> <mi>𝘼</mi><mo>,</mo><mi>𝙕</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet stor a komma, fet stor z")?;
    let expr = "<math> <mi>𝙖</mi><mo>,</mo><mi>𝙯</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet a komma, fet z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet stor a komma, fet stor z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet a komma, fet z")?;
    return Ok(());

}

#[test]
fn monospace() -> Result<()> {
    let expr = "<math> <mi>𝙰</mi><mo>,</mo><mi>𝚉</mi></math>";
    test("nb", "SimpleSpeak", expr, "stor a komma, stor z")?;
    let expr = "<math> <mi>𝚊</mi><mo>,</mo><mi>𝚣</mi></math>";
    test("nb", "SimpleSpeak", expr, "a komma, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "stor a komma, stor z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "a komma, z")?;
    return Ok(());

}


#[test]
fn bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝚨</mi><mo>,</mo><mi>𝛀</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet stor alfa komma, fet stor omega")?;
    let expr = "<math> <mi>𝛂</mi><mo>,</mo><mi>𝛚</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet alfa komma, fet omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet stor alfa komma, fet stor omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet alfa komma, fet omega")?;
    return Ok(());

}

#[test]
fn bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝛛</mi><mo>,</mo><mi>𝛡</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet partiell derivert, komma, fet pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet partiell derivert, komma, fet pi")?;
    return Ok(());

}


#[test]
fn italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝛢</mi><mo>,</mo><mi>𝛺</mi></math>";
    test("nb", "SimpleSpeak", expr, "stor alfa komma, stor omega")?;
    let expr = "<math> <mi>𝛼</mi><mo>,</mo><mi>𝜔</mi></math>";
    test("nb", "SimpleSpeak", expr, "alfa komma, omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "stor alfa komma, stor omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "alfa komma, omega")?;
    return Ok(());

}

#[test]
fn italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝜕</mi><mo>,</mo><mi>𝜛</mi></math>";
    test("nb", "SimpleSpeak", expr, "partiell derivert komma, pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "partiell derivert komma, pi")?;
    return Ok(());

}

#[test]
fn bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝜜</mi><mo>,</mo><mi>𝜴</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet stor alfa komma, fet stor omega")?;
    let expr = "<math> <mi>𝜶</mi><mo>,</mo><mi>𝝎</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet alfa komma, fet omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet stor alfa komma, fet stor omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet alfa komma, fet omega")?;
    return Ok(());

}

#[test]
fn bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝝏</mi><mo>,</mo><mi>𝝕</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet partiell derivert, komma, fet pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet partiell derivert, komma, fet pi")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝝖</mi><mo>,</mo><mi>𝝮</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet stor alfa komma, fet stor omega")?;
    let expr = "<math> <mi>𝝰</mi><mo>,</mo><mi>𝞈</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet alfa komma, fet omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet stor alfa komma, fet stor omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet alfa komma, fet omega")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝞉</mi><mo>,</mo><mi>𝞏</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet partiell derivert, komma, fet pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet partiell derivert, komma, fet pi")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝞐</mi><mo>,</mo><mi>𝞨</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet stor alfa komma, fet stor omega")?;
    let expr = "<math> <mi>𝞪</mi><mo>,</mo><mi>𝟂</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet alfa komma, fet omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet stor alfa komma, fet stor omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet alfa komma, fet omega")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝟃</mi><mo>,</mo><mi>𝟉</mi></math>";
    test("nb", "SimpleSpeak", expr, "fet partiell derivert, komma, fet pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("nb", "SimpleSpeak", expr, "fet partiell derivert, komma, fet pi")?;
    return Ok(());

}

#[test]
fn pua_regular() -> Result<()> {
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("nb", "SimpleSpeak", expr, "stor a komma, stor z")?;
  return Ok(());

}

#[test]
fn turned() -> Result<()> {
    let expr = "<math> <mi>Ⅎ</mi><mo>,</mo><mi>⅄</mi></math>";
    test("nb", "SimpleSpeak", expr, "rotert stor f komma, rotert sans-serif stor y")?;
    return Ok(());

  }

#[test]
fn enclosed_numbers() -> Result<()> {
  let expr = "<math> <mi>①</mi><mo>,</mo><mi>⑨</mi></math>";
  test("nb", "SimpleSpeak", expr, "1 i sirkel komma, 9 i sirkel")?;
  let expr = "<math> <mi>⑴</mi><mo>,</mo><mi>⑼</mi></math>";
  test("nb", "SimpleSpeak", expr, "1 i parentes komma, 9 i parentes")?;
  let expr = "<math> <mi>⒈</mi><mo>,</mo><mi>⒐</mi></math>";
  test("nb", "SimpleSpeak", expr, "1 med punktum komma, 9 med punktum")?;
  let expr = "<math> <mi>⓵</mi><mo>,</mo><mi>⓽</mi></math>";
  test("nb", "SimpleSpeak", expr, "1 i dobbel sirkel komma, 9 i dobbel sirkel")?;
  return Ok(());

}
