/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;


#[test]
fn special_alphabet_chars() -> Result<()> {
  let expr = "<math> <mi>ℌ</mi><mo>,</mo><mi>ℭ</mi></math>";
  test("pl", "SimpleSpeak", expr, "fraktura wielka h, przecinek, fraktura wielka c")?;
  let expr = "<math> <mi>ℍ</mi><mo>,</mo><mi>ℿ</mi></math>";
  test("pl", "SimpleSpeak", expr, "dwukreskowe wielka h, przecinek; dwukreskowe wielka pi")?;
  let expr = "<math> <mi>ℐ</mi><mo>,</mo><mi>ℳ</mi></math>";
  test("pl", "SimpleSpeak", expr, "pisane wielka i przecinek, pisane wielka m")?;
  return Ok(());

}

#[test]
fn greek() -> Result<()> {
    let expr = "<math> <mi>Α</mi><mo>,</mo><mi>Ω</mi></math>";
    test("pl", "SimpleSpeak", expr, "wielka alfa przecinek, wielka omega")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("pl", "SimpleSpeak", expr, "alfa przecinek, omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "dwukreskowe wielka delta, przecinek; dwukreskowe wielka ypsilon")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("pl", "SimpleSpeak", expr, "alfa przecinek, omega")?;
    return Ok(());

}

#[test]
fn cap_cyrillic() -> Result<()> {
    let expr = "<math> <mi>А</mi><mo>,</mo><mi>Я</mi></math>";
    test("pl", "SimpleSpeak", expr, "wielka a przecinek, wielka ja")?;
    return Ok(());

}

#[test]
fn parenthesized() -> Result<()> {
    let expr = "<math> <mi>⒜</mi><mo>,</mo><mi>⒵</mi></math>";
    test("pl", "SimpleSpeak", expr, "w nawiasie a przecinek, w nawiasie z")?;
    return Ok(());

}

#[test]
fn circled() -> Result<()> {
    let expr = "<math> <mi>Ⓐ</mi><mo>,</mo><mi>Ⓩ</mi></math>";
    test("pl", "SimpleSpeak", expr, "w kółku wielka a, przecinek, w kółku wielka z")?;
    let expr = "<math> <mi>🅐</mi><mo>,</mo><mi>🅩</mi></math>";
    test("pl", "SimpleSpeak", expr, "czarny w kółku wielka a, przecinek; czarny w kółku wielka z")?;
    let expr = "<math> <mi>ⓐ</mi><mo>,</mo><mi>ⓩ</mi></math>";
    test("pl", "SimpleSpeak", expr, "w kółku a przecinek, w kółku z")?;
    return Ok(());

}

#[test]
fn fraktur() -> Result<()> {
    let expr = "<math> <mi>𝔄</mi><mo>,</mo><mi>𝔜</mi></math>";
    test("pl", "SimpleSpeak", expr, "fraktura wielka a, przecinek, fraktura wielka y")?;
    let expr = "<math> <mi>𝔞</mi><mo>,</mo><mi>𝔷</mi></math>";
    test("pl", "SimpleSpeak", expr, "fraktura a przecinek, fraktura z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "fraktura wielka a, przecinek, fraktura wielka y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "fraktura a przecinek, fraktura z")?;
    return Ok(());

}

#[test]
fn bold_fraktur() -> Result<()> {
    let expr = "<math> <mi>𝕬</mi><mo>,</mo><mi>𝖅</mi></math>";
    test("pl", "SimpleSpeak", expr, "fraktura pogrubione wielka a, przecinek; fraktura pogrubione wielka z")?;
    let expr = "<math> <mi>𝖆</mi><mo>,</mo><mi>𝖟</mi></math>";
    test("pl", "SimpleSpeak", expr, "fraktura pogrubione a, przecinek; fraktura pogrubione z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "fraktura pogrubione wielka a, przecinek; fraktura pogrubione wielka z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "fraktura pogrubione a, przecinek; fraktura pogrubione z")?;
    return Ok(());

}

#[test]
fn double_struck() -> Result<()> {
    let expr = "<math> <mi>𝔸</mi><mo>,</mo><mi>𝕐</mi></math>";
    test("pl", "SimpleSpeak", expr, "dwukreskowe wielka a, przecinek; dwukreskowe wielka y")?;
    let expr = "<math> <mi>𝕒</mi><mo>,</mo><mi>𝕫</mi></math>";
    test("pl", "SimpleSpeak", expr, "dwukreskowe a przecinek, dwukreskowe z")?;
    let expr = "<math> <mi>𝟘</mi><mo>,</mo><mi>𝟡</mi></math>";
    test("pl", "SimpleSpeak", expr, "dwukreskowe 0 przecinek, dwukreskowe 9")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "dwukreskowe wielka a, przecinek; dwukreskowe wielka y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "dwukreskowe a przecinek, dwukreskowe z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "dwukreskowe 0 przecinek, dwukreskowe 9")?;
    return Ok(());

}

