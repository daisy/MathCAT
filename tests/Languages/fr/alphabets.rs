/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;


#[test]
fn special_alphabet_chars() -> Result<()> {
  let expr = "<math> <mi>ℌ</mi><mo>,</mo><mi>ℭ</mi></math>";
  test("fr", "SimpleSpeak", expr, "h majuscule gothique, virgule, c majuscule gothique")?;
  let expr = "<math> <mi>ℍ</mi><mo>,</mo><mi>ℿ</mi></math>";
  test("fr", "SimpleSpeak", expr, "h majuscule double barre, virgule; pi majuscule double barre")?;
  let expr = "<math> <mi>ℐ</mi><mo>,</mo><mi>ℳ</mi></math>";
  test("fr", "SimpleSpeak", expr, "i majuscule script, virgule, m majuscule script")?;
  Ok(())
}


#[test]
fn greek() -> Result<()> {
    let expr = "<math> <mi>Α</mi><mo>,</mo><mi>Ω</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha majuscule virgule, oméga majuscule")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha virgule, oméga")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "delta majuscule double barre, virgule; upsilon majuscule double barre")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha virgule, oméga")?;
    Ok(())
}


#[test]
fn cap_cyrillic() -> Result<()> {
    let expr = "<math> <mi>А</mi><mo>,</mo><mi>Я</mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule virgule, ya majuscule")?;
    Ok(())
}


#[test]
fn parenthesized() -> Result<()> {
    let expr = "<math> <mi>⒜</mi><mo>,</mo><mi>⒵</mi></math>";
    test("fr", "SimpleSpeak", expr, "a entre parenthèses, virgule, z entre parenthèses")?;
    Ok(())
}


#[test]
fn circled() -> Result<()> {
    let expr = "<math> <mi>Ⓐ</mi><mo>,</mo><mi>Ⓩ</mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule encerclé, virgule, z majuscule encerclé")?;
    let expr = "<math> <mi>🅐</mi><mo>,</mo><mi>🅩</mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule encerclé de noir, virgule; z majuscule encerclé de noir")?;
    let expr = "<math> <mi>ⓐ</mi><mo>,</mo><mi>ⓩ</mi></math>";
    test("fr", "SimpleSpeak", expr, "a encerclé virgule, z encerclé")?;
    Ok(())
}


#[test]
fn fraktur() -> Result<()> {
    let expr = "<math> <mi>𝔄</mi><mo>,</mo><mi>𝔜</mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule gothique, virgule, y majuscule gothique")?;
    let expr = "<math> <mi>𝔞</mi><mo>,</mo><mi>𝔷</mi></math>";
    test("fr", "SimpleSpeak", expr, "a gothique virgule, z gothique")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule gothique, virgule, y majuscule gothique")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a gothique virgule, z gothique")?;
    Ok(())
}


#[test]
fn bold_fraktur() -> Result<()> {
    let expr = "<math> <mi>𝕬</mi><mo>,</mo><mi>𝖅</mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule gothique gras, virgule; z majuscule gothique gras")?;
    let expr = "<math> <mi>𝖆</mi><mo>,</mo><mi>𝖟</mi></math>";
    test("fr", "SimpleSpeak", expr, "a gothique gras virgule, z gothique gras")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule gothique gras, virgule; z majuscule gothique gras")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a gothique gras virgule, z gothique gras")?;
    Ok(())
}


#[test]
fn double_struck() -> Result<()> {
    let expr = "<math> <mi>𝔸</mi><mo>,</mo><mi>𝕐</mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule double barre, virgule; y majuscule double barre")?;
    let expr = "<math> <mi>𝕒</mi><mo>,</mo><mi>𝕫</mi></math>";
    test("fr", "SimpleSpeak", expr, "a double barre virgule, z double barre")?;
    let expr = "<math> <mi>𝟘</mi><mo>,</mo><mi>𝟡</mi></math>";
    test("fr", "SimpleSpeak", expr, "0 double barre virgule, 9 double barre")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule double barre, virgule; y majuscule double barre")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a double barre virgule, z double barre")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "0 double barre virgule, 9 double barre")?;
    Ok(())
}


#[test]
fn script() -> Result<()> {
    let expr = "<math> <mi>𝒜</mi><mo>,</mo><mi>𝒵</mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule script, virgule, z majuscule script")?;
    let expr = "<math> <mi>𝒶</mi><mo>,</mo><mi>𝓏</mi></math>";
    test("fr", "SimpleSpeak", expr, "a script virgule, z script")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule script, virgule, z majuscule script")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a script virgule, z script")?;
    Ok(())
}


