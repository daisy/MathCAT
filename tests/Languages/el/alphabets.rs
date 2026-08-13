/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;


#[test]
fn special_alphabet_chars() -> Result<()> {
  let expr = "<math> <mi>ℌ</mi><mo>,</mo><mi>ℭ</mi></math>";
  test("el", "SimpleSpeak", expr, "φράκτουρ κεφαλαίο h, κόμμα; φράκτουρ κεφαλαίο c")?;
  let expr = "<math> <mi>ℍ</mi><mo>,</mo><mi>ℿ</mi></math>";
  test("el", "SimpleSpeak", expr, "με διπλή γραφή κεφαλαίο h; κόμμα; με διπλή γραφή κεφαλαίο π")?;
  let expr = "<math> <mi>ℐ</mi><mo>,</mo><mi>ℳ</mi></math>";
  test("el", "SimpleSpeak", expr, "καλλιγραφικό κεφαλαίο i; κόμμα; καλλιγραφικό κεφαλαίο m")?;
  return Ok(());

}

#[test]
fn greek() -> Result<()> {
    let expr = "<math> <mi>Α</mi><mo>,</mo><mi>Ω</mi></math>";
    test("el", "SimpleSpeak", expr, "κεφαλαίο άλφα, κόμμα; κεφαλαίο ωμέγα")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("el", "SimpleSpeak", expr, "άλφα κόμμα, ωμέγα")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "διπλής γραφής κεφαλαίο δέλτα; κόμμα; διπλής γραφής κεφαλαίο ύψιλον")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("el", "SimpleSpeak", expr, "άλφα κόμμα, ωμέγα")?;
    return Ok(());

}

#[test]
fn cap_cyrillic() -> Result<()> {
    let expr = "<math> <mi>А</mi><mo>,</mo><mi>Я</mi></math>";
    test("el", "SimpleSpeak", expr, "κεφαλαίο α, κόμμα; κεφαλαίο γιά")?;
    return Ok(());

}

#[test]
fn parenthesized() -> Result<()> {
    let expr = "<math> <mi>⒜</mi><mo>,</mo><mi>⒵</mi></math>";
    test("el", "SimpleSpeak", expr, "εντός παρενθέσεων a, κόμμα; εντός παρενθέσεων z")?;
    return Ok(());

}

#[test]
fn circled() -> Result<()> {
    let expr = "<math> <mi>Ⓐ</mi><mo>,</mo><mi>Ⓩ</mi></math>";
    test("el", "SimpleSpeak", expr, "κυκλωμένο κεφαλαίο A, κόμμα; κυκλωμένο κεφαλαίο z")?;
    let expr = "<math> <mi>🅐</mi><mo>,</mo><mi>🅩</mi></math>";
    test("el", "SimpleSpeak", expr, "μαύρο κυκλωμένο κεφαλαίο A; κόμμα; μαύρο κυκλωμένο κεφαλαίο z")?;
    let expr = "<math> <mi>ⓐ</mi><mo>,</mo><mi>ⓩ</mi></math>";
    test("el", "SimpleSpeak", expr, "κυκλωμένο a, κόμμα; κυκλωμένο z")?;
    return Ok(());

}

#[test]
fn fraktur() -> Result<()> {
    let expr = "<math> <mi>𝔄</mi><mo>,</mo><mi>𝔜</mi></math>";
    test("el", "SimpleSpeak", expr, "φρακτούρ κεφαλαίο A, κόμμα; φρακτούρ κεφαλαίο y")?;
    let expr = "<math> <mi>𝔞</mi><mo>,</mo><mi>𝔷</mi></math>";
    test("el", "SimpleSpeak", expr, "φρακτούρ a, κόμμα; φρακτούρ z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "φρακτούρ κεφαλαίο A, κόμμα; φρακτούρ κεφαλαίο y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "φρακτούρ a, κόμμα; φρακτούρ z")?;
    return Ok(());

}

#[test]
fn bold_fraktur() -> Result<()> {
    let expr = "<math> <mi>𝕬</mi><mo>,</mo><mi>𝖅</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο φρακτούρ κεφαλαίο A; κόμμα; έντονο φρακτούρ κεφαλαίο z")?;
    let expr = "<math> <mi>𝖆</mi><mo>,</mo><mi>𝖟</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο φρακτούρ a, κόμμα; έντονο φρακτούρ z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο φρακτούρ κεφαλαίο A; κόμμα; έντονο φρακτούρ κεφαλαίο z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο φρακτούρ a, κόμμα; έντονο φρακτούρ z")?;
    return Ok(());

}