#[test]
fn script() -> Result<()> {
    let expr = "<math> <mi>𝒜</mi><mo>,</mo><mi>𝒵</mi></math>";
    test("pl", "SimpleSpeak", expr, "pisane wielka a przecinek, pisane wielka z")?;
    let expr = "<math> <mi>𝒶</mi><mo>,</mo><mi>𝓏</mi></math>";
    test("pl", "SimpleSpeak", expr, "pisane a przecinek, pisane z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pisane wielka a przecinek, pisane wielka z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pisane a przecinek, pisane z")?;
    return Ok(());

}

#[test]
fn bold_script() -> Result<()> {
    let expr = "<math> <mi>𝓐</mi><mo>,</mo><mi>𝓩</mi></math>";
    test("pl", "SimpleSpeak", expr, "pisane pogrubione wielka a, przecinek; pisane pogrubione wielka z")?;
    let expr = "<math> <mi>𝓪</mi><mo>,</mo><mi>𝔃</mi></math>";
    test("pl", "SimpleSpeak", expr, "pisane pogrubione a, przecinek, pisane pogrubione z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pisane pogrubione wielka a, przecinek; pisane pogrubione wielka z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pisane pogrubione a, przecinek, pisane pogrubione z")?;
    return Ok(());

}

#[test]
fn bold() -> Result<()> {
    let expr = "<math> <mi>𝐀</mi><mo>,</mo><mi>𝐙</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione wielka a, przecinek; pogrubione wielka z")?;
    let expr = "<math> <mi>𝐚</mi><mo>,</mo><mi>𝐳</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione a przecinek, pogrubione z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione wielka a, przecinek; pogrubione wielka z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione a przecinek, pogrubione z")?;
    return Ok(());

}

#[test]
fn italic() -> Result<()> {
    let expr = "<math> <mi>𝐴</mi><mo>,</mo><mi>𝑍</mi></math>";
    test("pl", "SimpleSpeak", expr, "wielka a przecinek, wielka z")?;
    let expr = "<math> <mi>𝑎</mi><mo>,</mo><mi>𝑧</mi></math>";
    test("pl", "SimpleSpeak", expr, "a przecinek, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "wielka a przecinek, wielka z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "a przecinek, z")?;
    return Ok(());

}

#[test]
fn sans_serif() -> Result<()> {
  let expr = "<math> <mi>𝖠</mi><mo>,</mo><mi>𝖹</mi></math>";
  test("pl", "SimpleSpeak", expr, "wielka a przecinek, wielka z")?;
  let expr = "<math> <mi>𝖺</mi><mo>,</mo><mi>𝗓</mi></math>";
  test("pl", "SimpleSpeak", expr, "a przecinek, z")?;
  // MathType private space versions
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("pl", "SimpleSpeak", expr, "wielka a przecinek, wielka z")?;
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("pl", "SimpleSpeak", expr, "a przecinek, z")?;
  return Ok(());

}

#[test]
fn sans_serif_bold() -> Result<()> {
    let expr = "<math> <mi>𝗔</mi><mo>,</mo><mi>𝗭</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione wielka a, przecinek; pogrubione wielka z")?;
    let expr = "<math> <mi>𝗮</mi><mo>,</mo><mi>𝘇</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione a przecinek, pogrubione z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione wielka a, przecinek; pogrubione wielka z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione a przecinek, pogrubione z")?;
    return Ok(());

}

#[test]
fn sans_serif_italic() -> Result<()> {
    let expr = "<math> <mi>𝘈</mi><mo>,</mo><mi>𝘡</mi></math>";
    test("pl", "SimpleSpeak", expr, "wielka a przecinek, wielka z")?;
    let expr = "<math> <mi>𝘢</mi><mo>,</mo><mi>𝘻</mi></math>";
    test("pl", "SimpleSpeak", expr, "a przecinek, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "wielka a przecinek, wielka z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "a przecinek, z")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic() -> Result<()> {
    let expr = "<math> <mi>𝘼</mi><mo>,</mo><mi>𝙕</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione wielka a, przecinek; pogrubione wielka z")?;
    let expr = "<math> <mi>𝙖</mi><mo>,</mo><mi>𝙯</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione a przecinek, pogrubione z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione wielka a, przecinek; pogrubione wielka z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione a przecinek, pogrubione z")?;
    return Ok(());

}

#[test]
fn monospace() -> Result<()> {
    let expr = "<math> <mi>𝙰</mi><mo>,</mo><mi>𝚉</mi></math>";
    test("pl", "SimpleSpeak", expr, "wielka a przecinek, wielka z")?;
    let expr = "<math> <mi>𝚊</mi><mo>,</mo><mi>𝚣</mi></math>";
    test("pl", "SimpleSpeak", expr, "a przecinek, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "wielka a przecinek, wielka z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "a przecinek, z")?;
    return Ok(());

}


#[test]
fn bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝚨</mi><mo>,</mo><mi>𝛀</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione wielka alfa, przecinek; pogrubione wielka omega")?;
    let expr = "<math> <mi>𝛂</mi><mo>,</mo><mi>𝛚</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione alfa przecinek, pogrubione omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione wielka alfa, przecinek; pogrubione wielka omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione alfa przecinek, pogrubione omega")?;
    return Ok(());

}

