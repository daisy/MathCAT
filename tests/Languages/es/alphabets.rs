/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;


#[test]
fn special_alphabet_chars() -> Result<()> {
  let expr = "<math> <mi>ℌ</mi><mo>,</mo><mi>ℭ</mi></math>";
  test("es", "SimpleSpeak", expr, "g cursiva mayúscula h, coma, g cursiva mayúscula c")?;
  let expr = "<math> <mi>ℍ</mi><mo>,</mo><mi>ℿ</mi></math>";
  test("es", "SimpleSpeak", expr, "doble tachadura mayúscula h, coma; doble tachadura mayúscula pi")?;
  let expr = "<math> <mi>ℐ</mi><mo>,</mo><mi>ℳ</mi></math>";
  test("es", "SimpleSpeak", expr, "h barra mayúscula i, coma, h barra mayúscula m")?;
  return Ok(());

}

#[test]
fn greek() -> Result<()> {
    let expr = "<math> <mi>Α</mi><mo>,</mo><mi>Ω</mi></math>";
    test("es", "SimpleSpeak", expr, "mayúscula alfa coma, mayúscula omega")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("es", "SimpleSpeak", expr, "alfa coma, omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "doble tachadura mayúscula delta, coma; doble tachadura mayúscula ípsilon")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("es", "SimpleSpeak", expr, "alfa coma, omega")?;
    return Ok(());

}

// REVISAR (inglés: ya): «mayúscula a coma, mayúscula ya»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn cap_cyrillic() -> Result<()> {
    let expr = "<math> <mi>А</mi><mo>,</mo><mi>Я</mi></math>";
    test("es", "SimpleSpeak", expr, "mayúscula a coma, mayúscula ya")?;
    return Ok(());

}

#[test]
fn parenthesized() -> Result<()> {
    let expr = "<math> <mi>⒜</mi><mo>,</mo><mi>⒵</mi></math>";
    test("es", "SimpleSpeak", expr, "entre paréntesis a coma, entre paréntesis z")?;
    return Ok(());

}

#[test]
fn circled() -> Result<()> {
    let expr = "<math> <mi>Ⓐ</mi><mo>,</mo><mi>Ⓩ</mi></math>";
    test("es", "SimpleSpeak", expr, "dentro de uncírculo mayúscula a, coma; dentro de uncírculo mayúscula z")?;
    let expr = "<math> <mi>🅐</mi><mo>,</mo><mi>🅩</mi></math>";
    test("es", "SimpleSpeak", expr, "dentro de un círculo negro mayúscula a, coma; dentro de un círculo negro mayúscula z")?;
    let expr = "<math> <mi>ⓐ</mi><mo>,</mo><mi>ⓩ</mi></math>";
    test("es", "SimpleSpeak", expr, "dentro de un círculo a, coma, dentro de un círculo z")?;
    return Ok(());

}

// REVISAR (inglés: fraktur): «fraktur mayúscula a, coma, fraktur mayúscula y»
// REVISAR (inglés: fraktur): «fraktur a coma, fraktur z»
// REVISAR (inglés: fraktur): «fraktur mayúscula a, coma, fraktur mayúscula y»
// REVISAR (inglés: fraktur): «fraktur a coma, fraktur z»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn fraktur() -> Result<()> {
    let expr = "<math> <mi>𝔄</mi><mo>,</mo><mi>𝔜</mi></math>";
    test("es", "SimpleSpeak", expr, "fraktur mayúscula a, coma, fraktur mayúscula y")?;
    let expr = "<math> <mi>𝔞</mi><mo>,</mo><mi>𝔷</mi></math>";
    test("es", "SimpleSpeak", expr, "fraktur a coma, fraktur z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "fraktur mayúscula a, coma, fraktur mayúscula y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "fraktur a coma, fraktur z")?;
    return Ok(());

}