#[test]
fn double_struck() -> Result<()> {
    let expr = "<math> <mi>𝔸</mi><mo>,</mo><mi>𝕐</mi></math>";
    test("el", "SimpleSpeak", expr, "διπλής γραφής κεφαλαίο A; κόμμα; διπλής γραφής κεφαλαίο y")?;
    let expr = "<math> <mi>𝕒</mi><mo>,</mo><mi>𝕫</mi></math>";
    test("el", "SimpleSpeak", expr, "διπλής γραφής a, κόμμα; διπλής γραφής z")?;
    let expr = "<math> <mi>𝟘</mi><mo>,</mo><mi>𝟡</mi></math>";
    test("el", "SimpleSpeak", expr, "διπλής γραφής 0, κόμμα; διπλής γραφής 9")?;  
    // doesn't read 0 or 9
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "διπλής γραφής κεφαλαίο A; κόμμα; διπλής γραφής κεφαλαίο y")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "διπλής γραφής a, κόμμα; διπλής γραφής z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "διπλής γραφής 0, κόμμα; διπλής γραφής 9")?;
    return Ok(());

}

#[test]
fn script() -> Result<()> {
    let expr = "<math> <mi>𝒜</mi><mo>,</mo><mi>𝒵</mi></math>";
    test("el", "SimpleSpeak", expr, "καλλιγραφικό κεφαλαίο A; κόμμα; καλλιγραφικό κεφαλαίο z")?;
    let expr = "<math> <mi>𝒶</mi><mo>,</mo><mi>𝓏</mi></math>";
    test("el", "SimpleSpeak", expr, "καλλιγραφικό a, κόμμα; καλλιγραφικό z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "καλλιγραφικό κεφαλαίο A; κόμμα; καλλιγραφικό κεφαλαίο z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "καλλιγραφικό a, κόμμα; καλλιγραφικό z")?;
    return Ok(());

}

#[test]
fn bold_script() -> Result<()> {
    let expr = "<math> <mi>𝓐</mi><mo>,</mo><mi>𝓩</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο καλλιγραφικό κεφαλαίο A; κόμμα; έντονο καλλιγραφικό κεφαλαίο z")?;
    let expr = "<math> <mi>𝓪</mi><mo>,</mo><mi>𝔃</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο καλλιγραφικό a, κόμμα; έντονο καλλιγραφικό z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο καλλιγραφικό κεφαλαίο A; κόμμα; έντονο καλλιγραφικό κεφαλαίο z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο καλλιγραφικό a, κόμμα; έντονο καλλιγραφικό z")?;
    return Ok(());

}

#[test]
fn bold() -> Result<()> {
    let expr = "<math> <mi>𝐀</mi><mo>,</mo><mi>𝐙</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο κεφαλαίο A, κόμμα; έντονο κεφαλαίο z")?;
    let expr = "<math> <mi>𝐚</mi><mo>,</mo><mi>𝐳</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο a κόμμα, έντονο z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο κεφαλαίο A, κόμμα; έντονο κεφαλαίο z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο a κόμμα, έντονο z")?;
    return Ok(());

}

#[test]
fn italic() -> Result<()> {
    let expr = "<math> <mi>𝐴</mi><mo>,</mo><mi>𝑍</mi></math>";
    test("el", "SimpleSpeak", expr, "κεφαλαίο A, κόμμα; κεφαλαίο z")?;
    let expr = "<math> <mi>𝑎</mi><mo>,</mo><mi>𝑧</mi></math>";
    test("el", "SimpleSpeak", expr, "a κόμμα, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "κεφαλαίο A, κόμμα; κεφαλαίο z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "a κόμμα, z")?;
    return Ok(());

}