#[test]
fn bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝛛</mi><mo>,</mo><mi>𝛡</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione pochodna cząstkowa, przecinek, pogrubione pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione pochodna cząstkowa, przecinek, pogrubione pi")?;
    return Ok(());

}


#[test]
fn italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝛢</mi><mo>,</mo><mi>𝛺</mi></math>";
    test("pl", "SimpleSpeak", expr, "wielka alfa przecinek, wielka omega")?;
    let expr = "<math> <mi>𝛼</mi><mo>,</mo><mi>𝜔</mi></math>";
    test("pl", "SimpleSpeak", expr, "alfa przecinek, omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "wielka alfa przecinek, wielka omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "alfa przecinek, omega")?;
    return Ok(());

}

#[test]
fn italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝜕</mi><mo>,</mo><mi>𝜛</mi></math>";
    test("pl", "SimpleSpeak", expr, "pochodna cząstkowa, przecinek, pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pochodna cząstkowa, przecinek, pi")?;
    return Ok(());

}

#[test]
fn bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝜜</mi><mo>,</mo><mi>𝜴</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione wielka alfa, przecinek; pogrubione wielka omega")?;
    let expr = "<math> <mi>𝜶</mi><mo>,</mo><mi>𝝎</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione alfa przecinek, pogrubione omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione wielka alfa, przecinek; pogrubione wielka omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione alfa przecinek, pogrubione omega")?;
    return Ok(());

}

#[test]
fn bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝝏</mi><mo>,</mo><mi>𝝕</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione pochodna cząstkowa, przecinek, pogrubione pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione pochodna cząstkowa, przecinek, pogrubione pi")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝝖</mi><mo>,</mo><mi>𝝮</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione wielka alfa, przecinek; pogrubione wielka omega")?;
    let expr = "<math> <mi>𝝰</mi><mo>,</mo><mi>𝞈</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione alfa przecinek, pogrubione omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione wielka alfa, przecinek; pogrubione wielka omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione alfa przecinek, pogrubione omega")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝞉</mi><mo>,</mo><mi>𝞏</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione pochodna cząstkowa, przecinek, pogrubione pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione pochodna cząstkowa, przecinek, pogrubione pi")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝞐</mi><mo>,</mo><mi>𝞨</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione wielka alfa, przecinek; pogrubione wielka omega")?;
    let expr = "<math> <mi>𝞪</mi><mo>,</mo><mi>𝟂</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione alfa przecinek, pogrubione omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione wielka alfa, przecinek; pogrubione wielka omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione alfa przecinek, pogrubione omega")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝟃</mi><mo>,</mo><mi>𝟉</mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione pochodna cząstkowa, przecinek, pogrubione pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("pl", "SimpleSpeak", expr, "pogrubione pochodna cząstkowa, przecinek, pogrubione pi")?;
    return Ok(());

}

#[test]
fn pua_regular() -> Result<()> {
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("pl", "SimpleSpeak", expr, "wielka a przecinek, wielka z")?;
  return Ok(());

}

#[test]
fn turned() -> Result<()> {
    let expr = "<math> <mi>Ⅎ</mi><mo>,</mo><mi>⅄</mi></math>";
    test("pl", "SimpleSpeak", expr, "odwrócone wielka f, przecinek; odwrócony bezszeryfowe wielka y")?;
    return Ok(());

  }

#[test]
fn unicode_typo_regressions() -> Result<()> {
  test("pl", "SimpleSpeak", "<math><mi>ⁱ</mi></math>", "do potęgi i")?;
  test("pl", "SimpleSpeak", "<math><mi>☌</mi></math>", "koniunkcja")?;
  Ok(())
}

#[test]
fn enclosed_numbers() -> Result<()> {
  let expr = "<math> <mi>①</mi><mo>,</mo><mi>⑨</mi></math>";
  test("pl", "SimpleSpeak", expr, "w kółku 1 przecinek, w kółku 9")?;
  let expr = "<math> <mi>❶</mi><mo>,</mo><mi>㊿</mi></math>";
  test("pl", "SimpleSpeak", expr, "czarny w kółku jeden, przecinek; w kółku numer pięćdziesiąt")?;
  let expr = "<math> <mi>⑴</mi><mo>,</mo><mi>⑼</mi></math>";
  test("pl", "SimpleSpeak", expr, "w nawiasie 1 przecinek, w nawiasie 9")?;
  let expr = "<math> <mi>⒈</mi><mo>,</mo><mi>⒐</mi></math>";
  test("pl", "SimpleSpeak", expr, "1 z kropką przecinek, 9 z kropką")?;
  let expr = "<math> <mi>⓵</mi><mo>,</mo><mi>⓽</mi></math>";
  test("pl", "SimpleSpeak", expr, "podwójny w kółku 1, przecinek; podwójny w kółku 9")?;
  return Ok(());

}