// REVISAR (inglés: bold fraktur): «fraktur bold mayúscula a, coma, fraktur bold mayúscula z»
// REVISAR (inglés: bold fraktur): «fraktur bold a coma, fraktur bold z»
// REVISAR (inglés: bold fraktur): «fraktur bold mayúscula a, coma, fraktur bold mayúscula z»
// REVISAR (inglés: bold fraktur): «fraktur bold a coma, fraktur bold z»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn bold_fraktur() -> Result<()> {
    let expr = "<math> <mi>𝕬</mi><mo>,</mo><mi>𝖅</mi></math>";
    test("es", "SimpleSpeak", expr, "fraktur bold mayúscula a, coma, fraktur bold mayúscula z")?;
    let expr = "<math> <mi>𝖆</mi><mo>,</mo><mi>𝖟</mi></math>";
    test("es", "SimpleSpeak", expr, "fraktur bold a coma, fraktur bold z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "fraktur bold mayúscula a, coma, fraktur bold mayúscula z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "fraktur bold a coma, fraktur bold z")?;
    return Ok(());

}

#[test]
fn double_struck() -> Result<()> {
    let expr = "<math> <mi>𝔸</mi><mo>,</mo><mi>𝕐</mi></math>";
    test("es", "SimpleSpeak", expr, "doble tachadura mayúscula a, coma, doble tachadura mayúscula y")?;
    let expr = "<math> <mi>𝕒</mi><mo>,</mo><mi>𝕫</mi></math>";
    test("es", "SimpleSpeak", expr, "doble tachadura a coma, doble tachadura z")?;
    let expr = "<math> <mi>𝟘</mi><mo>,</mo><mi>𝟡</mi></math>";
    test("es", "SimpleSpeak", expr, "doble tachadura 0 coma, doble tachadura 9")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "doble tachadura mayúscula a, coma, doble tachadura mayúscula y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "doble tachadura a coma, doble tachadura z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "doble tachadura 0 coma, doble tachadura 9")?;
    return Ok(());

}

#[test]
fn script() -> Result<()> {
    let expr = "<math> <mi>𝒜</mi><mo>,</mo><mi>𝒵</mi></math>";
    test("es", "SimpleSpeak", expr, "guión mayúscula a, coma, guión mayúscula z")?;
    let expr = "<math> <mi>𝒶</mi><mo>,</mo><mi>𝓏</mi></math>";
    test("es", "SimpleSpeak", expr, "guión a coma, guión z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "guión mayúscula a, coma, guión mayúscula z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "guión a coma, guión z")?;
    return Ok(());

}

#[test]
fn bold_script() -> Result<()> {
    let expr = "<math> <mi>𝓐</mi><mo>,</mo><mi>𝓩</mi></math>";
    test("es", "SimpleSpeak", expr, "guión en negrita mayúscula a, coma; guión en negrita mayúscula z")?;
    let expr = "<math> <mi>𝓪</mi><mo>,</mo><mi>𝔃</mi></math>";
    test("es", "SimpleSpeak", expr, "guión en negrita a coma, guión en negrita z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "guión en negrita mayúscula a, coma; guión en negrita mayúscula z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "guión en negrita a coma, guión en negrita z")?;
    return Ok(());

}

#[test]
fn bold() -> Result<()> {
    let expr = "<math> <mi>𝐀</mi><mo>,</mo><mi>𝐙</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita mayúscula a, coma, negrita mayúscula z")?;
    let expr = "<math> <mi>𝐚</mi><mo>,</mo><mi>𝐳</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita a coma, negrita z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita mayúscula a, coma, negrita mayúscula z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita a coma, negrita z")?;
    return Ok(());

}

#[test]
fn italic() -> Result<()> {
    let expr = "<math> <mi>𝐴</mi><mo>,</mo><mi>𝑍</mi></math>";
    test("es", "SimpleSpeak", expr, "mayúscula a coma, mayúscula z")?;
    let expr = "<math> <mi>𝑎</mi><mo>,</mo><mi>𝑧</mi></math>";
    test("es", "SimpleSpeak", expr, "a coma, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "mayúscula a coma, mayúscula z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "a coma, z")?;
    return Ok(());

}