#[test]
fn sans_serif() -> Result<()> {
  let expr = "<math> <mi>𝖠</mi><mo>,</mo><mi>𝖹</mi></math>";
  test("el", "SimpleSpeak", expr, "κεφαλαίο A, κόμμα; κεφαλαίο z")?;
  let expr = "<math> <mi>𝖺</mi><mo>,</mo><mi>𝗓</mi></math>";
  test("el", "SimpleSpeak", expr, "a κόμμα, z")?;
  // MathType private space versions
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("el", "SimpleSpeak", expr, "κεφαλαίο A, κόμμα; κεφαλαίο z")?;
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("el", "SimpleSpeak", expr, "a κόμμα, z")?;
  return Ok(());

}

#[test]
fn sans_serif_bold() -> Result<()> {
    let expr = "<math> <mi>𝗔</mi><mo>,</mo><mi>𝗭</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο κεφαλαίο A, κόμμα; έντονο κεφαλαίο z")?;
    let expr = "<math> <mi>𝗮</mi><mo>,</mo><mi>𝘇</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο a κόμμα, έντονο z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο κεφαλαίο A, κόμμα; έντονο κεφαλαίο z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο a κόμμα, έντονο z")?;
    return Ok(());

}

#[test]
fn sans_serif_italic() -> Result<()> {
    let expr = "<math> <mi>𝘈</mi><mo>,</mo><mi>𝘡</mi></math>";
    test("el", "SimpleSpeak", expr, "κεφαλαίο A, κόμμα; κεφαλαίο z")?;
    let expr = "<math> <mi>𝘢</mi><mo>,</mo><mi>𝘻</mi></math>";
    test("el", "SimpleSpeak", expr, "a κόμμα, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "κεφαλαίο A, κόμμα; κεφαλαίο z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "a κόμμα, z")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic() -> Result<()> {
    let expr = "<math> <mi>𝘼</mi><mo>,</mo><mi>𝙕</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο κεφαλαίο A, κόμμα; έντονο κεφαλαίο z")?;
    let expr = "<math> <mi>𝙖</mi><mo>,</mo><mi>𝙯</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο a κόμμα, έντονο z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο κεφαλαίο A, κόμμα; έντονο κεφαλαίο z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο a κόμμα, έντονο z")?;
    return Ok(());

}

#[test]
fn monospace() -> Result<()> {
    let expr = "<math> <mi>𝙰</mi><mo>,</mo><mi>𝚉</mi></math>";
    test("el", "SimpleSpeak", expr, "κεφαλαίο A, κόμμα; κεφαλαίο z")?;
    let expr = "<math> <mi>𝚊</mi><mo>,</mo><mi>𝚣</mi></math>";
    test("el", "SimpleSpeak", expr, "a κόμμα, z")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "κεφαλαίο A, κόμμα; κεφαλαίο z")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "a κόμμα, z")?;
    return Ok(());

}


#[test]
fn bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝚨</mi><mo>,</mo><mi>𝛀</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο κεφαλαίο άλφα, κόμμα; έντονο κεφαλαίο ωμέγα")?;
    let expr = "<math> <mi>𝛂</mi><mo>,</mo><mi>𝛚</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο άλφα, κόμμα; έντονο ωμέγα")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο κεφαλαίο άλφα, κόμμα; έντονο κεφαλαίο ωμέγα")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο άλφα, κόμμα; έντονο ωμέγα")?;
    return Ok(());

}

#[test]
fn bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝛛</mi><mo>,</mo><mi>𝛡</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο μερικό διαφορικό; κόμμα, έντονο π")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο μερικό διαφορικό; κόμμα, έντονο π")?;
    return Ok(());

}


#[test]
fn italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝛢</mi><mo>,</mo><mi>𝛺</mi></math>";
    test("el", "SimpleSpeak", expr, "κεφαλαίο άλφα, κόμμα; κεφαλαίο ωμέγα")?;
    let expr = "<math> <mi>𝛼</mi><mo>,</mo><mi>𝜔</mi></math>";
    test("el", "SimpleSpeak", expr, "άλφα κόμμα, ωμέγα")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "κεφαλαίο άλφα, κόμμα; κεφαλαίο ωμέγα")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "άλφα κόμμα, ωμέγα")?;
    return Ok(());

}

#[test]
fn italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝜕</mi><mo>,</mo><mi>𝜛</mi></math>";
    test("el", "SimpleSpeak", expr, "μερικό διαφορικό, κόμμα, π")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "μερικό διαφορικό, κόμμα, π")?;
    return Ok(());

}

