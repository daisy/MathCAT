/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;

#[test]
fn enclosed_numbers() -> Result<()> {
  let expr = "<math> <mi>①</mi><mo>,</mo><mi>⑨</mi></math>";
  test("de", "SimpleSpeak", expr, "umkreiste 1 komma umkreiste 9")?;
  let expr = "<math> <mi>❶</mi><mo>,</mo><mi>㊿</mi></math>";
  test("de", "SimpleSpeak", expr, "schwarz umkreiste eins, komma umkreiste nummer fünfzig")?;
  let expr = "<math> <mi>⑴</mi><mo>,</mo><mi>⑼</mi></math>";
  test("de", "SimpleSpeak", expr, "klammern 1 komma klammern 9")?;
  let expr = "<math> <mi>⒈</mi><mo>,</mo><mi>⒐</mi></math>";
  test("de", "SimpleSpeak", expr, "1 mit periode komma 9 mit periode")?;
  let expr = "<math> <mi>⓵</mi><mo>,</mo><mi>⓽</mi></math>";
  test("de", "SimpleSpeak", expr, "doppelt umkreist 1 komma doppelt umkreist 9")?;
  return Ok(());

}

#[test]
fn mathematical_digit_names() -> Result<()> {
  let expr = "<math> <mi>𝟎</mi><mo>,</mo><mi>𝟏</mi><mo>,</mo><mi>𝟖</mi></math>";
  test("de", "SimpleSpeak", expr, "fett null komma fett eins komma fett acht")?;
  let expr = "<math> <mi>𝟪</mi><mo>,</mo><mi>𝟾</mi></math>";
  test("de", "SimpleSpeak", expr, "acht komma acht")?;
  Ok(())

}
