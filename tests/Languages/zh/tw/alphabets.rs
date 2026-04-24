/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;


#[test]
fn special_alphabet_chars() -> Result<()> {
  let expr = "<math> <mi>ℌ</mi><mo>,</mo><mi>ℭ</mi></math>";
  test("zh-tw", "SimpleSpeak", expr, "fraktur 大寫 h 逗號 fraktur 大寫 c")?;
  let expr = "<math> <mi>ℍ</mi><mo>,</mo><mi>ℿ</mi></math>";
  test("zh-tw", "SimpleSpeak", expr, "空心 大寫 h 逗號 空心 大寫 pi")?;
  let expr = "<math> <mi>ℐ</mi><mo>,</mo><mi>ℳ</mi></math>";
  test("zh-tw", "SimpleSpeak", expr, "草體 大寫 i 逗號 草體 大寫 m")?;
  return Ok(());

}

#[test]
fn greek() -> Result<()> {
    let expr = "<math> <mi>Α</mi><mo>,</mo><mi>Ω</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "大寫 alpha 逗號 大寫 omega")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "alpha 逗號 omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "空心 大寫 delta 逗號 空心 大寫 upsilon")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "alpha 逗號 omega")?;
    return Ok(());

}

#[test]
fn cap_cyrillic() -> Result<()> {
    let expr = "<math> <mi>А</mi><mo>,</mo><mi>Я</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "大寫 a 逗號 大寫 ya")?;
    return Ok(());

}

#[test]
fn parenthesized() -> Result<()> {
    let expr = "<math> <mi>⒜</mi><mo>,</mo><mi>⒵</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "括號圍繞 a 逗號 括號圍繞 z")?;
    return Ok(());

}

#[test]
fn circled() -> Result<()> {
    let expr = "<math> <mi>Ⓐ</mi><mo>,</mo><mi>Ⓩ</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "圈圈 大寫 a 逗號 圈圈 大寫 z")?;
    let expr = "<math> <mi>ⓐ</mi><mo>,</mo><mi>ⓩ</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "圈圈 a 逗號 圈圈 z")?;
    return Ok(());

}

#[test]
fn fraktur() -> Result<()> {
    let expr = "<math> <mi>𝔄</mi><mo>,</mo><mi>𝔜</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "fraktur 大寫 a 逗號 fraktur 大寫 y")?;
    let expr = "<math> <mi>𝔞</mi><mo>,</mo><mi>𝔷</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "fraktur a 逗號 fraktur z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "fraktur 大寫 a 逗號 fraktur 大寫 y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "fraktur a 逗號 fraktur z")?;
    return Ok(());

}

#[test]
fn bold_fraktur() -> Result<()> {
    let expr = "<math> <mi>𝕬</mi><mo>,</mo><mi>𝖅</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "fraktur 粗體 大寫 a, 逗號 fraktur 粗體 大寫 z")?;
    let expr = "<math> <mi>𝖆</mi><mo>,</mo><mi>𝖟</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "fraktur 粗體 a 逗號 fraktur 粗體 z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "fraktur 粗體 大寫 a, 逗號 fraktur 粗體 大寫 z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "fraktur 粗體 a 逗號 fraktur 粗體 z")?;
    return Ok(());

}

#[test]
fn double_struck() -> Result<()> {
    let expr = "<math> <mi>𝔸</mi><mo>,</mo><mi>𝕐</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "空心 大寫 a 逗號 空心 大寫 y")?;
    let expr = "<math> <mi>𝕒</mi><mo>,</mo><mi>𝕫</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "空心 a 逗號 空心 z")?;
    let expr = "<math> <mi>𝟘</mi><mo>,</mo><mi>𝟡</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "空心 0 逗號 空心 9")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "空心 大寫 a 逗號 空心 大寫 y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "空心 a 逗號 空心 z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "空心 0 逗號 空心 9")?;
    return Ok(());

}

#[test]
fn script() -> Result<()> {
    let expr = "<math> <mi>𝒜</mi><mo>,</mo><mi>𝒵</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "草體 大寫 a 逗號 草體 大寫 z")?;
    let expr = "<math> <mi>𝒶</mi><mo>,</mo><mi>𝓏</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "草體 a 逗號 草體 z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "草體 大寫 a 逗號 草體 大寫 z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "草體 a 逗號 草體 z")?;
    return Ok(());

}