#[test]
fn bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝜜</mi><mo>,</mo><mi>𝜴</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο κεφαλαίο άλφα, κόμμα; έντονο κεφαλαίο ωμέγα")?;
    let expr = "<math> <mi>𝜶</mi><mo>,</mo><mi>𝝎</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο άλφα, κόμμα; έντονο ωμέγα")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο κεφαλαίο άλφα, κόμμα; έντονο κεφαλαίο ωμέγα")?; 
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο άλφα, κόμμα; έντονο ωμέγα")?;
    return Ok(());

}

#[test]
fn bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝝏</mi><mo>,</mo><mi>𝝕</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο μερικό διαφορικό; κόμμα, έντονο π")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο μερικό διαφορικό; κόμμα, έντονο π")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝝖</mi><mo>,</mo><mi>𝝮</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο κεφαλαίο άλφα, κόμμα; έντονο κεφαλαίο ωμέγα")?;
    let expr = "<math> <mi>𝝰</mi><mo>,</mo><mi>𝞈</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο άλφα, κόμμα; έντονο ωμέγα")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο κεφαλαίο άλφα, κόμμα; έντονο κεφαλαίο ωμέγα")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο άλφα, κόμμα; έντονο ωμέγα")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝞉</mi><mo>,</mo><mi>𝞏</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο μερικό διαφορικό; κόμμα, έντονο π")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο μερικό διαφορικό; κόμμα, έντονο π")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝞐</mi><mo>,</mo><mi>𝞨</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο κεφαλαίο άλφα, κόμμα; έντονο κεφαλαίο ωμέγα")?;
    let expr = "<math> <mi>𝞪</mi><mo>,</mo><mi>𝟂</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο άλφα, κόμμα; έντονο ωμέγα")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο κεφαλαίο άλφα, κόμμα; έντονο κεφαλαίο ωμέγα")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο άλφα, κόμμα; έντονο ωμέγα")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝟃</mi><mo>,</mo><mi>𝟉</mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο μερικό διαφορικό; κόμμα, έντονο π")?;
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("el", "SimpleSpeak", expr, "έντονο μερικό διαφορικό; κόμμα, έντονο π")?;
    return Ok(());

}

#[test]
fn pua_regular() -> Result<()> {
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("el", "SimpleSpeak", expr, "κεφαλαίο A, κόμμα; κεφαλαίο z")?;
  return Ok(());

}

#[test]
fn turned() -> Result<()> {
    let expr = "<math> <mi>Ⅎ</mi><mo>,</mo><mi>⅄</mi></math>";
    test("el", "SimpleSpeak", expr, "ανάποδο κεφαλαίο f, κόμμα; ανάποδο χωρίς σέριφ κεφαλαίο y")?;
    return Ok(());

  }

#[test]
fn unicode_typo_regressions() -> Result<()> {
  test("el", "SimpleSpeak", "<math><mi>ⁱ</mi></math>", "εις την i οστή δύναμη")?;
  test("el", "SimpleSpeak", "<math><mi>☌</mi></math>", "σύνοδος")?; 
  Ok(())
}

#[test]
fn enclosed_numbers() -> Result<()> {
  let expr = "<math> <mi>①</mi><mo>,</mo><mi>⑨</mi></math>";
  test("el", "SimpleSpeak", expr, "κυκλωμένο 1, κόμμα; κυκλωμένο 9")?;
  let expr = "<math> <mi>❶</mi><mo>,</mo><mi>㊿</mi></math>";
  test("el", "SimpleSpeak", expr, "κυκλωμένο μαύρο ένα, κόμμα; κυκλωμένος αριθμός πενήντα")?;
  let expr = "<math> <mi>⑴</mi><mo>,</mo><mi>⑼</mi></math>";
  test("el", "SimpleSpeak", expr, "εντός παρενθέσεων 1, κόμμα; εντός παρενθέσεων 9")?;
  let expr = "<math> <mi>⒈</mi><mo>,</mo><mi>⒐</mi></math>";
  test("el", "SimpleSpeak", expr, "1 με τελεία, κόμμα; 9 με τελεία")?;
  let expr = "<math> <mi>⓵</mi><mo>,</mo><mi>⓽</mi></math>";
  test("el", "SimpleSpeak", expr, "διπλά κυκλωμένο 1, κόμμα; διπλά κυκλωμένο 9")?;
  return Ok(());

}
