/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;


#[test]
fn special_alphabet_chars() -> Result<()> {
  let expr = "<math> <mi>â„Œ</mi><mo>,</mo><mi>â„­</mi></math>";
  test("pt", "SimpleSpeak", expr, "fraktur cap h comma, fraktur cap c")?;
  let expr = "<math> <mi>â„</mi><mo>,</mo><mi>â„¿</mi></math>";
  test("pt", "SimpleSpeak", expr, "double struck cap h, comma, double struck cap pi")?;
  let expr = "<math> <mi>â„</mi><mo>,</mo><mi>â„³</mi></math>";
  test("pt", "SimpleSpeak", expr, "script cap i comma, script cap m")?;
  return Ok(());

}

#[test]
fn greek() -> Result<()> {
    let expr = "<math> <mi>Î‘</mi><mo>,</mo><mi>Î©</mi></math>";
    test("pt", "SimpleSpeak", expr, "cap alpha comma, cap omega")?;
    let expr = "<math> <mi>Î±</mi><mo>,</mo><mi>Ï‰</mi></math>";
    test("pt", "SimpleSpeak", expr, "alpha comma, omega")?;
    // MathType private space versions
    let expr = "<math> <mi>ïˆ</mi><mo>,</mo><mi>ïˆ‰</mi></math>";
    test("pt", "SimpleSpeak", expr, "double struck cap delta, comma, double struck cap upsilon")?;
    let expr = "<math> <mi>Î±</mi><mo>,</mo><mi>Ï‰</mi></math>";
    test("pt", "SimpleSpeak", expr, "alpha comma, omega")?;
    return Ok(());

}

#[test]
fn cap_cyrillic() -> Result<()> {
    let expr = "<math> <mi>Ð</mi><mo>,</mo><mi>Ð¯</mi></math>";
    test("pt", "SimpleSpeak", expr, "cap a comma, cap ya")?;
    return Ok(());

}

#[test]
fn parenthesized() -> Result<()> {
    let expr = "<math> <mi>â’œ</mi><mo>,</mo><mi>â’µ</mi></math>";
    test("pt", "SimpleSpeak", expr, "parenthesized eigh comma, parenthesized z")?;
    return Ok(());

}

#[test]
fn circled() -> Result<()> {
    let expr = "<math> <mi>â’¶</mi><mo>,</mo><mi>â“</mi></math>";
    test("pt", "SimpleSpeak", expr, "circled cap eigh comma, circled cap z")?;
    let expr = "<math> <mi>ðŸ…</mi><mo>,</mo><mi>ðŸ…©</mi></math>";
    test("pt", "SimpleSpeak", expr, "black circled cap eigh, comma, black circled cap z")?;
    let expr = "<math> <mi>â“</mi><mo>,</mo><mi>â“©</mi></math>";
    test("pt", "SimpleSpeak", expr, "circled eigh comma, circled z")?;
    return Ok(());

}

#[test]
fn fraktur() -> Result<()> {
    let expr = "<math> <mi>ð”„</mi><mo>,</mo><mi>ð”œ</mi></math>";
    test("pt", "SimpleSpeak", expr, "fraktur cap eigh comma, fraktur cap y")?;
    let expr = "<math> <mi>ð”ž</mi><mo>,</mo><mi>ð”·</mi></math>";
    test("pt", "SimpleSpeak", expr, "fraktur eigh comma, fraktur z")?;
    // MathType private space versions
    let expr = "<math> <mi>ï€€</mi><mo>,</mo><mi>ï€˜</mi></math>";
    test("pt", "SimpleSpeak", expr, "fraktur cap eigh comma, fraktur cap y")?;
    let expr = "<math> <mi>ï€š</mi><mo>,</mo><mi>ï€³</mi></math>";
    test("pt", "SimpleSpeak", expr, "fraktur eigh comma, fraktur z")?;
    return Ok(());

}