#[test]
fn bold_script() -> Result<()> {
    let expr = "<math> <mi>𝓐</mi><mo>,</mo><mi>𝓩</mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule script gras, virgule; z majuscule script gras")?;
    let expr = "<math> <mi>𝓪</mi><mo>,</mo><mi>𝔃</mi></math>";
    test("fr", "SimpleSpeak", expr, "a script gras virgule, z script gras")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule script gras, virgule; z majuscule script gras")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a script gras virgule, z script gras")?;
    Ok(())
}


#[test]
fn bold() -> Result<()> {
    let expr = "<math> <mi>𝐀</mi><mo>,</mo><mi>𝐙</mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule gras virgule, z majuscule gras")?;
    let expr = "<math> <mi>𝐚</mi><mo>,</mo><mi>𝐳</mi></math>";
    test("fr", "SimpleSpeak", expr, "a gras virgule, z gras")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule gras virgule, z majuscule gras")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a gras virgule, z gras")?;
    Ok(())
}


#[test]
fn italic() -> Result<()> {
    let expr = "<math> <mi>𝐴</mi><mo>,</mo><mi>𝑍</mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule virgule, z majuscule")?;
    let expr = "<math> <mi>𝑎</mi><mo>,</mo><mi>𝑧</mi></math>";
    test("fr", "SimpleSpeak", expr, "a virgule, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule virgule, z majuscule")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a virgule, z")?;
    Ok(())
}


#[test]
fn sans_serif() -> Result<()> {
  let expr = "<math> <mi>𝖠</mi><mo>,</mo><mi>𝖹</mi></math>";
  test("fr", "SimpleSpeak", expr, "a majuscule virgule, z majuscule")?;
  let expr = "<math> <mi>𝖺</mi><mo>,</mo><mi>𝗓</mi></math>";
  test("fr", "SimpleSpeak", expr, "a virgule, z")?;
  // MathType private space versions
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("fr", "SimpleSpeak", expr, "a majuscule virgule, z majuscule")?;
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("fr", "SimpleSpeak", expr, "a virgule, z")?;
  Ok(())
}


#[test]
fn sans_serif_bold() -> Result<()> {
    let expr = "<math> <mi>𝗔</mi><mo>,</mo><mi>𝗭</mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule gras virgule, z majuscule gras")?;
    let expr = "<math> <mi>𝗮</mi><mo>,</mo><mi>𝘇</mi></math>";
    test("fr", "SimpleSpeak", expr, "a gras virgule, z gras")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule gras virgule, z majuscule gras")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a gras virgule, z gras")?;
    Ok(())
}


#[test]
fn sans_serif_italic() -> Result<()> {
    let expr = "<math> <mi>𝘈</mi><mo>,</mo><mi>𝘡</mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule virgule, z majuscule")?;
    let expr = "<math> <mi>𝘢</mi><mo>,</mo><mi>𝘻</mi></math>";
    test("fr", "SimpleSpeak", expr, "a virgule, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule virgule, z majuscule")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a virgule, z")?;
    Ok(())
}


#[test]
fn sans_serif_bold_italic() -> Result<()> {
    let expr = "<math> <mi>𝘼</mi><mo>,</mo><mi>𝙕</mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule gras virgule, z majuscule gras")?;
    let expr = "<math> <mi>𝙖</mi><mo>,</mo><mi>𝙯</mi></math>";
    test("fr", "SimpleSpeak", expr, "a gras virgule, z gras")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule gras virgule, z majuscule gras")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a gras virgule, z gras")?;
    Ok(())
}


#[test]
fn monospace() -> Result<()> {
    let expr = "<math> <mi>𝙰</mi><mo>,</mo><mi>𝚉</mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule virgule, z majuscule")?;
    let expr = "<math> <mi>𝚊</mi><mo>,</mo><mi>𝚣</mi></math>";
    test("fr", "SimpleSpeak", expr, "a virgule, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a majuscule virgule, z majuscule")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "a virgule, z")?;
    Ok(())
}



#[test]
fn bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝚨</mi><mo>,</mo><mi>𝛀</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha majuscule gras, virgule, oméga majuscule gras")?;
    let expr = "<math> <mi>𝛂</mi><mo>,</mo><mi>𝛚</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha gras virgule, oméga gras")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha majuscule gras, virgule, oméga majuscule gras")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha gras virgule, oméga gras")?;
    Ok(())
}


#[test]
fn bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝛛</mi><mo>,</mo><mi>𝛡</mi></math>";
    test("fr", "SimpleSpeak", expr, "dérivée partielle gras, virgule, pi gras")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "dérivée partielle gras, virgule, pi gras")?;
    Ok(())
}