#[test]
fn sans_serif() -> Result<()> {
  let expr = "<math> <mi>𝖠</mi><mo>,</mo><mi>𝖹</mi></math>";
  test("es", "SimpleSpeak", expr, "mayúscula a coma, mayúscula z")?;
  let expr = "<math> <mi>𝖺</mi><mo>,</mo><mi>𝗓</mi></math>";
  test("es", "SimpleSpeak", expr, "a coma, z")?;
  // MathType private space versions
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("es", "SimpleSpeak", expr, "mayúscula a coma, mayúscula z")?;
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("es", "SimpleSpeak", expr, "a coma, z")?;
  return Ok(());

}

#[test]
fn sans_serif_bold() -> Result<()> {
    let expr = "<math> <mi>𝗔</mi><mo>,</mo><mi>𝗭</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita mayúscula a, coma, negrita mayúscula z")?;
    let expr = "<math> <mi>𝗮</mi><mo>,</mo><mi>𝘇</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita a coma, negrita z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita mayúscula a, coma, negrita mayúscula z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita a coma, negrita z")?;
    return Ok(());

}

#[test]
fn sans_serif_italic() -> Result<()> {
    let expr = "<math> <mi>𝘈</mi><mo>,</mo><mi>𝘡</mi></math>";
    test("es", "SimpleSpeak", expr, "mayúscula a coma, mayúscula z")?;
    let expr = "<math> <mi>𝘢</mi><mo>,</mo><mi>𝘻</mi></math>";
    test("es", "SimpleSpeak", expr, "a coma, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "mayúscula a coma, mayúscula z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "a coma, z")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic() -> Result<()> {
    let expr = "<math> <mi>𝘼</mi><mo>,</mo><mi>𝙕</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita mayúscula a, coma, negrita mayúscula z")?;
    let expr = "<math> <mi>𝙖</mi><mo>,</mo><mi>𝙯</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita a coma, negrita z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita mayúscula a, coma, negrita mayúscula z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita a coma, negrita z")?;
    return Ok(());

}

#[test]
fn monospace() -> Result<()> {
    let expr = "<math> <mi>𝙰</mi><mo>,</mo><mi>𝚉</mi></math>";
    test("es", "SimpleSpeak", expr, "mayúscula a coma, mayúscula z")?;
    let expr = "<math> <mi>𝚊</mi><mo>,</mo><mi>𝚣</mi></math>";
    test("es", "SimpleSpeak", expr, "a coma, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "mayúscula a coma, mayúscula z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "a coma, z")?;
    return Ok(());

}


#[test]
fn bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝚨</mi><mo>,</mo><mi>𝛀</mi></math>";
    test("es", "SimpleSpeak", expr, "j cursiva sin punto mayúscula alfa, coma; j cursiva sin punto mayúscula omega")?;
    let expr = "<math> <mi>𝛂</mi><mo>,</mo><mi>𝛚</mi></math>";
    test("es", "SimpleSpeak", expr, "j cursiva sin punto alfa, coma, j cursiva sin punto omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "j cursiva sin punto mayúscula alfa, coma; j cursiva sin punto mayúscula omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "j cursiva sin punto alfa, coma, j cursiva sin punto omega")?;
    return Ok(());

}

#[test]
fn bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝛛</mi><mo>,</mo><mi>𝛡</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita derivada parcial, coma, negrita pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita derivada parcial, coma, negrita pi")?;
    return Ok(());

}


#[test]
fn italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝛢</mi><mo>,</mo><mi>𝛺</mi></math>";
    test("es", "SimpleSpeak", expr, "mayúscula alfa coma, mayúscula omega")?;
    let expr = "<math> <mi>𝛼</mi><mo>,</mo><mi>𝜔</mi></math>";
    test("es", "SimpleSpeak", expr, "alfa coma, omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "mayúscula alfa coma, mayúscula omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "alfa coma, omega")?;
    return Ok(());

}

#[test]
fn italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝜕</mi><mo>,</mo><mi>𝜛</mi></math>";
    test("es", "SimpleSpeak", expr, "derivada parcial coma, pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "derivada parcial coma, pi")?;
    return Ok(());

}

#[test]
fn bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝜜</mi><mo>,</mo><mi>𝜴</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita mayúscula alfa, coma, negrita mayúscula omega")?;
    let expr = "<math> <mi>𝜶</mi><mo>,</mo><mi>𝝎</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita alfa coma, negrita omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita mayúscula alfa, coma, negrita mayúscula omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita alfa coma, negrita omega")?;
    return Ok(());

}