#[test]
fn bold_fraktur() -> Result<()> {
    let expr = "<math> <mi>ð•¬</mi><mo>,</mo><mi>ð–…</mi></math>";
    test("pt", "SimpleSpeak", expr, "fraktur bold cap eigh, comma, fraktur bold cap z")?;
    let expr = "<math> <mi>ð–†</mi><mo>,</mo><mi>ð–Ÿ</mi></math>";
    test("pt", "SimpleSpeak", expr, "fraktur bold eigh comma, fraktur bold z")?;
    // MathType private space versions
    let expr = "<math> <mi>ï€</mi><mo>,</mo><mi>ï™</mi></math>";
    test("pt", "SimpleSpeak", expr, "fraktur bold cap eigh, comma, fraktur bold cap z")?;
    let expr = "<math> <mi>ïš</mi><mo>,</mo><mi>ï³</mi></math>";
    test("pt", "SimpleSpeak", expr, "fraktur bold eigh comma, fraktur bold z")?;
    return Ok(());

}

#[test]
fn double_struck() -> Result<()> {
    let expr = "<math> <mi>ð”¸</mi><mo>,</mo><mi>ð•</mi></math>";
    test("pt", "SimpleSpeak", expr, "double struck cap eigh, comma, double struck cap y")?;
    let expr = "<math> <mi>ð•’</mi><mo>,</mo><mi>ð•«</mi></math>";
    test("pt", "SimpleSpeak", expr, "double struck eigh comma, double struck z")?;
    let expr = "<math> <mi>ðŸ˜</mi><mo>,</mo><mi>ðŸ¡</mi></math>";
    test("pt", "SimpleSpeak", expr, "double struck 0 comma, double struck 9")?;
    // MathType private space versions
    let expr = "<math> <mi>ï‚€</mi><mo>,</mo><mi>ï‚˜</mi></math>";
    test("pt", "SimpleSpeak", expr, "double struck cap eigh, comma, double struck cap y")?;
    let expr = "<math> <mi>ï‚š</mi><mo>,</mo><mi>ï‚³</mi></math>";
    test("pt", "SimpleSpeak", expr, "double struck eigh comma, double struck z")?;
    let expr = "<math> <mi>ïƒ€</mi><mo>,</mo><mi>ïƒ‰</mi></math>";
    test("pt", "SimpleSpeak", expr, "double struck 0 comma, double struck 9")?;
    return Ok(());

}

#[test]
fn script() -> Result<()> {
    let expr = "<math> <mi>ð’œ</mi><mo>,</mo><mi>ð’µ</mi></math>";
    test("pt", "SimpleSpeak", expr, "script cap eigh comma, script cap z")?;
    let expr = "<math> <mi>ð’¶</mi><mo>,</mo><mi>ð“</mi></math>";
    test("pt", "SimpleSpeak", expr, "script eigh comma, script z")?;
    // MathType private space versions
    let expr = "<math> <mi>ï„€</mi><mo>,</mo><mi>ï„™</mi></math>";
    test("pt", "SimpleSpeak", expr, "script cap eigh comma, script cap z")?;
    let expr = "<math> <mi>ï„š</mi><mo>,</mo><mi>ï„³</mi></math>";
    test("pt", "SimpleSpeak", expr, "script eigh comma, script z")?;
    return Ok(());

}

#[test]
fn bold_script() -> Result<()> {
    let expr = "<math> <mi>ð“</mi><mo>,</mo><mi>ð“©</mi></math>";
    test("pt", "SimpleSpeak", expr, "script bold cap eigh, comma, script bold cap z")?;
    let expr = "<math> <mi>ð“ª</mi><mo>,</mo><mi>ð”ƒ</mi></math>";
    test("pt", "SimpleSpeak", expr, "script bold eigh comma, script bold z")?;
    // MathType private space versions
    let expr = "<math> <mi>ï…€</mi><mo>,</mo><mi>ï…™</mi></math>";
    test("pt", "SimpleSpeak", expr, "script bold cap eigh, comma, script bold cap z")?;
    let expr = "<math> <mi>ï…š</mi><mo>,</mo><mi>ï…³</mi></math>";
    test("pt", "SimpleSpeak", expr, "script bold eigh comma, script bold z")?;
    return Ok(());

}