#[test]
fn bold_script() -> Result<()> {
    let expr = "<math> <mi>𝓐</mi><mo>,</mo><mi>𝓩</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗草體 大寫 a 逗號 粗草體 大寫 z")?;
    let expr = "<math> <mi>𝓪</mi><mo>,</mo><mi>𝔃</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗草體 a 逗號 粗草體 z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗草體 大寫 a 逗號 粗草體 大寫 z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗草體 a 逗號 粗草體 z")?;
    return Ok(());

}

#[test]
fn bold() -> Result<()> {
    let expr = "<math> <mi>𝐀</mi><mo>,</mo><mi>𝐙</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 大寫 a 逗號 粗體 大寫 z")?;
    let expr = "<math> <mi>𝐚</mi><mo>,</mo><mi>𝐳</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 a 逗號 粗體 z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 大寫 a 逗號 粗體 大寫 z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 a 逗號 粗體 z")?;
    return Ok(());

}

#[test]
fn italic() -> Result<()> {
    let expr = "<math> <mi>𝐴</mi><mo>,</mo><mi>𝑍</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "大寫 a 逗號 大寫 z")?;
    let expr = "<math> <mi>𝑎</mi><mo>,</mo><mi>𝑧</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "a 逗號 z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "大寫 a 逗號 大寫 z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "a 逗號 z")?;
    return Ok(());

}

#[test]
fn sans_serif() -> Result<()> {
  let expr = "<math> <mi>𝖠</mi><mo>,</mo><mi>𝖹</mi></math>";
  test("zh-tw", "SimpleSpeak", expr, "大寫 a 逗號 大寫 z")?;
  let expr = "<math> <mi>𝖺</mi><mo>,</mo><mi>𝗓</mi></math>";
  test("zh-tw", "SimpleSpeak", expr, "a 逗號 z")?;
  // MathType private space versions
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("zh-tw", "SimpleSpeak", expr, "大寫 a 逗號 大寫 z")?;
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("zh-tw", "SimpleSpeak", expr, "a 逗號 z")?;
  return Ok(());

}

#[test]
fn sans_serif_bold() -> Result<()> {
    let expr = "<math> <mi>𝗔</mi><mo>,</mo><mi>𝗭</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 大寫 a 逗號 粗體 大寫 z")?;
    let expr = "<math> <mi>𝗮</mi><mo>,</mo><mi>𝘇</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 a 逗號 粗體 z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 大寫 a 逗號 粗體 大寫 z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 a 逗號 粗體 z")?;
    return Ok(());

}

#[test]
fn sans_serif_italic() -> Result<()> {
    let expr = "<math> <mi>𝘈</mi><mo>,</mo><mi>𝘡</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "大寫 a 逗號 大寫 z")?;
    let expr = "<math> <mi>𝘢</mi><mo>,</mo><mi>𝘻</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "a 逗號 z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "大寫 a 逗號 大寫 z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "a 逗號 z")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic() -> Result<()> {
    let expr = "<math> <mi>𝘼</mi><mo>,</mo><mi>𝙕</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 大寫 a 逗號 粗斜體 大寫 z")?;
    let expr = "<math> <mi>𝙖</mi><mo>,</mo><mi>𝙯</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 a 逗號 粗斜體 z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 大寫 a 逗號 粗斜體 大寫 z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 a 逗號 粗斜體 z")?;
    return Ok(());

}

#[test]
fn monospace() -> Result<()> {
    let expr = "<math> <mi>𝙰</mi><mo>,</mo><mi>𝚉</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "大寫 a 逗號 大寫 z")?;
    let expr = "<math> <mi>𝚊</mi><mo>,</mo><mi>𝚣</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "a 逗號 z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "大寫 a 逗號 大寫 z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "a 逗號 z")?;
    return Ok(());

}