#[test]
fn italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝛢</mi><mo>,</mo><mi>𝛺</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha majuscule virgule, oméga majuscule")?;
    let expr = "<math> <mi>𝛼</mi><mo>,</mo><mi>𝜔</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha virgule, oméga")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha majuscule virgule, oméga majuscule")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha virgule, oméga")?;
    Ok(())
}


#[test]
fn italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝜕</mi><mo>,</mo><mi>𝜛</mi></math>";
    test("fr", "SimpleSpeak", expr, "dérivée partielle, virgule, pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "dérivée partielle, virgule, pi")?;
    Ok(())
}


#[test]
fn bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝜜</mi><mo>,</mo><mi>𝜴</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha majuscule gras, virgule, oméga majuscule gras")?;
    let expr = "<math> <mi>𝜶</mi><mo>,</mo><mi>𝝎</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha gras virgule, oméga gras")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha majuscule gras, virgule, oméga majuscule gras")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha gras virgule, oméga gras")?;
    Ok(())
}


#[test]
fn bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝝏</mi><mo>,</mo><mi>𝝕</mi></math>";
    test("fr", "SimpleSpeak", expr, "dérivée partielle gras, virgule, pi gras")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "dérivée partielle gras, virgule, pi gras")?;
    Ok(())
}


#[test]
fn sans_serif_bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝝖</mi><mo>,</mo><mi>𝝮</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha majuscule gras, virgule, oméga majuscule gras")?;
    let expr = "<math> <mi>𝝰</mi><mo>,</mo><mi>𝞈</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha gras virgule, oméga gras")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha majuscule gras, virgule, oméga majuscule gras")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha gras virgule, oméga gras")?;
    Ok(())
}


#[test]
fn sans_serif_bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝞉</mi><mo>,</mo><mi>𝞏</mi></math>";
    test("fr", "SimpleSpeak", expr, "dérivée partielle gras, virgule, pi gras")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "dérivée partielle gras, virgule, pi gras")?;
    Ok(())
}


#[test]
fn sans_serif_bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝞐</mi><mo>,</mo><mi>𝞨</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha majuscule gras, virgule, oméga majuscule gras")?;
    let expr = "<math> <mi>𝞪</mi><mo>,</mo><mi>𝟂</mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha gras virgule, oméga gras")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha majuscule gras, virgule, oméga majuscule gras")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "alpha gras virgule, oméga gras")?;
    Ok(())
}


#[test]
fn sans_serif_bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝟃</mi><mo>,</mo><mi>𝟉</mi></math>";
    test("fr", "SimpleSpeak", expr, "dérivée partielle gras, virgule, pi gras")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fr", "SimpleSpeak", expr, "dérivée partielle gras, virgule, pi gras")?;
    Ok(())
}


#[test]
fn pua_regular() -> Result<()> {
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("fr", "SimpleSpeak", expr, "a majuscule virgule, z majuscule")?;
  Ok(())
}


#[test]
fn turned() -> Result<()> {
    let expr = "<math> <mi>Ⅎ</mi><mo>,</mo><mi>⅄</mi></math>";
    test("fr", "SimpleSpeak", expr, "f majuscule tourné, virgule; y majuscule tourné sans-serif")?;
    Ok(())
  }


#[test]
fn unicode_typo_regressions() -> Result<()> {
  test("fr", "SimpleSpeak", "<math><mi>ⁱ</mi></math>", "exposant i")?;
  test("fr", "SimpleSpeak", "<math><mi>☌</mi></math>", "conjonction")?;
  Ok(())
}


#[test]
fn enclosed_numbers() -> Result<()> {
  let expr = "<math> <mi>①</mi><mo>,</mo><mi>⑨</mi></math>";
  test("fr", "SimpleSpeak", expr, "1 encerclé virgule, 9 encerclé")?;
  let expr = "<math> <mi>❶</mi><mo>,</mo><mi>㊿</mi></math>";
  test("fr", "SimpleSpeak", expr, "un encerclé de noir, virgule; numéro cinquante encerclé")?;
  let expr = "<math> <mi>⑴</mi><mo>,</mo><mi>⑼</mi></math>";
  test("fr", "SimpleSpeak", expr, "1 entre parenthèses, virgule, 9 entre parenthèses")?;
  let expr = "<math> <mi>⒈</mi><mo>,</mo><mi>⒐</mi></math>";
  test("fr", "SimpleSpeak", expr, "1 avec point virgule, 9 avec point")?;
  let expr = "<math> <mi>⓵</mi><mo>,</mo><mi>⓽</mi></math>";
  test("fr", "SimpleSpeak", expr, "1 double encerclé, virgule, 9 double encerclé")?;
  Ok(())
}