#[test]
fn bold() -> Result<()> {
    let expr = "<math> <mi>ð€</mi><mo>,</mo><mi>ð™</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold cap eigh comma, bold cap z")?;
    let expr = "<math> <mi>ðš</mi><mo>,</mo><mi>ð³</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold eigh comma, bold z")?;
    // MathType private space versions
    let expr = "<math> <mi>ï‰ </mi><mo>,</mo><mi>ï‰¹</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold cap eigh comma, bold cap z")?;
    let expr = "<math> <mi>ï‰º</mi><mo>,</mo><mi>ïŠ“</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold eigh comma, bold z")?;
    return Ok(());

}

#[test]
fn italic() -> Result<()> {
    let expr = "<math> <mi>ð´</mi><mo>,</mo><mi>ð‘</mi></math>";
    test("pt", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
    let expr = "<math> <mi>ð‘Ž</mi><mo>,</mo><mi>ð‘§</mi></math>";
    test("pt", "SimpleSpeak", expr, "eigh comma, z")?;
    // MathType private space versions
    let expr = "<math> <mi>ïŠ”</mi><mo>,</mo><mi>ïŠ­</mi></math>";
    test("pt", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
    let expr = "<math> <mi>ïŠ®</mi><mo>,</mo><mi>ï‹‡</mi></math>";
    test("pt", "SimpleSpeak", expr, "eigh comma, z")?;
    return Ok(());

}

#[test]
fn sans_serif() -> Result<()> {
  let expr = "<math> <mi>ð– </mi><mo>,</mo><mi>ð–¹</mi></math>";
  test("pt", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
  let expr = "<math> <mi>ð–º</mi><mo>,</mo><mi>ð—“</mi></math>";
  test("pt", "SimpleSpeak", expr, "eigh comma, z")?;
  // MathType private space versions
  let expr = "<math> <mi>ïŒ€</mi><mo>,</mo><mi>ïŒ™</mi></math>";
  test("pt", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
  let expr = "<math> <mi>ïŒš</mi><mo>,</mo><mi>ïŒ³</mi></math>";
  test("pt", "SimpleSpeak", expr, "eigh comma, z")?;
  return Ok(());

}

#[test]
fn sans_serif_bold() -> Result<()> {
    let expr = "<math> <mi>ð—”</mi><mo>,</mo><mi>ð—­</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold cap eigh comma, bold cap z")?;
    let expr = "<math> <mi>ð—®</mi><mo>,</mo><mi>ð˜‡</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold eigh comma, bold z")?;
    // MathType private space versions
    let expr = "<math> <mi>ïŒ´</mi><mo>,</mo><mi>ï</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold cap eigh comma, bold cap z")?;
    let expr = "<math> <mi>ïŽ</mi><mo>,</mo><mi>ï§</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold eigh comma, bold z")?;
    return Ok(());

}

#[test]
fn sans_serif_italic() -> Result<()> {
    let expr = "<math> <mi>ð˜ˆ</mi><mo>,</mo><mi>ð˜¡</mi></math>";
    test("pt", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
    let expr = "<math> <mi>ð˜¢</mi><mo>,</mo><mi>ð˜»</mi></math>";
    test("pt", "SimpleSpeak", expr, "eigh comma, z")?;
    // MathType private space versions
    let expr = "<math> <mi>ï¨</mi><mo>,</mo><mi>ïŽ</mi></math>";
    test("pt", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
    let expr = "<math> <mi>ïŽ‚</mi><mo>,</mo><mi>ïŽ›</mi></math>";
    test("pt", "SimpleSpeak", expr, "eigh comma, z")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic() -> Result<()> {
    let expr = "<math> <mi>ð˜¼</mi><mo>,</mo><mi>ð™•</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold cap eigh comma, bold cap z")?;
    let expr = "<math> <mi>ð™–</mi><mo>,</mo><mi>ð™¯</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold eigh comma, bold z")?;
    // MathType private space versions
    let expr = "<math> <mi>ïŽœ</mi><mo>,</mo><mi>ïŽµ</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold cap eigh comma, bold cap z")?;
    let expr = "<math> <mi>ïŽ¶</mi><mo>,</mo><mi>ï</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold eigh comma, bold z")?;
    return Ok(());

}