#[test]
fn bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝝏</mi><mo>,</mo><mi>𝝕</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita derivada parcial, coma, negrita pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita derivada parcial, coma, negrita pi")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝝖</mi><mo>,</mo><mi>𝝮</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita mayúscula alfa, coma, negrita mayúscula omega")?;
    let expr = "<math> <mi>𝝰</mi><mo>,</mo><mi>𝞈</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita alfa coma, negrita omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita mayúscula alfa, coma, negrita mayúscula omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita alfa coma, negrita omega")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝞉</mi><mo>,</mo><mi>𝞏</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita derivada parcial, coma, negrita pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita derivada parcial, coma, negrita pi")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝞐</mi><mo>,</mo><mi>𝞨</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita mayúscula alfa, coma, negrita mayúscula omega")?;
    let expr = "<math> <mi>𝞪</mi><mo>,</mo><mi>𝟂</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita alfa coma, negrita omega")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita mayúscula alfa, coma, negrita mayúscula omega")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita alfa coma, negrita omega")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝟃</mi><mo>,</mo><mi>𝟉</mi></math>";
    test("es", "SimpleSpeak", expr, "negrita derivada parcial, coma, negrita pi")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("es", "SimpleSpeak", expr, "negrita derivada parcial, coma, negrita pi")?;
    return Ok(());

}

#[test]
fn pua_regular() -> Result<()> {
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("es", "SimpleSpeak", expr, "mayúscula a coma, mayúscula z")?;
  return Ok(());

}

#[test]
fn turned() -> Result<()> {
    let expr = "<math> <mi>Ⅎ</mi><mo>,</mo><mi>⅄</mi></math>";
    test("es", "SimpleSpeak", expr, "transformado mayúscula f, coma, e cursiva mayúscula y")?;
    return Ok(());

  }

#[test]
fn up_tack_330() -> Result<()> {
    // perpendicular and up tack look the same. I added a special LiteralSpeak rule for up tack not to say "perpendicular"
    let perp = "<math><mi>a</mi><mo>⟂</mo><mi>b</mi></math>"; // 0x27c2
    test("es", "SimpleSpeak", perp, "a es perpendicular a b")?;
    test("es", "LiteralSpeak", perp, "a es perpendicular a b")?;
    let up_tack = "<math><mi>a</mi><mo>⊥</mo><mi>b</mi></math>"; // 0x22a5
    test("es", "ClearSpeak", up_tack, "a es perpendicular invertida b")?;
    test("es", "LiteralSpeak", up_tack, "a es perpendicular invertida b")?;
    return Ok(());
  }

#[test]
fn unicode_typo_regressions() -> Result<()> {
  test("es", "SimpleSpeak", "<math><mi>ⁱ</mi></math>", "superíndice i")?;
  test("es", "SimpleSpeak", "<math><mi>☌</mi></math>", "☌")?;
  Ok(())
}

#[test]
fn enclosed_numbers() -> Result<()> {
  let expr = "<math> <mi>①</mi><mo>,</mo><mi>⑨</mi></math>";
  test("es", "SimpleSpeak", expr, "corchete rotacional inferior 1, coma; corchete rotacional inferior 9")?;
  let expr = "<math> <mi>❶</mi><mo>,</mo><mi>㊿</mi></math>";
  test("es", "SimpleSpeak", expr, "círculo negro rodeando uno, coma; número cincuenta rodeado por un círculo")?;
  let expr = "<math> <mi>⑴</mi><mo>,</mo><mi>⑼</mi></math>";
  test("es", "SimpleSpeak", expr, "entre paréntesis 1 coma, entre paréntesis 9")?;
  let expr = "<math> <mi>⒈</mi><mo>,</mo><mi>⒐</mi></math>";
  test("es", "SimpleSpeak", expr, "1 con periodo coma, 9 con periodo")?;
  let expr = "<math> <mi>⓵</mi><mo>,</mo><mi>⓽</mi></math>";
  test("es", "SimpleSpeak", expr, "doble círculo 1 coma, doble círculo 9")?;
  return Ok(());

}