#[test]
fn bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝚨</mi><mo>,</mo><mi>𝛀</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 大寫 alpha 逗號 粗體 大寫 omega")?;
    let expr = "<math> <mi>𝛂</mi><mo>,</mo><mi>𝛚</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 alpha 逗號 粗體 omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 大寫 alpha 逗號 粗體 大寫 omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 alpha 逗號 粗體 omega")?;
    return Ok(());

}

#[test]
fn bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝛛</mi><mo>,</mo><mi>𝛡</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 偏微分 逗號 粗體 pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 偏微分 逗號 粗體 pi")?;
    return Ok(());

}


#[test]
fn italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝛢</mi><mo>,</mo><mi>𝛺</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "大寫 alpha 逗號 大寫 omega")?;
    let expr = "<math> <mi>𝛼</mi><mo>,</mo><mi>𝜔</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "alpha 逗號 omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "大寫 alpha 逗號 大寫 omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "alpha 逗號 omega")?;
    return Ok(());

}

#[test]
fn italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝜕</mi><mo>,</mo><mi>𝜛</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "偏微分 逗號 pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "偏微分 逗號 pi")?;
    return Ok(());

}

#[test]
fn bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝜜</mi><mo>,</mo><mi>𝜴</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 大寫 alpha, 逗號 粗斜體 大寫 omega")?;
    let expr = "<math> <mi>𝜶</mi><mo>,</mo><mi>𝝎</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 alpha 逗號 粗斜體 omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 大寫 alpha, 逗號 粗斜體 大寫 omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 alpha 逗號 粗斜體 omega")?;
    return Ok(());

}

#[test]
fn bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝝏</mi><mo>,</mo><mi>𝝕</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 偏微分 逗號 粗斜體 pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 偏微分 逗號 粗斜體 pi")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝝖</mi><mo>,</mo><mi>𝝮</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 大寫 alpha 逗號 粗體 大寫 omega")?;
    let expr = "<math> <mi>𝝰</mi><mo>,</mo><mi>𝞈</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 alpha 逗號 粗體 omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 大寫 alpha 逗號 粗體 大寫 omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 alpha 逗號 粗體 omega")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝞉</mi><mo>,</mo><mi>𝞏</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 偏微分 逗號 粗體 pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗體 偏微分 逗號 粗體 pi")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝞐</mi><mo>,</mo><mi>𝞨</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 大寫 alpha, 逗號 粗斜體 大寫 omega")?;
    let expr = "<math> <mi>𝞪</mi><mo>,</mo><mi>𝟂</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 alpha 逗號 粗斜體 omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 大寫 alpha, 逗號 粗斜體 大寫 omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 alpha 逗號 粗斜體 omega")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝟃</mi><mo>,</mo><mi>𝟉</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 偏微分 逗號 粗斜體 pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "粗斜體 偏微分 逗號 粗斜體 pi")?;
    return Ok(());

}

#[test]
fn pua_regular() -> Result<()> {
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("zh-tw", "SimpleSpeak", expr, "大寫 a 逗號 大寫 z")?;
  return Ok(());

}

#[test]
fn turned() -> Result<()> {
    let expr = "<math> <mi>Ⅎ</mi><mo>,</mo><mi>⅄</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "翻身 大寫 f 逗號 翻身sanserif 大寫 y")?;
    return Ok(());

  }

#[test]
fn enclosed_numbers() -> Result<()> {
  let expr = "<math> <mi>①</mi><mo>,</mo><mi>⑨</mi></math>";
  test("zh-tw", "SimpleSpeak", expr, "圈圈 1 逗號 圈圈 9")?;
  let expr = "<math> <mi>⑴</mi><mo>,</mo><mi>⑼</mi></math>";
  test("zh-tw", "SimpleSpeak", expr, "括號圍繞 1 逗號 括號圍繞 9")?;
  let expr = "<math> <mi>⒈</mi><mo>,</mo><mi>⒐</mi></math>";
  test("zh-tw", "SimpleSpeak", expr, "1 點 逗號 9 點")?;
  let expr = "<math> <mi>⓵</mi><mo>,</mo><mi>⓽</mi></math>";
  test("zh-tw", "SimpleSpeak", expr, "雙圈 1 逗號 雙圈 9")?;
  return Ok(());

}