#[test]
fn monospace() -> Result<()> {
    let expr = "<math> <mi>ð™°</mi><mo>,</mo><mi>ðš‰</mi></math>";
    test("pt", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
    let expr = "<math> <mi>ðšŠ</mi><mo>,</mo><mi>ðš£</mi></math>";
    test("pt", "SimpleSpeak", expr, "eigh comma, z")?;
    // MathType private space versions
    let expr = "<math> <mi>ï</mi><mo>,</mo><mi>ï©</mi></math>";
    test("pt", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
    let expr = "<math> <mi>ïª</mi><mo>,</mo><mi>ïƒ</mi></math>";
    test("pt", "SimpleSpeak", expr, "eigh comma, z")?;
    return Ok(());

}


#[test]
fn bold_greek() -> Result<()> {
    let expr = "<math> <mi>ðš¨</mi><mo>,</mo><mi>ð›€</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi>ð›‚</mi><mo>,</mo><mi>ð›š</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    // MathType private space versions
    let expr = "<math> <mi>ïˆ</mi><mo>,</mo><mi>ï </mi></math>";
    test("pt", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi>ï¢</mi><mo>,</mo><mi>ïº</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    return Ok(());

}

#[test]
fn bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>ð››</mi><mo>,</mo><mi>ð›¡</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    // MathType private space versions
    let expr = "<math> <mi>ï»</mi><mo>,</mo><mi>ï‘</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    return Ok(());

}


#[test]
fn italic_greek() -> Result<()> {
    let expr = "<math> <mi>ð›¢</mi><mo>,</mo><mi>ð›º</mi></math>";
    test("pt", "SimpleSpeak", expr, "cap alpha comma, cap omega")?;
    let expr = "<math> <mi>ð›¼</mi><mo>,</mo><mi>ðœ”</mi></math>";
    test("pt", "SimpleSpeak", expr, "alpha comma, omega")?;
    // MathType private space versions
    let expr = "<math> <mi>ï‘‚</mi><mo>,</mo><mi>ï‘š</mi></math>";
    test("pt", "SimpleSpeak", expr, "cap alpha comma, cap omega")?;
    let expr = "<math> <mi>ï‘œ</mi><mo>,</mo><mi>ï‘´</mi></math>";
    test("pt", "SimpleSpeak", expr, "alpha comma, omega")?;
    return Ok(());

}

#[test]
fn italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>ðœ•</mi><mo>,</mo><mi>ðœ›</mi></math>";
    test("pt", "SimpleSpeak", expr, "partial derivative comma, pi")?;
    // MathType private space versions
    let expr = "<math> <mi>ï‘µ</mi><mo>,</mo><mi>ï‘»</mi></math>";
    test("pt", "SimpleSpeak", expr, "partial derivative comma, pi")?;
    return Ok(());

}

#[test]
fn bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>ðœœ</mi><mo>,</mo><mi>ðœ´</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi>ðœ¶</mi><mo>,</mo><mi>ðŽ</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    // MathType private space versions
    let expr = "<math> <mi>ï‘¼</mi><mo>,</mo><mi>ï’”</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi>ï’–</mi><mo>,</mo><mi>ï’®</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    return Ok(());

}

#[test]
fn bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>ð</mi><mo>,</mo><mi>ð•</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    // MathType private space versions
    let expr = "<math> <mi>ï’¯</mi><mo>,</mo><mi>ï’µ</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_greek() -> Result<()> {
    let expr = "<math> <mi>ð–</mi><mo>,</mo><mi>ð®</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi>ð°</mi><mo>,</mo><mi>ðžˆ</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    // MathType private space versions
    let expr = "<math> <mi>ï’¶</mi><mo>,</mo><mi>ï“Ž</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi>ï“</mi><mo>,</mo><mi>ï“¨</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>ðž‰</mi><mo>,</mo><mi>ðž</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    // MathType private space versions
    let expr = "<math> <mi>ï“©</mi><mo>,</mo><mi>ï“¯</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>ðž</mi><mo>,</mo><mi>ðž¨</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi>ðžª</mi><mo>,</mo><mi>ðŸ‚</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    // MathType private space versions
    let expr = "<math> <mi>ï“°</mi><mo>,</mo><mi>ï”ˆ</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold cap alpha comma, bold cap omega")?;
    let expr = "<math> <mi>ï”Š</mi><mo>,</mo><mi>ï”¢</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold alpha comma, bold omega")?;
    return Ok(());

}

#[test]
fn sans_serif_bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>ðŸƒ</mi><mo>,</mo><mi>ðŸ‰</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    // MathType private space versions
    let expr = "<math> <mi>ï”£</mi><mo>,</mo><mi>ï”©</mi></math>";
    test("pt", "SimpleSpeak", expr, "bold partial derivative, comma, bold pi")?;
    return Ok(());

}

#[test]
fn pua_regular() -> Result<()> {
  let expr = "<math> <mi>ï†€</mi><mo>,</mo><mi>ï†™</mi></math>";
  test("pt", "SimpleSpeak", expr, "cap eigh comma, cap z")?;
  return Ok(());

}

#[test]
fn turned() -> Result<()> {
    let expr = "<math> <mi>â„²</mi><mo>,</mo><mi>â…„</mi></math>";
    test("pt", "SimpleSpeak", expr, "turned cap f comma, turned sans-serif cap y")?;
    return Ok(());

  }

#[test]
fn up_tack_330() -> Result<()> {
    // perpendicular and up tack look the same. I added a special LiteralSpeak rule for up tack not to say "perpendicular"
    let perp = "<math><mi>a</mi><mo>âŸ‚</mo><mi>b</mi></math>"; // 0x27c2
    test("pt", "SimpleSpeak", perp, "eigh is perpendicular to b")?;
    test("pt", "LiteralSpeak", perp, "eigh is perpendicular to b")?;
    let up_tack = "<math><mi>a</mi><mo>âŠ¥</mo><mi>b</mi></math>"; // 0x22a5
    test("pt", "ClearSpeak", up_tack, "eigh is perpendicular to b")?;
    test("pt", "LiteralSpeak", up_tack, "eigh up tack b")?;
    return Ok(());
  }

#[test]
fn unicode_typo_regressions() -> Result<()> {
  test("pt", "SimpleSpeak", "<math><mi>â±</mi></math>", "to the i-th power")?;
  test("pt", "SimpleSpeak", "<math><mi>â˜Œ</mi></math>", "conjunction")?;
  Ok(())
}

#[test]
fn enclosed_numbers() -> Result<()> {
  let expr = "<math> <mi>â‘ </mi><mo>,</mo><mi>â‘¨</mi></math>";
  test("pt", "SimpleSpeak", expr, "circled 1 comma, circled 9")?;
  let expr = "<math> <mi>â¶</mi><mo>,</mo><mi>ãŠ¿</mi></math>";
  test("pt", "SimpleSpeak", expr, "black circled one comma, circled number fifty")?;
  let expr = "<math> <mi>â‘´</mi><mo>,</mo><mi>â‘¼</mi></math>";
  test("pt", "SimpleSpeak", expr, "parenthesized 1 comma, parenthesized 9")?;
  let expr = "<math> <mi>â’ˆ</mi><mo>,</mo><mi>â’</mi></math>";
  test("pt", "SimpleSpeak", expr, "1 with period comma, 9 with period")?;
  let expr = "<math> <mi>â“µ</mi><mo>,</mo><mi>â“½</mi></math>";
  test("pt", "SimpleSpeak", expr, "double circled 1 comma, double circled 9")?;
  return Ok(());

}
