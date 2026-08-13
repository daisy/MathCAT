use crate::common::*;
use anyhow::Result;

#[test]
fn uoa_corpus_001() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mn>1</mn><mo>−</mo><mfrac>
    <mn>1</mn>
    <mn>2</mn>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mn>3</mn>
   </mfrac>
   <mo>−</mo><mfrac>
    <mn>1</mn>
    <mn>4</mn>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mn>5</mn>
   </mfrac>
   <mo>−</mo><mo>⋯</mo><mo>=</mo><mi>ln</mi><mn>2</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "1 μείον 1 δεύτερο συν 1 τρίτο μείον 1 τέταρτο συν 1 πέμπτο μείον αποσιωπητικά; ισούται με, το φυσικό λογάριθμο, του 2")?;
    return Ok(());
}

#[test]
fn uoa_corpus_002() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>1</mn><mo>−</mo><mfrac>
    <mn>1</mn>
    <mn>3</mn>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mn>5</mn>
   </mfrac>
   <mo>−</mo><mfrac>
    <mn>1</mn>
    <mn>7</mn>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mn>9</mn>
   </mfrac>
   <mo>−</mo><mo>⋯</mo><mo>=</mo><mfrac>
    <mi>π</mi>
    <mn>4</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "1 μείον 1 τρίτο συν 1 πέμπτο μείον 1 έβδομο συν 1 ένατο μείον αποσιωπητικά; ισούται με, π προς 4")?;
    return Ok(());
}

#[test]
fn uoa_corpus_003() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mn>2</mn>
   </mfrac>
   <mo>−</mo><mfrac>
    <mn>1</mn>
    <mn>5</mn>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mn>8</mn>
   </mfrac>
   <mo>−</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <mn>11</mn></mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <mn>14</mn></mrow>
   </mfrac>
   <mo>−</mo><mo>⋯</mo><mo>=</mo><mfrac>
    <mrow>
     <mi>π</mi><msqrt>
      <mn>3</mn>
     </msqrt>
     </mrow>
    <mn>9</mn>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mn>3</mn>
   </mfrac>
   <mi>ln</mi><mn>2</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "1 δεύτερο μείον 1 πέμπτο συν 1 όγδοο μείον 1 προς 11, συν 1 προς 14, μείον αποσιωπητικά; ισούται με; κλάσμα, π, η τετραγωνική ρίζα του 3; προς 9, τέλος κλάσματος; συν; 1 τρίτο, ο φυσικός λογάριθμος, του 2")?;
    return Ok(());
}

#[test]
fn uoa_corpus_004() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>1</mn>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>2</mn>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>3</mn>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>4</mn>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mo>⋯</mo><mo>=</mo><mfrac>
    <mrow>
     <msup>
      <mi>π</mi>
      <mn>2</mn>
     </msup>
     </mrow>
    <mn>6</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, 1 προς, 1 στο τετράγωνο, τέλος κλάσματος; συν; κλάσμα, 1 προς, 2 στο τετράγωνο, τέλος κλάσματος; συν; κλάσμα, 1 προς, 3 στο τετράγωνο, τέλος κλάσματος; συν; κλάσμα, 1 προς, 4 στο τετράγωνο, τέλος κλάσματος; συν αποσιωπητικά; ισούται με; κλάσμα, π στο τετράγωνο, προς 6, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_005() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>1</mn>
      <mn>4</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>2</mn>
      <mn>4</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>3</mn>
      <mn>4</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>4</mn>
      <mn>4</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mo>⋯</mo><mo>=</mo><mfrac>
    <mrow>
     <msup>
      <mi>π</mi>
      <mn>4</mn>
     </msup>
     </mrow>
    <mrow>
     <mn>90</mn></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, 1 προς, 1 στην τέταρτη, τέλος κλάσματος; συν; κλάσμα, 1 προς, 2 στην τέταρτη, τέλος κλάσματος; συν; κλάσμα, 1 προς, 3 στην τέταρτη, τέλος κλάσματος; συν; κλάσμα, 1 προς, 4 στην τέταρτη, τέλος κλάσματος; συν αποσιωπητικά; ισούται με; κλάσμα, π στην τέταρτη, προς 90, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_006() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>1</mn>
      <mn>6</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>2</mn>
      <mn>6</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>3</mn>
      <mn>6</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>4</mn>
      <mn>6</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mo>⋯</mo><mo>=</mo><mfrac>
    <mrow>
     <msup>
      <mi>π</mi>
      <mn>6</mn>
     </msup>
     </mrow>
    <mrow>
     <mn>945</mn></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, 1 προς, 1 στην έκτη, τέλος κλάσματος; συν; κλάσμα, 1 προς, 2 στην έκτη, τέλος κλάσματος; συν; κλάσμα, 1 προς, 3 στην έκτη, τέλος κλάσματος; συν; κλάσμα, 1 προς, 4 στην έκτη, τέλος κλάσματος; συν αποσιωπητικά; ισούται με; κλάσμα, π στην έκτη, προς 945, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_007() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>1</mn>
      <mn>6</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>2</mn>
      <mn>6</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>3</mn>
      <mn>6</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>4</mn>
      <mn>6</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mo>⋯</mo><mo>=</mo><mfrac>
    <mrow>
     <mn>31</mn><msup>
      <mi>π</mi>
      <mn>6</mn>
     </msup>
     </mrow>
    <mrow>
     <mn>30,240</mn></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, 1 προς, 1 στην έκτη, τέλος κλάσματος; συν; κλάσμα, 1 προς, 2 στην έκτη, τέλος κλάσματος; συν; κλάσμα, 1 προς, 3 στην έκτη, τέλος κλάσματος; συν; κλάσμα, 1 προς, 4 στην έκτη, τέλος κλάσματος; συν αποσιωπητικά; ισούται με; κλάσμα, 31 π στην έκτη, προς 30,240, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_008() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>1</mn>
      <mn>3</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>3</mn>
      <mn>3</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>−</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>5</mn>
      <mn>3</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>−</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>7</mn>
      <mn>3</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mo>⋯</mo><mo>=</mo><mfrac>
    <mrow>
     <mn>3</mn><msup>
      <mi>π</mi>
      <mn>3</mn>
     </msup>
     <msqrt>
      <mn>2</mn>
     </msqrt>
     </mrow>
    <mrow>
     <mn>128</mn></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, 1 προς, 1 στον κύβο, τέλος κλάσματος; συν; κλάσμα, 1 προς, 3 στον κύβο, τέλος κλάσματος; μείον; κλάσμα, 1 προς, 5 στον κύβο, τέλος κλάσματος; μείον; κλάσμα, 1 προς, 7 στον κύβο, τέλος κλάσματος; συν αποσιωπητικά; ισούται με; κλάσμα, 3 π στον κύβο; η τετραγωνική ρίζα του 2; προς 128, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_009() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mrow>
     <mn>1</mn><mo>×</mo><mn>3</mn></mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <mn>3</mn><mo>×</mo><mn>5</mn></mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <mn>5</mn><mo>×</mo><mn>7</mn></mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <mn>7</mn><mo>×</mo><mn>9</mn></mrow>
   </mfrac>
   <mo>+</mo><mo>⋯</mo><mo>=</mo><mfrac>
    <mn>1</mn>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, 1 προς, 1 επί 3, τέλος κλάσματος; συν; κλάσμα, 1 προς, 3 επί 5, τέλος κλάσματος; συν; κλάσμα, 1 προς, 5 επί 7, τέλος κλάσματος; συν; κλάσμα, 1 προς, 7 επί 9, τέλος κλάσματος; συν αποσιωπητικά; ισούται με, 1 δεύτερο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_010() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>1</mn>
      <mn>2</mn>
     </msup>
     <mo>×</mo><msup>
      <mn>3</mn>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>3</mn>
      <mn>2</mn>
     </msup>
     <mo>×</mo><msup>
      <mn>5</mn>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>5</mn>
      <mn>2</mn>
     </msup>
     <mo>×</mo><msup>
      <mn>7</mn>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>7</mn>
      <mn>2</mn>
     </msup>
     <mo>×</mo><msup>
      <mn>9</mn>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mo>⋯</mo><mo>=</mo><mfrac>
    <mrow>
     <msup>
      <mi>π</mi>
      <mn>2</mn>
     </msup>
     <mo>−</mo><mn>8</mn></mrow>
    <mrow>
     <mn>16</mn></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, 1 προς, 1 στο τετράγωνο, επί 3 στο τετράγωνο, τέλος κλάσματος; συν; κλάσμα, 1 προς, 3 στο τετράγωνο, επί 5 στο τετράγωνο, τέλος κλάσματος; συν; κλάσμα, 1 προς, 5 στο τετράγωνο, επί 7 στο τετράγωνο, τέλος κλάσματος; συν; κλάσμα, 1 προς, 7 στο τετράγωνο, επί 9 στο τετράγωνο, τέλος κλάσματος; συν αποσιωπητικά; ισούται με; κλάσμα, π στο τετράγωνο, μείον 8, προς 16, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_011() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>β</mi><mo>+</mo><mfrac>
    <mi>γ</mi>
    <mi>δ</mi>
   </mfrac>
   <mo>−</mo><mi>ε</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "βήτα συν γάμμα προς δέλτα; μείον έψιλον")?;
    return Ok(());
}

#[test]
fn uoa_corpus_012() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <mi>β</mi><mo>+</mo><mi>γ</mi></mrow>
    <mrow>
     <mi>δ</mi><mo>+</mo><mi>ε</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, βήτα συν γάμμα, προς, δέλτα συν έψιλον, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_013() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <mi>β</mi><mo>−</mo><mi>γ</mi></mrow>
    <mi>δ</mi>
   </mfrac>
   <mo>+</mo><mi>ε</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, βήτα μείον γάμμα, προς δέλτα, τέλος κλάσματος; συν έψιλον")?;
    return Ok(());
}

#[test]
fn uoa_corpus_014() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>α</mi><mo>+</mo><mi>β</mi><mo>+</mo><mi>γ</mi><mo>+</mo><mi>δ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άλφα συν βήτα συν γάμμα συν δέλτα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_015() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>α</mi><mo>+</mo><mfrac>
    <mi>β</mi>
    <mi>γ</mi>
   </mfrac>
   <mo>+</mo><mi>δ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άλφα συν βήτα προς γάμμα; συν δέλτα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_016() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mi>α</mi>
    <mrow>
     <mi>β</mi><mo>+</mo><mi>γ</mi><mo>+</mo><mi>δ</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, άλφα προς, βήτα συν γάμμα συν δέλτα, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_017() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <mi>α</mi><mo>+</mo><mi>β</mi></mrow>
    <mrow>
     <mi>γ</mi><mo>+</mo><mi>δ</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, άλφα συν βήτα, προς, γάμμα συν δέλτα, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_018() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mi>α</mi><mo>+</mo><mi>β</mi></mrow>
   <mo>)</mo></mrow><mo>⋅</mo><mrow><mo>(</mo>
    <mrow>
     <mi>γ</mi><mo>+</mo><mi>δ</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση; άλφα συν βήτα; κλείνει παρένθεση; φορές; ανοίγει παρένθεση; γάμμα συν δέλτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_019() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <mi>χ</mi><mo>+</mo><msup>
      <mi>ψ</mi>
      <mn>2</mn>
     </msup>
     </mrow>
    <mrow>
     <mi>κ</mi><mo>+</mo><mn>1</mn></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, χ συν ψ στο τετράγωνο, προς, καπα συν 1, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_020() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>χ</mi><mo>+</mo><mfrac>
    <mrow>
     <msup>
      <mi>ψ</mi>
      <mn>2</mn>
     </msup>
     </mrow>
    <mi>κ</mi>
   </mfrac>
   <mo>+</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ συν; κλάσμα, ψ στο τετράγωνο, προς καπα, τέλος κλάσματος; συν 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_021() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <mi>β</mi><mo>×</mo><mrow><mo>(</mo>
      <mrow>
       <mi>γ</mi><mo>+</mo><mi>δ</mi></mrow>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <msup>
      <mi>ε</mi>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, βήτα επί; ανοίγει παρένθεση; γάμμα συν δέλτα; κλείνει παρένθεση, προς, έψιλον στο τετράγωνο, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_022() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>1</mn>
      <mrow>
       <mn>2</mn><mi>π</mi></mrow>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>3</mn>
      <mrow>
       <mn>2</mn><mi>π</mi></mrow>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>5</mn>
      <mrow>
       <mn>2</mn><mi>π</mi></mrow>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>7</mn>
      <mrow>
       <mn>2</mn><mi>π</mi></mrow>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mo>⋯</mo><mo>=</mo><mfrac>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <msup>
        <mn>2</mn>
        <mrow>
         <mn>2</mn><mi>π</mi></mrow>
       </msup>
       <mo>−</mo><mn>1</mn></mrow>
     <mo>)</mo></mrow><msup>
      <mi>π</mi>
      <mrow>
       <mn>2</mn><mi>π</mi></mrow>
     </msup>
     <msub>
      <mi>Β</mi>
      <mi>π</mi>
     </msub>
     </mrow>
    <mrow>
     <mn>2</mn><mrow><mo>(</mo>
      <mrow>
       <mn>2</mn><mi>π</mi></mrow>
     <mo>)</mo></mrow><mo>!</mo></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, 1 προς, 1 που υψώνεται στη 2 π τέλος δύναμης; τέλος κλάσματος; συν; κλάσμα, 1 προς, 3 που υψώνεται στη 2 π τέλος δύναμης; τέλος κλάσματος; συν; κλάσμα, 1 προς, 5 που υψώνεται στη 2 π τέλος δύναμης; τέλος κλάσματος; συν; κλάσμα, 1 προς, 7 που υψώνεται στη 2 π τέλος δύναμης; τέλος κλάσματος; συν αποσιωπητικά; ισούται με; κλάσμα, ανοίγει παρένθεση; 2 που υψώνεται στη 2 π τέλος δύναμης; μείον 1; κλείνει παρένθεση; π που υψώνεται στη 2 π τέλος δύναμης; κεφαλαίο βήτα δείκτης π; προς, 2; ανοίγει παρένθεση, 2 π, κλείνει παρένθεση; παραγοντικό, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_023() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>1</mn>
      <mrow>
       <mn>2</mn><mi>π</mi></mrow>
     </msup>
     </mrow>
   </mfrac>
   <mo>−</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>2</mn>
      <mrow>
       <mn>2</mn><mi>π</mi></mrow>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>3</mn>
      <mrow>
       <mn>2</mn><mi>π</mi></mrow>
     </msup>
     </mrow>
   </mfrac>
   <mo>−</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>4</mn>
      <mrow>
       <mn>2</mn><mi>π</mi></mrow>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mo>⋯</mo><mo>=</mo><mfrac>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <msup>
        <mn>2</mn>
        <mrow>
         <mn>2</mn><mi>π</mi><mo>−</mo><mn>1</mn></mrow>
       </msup>
       <mo>−</mo><mn>1</mn></mrow>
     <mo>)</mo></mrow><msup>
      <mi>π</mi>
      <mrow>
       <mn>2</mn><mi>π</mi></mrow>
     </msup>
     <msub>
      <mi>Β</mi>
      <mi>π</mi>
     </msub>
     </mrow>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mn>2</mn><mi>π</mi></mrow>
     <mo>)</mo></mrow><mo>!</mo></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, 1 προς, 1 που υψώνεται στη 2 π τέλος δύναμης; τέλος κλάσματος; μείον; κλάσμα, 1 προς, 2 που υψώνεται στη 2 π τέλος δύναμης; τέλος κλάσματος; συν; κλάσμα, 1 προς, 3 που υψώνεται στη 2 π τέλος δύναμης; τέλος κλάσματος; μείον; κλάσμα, 1 προς, 4 που υψώνεται στη 2 π τέλος δύναμης; τέλος κλάσματος; συν αποσιωπητικά; ισούται με; κλάσμα, ανοίγει παρένθεση; 2 που υψώνεται στη 2 π μείον 1 τέλος δύναμης; μείον 1; κλείνει παρένθεση; π που υψώνεται στη 2 π τέλος δύναμης; κεφαλαίο βήτα δείκτης π; προς, ανοίγει παρένθεση, 2 π, κλείνει παρένθεση; παραγοντικό, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_024() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>1</mn>
      <mrow>
       <mn>2</mn><mi>π</mi><mo>+</mo><mn>1</mn></mrow>
     </msup>
     </mrow>
   </mfrac>
   <mo>−</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>3</mn>
      <mrow>
       <mn>2</mn><mi>π</mi><mo>+</mo><mn>1</mn></mrow>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>5</mn>
      <mrow>
       <mn>2</mn><mi>π</mi><mo>+</mo><mn>1</mn></mrow>
     </msup>
     </mrow>
   </mfrac>
   <mo>−</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mn>7</mn>
      <mrow>
       <mn>2</mn><mi>π</mi><mo>+</mo><mn>1</mn></mrow>
     </msup>
     </mrow>
   </mfrac>
   <mo>+</mo><mo>⋯</mo><mo>=</mo><mfrac>
    <mrow>
     <msup>
      <mi>π</mi>
      <mrow>
       <mn>2</mn><mi>π</mi><mo>+</mo><mn>1</mn></mrow>
     </msup>
     <msub>
      <mi>Ε</mi>
      <mi>π</mi>
     </msub>
     </mrow>
    <mrow>
     <msup>
      <mn>2</mn>
      <mrow>
       <mn>2</mn><mi>π</mi><mo>+</mo><mn>2</mn></mrow>
     </msup>
     <mrow><mo>(</mo>
      <mrow>
       <mn>2</mn><mi>π</mi></mrow>
     <mo>)</mo></mrow><mo>!</mo></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, 1 προς, 1 που υψώνεται στη 2 π συν 1 τέλος δύναμης; τέλος κλάσματος; μείον; κλάσμα, 1 προς, 3 που υψώνεται στη 2 π συν 1 τέλος δύναμης; τέλος κλάσματος; συν; κλάσμα, 1 προς, 5 που υψώνεται στη 2 π συν 1 τέλος δύναμης; τέλος κλάσματος; μείον; κλάσμα, 1 προς, 7 που υψώνεται στη 2 π συν 1 τέλος δύναμης; τέλος κλάσματος; συν αποσιωπητικά; ισούται με; κλάσμα, π που υψώνεται στη 2 π συν 1 τέλος δύναμης; κεφαλαίο έψιλον δείκτης π; προς, 2 που υψώνεται στη 2 π συν 2 τέλος δύναμης; ανοίγει παρένθεση, 2 π, κλείνει παρένθεση; παραγοντικό, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_025() -> Result<()> { //
    let expr = r#"<math>
 <mtable columnalign="left">
   <mtr>
    <mtd>
     <msup>
      <mrow><mo>(</mo>
       <mrow>
        <mi>α</mi><mo>+</mo><mi>χ</mi></mrow>
      <mo>)</mo></mrow>
      <mi>ν</mi>
     </msup>
     <mo>=</mo><msup>
      <mi>α</mi>
      <mi>ν</mi>
     </msup>
     <mo>+</mo><mi>ν</mi><msup>
      <mi>α</mi>
      <mrow>
       <mi>ν</mi><mo>−</mo><mn>1</mn></mrow>
     </msup>
     <mi>χ</mi><mo>+</mo><mfrac>
      <mrow>
       <mi>ν</mi><mo stretchy="false">(</mo><mi>ν</mi><mo>−</mo><mn>1</mn><mo stretchy="false">)</mo></mrow>
      <mrow>
       <mn>2</mn><mo>!</mo></mrow>
     </mfrac>
     <msup>
      <mi>α</mi>
      <mrow>
       <mi>ν</mi><mo>−</mo><mn>2</mn></mrow>
     </msup>
     <msup>
      <mi>χ</mi>
      <mn>2</mn>
     </msup>
     
    </mtd>
   </mtr>
   <mtr>
    <mtd>
     <mo>+</mo><mfrac>
      <mrow>
       <mi>ν</mi><mo stretchy="false">(</mo><mi>ν</mi><mo>−</mo><mn>1</mn><mo stretchy="false">)</mo><mo stretchy="false">(</mo><mi>ν</mi><mo>−</mo><mn>2</mn><mo stretchy="false">)</mo></mrow>
      <mrow>
       <mn>3</mn><mo>!</mo></mrow>
     </mfrac>
     <msup>
      <mi>α</mi>
      <mrow>
       <mi>ν</mi><mo>−</mo><mn>3</mn></mrow>
     </msup>
     <msup>
      <mi>χ</mi>
      <mn>3</mn>
     </msup>
     <mo>+</mo><mo>⋯</mo>
    </mtd>
   </mtr>
  </mtable> 
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 σειρές; σειρά 1; ανοίγει παρένθεση, άλφα συν χ, κλείνει παρένθεση στην νί οστή δύναμη; ισούται με; άλφα στην νί οστή δύναμη; συν; νί; άλφα υψωμένο στην νί μείον 1 τέλος δύναμης; χ; συν; το κλάσμα με αριθμητή; νί; ανοίγει παρένθεση, νί μείον 1, κλείνει παρένθεση; και παρονομαστή 2 παραγοντικό; άλφα υψωμένο στην νί μείον 2 τέλος δύναμης; χ στο τετράγωνο; σειρά 2; συν το κλάσμα με αριθμητή; νί; ανοίγει παρένθεση, νί μείον 1, κλείνει παρένθεση; ανοίγει παρένθεση, νί μείον 2, κλείνει παρένθεση; και παρονομαστή 3 παραγοντικό; άλφα υψωμένο στην νί μείον 3 τέλος δύναμης; χ στον κύβο; συν αποσιωπητικά")?;
    return Ok(());
    //theodora. fails. In Clearspeak it is recognized correctly as two rows.
    //Now it reads: 2 εξισώσεις; εξίσωση 1; ανοίγει παρένθεση, άλφα συν χ, κλείνει παρένθεση στην νί οστή; ισούται με; άλφα στην νί οστή, συν; νί; άλφα που υψώνεται στη νί μείον 1 τέλος δύναμης; χ; συν; κλάσμα, νί; ανοίγει παρένθεση, νί μείον 1, κλείνει παρένθεση, προς, 2 παραγοντικό, τέλος κλάσματος; άλφα που υψώνεται στη νί μείον 2 τέλος δύναμης; χ στο τετράγωνο; εξίσωση 2; συν κλάσμα, νί; ανοίγει παρένθεση, νί μείον 1, κλείνει παρένθεση; ανοίγει παρένθεση, νί μείον 2, κλείνει παρένθεση, προς, 3 παραγοντικό, τέλος κλάσματος; άλφα που υψώνεται στη νί μείον 3 τέλος δύναμης; χ στον κύβο; συν αποσιωπητικά

}
#[test]
fn uoa_corpus_026() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>α</mi>
    <mi>χ</mi>
   </msup>
   <mo>=</mo><msup>
    <mi>e</mi>
    <mrow>
     <mi>χ</mi><mi>ln</mi><mi>α</mi></mrow>
   </msup>
   <mo>=</mo><mn>1</mn><mo>+</mo><mi>χ</mi><mi>ln</mi><mi>α</mi><mo>+</mo><mfrac>
    <mrow>
     <msup>
      <mrow>
       <mrow><mo>(</mo>
        <mrow>
         <mi>χ</mi><mi>ln</mi><mi>α</mi></mrow>
       <mo>)</mo></mrow></mrow>
      <mn>2</mn>
     </msup>
     </mrow>
    <mrow>
     <mn>2</mn><mo>!</mo></mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mrow>
     <msup>
      <mrow>
       <mrow><mo>(</mo>
        <mrow>
         <mi>χ</mi><mi>ln</mi><mi>α</mi></mrow>
       <mo>)</mo></mrow></mrow>
      <mn>3</mn>
     </msup>
     </mrow>
    <mrow>
     <mn>3</mn><mo>!</mo></mrow>
   </mfrac>
   <mo>+</mo><mo>⋯</mo></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άλφα στην χ οστή, ισούται με; e που υψώνεται στη χ, ο φυσικός λογάριθμος, του άλφα τέλος δύναμης; ισούται με; 1 συν; χ, ο φυσικός λογάριθμος, του άλφα; συν; κλάσμα, ανοίγει παρένθεση; χ, ο φυσικός λογάριθμος, του άλφα; κλείνει παρένθεση στο τετράγωνο, προς, 2 παραγοντικό, τέλος κλάσματος; συν; κλάσμα, ανοίγει παρένθεση; χ, ο φυσικός λογάριθμος, του άλφα; κλείνει παρένθεση στον κύβο, προς, 3 παραγοντικό, τέλος κλάσματος; συν αποσιωπητικά")?;
    return Ok(());
}

#[test]
fn uoa_corpus_027() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>ln</mi><mi>χ</mi><mo>=</mo><mn>2</mn><mrow><mo>{</mo> <mrow>
    <mrow><mo>(</mo>
     <mrow>
      <mfrac>
       <mrow>
        <mi>χ</mi><mo>−</mo><mn>1</mn></mrow>
       <mrow>
        <mi>χ</mi><mo>+</mo><mn>1</mn></mrow>
      </mfrac>
      </mrow>
    <mo>)</mo></mrow><mo>+</mo><mfrac>
     <mn>1</mn>
     <mn>3</mn>
    </mfrac>
    <msup>
     <mrow>
      <mrow><mo>(</mo>
       <mrow>
        <mfrac>
         <mrow>
          <mi>χ</mi><mo>−</mo><mn>1</mn></mrow>
         <mrow>
          <mi>χ</mi><mo>+</mo><mn>1</mn></mrow>
        </mfrac>
        </mrow>
      <mo>)</mo></mrow></mrow>
     <mn>3</mn>
    </msup>
    <mo>+</mo><mfrac>
     <mn>1</mn>
     <mn>5</mn>
    </mfrac>
    <msup>
     <mrow>
      <mrow><mo>(</mo>
       <mrow>
        <mfrac>
         <mrow>
          <mi>χ</mi><mo>−</mo><mn>1</mn></mrow>
         <mrow>
          <mi>χ</mi><mo>+</mo><mn>1</mn></mrow>
        </mfrac>
        </mrow>
      <mo>)</mo></mrow></mrow>
     <mn>5</mn>
    </msup>
    <mo>+</mo><mo>⋯</mo></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ο φυσικός λογάριθμος, του χ; ισούται με; 2; ανοίγει άγκιστρο; ανοίγει παρένθεση; κλάσμα, χ μείον 1, προς, χ συν 1, τέλος κλάσματος; κλείνει παρένθεση; συν; 1 τρίτο; ανοίγει παρένθεση; κλάσμα, χ μείον 1, προς, χ συν 1, τέλος κλάσματος; κλείνει παρένθεση στον κύβο; συν; 1 πέμπτο; ανοίγει παρένθεση; κλάσμα, χ μείον 1, προς, χ συν 1, τέλος κλάσματος; κλείνει παρένθεση στην πέμπτη; συν αποσιωπητικά; κλείνει άγκιστρο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_028() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <mn>2</mn><mi>sin</mi><mi>μ</mi><mi>π</mi></mrow>
    <mi>π</mi>
   </mfrac>
   <mrow><mo>(</mo>
    <mrow>
     <mfrac>
      <mrow>
       <mi>sin</mi><mi>χ</mi></mrow>
      <mrow>
       <mn>1</mn><mo>−</mo><msup>
        <mi>μ</mi>
        <mn>2</mn>
       </msup>
       </mrow>
     </mfrac>
     <mo>−</mo><mfrac>
      <mrow>
       <mn>2</mn><mi>sin</mi><mn>2</mn><mi>χ</mi></mrow>
      <mrow>
       <msup>
        <mn>2</mn>
        <mn>2</mn>
       </msup>
       <mo>−</mo><msup>
        <mi>μ</mi>
        <mn>2</mn>
       </msup>
       </mrow>
     </mfrac>
     <mo>+</mo><mfrac>
      <mrow>
       <mn>3</mn><mi>sin</mi><mn>3</mn><mi>χ</mi></mrow>
      <mrow>
       <msup>
        <mn>3</mn>
        <mn>2</mn>
       </msup>
       <mo>−</mo><msup>
        <mi>μ</mi>
        <mn>2</mn>
       </msup>
       </mrow>
     </mfrac>
     <mo>−</mo><mo>⋯</mo></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, 2, ημίτονο του μί π, προς π, τέλος κλάσματος; ανοίγει παρένθεση; κλάσμα, ημίτονο του χ, προς, 1 μείον, μί στο τετράγωνο, τέλος κλάσματος; μείον; κλάσμα, 2, ημίτονο του 2 χ, προς, 2 στο τετράγωνο, μείον, μί στο τετράγωνο, τέλος κλάσματος; συν; κλάσμα, 3, ημίτονο του 3 χ, προς, 3 στο τετράγωνο, μείον, μί στο τετράγωνο, τέλος κλάσματος; μείον αποσιωπητικά; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_029() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mn>1</mn><mo>+</mo><mfrac>
    <mi>χ</mi>
    <mrow>
     <mn>1</mn><mo>+</mo><mfrac>
      <mi>χ</mi>
      <mrow>
       <mn>1</mn><mo>+</mo><mfrac>
        <mi>χ</mi>
        <mrow>
         <mn>1</mn><mo>+</mo><mfrac>
          <mi>χ</mi>
          <mrow>
           <mn>1</mn><mo>+</mo><mfrac>
            <mi>χ</mi>
            <mo>…</mo>
           </mfrac>
           </mrow>
         </mfrac>
         </mrow>
       </mfrac>
       </mrow>
     </mfrac>
     </mrow>
   </mfrac>
   </mrow>
  </math>"#;
    // TODO: add expected speech
    test("el", "SimpleSpeak", expr, "1 συν; κλάσμα, χ προς, 1 συν; κλάσμα, χ προς, 1 συν; κλάσμα, χ προς, 1 συν; κλάσμα, χ προς, 1 συν; κλάσμα, χ προς αποσιωπητικά, τέλος κλάσματος; τέλος κλάσματος; τέλος κλάσματος; τέλος κλάσματος; τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_030() -> Result<()> { //
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mfrac><mrow><mi>&#x3C6;</mi><mo>'</mo><mo>(</mo><mi>&#x3C7;</mi><mo>)</mo><mo>+</mo><mi>&#x3C6;</mi><mo>(</mo><mi>&#x3C7;</mi><mo>)</mo><mo>&#xD7;</mo><mi>&#x3C8;</mi><mo>+</mo><mi>&#x3B6;</mi></mrow><mrow><mi>&#x3C7;</mi><mo>&#xD7;</mo><mi>&#x3C8;</mi></mrow></mfrac></math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, φ τόνος, του χ, συν, φ του χ, επί ψ; συν ζήτα, προς, χ επί ψ, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_031() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>χ</mi><mo>=</mo><mfrac>
    <mrow>
     <mo>−</mo><mi>β</mi><mo>±</mo><msqrt>
      <mrow>
       <msup>
        <mi>β</mi>
        <mn>2</mn>
       </msup>
       <mo>−</mo><mn>4</mn><mo>⋅</mo><mi>α</mi><mo>⋅</mo><mi>γ</mi></mrow>
     </msqrt>
     </mrow>
    <mrow>
     <mn>2</mn><mo>⋅</mo><mi>α</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ ισούται με; κλάσμα, μείον βήτα, συν πλήν; η τετραγωνική ρίζα του βήτα στο τετράγωνο, μείον, 4 φορές άλφα φορές γάμμα, τέλος ρίζας; προς, 2 φορές άλφα, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_032() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msqrt>
    <mrow>
     <mfrac>
      <mi>β</mi>
      <mi>γ</mi>
     </mfrac>
     </mrow>
   </msqrt>
   <mo>+</mo><mi>δ</mi><mo>−</mo><mi>ε</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η τετραγωνική ρίζα του βήτα προς γάμμα; τέλος ρίζας; συν δέλτα μείον έψιλον")?;
    return Ok(());
}

#[test]
fn uoa_corpus_033() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msqrt>
    <mrow>
     <mfrac>
      <mi>β</mi>
      <mrow>
       <mi>γ</mi><mo>+</mo><mi>δ</mi></mrow>
     </mfrac>
     </mrow>
   </msqrt>
   <mo>−</mo><mi>ε</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η τετραγωνική ρίζα του κλάσματος, βήτα προς, γάμμα συν δέλτα, τέλος κλάσματος; τέλος ρίζας; μείον έψιλον")?;
    return Ok(());

}

#[test]
fn uoa_corpus_034() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msqrt>
    <mrow>
     <mfrac>
      <mi>β</mi>
      <mi>γ</mi>
     </mfrac>
     <mo>−</mo><mi>ε</mi></mrow>
   </msqrt>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η τετραγωνική ρίζα του βήτα προς γάμμα; μείον έψιλον, τέλος ρίζας")?;
    return Ok(());
}

#[test]
fn uoa_corpus_035() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msqrt>
    <mrow>
     <mi>β</mi><mo>+</mo><mi>γ</mi></mrow>
   </msqrt>
   <mo>+</mo><msup>
    <mi>δ</mi>
    <mn>2</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η τετραγωνική ρίζα του βήτα συν γάμμα, τέλος ρίζας; συν, δέλτα στο τετράγωνο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_036() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <msqrt>
      <mi>β</mi>
     </msqrt>
     <mo>+</mo><mi>γ</mi></mrow>
    <mi>δ</mi>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, η τετραγωνική ρίζα του βήτα; συν γάμμα, προς δέλτα, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_037() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <msqrt>
      <mi>π</mi>
     </msqrt>
     </mrow>
    <mn>2</mn>
   </mfrac>
   <mo>≠</mo><msqrt>
    <mrow>
     <mfrac>
      <mi>π</mi>
      <mn>2</mn>
     </mfrac>
     </mrow>
   </msqrt>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, η τετραγωνική ρίζα του π; προς 2, τέλος κλάσματος; είναι διάφορο της τετραγωνικής ρίζα του π προς 2; τέλος ρίζας")?;
    return Ok(());
    //theodora. fails with current rules. Should be okay when adding genitive clause
}

#[test]
fn uoa_corpus_038() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msqrt>
    <mrow>
     <mn>1</mn><mo>+</mo><msqrt>
      <mrow>
       <mn>2</mn><mo>+</mo><msqrt>
        <mrow>
         <mn>2</mn><mo>+</mo><msqrt>
          <mrow>
           <mn>2</mn><mo>+</mo><mo>…</mo></mrow>
         </msqrt>
         </mrow>
       </msqrt>
       </mrow>
     </msqrt>
     </mrow>
   </msqrt>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η τετραγωνική ρίζα του 1 συν; η τετραγωνική ρίζα του 2 συν; η τετραγωνική ρίζα του 2 συν; η τετραγωνική ρίζα του 2 συν αποσιωπητικά, τέλος ρίζας; τέλος ρίζας; τέλος ρίζας; τέλος ρίζας")?;
    return Ok(());
}

#[test]
fn uoa_corpus_039() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>d</mi><mrow><mo>(</mo>
    <mrow>
     <mi>χ</mi><mo>,</mo><mi>ψ</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><msqrt>
    <mrow>
     <msup>
      <mrow>
       <mrow><mo>(</mo>
        <mrow>
         <msub>
          <mi>χ</mi>
          <mn>1</mn>
         </msub>
         <mo>−</mo><msub>
          <mi>ψ</mi>
          <mn>1</mn>
         </msub>
         </mrow>
       <mo>)</mo></mrow></mrow>
      <mn>2</mn>
     </msup>
     <mo>+</mo><msup>
      <mrow>
       <mrow><mo>(</mo>
        <mrow>
         <msub>
          <mi>χ</mi>
          <mn>2</mn>
         </msub>
         <mo>−</mo><msub>
          <mi>ψ</mi>
          <mn>2</mn>
         </msub>
         </mrow>
       <mo>)</mo></mrow></mrow>
      <mn>2</mn>
     </msup>
     </mrow>
   </msqrt>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "d; ανοίγει παρένθεση; χ κόμμα, ψ; κλείνει παρένθεση; ισούται με; την τετραγωνική ρίζα του ανοίγει παρένθεση; χ δείκτης 1; μείον, ψ δείκτης 1; κλείνει παρένθεση στο τετράγωνο; συν; ανοίγει παρένθεση; χ δείκτης 2; μείον, ψ δείκτης 2; κλείνει παρένθεση στο τετράγωνο, τέλος ρίζας")?;
    return Ok(());
}

#[test]
fn uoa_corpus_040() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>Σ</mi><mo>=</mo><mstyle displaystyle="true">
    <mrow>
     <msubsup>
      <mo>∫</mo>
      <mi>α</mi>
      <mi>β</mi>
     </msubsup>
     <mrow>
      <mn>2</mn><mi>π</mi><mo>×</mo><mi>φ</mi><mo stretchy="false">(</mo><mi>χ</mi><mo stretchy="false">)</mo><mo>×</mo><msqrt>
       <mrow>
        <mn>1</mn><mo>+</mo><msup>
         <mrow>
          <mrow><mo>[</mo> <mrow>
           <mi>φ</mi><mo>'</mo><mo stretchy="false">(</mo><mi>χ</mi><mo stretchy="false">)</mo></mrow> <mo>]</mo></mrow></mrow>
         <mn>2</mn>
        </msup>
        </mrow>
      </msqrt>
      <mi>d</mi><mi>χ</mi></mrow>
    </mrow>
    
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο σίγμα, ισούται με; ολοκλήρωμα από άλφα ως βήτα του; 2 π επί φ του χ, επί; η τετραγωνική ρίζα του 1 συν; ανοίγει αγκύλη, φ τόνος, του χ; κλείνει αγκύλη στο τετράγωνο, τέλος ρίζας; d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_041() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>σ</mi><mo>=</mo><msqrt>
    <mrow>
     <mrow><mo>[</mo> <mrow>
      <mfrac>
       <mrow>
        <mstyle displaystyle="true">
         <mo>∑</mo> <mrow>
          <msup>
           <mrow>
            <mrow><mo>(</mo>
             <mrow>
              <mi>χ</mi><mo>−</mo><mover accent="true">
               <mi>χ</mi>
               <mo stretchy="true">¯</mo>
              </mover>
              </mrow>
            <mo>)</mo></mrow></mrow>
           <mn>2</mn>
          </msup>
          </mrow>
        </mstyle></mrow>
       <mi>ν</mi>
      </mfrac>
      </mrow> <mo>]</mo></mrow></mrow>
   </msqrt>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "σίγμα ισούται με; την τετραγωνική ρίζα του ανοίγει αγκύλη; κλάσμα, άθροισμα του ανοίγει παρένθεση; χ μείον χ παύλα; κλείνει παρένθεση στο τετράγωνο, προς νί, τέλος κλάσματος; κλείνει αγκύλη, τέλος ρίζας")?;
    return Ok(());
}

#[test]
fn uoa_corpus_042() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mo>∂</mo><mi>Ω</mi><mo>=</mo><mfrac>
    <mover accent="true">
     <mi>Ω</mi>
     <mo>¯</mo>
    </mover>
    
    <mi>Ω</mi>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "μερικό διαφορικό, κεφαλαίο ωμέγα; ισούται με; κλάσμα, κεφαλαίο ωμέγα παύλα; προς κεφαλαίο ωμέγα, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_043() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>υ</mi>
    <mi>τ</mi>
   </msub>
   <mrow><mo>(</mo>
    <mrow>
     <mi>χ</mi><mo>,</mo><mi>τ</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mo>∇</mo><mrow><mo>(</mo>
    <mrow>
     <mi>Λ</mi><mrow><mo>(</mo>
      <mi>χ</mi>
     <mo>)</mo></mrow><mo>∇</mo><mi>υ</mi><mrow><mo>(</mo>
      <mrow>
       <mi>χ</mi><mo>,</mo><mi>τ</mi></mrow>
     <mo>)</mo></mrow></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ύψιλον δείκτης τάαφ; ανοίγει παρένθεση; χ κόμμα, τάαφ; κλείνει παρένθεση; ισούται με; ανάδελτα του ανοίγει παρένθεση; κεφαλαίο λάμδα, του χ; ανάδελτα του ύψιλον; ανοίγει παρένθεση; χ κόμμα, τάαφ; κλείνει παρένθεση; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_044() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mo>∇</mo>
    <mi>Δ</mi>
   </msub>
   <mi>υ</mi><mrow><mo>(</mo>
    <mi>χ</mi>
   <mo>)</mo></mrow><mo>=</mo><msub>
    <mo>∇</mo>
    <mrow>
     <mi>Κ</mi><mo>,</mo><mi>σ</mi></mrow>
   </msub>
   <mi>υ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το ανάδελτα δείκτης, κεφαλαίο δέλτα; ύψιλον του χ; ισούται με; το ανάδελτα δείκτης; κεφαλαίο καπα, κόμμα, σίγμα τέλος δείκτη; ύψιλον")?;
    return Ok(());
}

#[test]
fn uoa_corpus_045() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mo>∂</mo>
    <mn>0</mn>
   </msup>
   <msub>
    <mi>υ</mi>
    <mi>ν</mi>
   </msub>
   <mo>=</mo><msub>
    <mi>υ</mi>
    <mi>ν</mi>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "μερικό διαφορικό στην 0; ύψιλον δείκτης νί; ισούται με, ύψιλον δείκτης νί")?;
    return Ok(());
}

#[test]
fn uoa_corpus_046() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mo>∂</mo>
    <mn>2</mn>
   </msup>
   <msub>
    <mi>υ</mi>
    <mi>ν</mi>
   </msub>
   <mo>=</mo><mfrac>
    <mn>1</mn>
    <mi>κ</mi>
   </mfrac>
   <mrow><mo>(</mo>
    <mrow>
     <msup>
      <mo>∂</mo>
      <mn>1</mn>
     </msup>
     <msub>
      <mi>υ</mi>
      <mi>ν</mi>
     </msub>
     <mo>−</mo><msup>
      <mo>∂</mo>
      <mn>1</mn>
     </msup>
     <msub>
      <mi>υ</mi>
      <mrow>
       <mi>ν</mi><mo>−</mo><mn>1</mn></mrow>
     </msub>
     </mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "μερικό διαφορικό στο τετράγωνο; ύψιλον δείκτης νί; ισούται με; 1 προς καπα; ανοίγει παρένθεση; μερικό διαφορικό στην πρώτη; ύψιλον δείκτης νί; μείον; μερικό διαφορικό στην πρώτη; ύψιλον δείκτης, νί μείον 1 τέλος δείκτη; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_047() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>α</mi><mrow><mo>‖</mo> <mrow>
    <msub>
     <mo>∇</mo>
     <mi>Δ</mi>
    </msub>
    <msubsup>
     <mi>η</mi>
     <mi>Δ</mi>
     <mi>ν</mi>
    </msubsup>
    </mrow> <mo>‖</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άλφα; νόρμα του ανάδελτα δείκτης, κεφαλαίο δέλτα; ήτα δείκτης, κεφαλαίο δέλτα, στην νί οστή")?;
    return Ok(());
    //theodora. fails with current rules. Sould be fine when we add genitive. 
    // //Now reads: άλφα; νόρμα του το ανάδελτα δείκτης, κεφαλαίο δέλτα; ήτα δείκτης, κεφαλαίο δέλτα, στην νί οστή
}

#[test]
fn uoa_corpus_048() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <msup>
      <mi>d</mi>
      <mn>2</mn>
     </msup>
     <mi>υ</mi></mrow>
    <mrow>
     <mi>d</mi><msup>
      <mi>λ</mi>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, d στο τετράγωνο, ύψιλον, προς, d, λάμδα στο τετράγωνο, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_049() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mo>∇</mo><mrow><mo>(</mo>
    <mrow>
     <mi>Φ</mi><mrow><mo>(</mo>
      <mrow>
       <mi>τ</mi><mo>,</mo><mi>υ</mi><mrow><mo>(</mo>
        <mi>τ</mi>
       <mo>)</mo></mrow></mrow>
     <mo>)</mo></mrow></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανάδελτα του ανοίγει παρένθεση; κεφαλαίο φ; ανοίγει παρένθεση; τάαφ κόμμα; ύψιλον του τάαφ; κλείνει παρένθεση; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_050() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mover accent="true">
    <mi>υ</mi>
    <mo>¨</mo>
   </mover>
   <mrow><mo>(</mo>
    <mi>τ</mi>
   <mo>)</mo></mrow><mo>=</mo><mo>∇</mo><mi>Φ</mi><mrow><mo>(</mo>
    <mrow>
     <mi>τ</mi><mo>,</mo><mi>υ</mi><mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ύψιλον διπλή κουκκίδα; του τάαφ; ισούται με; ανάδελτα του κεφαλαίο φ; ανοίγει παρένθεση; τάαφ κόμμα; ύψιλον του τάαφ; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_051() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>|</mo> <mrow>
    <mo>∇</mo><mi>Η</mi><mrow><mo>(</mo>
     <mrow>
      <mi>τ</mi><mo>,</mo><mi>χ</mi></mrow>
    <mo>)</mo></mrow></mrow> <mo>|</mo></mrow><mo>≤</mo><mi>φ</mi><mrow><mo>(</mo>
    <mi>τ</mi>
   <mo>)</mo></mrow><msup>
    <mrow>
     <mrow><mo>|</mo> <mi>χ</mi> <mo>|</mo></mrow></mrow>
    <mi>α</mi>
   </msup>
   <mo>+</mo><mi>γ</mi><mrow><mo>(</mo>
    <mi>τ</mi>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η απόλυτη τιμή του ανάδελτα του κεφαλαίο ήτα; ανοίγει παρένθεση; τάαφ κόμμα, χ; κλείνει παρένθεση τέλος απόλυτης τιμής; είναι μικρότερο από ή ίσο με; φ του τάαφ; η απόλυτη τιμή του χ στην άλφα οστή; συν, γάμμα του τάαφ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_052() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>φ</mi>
    <mo>′</mo>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <msub>
      <mi>υ</mi>
      <mi>ν</mi>
     </msub>
     </mrow>
   <mo>)</mo></mrow><mo>→</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "φ τόνος, του; ανοίγει παρένθεση; ύψιλον δείκτης νί; κλείνει παρένθεση; βέλος προς τα δεξιά 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_053() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>‖</mo> <mrow>
    <msub>
     <mover accent="true">
      <mi>υ</mi>
      <mo>˜</mo>
     </mover>
     
     <mi>ν</mi>
    </msub>
    </mrow> <mo>‖</mo></mrow><mo>≥</mo><mrow><mo>|</mo> <mrow>
    <mrow><mo>〈</mo> <mrow>
     <msup>
      <mi>φ</mi>
      <mo>′</mo>
     </msup>
     <mrow><mo>(</mo>
      <mrow>
       <msub>
        <mi>υ</mi>
        <mi>ν</mi>
       </msub>
       </mrow>
     <mo>)</mo></mrow><mo>,</mo><msub>
      <mover accent="true">
       <mi>υ</mi>
       <mo>˜</mo>
      </mover>
      
      <mi>ν</mi>
     </msub>
     </mrow> <mo>〉</mo></mrow></mrow> <mo>|</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "νόρμα του ύψιλον κυματοειδής γραμμή, δείκτης νί; είναι μεγαλύτερο από ή ίσο με; την απόλυτη τιμή του αριστερά γωνιακή αγκύλη; φ τόνος, του; ανοίγει παρένθεση; ύψιλον δείκτης νί; κλείνει παρένθεση; κόμμα; ύψιλον κυματοειδής γραμμή, δείκτης νί; δεξιά γωνιακή αγκύλη")?;
    return Ok(());
    //theodora. fails with current rules. Needs accusative rule for absolute value in SimpleSpeak because right now it only exists in Clearspeak and it probably gets it from definitions
    // Now reads: νόρμα του ύψιλον κυματοειδής γραμμή, δείκτης νί; είναι μεγαλύτερο από ή ίσο με; η απόλυτη τιμή του αριστερά γωνιακή αγκύλη; φ τόνος, του; ανοίγει παρένθεση; ύψιλον δείκτης νί; κλείνει παρένθεση; κόμμα; ύψιλον κυματοειδής γραμμή, δείκτης νί; δεξιά γωνιακή αγκύλη
  }

#[test]
fn uoa_corpus_054() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <msup>
      <mo>∂</mo>
      <mi>ξ</mi>
     </msup>
     <msub>
      <mi>υ</mi>
      <mn>1</mn>
     </msub>
     </mrow>
    <mrow>
     <mo>∂</mo><msup>
      <mi>τ</mi>
      <mi>ξ</mi>
     </msup>
     </mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, μερικό διαφορικό στην ξ οστή; ύψιλον δείκτης 1; προς, μερικό διαφορικό, τάαφ στην ξ οστή, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_055() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>ι</mi><mi>Δ</mi><mi>υ</mi><mo>−</mo><msub>
    <mi>υ</mi>
    <mi>τ</mi>
   </msub>
   <mi>φ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ιΔυ μείον, ύψιλον δείκτης τάαφ; φ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_056() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>Δ</mi>
    <mi>ω</mi>
   </msub>
   <mi>υ</mi><mo>+</mo><mi>κ</mi><mo>×</mo><mi>υ</mi><mo>=</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο δέλτα δείκτης ωμέγα; ύψιλον; συν, καπα επί ύψιλον; ισούται με 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_057() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>χ</mi><mo>=</mo><mi>ρ</mi><mo>×</mo><msup>
    <mi>χ</mi>
    <mo>′</mo>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ ισούται με, ρ επί χ τόνος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_058() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mrow>
     <mrow><mrow>
      <mfrac>
       <mrow>
        <mo>∂</mo><mi>υ</mi></mrow>
       <mrow>
        <mo>∂</mo><mi>ω</mi></mrow>
      </mfrac>
      </mrow><mo>|</mo></mrow></mrow>
    <mrow>
     <mi>ω</mi><mo>=</mo><mn>0</mn></mrow>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, μερικό διαφορικό, ύψιλον, προς, μερικό διαφορικό, ωμέγα, τέλος κλάσματος; κάθετη γραμμη δείκτης, ωμέγα ισούται με 0 τέλος δείκτη")?;
    return Ok(());
}

#[test]
fn uoa_corpus_059() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <msup>
      <mo>∂</mo>
      <mn>2</mn>
     </msup>
     <mi>υ</mi></mrow>
    <mrow>
     <mo>∂</mo><msup>
      <mi>v</mi>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   <mo>−</mo><msup>
    <mi>λ</mi>
    <mn>2</mn>
   </msup>
   <mi>υ</mi><mo>=</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, μερικό διαφορικό στο τετράγωνο; ύψιλον, προς, μερικό διαφορικό, v στο τετράγωνο, τέλος κλάσματος; μείον, λάμδα στο τετράγωνο, ύψιλον; ισούται με 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_060() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mo>∂</mo>
    <mrow>
     <mo>∂</mo><msub>
      <mi>χ</mi>
      <mi>ξ</mi>
     </msub>
     </mrow>
   </mfrac>
   <msub>
    <mi>γ</mi>
    <mn>2</mn>
   </msub>
   <mrow><mo>(</mo>
    <mi>τ</mi>
   <mo>)</mo></mrow><mi>ln</mi><mi>ρ</mi><mo>=</mo><msub>
    <mi>γ</mi>
    <mn>2</mn>
   </msub>
   <mrow><mo>(</mo>
    <mi>τ</mi>
   <mo>)</mo></mrow><mfrac>
    <mrow>
     <msub>
      <mi>χ</mi>
      <mi>ξ</mi>
     </msub>
     </mrow>
    <mrow>
     <msup>
      <mi>ρ</mi>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   </mrow>
  </math>"#;
    // TODO: add expected speech
    test("el", "SimpleSpeak", expr, "κλάσμα, μερικό διαφορικό προς, μερικό διαφορικό, χ δείκτης ξ; τέλος κλάσματος; γάμμα δείκτης 2; του τάαφ; ο φυσικός λογάριθμος, του ρ; ισούται με; γάμμα δείκτης 2; του τάαφ; κλάσμα, χ δείκτης ξ; προς, ρ στο τετράγωνο, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_061() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mi>ρ</mi>
   </mfrac>
   <mi>sin</mi><mfrac>
    <mrow>
     <mi>ω</mi><mi>π</mi></mrow>
    <mrow>
     <msub>
      <mi>ω</mi>
      <mn>0</mn>
     </msub>
     </mrow>
   </mfrac>
   <mfrac>
    <mo>∂</mo>
    <mrow>
     <mo>∂</mo><mi>ω</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    // TODO: add expected speech
    test("el", "SimpleSpeak", expr, "1 προς ρ; ημίτονο του; κλάσμα, ωμέγα π, προς, ωμέγα δείκτης 0; τέλος κλάσματος; κλάσμα, μερικό διαφορικό προς, μερικό διαφορικό, ωμέγα, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_062() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>υ</mi><mrow><mo>(</mo>
    <mn>0</mn>
   <mo>)</mo></mrow><mo>=</mo><msup>
    <mi>υ</mi>
    <mo>′</mo>
   </msup>
   <mrow><mo>(</mo>
    <mn>1</mn>
   <mo>)</mo></mrow><mo>=</mo><msup>
    <mi>υ</mi>
    <mo>″</mo>
   </msup>
   <mrow><mo>(</mo>
    <mn>0</mn>
   <mo>)</mo></mrow><mo>=</mo><msup>
    <mi>υ</mi>
    <mo>‴</mo>
   </msup>
   <mrow><mo>(</mo>
    <mn>1</mn>
   <mo>)</mo></mrow><mo>=</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ύψιλον του 0, ισούται με, ύψιλον τόνος; του 1; ισούται με; ύψιλον διπλή παράγωγος; του 0; ισούται με; ύψιλον τριπλή παράγωγος; του 1; ισούται με 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_063() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>υ</mi>
    <mrow>
     <mrow><mo>(</mo>
      <mn>4</mn>
     <mo>)</mo></mrow></mrow>
   </msup>
   <mrow><mo>(</mo>
    <mi>τ</mi>
   <mo>)</mo></mrow><mo>=</mo><mi>φ</mi><mrow><mo>(</mo>
    <mrow>
     <mi>τ</mi><mo>,</mo><mi>υ</mi><mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η ανοίγει παρένθεση 4 κλείνει παρένθεση δύναμη του, ύψιλον; του τάαφ; ισούται με; φ; ανοίγει παρένθεση; τάαφ κόμμα; ύψιλον του τάαφ; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_064() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <msup>
      <mo>∂</mo>
      <mn>2</mn>
     </msup>
     </mrow>
    <mrow>
     <mo>∂</mo><msup>
      <mi>u</mi>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   <mi>Ξ</mi><mrow><mo>(</mo>
    <mrow>
     <mi>τ</mi><mo>,</mo><mi>σ</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mo>−</mo><mi>Γ</mi><mrow><mo>(</mo>
    <mrow>
     <mi>τ</mi><mo>,</mo><mi>σ</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, μερικό διαφορικό στο τετράγωνο, προς, μερικό διαφορικό, u στο τετράγωνο, τέλος κλάσματος; κεφαλαίο ξ; ανοίγει παρένθεση; τάαφ κόμμα, σίγμα; κλείνει παρένθεση; ισούται με; μείον κεφαλαίο γάμμα; ανοίγει παρένθεση; τάαφ κόμμα, σίγμα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_065() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>|</mo> <mrow>
    <mrow><mo>‖</mo> <mi>υ</mi> <mo>‖</mo></mrow></mrow> <mo>|</mo></mrow><mo>=</mo><mi>max</mi><mrow><mo>{</mo> <mrow>
    <mrow><mo>‖</mo> <mi>υ</mi> <mo>‖</mo></mrow><mo>,</mo><mrow><mo>‖</mo> <msup>
     <mi>υ</mi>
     <mo>′</mo>
    </msup>
     <mo>‖</mo></mrow><mo>,</mo><mrow><mo>‖</mo> <msup>
     <mi>υ</mi>
     <mo>″</mo>
    </msup>
     <mo>‖</mo></mrow></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η απόλυτη τιμή του νόρμα του ύψιλον τέλος απόλυτης τιμής; ισούται με; μέγιστο του; συνόλου νόρμα του ύψιλον, κόμμα; νόρμα του ύψιλον τόνος; κόμμα; νόρμα του ύψιλον διπλή παράγωγος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_066() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <msup>
        <mi>υ</mi>
        <mo>*</mo>
       </msup>
       </mrow>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <mo>′</mo><mtext>​</mtext><mo>′</mo></mrow>
   </msup>
   <mrow><mo>(</mo>
    <mi>τ</mi>
   <mo>)</mo></mrow><mo>=</mo><msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mi>Τ</mi><msup>
        <mi>υ</mi>
        <mo>*</mo>
       </msup>
       </mrow>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <mo>′</mo><mtext>​</mtext><mo>′</mo></mrow>
   </msup>
   <mrow><mo>(</mo>
    <mi>τ</mi>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση; ύψιλον αστερίσκος; κλείνει παρένθεση που υψώνεται στη τόνος τόνος, τέλος δύναμης; του τάαφ; ισούται με; ανοίγει παρένθεση; κεφαλαίο τάαφ; ύψιλον αστερίσκος; κλείνει παρένθεση που υψώνεται στη τόνος τόνος, τέλος δύναμης; του τάαφ")?;
    return Ok(());
    //theodora. fails. Now reads: ανοίγει παρένθεση; ύψιλον αστερίσκος; κλείνει παρένθεση που υψώνεται στη τόνος τόνος, τέλος δύναμης; τάαφ; ισούται με; ανοίγει παρένθεση; κεφαλαίο τάαφ; ύψιλον αστερίσκος; κλείνει παρένθεση που υψώνεται στη τόνος τόνος, τέλος δύναμης; τάαφ
}

#[test]
fn uoa_corpus_067() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mi>d</mi>
    <mrow>
     <mi>d</mi><mi>τ</mi></mrow>
   </mfrac>
   <mrow><mo>(</mo>
    <mrow>
     <msup>
      <mrow>
       <mrow><mo>|</mo> <mrow>
        <mover accent="true">
         <mi>υ</mi>
         <mo>˙</mo>
        </mover>
        <mrow><mo>(</mo>
         <mi>τ</mi>
        <mo>)</mo></mrow></mrow> <mo>|</mo></mrow></mrow>
      <mrow>
       <mi>π</mi><mrow><mo>(</mo>
        <mi>τ</mi>
       <mo>)</mo></mrow><mo>−</mo><mn>2</mn></mrow>
     </msup>
     <mover accent="true">
      <mi>υ</mi>
      <mo>˙</mo>
     </mover>
     <mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow></mrow>
   <mo>)</mo></mrow><mo>=</mo><mo>∇</mo><mi>Φ</mi><mrow><mo>(</mo>
    <mrow>
     <mi>τ</mi><mo>,</mo><mi>υ</mi><mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, d προς, d τάαφ, τέλος κλάσματος; ανοίγει παρένθεση; η απόλυτη τιμή του ύψιλον τελεία; του τάαφ τέλος απόλυτης τιμής που υψώνεται στη π του τάαφ, μείον 2 τέλος δύναμης; ύψιλον τελεία; του τάαφ; κλείνει παρένθεση; ισούται με; ανάδελτα του κεφαλαίο φ; ανοίγει παρένθεση; τάαφ κόμμα; ύψιλον του τάαφ; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_068() -> Result<()> { //
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mfrac><mi>d</mi><mrow><mi>d</mi><mi>&#x3C4;</mi></mrow></mfrac><mrow><mo>(</mo><msup><mrow><mo>&#x2016;</mo><mo>&#x2207;</mo><mi>&#x3C5;</mi><mo>&#x2016;</mo></mrow><mn>2</mn></msup><mo>+</mo><mn>2</mn><munder><mo>&#x222B;</mo><mi>&#x3A9;</mi></munder><mi>&#x3A6;</mi><mrow><mo>(</mo><mi>&#x3C5;</mi><mo>)</mo></mrow><mi>d</mi><mi>&#x3C7;</mi><mo>)</mo></mrow><mo>+</mo><mn>2</mn><msup><mrow><mo>&#x2016;</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>&#x2016;</mo></mrow><mn>2</mn></msup><mspace linebreak="newline"/><mo>=</mo><mn>2</mn><mrow><mo>(</mo><mrow><mo>(</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3B1;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>,</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>)</mo></mrow><mo>)</mo></mrow></math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, d προς, d τάαφ, τέλος κλάσματος; ανοίγει παρένθεση; νόρμα του ανάδελτα του ύψιλον στο τετράγωνο; συν; 2; ολοκλήρωμα επί του συνόλου κεφαλαίο ωμέγα του; κεφαλαίο φ του ύψιλον; d χ; κλείνει παρένθεση; συν; 2; νόρμα του κλάσματος, μερικό διαφορικό, ύψιλον, προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; στο τετράγωνο; ισούται με; 2; ανοίγει παρένθεση; ανοίγει παρένθεση; κλάσμα, μερικό διαφορικό, άλφα, προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; κόμμα; κλάσμα, μερικό διαφορικό, ύψιλον, προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; κλείνει παρένθεση; κλείνει παρένθεση")?;
    return Ok(());
    //theodora. fails. Needs genitive. Now reads: κλάσμα, d προς, d τάαφ, τέλος κλάσματος; ανοίγει παρένθεση; νόρμα του ανάδελτα του ύψιλον στο τετράγωνο; συν; 2; ολοκλήρωμα επί του συνόλου κεφαλαίο ωμέγα του; κεφαλαίο φ του ύψιλον; d χ; κλείνει παρένθεση; συν; 2; νόρμα του κλάσμα, μερικό διαφορικό, ύψιλον, προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; στο τετράγωνο; ισούται με; 2; ανοίγει παρένθεση; ανοίγει παρένθεση; κλάσμα, μερικό διαφορικό, άλφα, προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; κόμμα; κλάσμα, μερικό διαφορικό, ύψιλον, προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; κλείνει παρένθεση; κλείνει παρένθεση
}

#[test]
fn uoa_corpus_069() -> Result<()> { //
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mfrac><mo>&#x2202;</mo><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mrow><mo>(</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>)</mo></mrow><mo>&#x2212;</mo><mi>&#x394;</mi><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>+</mo><msup><mi>&#x3C6;</mi><mo>'</mo></msup><mrow><mo>(</mo><mi>&#x3C5;</mi><mo>)</mo></mrow><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mspace linebreak="newline"/><mspace width="0em"/><mo>=</mo><mo>&#x2212;</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3B1;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>+</mo><mi>&#x394;</mi><mi>&#x3B1;</mi><mo>&#x2212;</mo><mi>&#x3C5;</mi><mo>&#x2212;</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac></math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, μερικό διαφορικό προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; ανοίγει παρένθεση; κλάσμα, μερικό διαφορικό, ύψιλον, προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; κλείνει παρένθεση; μείον; κεφαλαίο δέλτα; κλάσμα, μερικό διαφορικό, ύψιλον, προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; συν; φ τόνος, του ύψιλον; κλάσμα, μερικό διαφορικό, ύψιλον, προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; ισούται με; μείον κλάσμα, μερικό διαφορικό, άλφα, προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; συν, κεφαλαίο δέλτα, άλφα; μείον ύψιλον μείον; κλάσμα, μερικό διαφορικό, ύψιλον, προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_070() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mi>d</mi>
    <mrow>
     <mi>d</mi><mi>τ</mi></mrow>
   </mfrac>
   <mrow><mo>(</mo>
    <mrow>
     <msup>
      <mrow>
       <mrow><mo>‖</mo> <mrow>
        <msup>
         <mi>Α</mi>
         <mrow>
          <mfrac>
           <mn>3</mn>
           <mn>2</mn>
          </mfrac>
          </mrow>
        </msup>
        <mi>α</mi></mrow> <mo>‖</mo></mrow></mrow>
      <mn>2</mn>
     </msup>
     <mo>+</mo><msup>
      <mrow>
       <mrow><mo>‖</mo> <mrow>
        <mi>Α</mi><mfrac>
         <mrow>
          <mi>d</mi><mi>α</mi></mrow>
         <mrow>
          <mi>d</mi><mi>τ</mi></mrow>
        </mfrac>
        </mrow> <mo>‖</mo></mrow></mrow>
      <mn>2</mn>
     </msup>
     </mrow>
   <mo>)</mo></mrow><mo>≤</mo><mi>γ</mi><msup>
    <mrow>
     <mrow><mo>‖</mo> <mrow>
      <mi>Δ</mi><mi>υ</mi></mrow> <mo>‖</mo></mrow></mrow>
    <mn>2</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, d προς, d τάαφ, τέλος κλάσματος; ανοίγει παρένθεση; νόρμα του κεφαλαίο άλφα που υψώνεται στη 3 δεύτερα τέλος δύναμης; άλφα στο τετράγωνο; συν; νόρμα του κεφαλαίο άλφα; κλάσμα, d άλφα, προς, d τάαφ, τέλος κλάσματος; στο τετράγωνο; κλείνει παρένθεση; είναι μικρότερο από ή ίσο με; γάμμα; νόρμα του κεφαλαίο δέλτα, ύψιλον στο τετράγωνο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_071() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <mo>∂</mo><msub>
      <mi>υ</mi>
      <mi>μ</mi>
     </msub>
     </mrow>
    <mrow>
     <mo>∂</mo><mi>τ</mi></mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mrow>
     <mo>∂</mo><msub>
      <mi>ν</mi>
      <mi>μ</mi>
     </msub>
     </mrow>
    <mrow>
     <mo>∂</mo><mi>τ</mi></mrow>
   </mfrac>
   <mo>+</mo><mfrac>
    <mrow>
     <mo>∂</mo><msub>
      <mover accent="true">
       <mi>υ</mi>
       <mo>¯</mo>
      </mover>
      
      <mi>μ</mi>
     </msub>
     </mrow>
    <mrow>
     <mo>∂</mo><mi>χ</mi></mrow>
   </mfrac>
   <mo>=</mo><msub>
    <mover accent="true">
     <mi>φ</mi>
     <mo>¯</mo>
    </mover>
    
    <mi>μ</mi>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, μερικό διαφορικό; ύψιλον δείκτης μί; προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; συν; κλάσμα, μερικό διαφορικό; νί δείκτης μί; προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; συν; κλάσμα, μερικό διαφορικό; ύψιλον παύλα, δείκτης μί; προς, μερικό διαφορικό χ, τέλος κλάσματος; ισούται με, φ παύλα, δείκτης μί")?;
    return Ok(());
}

#[test]
fn uoa_corpus_072() -> Result<()> { //
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><munder><mi>sup</mi><mrow><mn>0</mn><mo>&#x2264;</mo><mi>&#x3C4;</mi><mo>&#x2264;</mo><mi>&#x3A4;</mi></mrow></munder><mrow><mo>[</mo><mfrac><mrow><mo>&#x2202;</mo><mrow><mo>(</mo><msub><mi>&#x3C5;</mi><mi>&#x3BD;</mi></msub><mo>&#x2212;</mo><mi>&#x3C5;</mi><mo>)</mo></mrow></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>]</mo></mrow><mo>+</mo><munder><mi>sup</mi><mrow><mn>0</mn><mo>&#x2264;</mo><mi>&#x3C4;</mi><mo>&#x2264;</mo><mi>&#x3A4;</mi></mrow></munder><mrow><mo>[</mo><mfrac><mrow><mo>&#x2202;</mo><mrow><mo>(</mo><msub><mi>&#x3B8;</mi><mi>&#x3BD;</mi></msub><mo>&#x2212;</mo><mi>&#x3B8;</mi><mo>)</mo></mrow></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>]</mo></mrow><mspace linebreak="newline"/><mo>&#x2264;</mo><mo>&#xA0;</mo><mi>&#x393;</mi><mrow><mo>(</mo><msubsup><mi>&#x3BB;</mi><mrow><mi>&#x3BD;</mi><mo>+</mo><mn>1</mn></mrow><mrow><mo>&#x2212;</mo><mfrac><mn>1</mn><mn>8</mn></mfrac></mrow></msubsup><mo>+</mo><msubsup><mi>&#x3BC;</mi><mrow><mi>&#x3BD;</mi><mo>+</mo><mn>1</mn></mrow><mrow><mo>&#x2212;</mo><mfrac><mn>1</mn><mn>8</mn></mfrac></mrow></msubsup><mo>)</mo></mrow></math>"#;
    test("el", "SimpleSpeak", expr, "σουπρέμουμ για 0, είναι μικρότερο από ή ίσο με; τάαφ, είναι μικρότερο από ή ίσο με; κεφαλαίο τάαφ; του; ανοίγει αγκύλη; κλάσμα, μερικό διαφορικό; ανοίγει παρένθεση; ύψιλον δείκτης νί; μείον ύψιλον; κλείνει παρένθεση, προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; κλείνει αγκύλη; συν; σουπρέμουμ για 0, είναι μικρότερο από ή ίσο με; τάαφ, είναι μικρότερο από ή ίσο με; κεφαλαίο τάαφ; του; ανοίγει αγκύλη; κλάσμα, μερικό διαφορικό; ανοίγει παρένθεση; θήτα δείκτης νί; μείον θήτα; κλείνει παρένθεση, προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; κλείνει αγκύλη; είναι μικρότερο από ή ίσο με; κεφαλαίο γάμμα; ανοίγει παρένθεση; λάμδα δείκτης, νί συν 1 τέλος δείκτη, που υψώνεται στη μείον 1 όγδοο τέλος δύναμης; συν; μί δείκτης, νί συν 1 τέλος δείκτη, που υψώνεται στη μείον 1 όγδοο τέλος δύναμης; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_073() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <mi>d</mi><mrow><mo>[</mo> <mrow>
      <mfrac>
       <mrow>
        <mi>φ</mi><mo stretchy="false">(</mo><mi>χ</mi><mo stretchy="false">)</mo></mrow>
       <mrow>
        <mi>τ</mi><mo stretchy="false">(</mo><mi>χ</mi><mo stretchy="false">)</mo></mrow>
      </mfrac>
      </mrow> <mo>]</mo></mrow></mrow>
    <mrow>
     <mi>d</mi><mi>x</mi></mrow>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mi>τ</mi><mo stretchy="false">(</mo><mi>χ</mi><mo stretchy="false">)</mo><mo>×</mo><mfrac>
      <mrow>
       <mi>d</mi><mrow><mo>[</mo> <mrow>
        <mi>φ</mi><mo stretchy="false">(</mo><mi>χ</mi><mo stretchy="false">)</mo></mrow> <mo>]</mo></mrow></mrow>
      <mrow>
       <mi>d</mi><mi>x</mi></mrow>
     </mfrac>
     <mo>−</mo><mi>φ</mi><mo stretchy="false">(</mo><mi>χ</mi><mo stretchy="false">)</mo><mo>×</mo><mfrac>
      <mrow>
       <mi>d</mi><mrow><mo>[</mo> <mrow>
        <mi>τ</mi><mo stretchy="false">(</mo><mi>χ</mi><mo stretchy="false">)</mo></mrow> <mo>]</mo></mrow></mrow>
      <mrow>
       <mi>d</mi><mi>x</mi></mrow>
     </mfrac>
     </mrow>
    <mrow>
     <msup>
      <mrow>
       <mrow><mo>[</mo> <mrow>
        <mi>τ</mi><mo stretchy="false">(</mo><mi>χ</mi><mo stretchy="false">)</mo></mrow> <mo>]</mo></mrow></mrow>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, d του; ανοίγει αγκύλη; κλάσμα, φ του χ, προς, τάαφ του χ, τέλος κλάσματος; κλείνει αγκύλη, προς, d x, τέλος κλάσματος; ισούται με; κλάσμα, τάαφ του χ, επί; κλάσμα, d; ανοίγει αγκύλη, φ του χ, κλείνει αγκύλη, προς, d x, τέλος κλάσματος; μείον; φ του χ, επί; κλάσμα, d; ανοίγει αγκύλη, τάαφ του χ, κλείνει αγκύλη, προς, d x, τέλος κλάσματος; προς, ανοίγει αγκύλη, τάαφ του χ, κλείνει αγκύλη στο τετράγωνο, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_074() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>δ</mi>
    <mrow>
     <mi>Κ</mi><mo>,</mo><mi>σ</mi></mrow>
   </msub>
   <mo>&gt;</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "δέλτα δείκτης; κεφαλαίο καπα, κόμμα, σίγμα τέλος δείκτη; είναι μεγαλύτερο από 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_075() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msubsup>
    <mi>β</mi>
    <mi>σ</mi>
    <mi>Κ</mi>
   </msubsup>
   <mo>≠</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "βήτα δείκτης σίγμα, στην κεφαλαίο καπα οστή; είναι διάφορο του 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_076() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>χ</mi>
    <mi>σ</mi>
   </msub>
   <mo>=</mo><mstyle displaystyle="true">
    <munder>
     <mo>∑</mo>
     <mrow>
      <mi>Κ</mi><mo>∈</mo><mi>Μ</mi></mrow>
    </munder>
    <mrow>
     <msubsup>
      <mi>β</mi>
      <mi>σ</mi>
      <mi>Κ</mi>
     </msubsup>
     <msub>
      <mi>χ</mi>
      <mi>Κ</mi>
     </msub>
     </mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ δείκτης σίγμα; ισούται με; άθροισμα για κεφαλαίο καπα, ανήκει κεφαλαίο μί του; βήτα δείκτης σίγμα, στην κεφαλαίο καπα οστή; χ δείκτης, κεφαλαίο καπα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_077() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mrow>
     <mrow><mo>{</mo> <mrow>
      <msub>
       <mi>χ</mi>
       <mi>Κ</mi>
      </msub>
      </mrow> <mo>}</mo></mrow></mrow>
    <mi>Κ</mi>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει άγκιστρο; χ δείκτης, κεφαλαίο καπα; κλείνει άγκιστρο δείκτης, κεφαλαίο καπα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_078() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>υ</mi>
    <mi>Κ</mi>
   </msub>
   <mo>=</mo><mi>φ</mi><mrow><mo>(</mo>
    <mrow>
     <msub>
      <mi>χ</mi>
      <mi>Κ</mi>
     </msub>
     </mrow>
   <mo>)</mo></mrow><mo>,</mo><mo>∀</mo><mi>Κ</mi><mo>∈</mo><mi>Μ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ύψιλον δείκτης, κεφαλαίο καπα; ισούται με; φ του; ανοίγει παρένθεση; χ δείκτης, κεφαλαίο καπα; κλείνει παρένθεση; κόμμα; για κάθε; κεφαλαίο καπα, ανήκει κεφαλαίο μί")?;
    return Ok(());
}

#[test]
fn uoa_corpus_079() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mo>∂</mo>
    <mn>1</mn>
   </msup>
   <msup>
    <mi>υ</mi>
    <mi>ν</mi>
   </msup>
   <mo>=</mo><mfrac>
    <mrow>
     <msup>
      <mi>υ</mi>
      <mi>ν</mi>
     </msup>
     <mo>−</mo><msup>
      <mi>υ</mi>
      <mrow>
       <mi>ν</mi><mo>−</mo><mn>1</mn></mrow>
     </msup>
     </mrow>
    <mi>κ</mi>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "μερικό διαφορικό στην πρώτη; ύψιλον στην νί οστή; ισούται με; κλάσμα, ύψιλον στην νί οστή, μείον; ύψιλον που υψώνεται στη νί μείον 1 τέλος δύναμης; προς καπα, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_080() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>Φ</mi>
    <mrow>
     <mi>Κ</mi><mo>,</mo><mi>σ</mi></mrow>
   </msub>
   <mrow><mo>(</mo>
    <mi>υ</mi>
   <mo>)</mo></mrow><mo>=</mo><mstyle displaystyle="true">
    <munder>
     <mo>∑</mo>
     <mrow>
      <msup>
       <mi>σ</mi>
       <mo>′</mo>
      </msup>
      <mo>∈</mo><msub>
       <mi>Ε</mi>
       <mi>κ</mi>
      </msub>
      </mrow>
    </munder>
    <mrow>
     <msup>
      <mi>Α</mi>
      <mrow>
       <mi>σ</mi><msup>
        <mi>σ</mi>
        <mo>′</mo>
       </msup>
       </mrow>
     </msup>
     <mrow><mo>(</mo>
      <mrow>
       <msub>
        <mi>υ</mi>
        <mi>κ</mi>
       </msub>
       <mo>−</mo><msub>
        <mi>υ</mi>
        <msup>
         <mi>σ</mi>
         <mo>′</mo>
        </msup>
        
       </msub>
       </mrow>
     <mo>)</mo></mrow></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο φ δείκτης; κεφαλαίο καπα, κόμμα, σίγμα τέλος δείκτη; του ύψιλον; ισούται με; άθροισμα για σίγμα τόνος; ανήκει, κεφαλαίο έψιλον δείκτης καπα, του; κεφαλαίο άλφα που υψώνεται στη σίγμα σίγμα τόνος, τέλος δύναμης; ανοίγει παρένθεση; ύψιλον δείκτης καπα; μείον; ύψιλον δείκτης, σίγμα τόνος, τέλος δείκτη; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_081() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>α</mi><msubsup>
    <mrow>
     <mrow><mo>|</mo> <mi>υ</mi> <mo>|</mo></mrow></mrow>
    <mi>χ</mi>
    <mn>2</mn>
   </msubsup>
   <mo>≤</mo><msub>
    <mrow>
     <mrow><mo>〈</mo> <mrow>
      <mi>υ</mi><mo>,</mo><mi>υ</mi></mrow> <mo>〉</mo></mrow></mrow>
    <mi>Φ</mi>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άλφα; η απόλυτη τιμή του ύψιλον δείκτης χ, στο τετράγωνο; είναι μικρότερο από ή ίσο με; αριστερά γωνιακή αγκύλη; ύψιλον κόμμα, ύψιλον; δεξιά γωνιακή αγκύλη δείκτης, κεφαλαίο φ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_082() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>Γ</mi>
    <mn>6</mn>
   </msub>
   <mo>=</mo><msup>
    <mi>δ</mi>
    <mn>3</mn>
   </msup>
   <mi>θ</mi><mo>+</mo><msup>
    <mi>δ</mi>
    <mrow>
     <mfrac>
      <mn>7</mn>
      <mn>2</mn>
     </mfrac>
     </mrow>
   </msup>
   <msup>
    <mi>θ</mi>
    <mn>2</mn>
   </msup>
   <mo>+</mo><msup>
    <mi>δ</mi>
    <mrow>
     <mfrac>
      <mn>5</mn>
      <mn>2</mn>
     </mfrac>
     </mrow>
   </msup>
   <mi>θ</mi><mo>+</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο γάμμα δείκτης 6; ισούται με; δέλτα στον κύβο, θήτα; συν; δέλτα που υψώνεται στη 7 δεύτερα τέλος δύναμης; θήτα στο τετράγωνο; συν; δέλτα που υψώνεται στη 5 δεύτερα τέλος δύναμης; θήτα; συν 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_083() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msubsup>
    <mover accent="true">
     <mi>υ</mi>
     <mo>¯</mo>
    </mover>
    
    <mi>Δ</mi>
    <mn>0</mn>
   </msubsup>
   <mo>=</mo><msubsup>
    <mi>υ</mi>
    <mi>Δ</mi>
    <mn>0</mn>
   </msubsup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ύψιλον παύλα, δείκτης, κεφαλαίο δέλτα, στην 0; ισούται με; ύψιλον δείκτης, κεφαλαίο δέλτα, στην 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_084() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>θ</mi>
    <mn>2</mn>
   </msup>
   <msup>
    <mi>υ</mi>
    <mi>ν</mi>
   </msup>
   <mo>=</mo><msup>
    <mi>κ</mi>
    <mrow>
     <mo>−</mo><mn>1</mn></mrow>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <msup>
      <mo>∂</mo>
      <mn>1</mn>
     </msup>
     <msup>
      <mi>υ</mi>
      <mi>ν</mi>
     </msup>
     <mo>−</mo><msup>
      <mo>∂</mo>
      <mn>1</mn>
     </msup>
     <msup>
      <mi>υ</mi>
      <mrow>
       <mi>ν</mi><mo>−</mo><mn>1</mn></mrow>
     </msup>
     </mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "θήτα στο τετράγωνο; ύψιλον στην νί οστή; ισούται με; καπα στην μείον 1; ανοίγει παρένθεση; μερικό διαφορικό στην πρώτη; ύψιλον στην νί οστή; μείον; μερικό διαφορικό στην πρώτη; ύψιλον που υψώνεται στη νί μείον 1 τέλος δύναμης; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_085() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>‖</mo> <mi>χ</mi> <mo>‖</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "νόρμα του χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_086() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>α</mi><mrow><mo>‖</mo> <mrow>
    <msubsup>
     <mi>η</mi>
     <mi>Δ</mi>
     <mrow>
      <mi>ν</mi><mo>+</mo><mn>1</mn></mrow>
    </msubsup>
    </mrow> <mo>‖</mo></mrow><mi>χ</mi><mo>≤</mo><msub>
    <mi>Γ</mi>
    <mi>π</mi>
   </msub>
   <msubsup>
    <mi>Μ</mi>
    <mi>Δ</mi>
    <mrow>
     <mi>ν</mi><mo>+</mo><mn>1</mn></mrow>
   </msubsup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άλφα; νόρμα του ήτα δείκτης, κεφαλαίο δέλτα, που υψώνεται στη νί συν 1 τέλος δύναμης; χ; είναι μικρότερο από ή ίσο με; κεφαλαίο γάμμα δείκτης π; κεφαλαίο μί δείκτης, κεφαλαίο δέλτα, που υψώνεται στη νί συν 1 τέλος δύναμης")?;
    return Ok(());
}

#[test]
fn uoa_corpus_087() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>Φ</mi>
    <mn>2</mn>
   </msub>
   <mrow><mo>(</mo>
    <mrow>
     <mi>τ</mi><mo>,</mo><mi>χ</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mrow><mo>|</mo> <mrow>
    <mi>sin</mi><mi>ω</mi><mi>τ</mi></mrow> <mo>|</mo></mrow><msup>
    <mrow>
     <mrow><mo>|</mo> <mi>χ</mi> <mo>|</mo></mrow></mrow>
    <mn>3</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο φ δείκτης 2; ανοίγει παρένθεση; τάαφ κόμμα, χ; κλείνει παρένθεση; ισούται με; η απόλυτη τιμή του ημίτονο του ωμέγα τάαφ τέλος απόλυτης τιμής; η απόλυτη τιμή του χ στον κύβο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_088() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>Η</mi><mrow><mo>(</mo>
    <mrow>
     <mi>τ</mi><mo>,</mo><mi>χ</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mo>−</mo><msup>
    <mrow>
     <mrow><mo>|</mo> <mi>χ</mi> <mo>|</mo></mrow></mrow>
    <mrow>
     <mn>1</mn><mo>+</mo><mi>α</mi></mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο ήτα; ανοίγει παρένθεση; τάαφ κόμμα, χ; κλείνει παρένθεση; ισούται με; μείον η απόλυτη τιμή του χ που υψώνεται στη 1 συν άλφα τέλος δύναμης")?;
    return Ok(());
}

#[test]
fn uoa_corpus_089() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>γ</mi><mo>=</mo><msub>
    <mrow>
     <mi>log</mi></mrow>
    <mrow>
     <mn>2</mn><mi>λ</mi></mrow>
   </msub>
   <mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>μ</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "γάμμα ισούται με; το λογάριθμο με βάση 2 λάμδα; του; ανοίγει παρένθεση, 2 μί, κλείνει παρένθεση")?;
    return Ok(());
      //theodora. fails. accusative rule doesn't include log with base
}

#[test]
fn uoa_corpus_090() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mn>2</mn>
   </mfrac>
   <mo>≤</mo><mi>μ</mi><mo>&lt;</mo><msup>
    <mn>2</mn>
    <mrow>
     <msup>
      <mi>π</mi>
      <mo>−</mo>
     </msup>
     <mo>−</mo><mn>1</mn></mrow>
   </msup>
   <msup>
    <mi>λ</mi>
    <mrow>
     <msup>
      <mi>π</mi>
      <mo>−</mo>
     </msup>
     </mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "1 δεύτερο, είναι μικρότερο από ή ίσο με; μί είναι μικρότερο από; 2 που υψώνεται στη π εκθέτης μείον; μείον 1 τέλος δύναμης; λάμδα που υψώνεται στη π εκθέτης μείον, τέλος δύναμης")?;
    return Ok(());
}

#[test]
fn uoa_corpus_091() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mrow>
     <mrow><mo>|</mo> <mi>υ</mi> <mo>|</mo></mrow></mrow>
    <mrow>
     <mi>π</mi><mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow></mrow>
   </msub>
   <mo>&gt;</mo><mn>1</mn><mo>⇒</mo><msubsup>
    <mrow>
     <mrow><mo>|</mo> <mi>υ</mi> <mo>|</mo></mrow></mrow>
    <mrow>
     <mi>π</mi><mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <msup>
      <mi>π</mi>
      <mo>−</mo>
     </msup>
     </mrow>
   </msubsup>
   <mo>≤</mo><mi>ρ</mi><mrow><mo>(</mo>
    <mi>υ</mi>
   <mo>)</mo></mrow><mo>≤</mo><msubsup>
    <mrow>
     <mrow><mo>|</mo> <mi>υ</mi> <mo>|</mo></mrow></mrow>
    <mrow>
     <mi>π</mi><mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <msup>
      <mi>π</mi>
      <mo>+</mo>
     </msup>
     </mrow>
   </msubsup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η απόλυτη τιμή του ύψιλον δείκτης, π του τάαφ; είναι μεγαλύτερο από; 1, διπλό βέλος προς τα δεξιά; η απόλυτη τιμή του ύψιλον δείκτης, π του τάαφ, που υψώνεται στη π εκθέτης μείον, τέλος δύναμης; είναι μικρότερο από ή ίσο με; ρ του ύψιλον; είναι μικρότερο από ή ίσο με; η απόλυτη τιμή του ύψιλον δείκτης, π του τάαφ, που υψώνεται στη π εκθέτης συν, τέλος δύναμης")?;
    return Ok(());
}

#[test]
fn uoa_corpus_092() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msubsup>
    <mi>Β</mi>
    <mi>Τ</mi>
    <mrow>
     <mn>1,</mn><mi>π</mi><mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow></mrow>
   </msubsup>
   <mo>=</mo><msubsup>
    <mover accent="true">
     <mi>Β</mi>
     <mo>˜</mo>
    </mover>
    
    <mi>Τ</mi>
    <mrow>
     <mn>1,</mn><mi>π</mi><mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow></mrow>
   </msubsup>
   <mo>⊕</mo><msup>
    <mi>ℝ</mi>
    <mi>Ν</mi>
   </msup>
   </mrow>
  </math>"#;
    // TODO: add expected speech
    test("el", "SimpleSpeak", expr, "κεφαλαίο βήτα δείκτης, κεφαλαίο τάαφ, που υψώνεται στη 1, π του τάαφ τέλος δύναμης; ισούται με; κεφαλαίο βήτα κυματοειδής γραμμή, δείκτης, κεφαλαίο τάαφ, που υψώνεται στη 1, π του τάαφ τέλος δύναμης; κυκλωμένο συν; οι πραγματικοί αριθμοί στην κεφαλαίο νί οστή")?;
    return Ok(());
}

#[test]
fn uoa_corpus_093() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>υ</mi><mrow><mo>(</mo>
    <mrow>
     <mi>χ</mi><mo>,</mo><mi>τ</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><msub>
    <mi>γ</mi>
    <mn>1</mn>
   </msub>
   <mrow><mo>(</mo>
    <mi>τ</mi>
   <mo>)</mo></mrow><mo>+</mo><msub>
    <mi>γ</mi>
    <mn>2</mn>
   </msub>
   <mrow><mo>(</mo>
    <mi>τ</mi>
   <mo>)</mo></mrow><mi>ln</mi><mi>ρ</mi><mo>+</mo><msub>
    <mi>υ</mi>
    <mn>0</mn>
   </msub>
   <mrow><mo>(</mo>
    <mrow>
     <mi>χ</mi><mo>,</mo><mi>τ</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ύψιλον; ανοίγει παρένθεση; χ κόμμα, τάαφ; κλείνει παρένθεση; ισούται με; γάμμα δείκτης 1; του τάαφ; συν; γάμμα δείκτης 2; του τάαφ; ο φυσικός λογάριθμος, του ρ; συν; ύψιλον δείκτης 0; ανοίγει παρένθεση; χ κόμμα, τάαφ; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_094() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>|</mo> <mrow>
    <msub>
     <mi>υ</mi>
     <mn>1</mn>
    </msub>
    </mrow> <mo>|</mo></mrow><mo>≤</mo><mi>Γ</mi><msup>
    <mrow>
     <mrow><mo>|</mo> <mi>χ</mi> <mo>|</mo></mrow></mrow>
    <mrow>
     <mi>Im</mi><msub>
      <mi>λ</mi>
      <mn>1</mn>
     </msub>
     </mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η απόλυτη τιμή του ύψιλον δείκτης 1, τέλος απόλυτης τιμής; είναι μικρότερο από ή ίσο με; κεφαλαίο γάμμα; η απόλυτη τιμή του χ που υψώνεται στη φανταστικό μέρος, του, λάμδα δείκτης 1, τέλος δύναμης")?;
    return Ok(());
}

#[test]
fn uoa_corpus_095() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>Im</mi><mi>λ</mi><mrow><mo>(</mo>
    <mi>τ</mi>
   <mo>)</mo></mrow><mo>=</mo><mi>β</mi><mo>+</mo><mi>α</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "φανταστικό μέρος, του λάμδα του τάαφ; ισούται με, βήτα συν άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_096() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>Im</mi><msub>
    <mi>λ</mi>
    <mrow>
     <msub>
      <mi>Ν</mi>
      <mn>0</mn>
     </msub>
     </mrow>
   </msub>
   <mrow><mo>(</mo>
    <mrow>
     <msub>
      <mi>τ</mi>
      <mn>0</mn>
     </msub>
     </mrow>
   <mo>)</mo></mrow><mo>=</mo><mo>−</mo><msub>
    <mi>β</mi>
    <mn>1</mn>
   </msub>
   <mo>+</mo><mn>2</mn><mi>μ</mi><mo>+</mo><msub>
    <mi>λ</mi>
    <mn>1</mn>
   </msub>
   <mo>−</mo><mfrac>
    <mi>ν</mi>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "φανταστικό μέρος, του; λάμδα δείκτης, κεφαλαίο νί δείκτης 0, τέλος δείκτη; του; ανοίγει παρένθεση; τάαφ δείκτης 0; κλείνει παρένθεση; ισούται με; μείον βήτα δείκτης 1; συν 2 μί συν, λάμδα δείκτης 1; μείον νί προς 2")?;
    return Ok(());
}

#[test]
fn uoa_corpus_097() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mover accent="true">
    <mi>φ</mi>
    <mo>^</mo>
   </mover>
   <mo>=</mo><msub>
    <mi>κ</mi>
    <mn>0</mn>
   </msub>
   <mi>φ</mi><mo>+</mo><msub>
    <mi>Λ</mi>
    <mn>1</mn>
   </msub>
   <mi>υ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "φ καπέλο; ισούται με; καπα δείκτης 0; φ; συν, κεφαλαίο λάμδα δείκτης 1; ύψιλον")?;
    return Ok(());
}

#[test]
fn uoa_corpus_098() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>Λ</mi><msub>
    <mi>Σ</mi>
    <mi>δ</mi>
   </msub>
   <mo>=</mo><msub>
    <mi>Α</mi>
    <mi>δ</mi>
   </msub>
   <mo>+</mo><msub>
    <mi>Ρ</mi>
    <mi>δ</mi>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο λάμδα; κεφαλαίο σίγμα δείκτης δέλτα; ισούται με; κεφαλαίο άλφα δείκτης δέλτα; συν, κεφαλαίο ρ δείκτης δέλτα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_099() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>ι</mi><msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mo>−</mo><mn>1</mn></mrow>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <mi>μ</mi><mo>−</mo><mn>1</mn></mrow>
   </msup>
   <mi>Λ</mi><mi>υ</mi><mo>−</mo><msub>
    <mi>υ</mi>
    <mi>τ</mi>
   </msub>
   <mo>=</mo><mi>φ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ιότα; ανοίγει παρένθεση, μείον 1, κλείνει παρένθεση που υψώνεται στη μί μείον 1 τέλος δύναμης; κεφαλαίο λάμδα, ύψιλον; μείον, ύψιλον δείκτης τάαφ; ισούται με φ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_100() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>χ</mi>
    <mn>0</mn>
   </msup>
   <mo>+</mo><mi>χ</mi><mo>+</mo><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mo>+</mo><msup>
    <mi>χ</mi>
    <mn>3</mn>
   </msup>
   <mo>+</mo><msup>
    <mi>χ</mi>
    <mn>4</mn>
   </msup>
   <mo>+</mo><msup>
    <mi>χ</mi>
    <mn>5</mn>
   </msup>
   <mo>+</mo><msup>
    <mi>χ</mi>
    <mn>6</mn>
   </msup>
   <mo>+</mo><mo>…</mo><mo>+</mo><msup>
    <mi>χ</mi>
    <mrow>
     <mi>ν</mi><mo>−</mo><mn>1</mn></mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ στην 0 συν χ συν χ στο τετράγωνο, συν χ στον κύβο συν χ στην τέταρτη, συν χ στην πέμπτη, συν χ στην έκτη συν αποσιωπητικά, συν; χ που υψώνεται στη νί μείον 1 τέλος δύναμης")?;
    return Ok(());
}

#[test]
fn uoa_corpus_101() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mi>χ</mi><mo>+</mo><mi>ψ</mi><mo>+</mo><mi>ζ</mi></mrow>
     <mo>)</mo></mrow></mrow>
    <mn>2</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση; χ συν ψ συν ζήτα; κλείνει παρένθεση στο τετράγωνο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_102() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mi>χ</mi><mo>+</mo><mi>ψ</mi></mrow>
     <mo>)</mo></mrow></mrow>
    <mn>2</mn>
   </msup>
   <mo>+</mo><mi>ζ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση, χ συν ψ, κλείνει παρένθεση στο τετράγωνο; συν ζήτα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_103() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>χ</mi><mo>+</mo><msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mi>ψ</mi><mo>+</mo><mi>ζ</mi></mrow>
     <mo>)</mo></mrow></mrow>
    <mn>2</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ συν; ανοίγει παρένθεση; ψ συν ζήτα; κλείνει παρένθεση στο τετράγωνο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_104() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>χ</mi><mo>+</mo><mi>ψ</mi><mo>+</mo><msup>
    <mi>ζ</mi>
    <mn>2</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ συν ψ συν ζήτα στο τετράγωνο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_105() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>χ</mi>
    <mn>0</mn>
   </msub>
   <mo>+</mo><msub>
    <mi>χ</mi>
    <mn>1</mn>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ δείκτης 0; συν χ δείκτης 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_106() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mover accent="true">
    <mi>χ</mi>
    <mo>˙</mo>
   </mover>
   <mo>+</mo><mover accent="true">
    <mi>χ</mi>
    <mo>¨</mo>
   </mover>
   <mo>+</mo><mover accent="true">
    <mi>χ</mi>
    <mo>˜</mo>
   </mover>
   <mo>+</mo><mover accent="true">
    <mi>χ</mi>
    <mo>^</mo>
   </mover>
   <mo>+</mo><msup>
    <mi>χ</mi>
    <mo>″</mo>
   </msup>
   <mo>+</mo><msup>
    <mi>χ</mi>
    <mo>‴</mo>
   </msup>
   <mo>+</mo><msup>
    <mi>χ</mi>
    <mo>′</mo>
   </msup>
   <mo>+</mo><msup>
    <mi>χ</mi>
    <mo>*</mo>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ τελεία, συν χ διπλή κουκκίδα; συν, χ κυματοειδής γραμμή; συν χ καπέλο, συν, χ διπλή παράγωγος; συν, χ τριπλή παράγωγος; συν χ τόνος, συν χ αστερίσκος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_107() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>α</mi>
    <mrow>
     <mn>1,1</mn></mrow>
   </msup>
   <mo>+</mo><msup>
    <mi>α</mi>
    <mrow>
     <mn>2,2</mn></mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άλφα στην 1,1, συν άλφα στην 2,2")?;
    return Ok(());
}

#[test]
fn uoa_corpus_108() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mrow>
     <mrow><mo>〈</mo> <mrow>
      <mi>υ</mi><mo>,</mo><mi>υ</mi></mrow> <mo>〉</mo></mrow></mrow>
    <mi>φ</mi>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "αριστερά γωνιακή αγκύλη; ύψιλον κόμμα, ύψιλον; δεξιά γωνιακή αγκύλη δείκτης φ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_109() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <munder>
     <mo>∑</mo>
     <mrow>
      <mi>κ</mi><mo>∈</mo><mi>ℝ</mi></mrow>
    </munder>
    <mrow>
     <msub>
      <mi>α</mi>
      <mi>κ</mi>
     </msub>
     </mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άθροισμα για καπα ανήκει, στους πραγματικούς αριθμούς του; άλφα δείκτης καπα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_110() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>ι</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>Ν</mi>
    </munderover>
    <mrow>
     <msup>
      <mi>α</mi>
      <mn>2</mn>
     </msup>
     </mrow>
   </mstyle><mo>+</mo><mi>β</mi><mo>+</mo><msup>
    <mi>γ</mi>
    <mn>2</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άθροισμα από ιότα ισούται με 1 ως κεφαλαίο νί του; άλφα στο τετράγωνο; συν βήτα συν, γάμμα στο τετράγωνο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_111() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>ε</mi>
    <mrow>
     <msup>
      <mi>ε</mi>
      <mrow>
       <msup>
        <mi>ε</mi>
        <mi>χ</mi>
       </msup>
       </mrow>
     </msup>
     </mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "έψιλον που υψώνεται στη έψιλον που υψώνεται στη έψιλον στην χ οστή τέλος δύναμης; τέλος εκθέτη")?;
    return Ok(());
}

#[test]
fn uoa_corpus_112() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>ε</mi>
    <mrow>
     <msup>
      <mi>ε</mi>
      <mrow>
       <msup>
        <mi>ε</mi>
        <mi>χ</mi>
       </msup>
       </mrow>
     </msup>
     </mrow>
   </msup>
   <mo>⋅</mo><msup>
    <mi>ε</mi>
    <mrow>
     <msup>
      <mi>ε</mi>
      <mi>χ</mi>
     </msup>
     </mrow>
   </msup>
   <mo>⋅</mo><msup>
    <mi>ε</mi>
    <mi>χ</mi>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "έψιλον που υψώνεται στη έψιλον που υψώνεται στη έψιλον στην χ οστή τέλος δύναμης; τέλος εκθέτη; φορές; έψιλον που υψώνεται στη έψιλον στην χ οστή τέλος δύναμης; φορές, έψιλον στην χ οστή")?;
    return Ok(());
}

#[test]
fn uoa_corpus_113() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>ε</mi>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <msup>
        <mi>ε</mi>
        <mrow>
         <msup>
          <mi>ε</mi>
          <mi>χ</mi>
         </msup>
         </mrow>
       </msup>
       <mo>+</mo><msup>
        <mi>ε</mi>
        <mi>χ</mi>
       </msup>
       <mo>+</mo><mi>χ</mi></mrow>
     <mo>)</mo></mrow></mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "έψιλον που υψώνεται στη ανοίγει παρένθεση; έψιλον που υψώνεται στη έψιλον στην χ οστή τέλος δύναμης; συν, έψιλον στην χ οστή, συν χ; κλείνει παρένθεση τέλος δύναμης")?;
    return Ok(());
}

#[test]
fn uoa_corpus_114() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>ι</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>ν</mi>
    </munderover>
    <mrow>
     <msub>
      <mi>α</mi>
      <mi>ι</mi>
     </msub>
     </mrow>
   </mstyle><mo>=</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άθροισμα από ιότα ισούται με 1 ως νί του; άλφα δείκτης ιότα; ισούται με 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_115() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>ι</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>ν</mi>
    </munderover>
    <mrow>
     <msub>
      <mi>α</mi>
      <mi>ι</mi>
     </msub>
     <mo>+</mo><msub>
      <mi>β</mi>
      <mi>ι</mi>
     </msub>
     </mrow>
   </mstyle><mo>=</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άθροισμα από ιότα ισούται με 1 ως νί του; άλφα δείκτης ιότα; συν, βήτα δείκτης ιότα; ισούται με 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_116() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mn>1</mn><mo>+</mo><mi>χ</mi><mo>+</mo><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mo>+</mo><msup>
    <mi>χ</mi>
    <mn>3</mn>
   </msup>
   <mo>+</mo><msup>
    <mi>χ</mi>
    <mn>4</mn>
   </msup>
   <mo>+</mo><mo>…</mo><mo>+</mo><msup>
    <mi>χ</mi>
    <mrow>
     <mi>ν</mi><mo>−</mo><mn>1</mn></mrow>
   </msup>
   <mo>+</mo><mo>…</mo><mo>=</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <mn>1</mn><mo>+</mo><mi>χ</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "1 συν χ συν χ στο τετράγωνο, συν χ στον κύβο συν χ στην τέταρτη, συν αποσιωπητικά, συν; χ που υψώνεται στη νί μείον 1 τέλος δύναμης; συν αποσιωπητικά; ισούται με; κλάσμα, 1 προς, 1 συν χ, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_117() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>χ</mi><mo>−</mo><mfrac>
    <mrow>
     <msup>
      <mi>χ</mi>
      <mn>2</mn>
     </msup>
     </mrow>
    <mn>2</mn>
   </mfrac>
   <mo>+</mo><mfrac>
    <mrow>
     <msup>
      <mi>χ</mi>
      <mn>3</mn>
     </msup>
     </mrow>
    <mn>3</mn>
   </mfrac>
   <mo>−</mo><mfrac>
    <mrow>
     <msup>
      <mi>χ</mi>
      <mn>4</mn>
     </msup>
     </mrow>
    <mn>4</mn>
   </mfrac>
   <mo>+</mo><mfrac>
    <mrow>
     <msup>
      <mi>χ</mi>
      <mn>5</mn>
     </msup>
     </mrow>
    <mn>5</mn>
   </mfrac>
   <mo>±</mo><mo>…</mo><mo>=</mo><mi>log</mi><mrow><mo>(</mo>
    <mrow>
     <mn>1</mn><mo>+</mo><mi>χ</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ μείον; κλάσμα, χ στο τετράγωνο, προς 2, τέλος κλάσματος; συν; κλάσμα, χ στον κύβο, προς 3, τέλος κλάσματος; μείον; κλάσμα, χ στην τέταρτη, προς 4, τέλος κλάσματος; συν; κλάσμα, χ στην πέμπτη, προς 5, τέλος κλάσματος; συν πλήν, αποσιωπητικά; ισούται με; το λογάριθμο, του; ανοίγει παρένθεση, 1 συν χ, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_118() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mi>α</mi><mo>+</mo><mi>β</mi></mrow>
     <mo>)</mo></mrow></mrow>
    <mn>3</mn>
   </msup>
   <mo>=</mo><msup>
    <mi>α</mi>
    <mn>3</mn>
   </msup>
   <mo>+</mo><mn>3</mn><mo>⋅</mo><msup>
    <mi>α</mi>
    <mn>2</mn>
   </msup>
   <mo>⋅</mo><mi>β</mi><mo>+</mo><mn>3</mn><mo>⋅</mo><mi>α</mi><mo>⋅</mo><msup>
    <mi>β</mi>
    <mn>2</mn>
   </msup>
   <mo>+</mo><msup>
    <mi>β</mi>
    <mn>3</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση; άλφα συν βήτα; κλείνει παρένθεση στον κύβο; ισούται με; άλφα στον κύβο, συν; 3 φορές, άλφα στο τετράγωνο, φορές βήτα; συν; 3 φορές άλφα φορές, βήτα στο τετράγωνο; συν βήτα στον κύβο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_119() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msubsup>
    <mi>χ</mi>
    <mn>1</mn>
    <mi>κ</mi>
   </msubsup>
   <mo>+</mo><msubsup>
    <mi>χ</mi>
    <mn>2</mn>
    <mi>κ</mi>
   </msubsup>
   <mo>+</mo><msubsup>
    <mi>χ</mi>
    <mn>3</mn>
    <mi>κ</mi>
   </msubsup>
   <mo>+</mo><mo>…</mo><mo>+</mo><msubsup>
    <mi>χ</mi>
    <mi>ν</mi>
    <mi>κ</mi>
   </msubsup>
   <mo>=</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ δείκτης 1, στην καπα οστή; συν, χ δείκτης 2, στην καπα οστή; συν, χ δείκτης 3, στην καπα οστή; συν αποσιωπητικά, συν, χ δείκτης νί, στην καπα οστή; ισούται με 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_120() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>χ</mi>
    <mrow>
     <msup>
      <mi>κ</mi>
      <mn>1</mn>
     </msup>
     </mrow>
   </msub>
   <mo>+</mo><msub>
    <mi>χ</mi>
    <mrow>
     <msup>
      <mi>κ</mi>
      <mn>2</mn>
     </msup>
     </mrow>
   </msub>
   <mo>+</mo><msub>
    <mi>χ</mi>
    <mrow>
     <msup>
      <mi>κ</mi>
      <mn>3</mn>
     </msup>
     </mrow>
   </msub>
   <mo>+</mo><mo>…</mo><msub>
    <mi>χ</mi>
    <mrow>
     <msup>
      <mi>κ</mi>
      <mi>ν</mi>
     </msup>
     </mrow>
   </msub>
   <mo>=</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ δείκτης, καπα στην πρώτη τέλος δείκτη; συν; χ δείκτης, καπα στο τετράγωνο τέλος δείκτη; συν, χ δείκτης, καπα στον κύβο τέλος δείκτη; συν; αποσιωπητικά; χ δείκτης, καπα στην νί οστή τέλος δείκτη; ισούται με 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_121() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>χ</mi>
    <mrow>
     <msub>
      <mi>κ</mi>
      <mn>1</mn>
     </msub>
     </mrow>
   </msub>
   <mo>+</mo><msub>
    <mi>χ</mi>
    <mrow>
     <msub>
      <mi>κ</mi>
      <mn>2</mn>
     </msub>
     </mrow>
   </msub>
   <mo>+</mo><msub>
    <mi>χ</mi>
    <mrow>
     <msub>
      <mi>κ</mi>
      <mn>3</mn>
     </msub>
     </mrow>
   </msub>
   <mo>+</mo><mo>…</mo><mo>+</mo><msub>
    <mi>χ</mi>
    <mrow>
     <msub>
      <mi>κ</mi>
      <mi>ν</mi>
     </msub>
     </mrow>
   </msub>
   <mo>=</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ δείκτης, καπα δείκτης 1, τέλος δείκτη; συν; χ δείκτης, καπα δείκτης 2, τέλος δείκτη; συν; χ δείκτης, καπα δείκτης 3, τέλος δείκτη; συν αποσιωπητικά, συν; χ δείκτης, καπα δείκτης νί, τέλος δείκτη; ισούται με 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_122() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>χ</mi>
    <mrow>
     <msup>
      <mn>2</mn>
      <mi>ψ</mi>
     </msup>
     </mrow>
   </msup>
   <mo>≠</mo><msup>
    <mi>χ</mi>
    <mrow>
     <msup>
      <mn>2</mn>
      <mi>ψ</mi>
     </msup>
     </mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ που υψώνεται στη 2 στην ψ οστή τέλος δύναμης; είναι διάφορο του; χ που υψώνεται στη 2 στην ψ οστή τέλος δύναμης")?;
    return Ok(());
}

#[test]
fn uoa_corpus_123() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>α</mi><mo>⋅</mo><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mo>+</mo><mi>β</mi><mo>⋅</mo><mi>χ</mi><mo>+</mo><mi>γ</mi><mo>=</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άλφα φορές, χ στο τετράγωνο; συν βήτα φορές χ, συν γάμμα; ισούται με 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_124() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>log</mi><mn>3</mn><msup>
    <mrow>
     <mo stretchy="false">(</mo><mi>χ</mi><mo>+</mo><mi>ψ</mi><mo stretchy="false">)</mo></mrow>
    <mn>3</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ο λογάριθμος, του 3; ανοίγει παρένθεση, χ συν ψ, κλείνει παρένθεση στον κύβο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_125() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>log</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>χ</mi><mo>≠</mo><mn>2</mn><mo>⋅</mo><mi>log</mi><mi>χ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ο λογάριθμος στο τετράγωνο; του χ; είναι διάφορο του; 2 φορές, ο λογάριθμος, του χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_126() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <mi>log</mi><mi>χ</mi></mrow>
    <mrow>
     <mi>log</mi><mi>α</mi></mrow>
   </mfrac>
   <mo>=</mo><msub>
    <mrow>
     <mi>log</mi></mrow>
    <mi>α</mi>
   </msub>
   <mi>χ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, ο λογάριθμος, του χ, προς, ο λογάριθμος, του άλφα, τέλος κλάσματος; ισούται με; το λογάριθμο με βάση άλφα; του χ")?;
    return Ok(());
    //theodora. fails. Needs to fix accusative for logarithm with base. Now reads: κλάσμα, ο λογάριθμος, του χ, προς, ο λογάριθμος, του άλφα, τέλος κλάσματος; ισούται με; ο λογάριθμος με βάση άλφα; του χ

}

#[test]
fn uoa_corpus_127() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mover accent="true">
    <mi>χ</mi>
    <mo>¯</mo>
   </mover>
   <mo>+</mo><mi>χ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ παύλα, συν χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_128() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>arg</mi><mrow><mo>(</mo>
    <mi>ζ</mi>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "όρισμα του ζήτα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_129() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>υ</mi>
    <mrow>
     <mrow><mo>(</mo>
      <mn>0</mn>
     <mo>)</mo></mrow></mrow>
   </msup>
   <mo>=</mo><mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>ζ</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>ν</mi>
    </munderover>
    <mrow>
     <msub>
      <mi>ξ</mi>
      <mi>ζ</mi>
     </msub>
     <msup>
      <mrow/>
      <mrow>
       <mrow><mo>(</mo>
        <mn>1</mn>
       <mo>)</mo></mrow></mrow>
     </msup>
     <msub>
      <mi>ρ</mi>
      <mi>ζ</mi>
     </msub>
     <mrow><mo>(</mo>
      <mrow>
       <msup>
        <mi>υ</mi>
        <mrow>
         <mrow><mo>(</mo>
          <mi>ι</mi>
         <mo>)</mo></mrow></mrow>
       </msup>
       </mrow>
     <mo>)</mo></mrow></mrow>
   </mstyle><mo>=</mo><mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>ζ</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>ν</mi>
    </munderover>
    <mrow>
     <msub>
      <mi>ξ</mi>
      <mi>ζ</mi>
     </msub>
     <msup>
      <mrow/>
      <mrow>
       <mrow><mo>(</mo>
        <mn>2</mn>
       <mo>)</mo></mrow></mrow>
     </msup>
     <msub>
      <mi>ρ</mi>
      <mi>ζ</mi>
     </msub>
     <mrow><mo>(</mo>
      <mrow>
       <msup>
        <mi>υ</mi>
        <mrow>
         <mrow><mo>(</mo>
          <mn>2</mn>
         <mo>)</mo></mrow></mrow>
       </msup>
       </mrow>
     <mo>)</mo></mrow></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ύψιλον υψωμένο στην ανοίγει παρένθεση 0 κλείνει παρένθεση τέλος δύναμης; ισούται με; άθροισμα από ζήτα ισούται με 1 ως νί του; ξ δείκτης ζήτα; εκθέτης ανοίγει παρένθεση 1 κλείνει παρένθεση, ρ δείκτης ζήτα; του; ανοίγει παρένθεση; ύψιλον υψωμένο στην ανοίγει παρένθεση, ιότα, κλείνει παρένθεση τέλος δύναμης; κλείνει παρένθεση; ισούται με; άθροισμα από ζήτα ισούται με 1 ως νί του; ξ δείκτης ζήτα; εκθέτης ανοίγει παρένθεση 2 κλείνει παρένθεση, ρ, δείκτης ζήτα; του; ανοίγει παρένθεση; ύψιλον υψωμένο στην ανοίγει παρένθεση 2 κλείνει παρένθεση τέλος δύναμης; κλείνει παρένθεση")?;
    return Ok(());
    // theodora. fails . a mathml or rules issue?

}

#[test]
fn uoa_corpus_130() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>|</mo> <mi>υ</mi> <mo>|</mo></mrow><mover>
    <mo>=</mo>
    <mrow>
     <mi>ο</mi><mi>ρ</mi><mi>ι</mi><mi>σ</mi><mi>μ</mi><mi>ο</mi><mi>ς</mi></mrow>
   </mover>
   <msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mstyle displaystyle="true">
        <munderover>
         <mo>∑</mo>
         <mrow>
          <mi>κ</mi><mo>=</mo><mn>1</mn></mrow>
         <mi>ν</mi>
        </munderover>
        <mrow>
         <msubsup>
          <mi>υ</mi>
          <mi>κ</mi>
          <mn>2</mn>
         </msubsup>
         </mrow>
       </mstyle></mrow>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <mfrac>
      <mn>1</mn>
      <mn>2</mn>
     </mfrac>
     </mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η απόλυτη τιμή του ύψιλον; ισούται με με ορισμος από πάνω; ανοίγει παρένθεση; άθροισμα από καπα ισούται με 1 ως νί του; ύψιλον δείκτης καπα, στο τετράγωνο; κλείνει παρένθεση που υψώνεται στη 1 δεύτερο τέλος δύναμης")?;
    return Ok(());
}

#[test]
fn uoa_corpus_131() -> Result<()> { //
    let expr = r#"<math>
 <mtable columnalign="left">
   <mtr>
    <mtd>
     <mstyle displaystyle="true">
      <mrow><munderover>
       <mo>∫</mo>
       <mn>0</mn>
       <mi>∞</mi>
      </munderover>
      <mrow>
       <mrow><mo>|</mo> <mrow>
        <msub>
         <mi>ξ</mi>
         <mi>ι</mi>
        </msub>
        <msup>
         <mrow/>
         <mrow>
          <mrow><mo>(</mo>
           <mn>1</mn>
          <mo>)</mo></mrow></mrow>
        </msup>
        <mrow><mo>(</mo>
         <mrow>
          <mi>τ</mi><mo>,</mo><mi>χ</mi></mrow>
        <mo>)</mo></mrow></mrow> <mo>|</mo></mrow><mi>d</mi><mi>χ</mi></mrow>
     </mrow>
     
    </mstyle><mo>≤</mo>
   </mtd>
  </mtr>
  <mtr>
   <mtd>
    <mi>Γ</mi><mrow><mo>(</mo>
     <mrow>
      <msub>
       <mrow>
        <mrow><mo>‖</mo> <mrow>
         <msup>
          <mi>φ</mi>
          <mrow>
           <mrow><mo>(</mo>
            <mn>0</mn>
           <mo>)</mo></mrow></mrow>
         </msup>
         </mrow> <mo>‖</mo></mrow></mrow>
       <mrow>
        <msup>
         <mi>Λ</mi>
         <mn>1</mn>
        </msup>
        <mrow><mo>(</mo>
         <mrow>
          <msup>
           <mi>ℝ</mi>
           <mo>+</mo>
          </msup>
          </mrow>
        <mo>)</mo></mrow></mrow>
      </msub>
      <mo>+</mo><msub>
       <mrow>
        <mrow><mo>‖</mo> <mrow>
         <msup>
          <mi>α</mi>
          <mrow>
           <mrow><mo>(</mo>
            <mn>0</mn>
           <mo>)</mo></mrow></mrow>
         </msup>
         </mrow> <mo>‖</mo></mrow></mrow>
       <mrow>
        <msup>
         <mi>Λ</mi>
         <mn>1</mn>
        </msup>
        <mrow><mo>(</mo>
         <mrow>
          <msup>
           <mi>ℝ</mi>
           <mo>+</mo>
          </msup>
          </mrow>
        <mo>)</mo></mrow></mrow>
      </msub>
      <mo>+</mo><msub>
       <mrow>
        <mrow><mo>‖</mo> <mrow>
         <msup>
          <mi>η</mi>
          <mrow>
           <mrow><mo>(</mo>
            <mn>0</mn>
           <mo>)</mo></mrow></mrow>
         </msup>
         </mrow> <mo>‖</mo></mrow></mrow>
       <mrow>
        <msup>
         <mi>Λ</mi>
         <mn>1</mn>
        </msup>
        <mrow><mo>(</mo>
         <mrow>
          <msup>
           <mi>ℝ</mi>
           <mo>+</mo>
          </msup>
          </mrow>
        <mo>)</mo></mrow></mrow>
      </msub>
      </mrow>
    <mo>)</mo></mrow>
   </mtd>
  </mtr>
 </mtable>
 
 </math>"#;
    test("el", "SimpleSpeak", expr, "πίνακας με 2 γραμμές και 1 στήλες; γραμμή 1; στήλη 1; ολοκλήρωμα από 0 ως άπειρο του; η απόλυτη τιμή του ξ με 2 μετατεταγμένα μέρη, δείκτης ιότα εκθέτης 1; ανοίγει παρένθεση; τάαφ κόμμα, χ; κλείνει παρένθεση τέλος απόλυτης τιμής; d χ; είναι μικρότερο από ή ίσο με; γραμμή 2; στήλη 1; κεφαλαίο γάμμα; ανοίγει παρένθεση; νόρμα του φ που υψώνεται στη ανοίγει παρένθεση 0 κλείνει παρένθεση τέλος δύναμης, δείκτης; η πρώτη δύναμη του, κεφαλαίο λάμδα; του; ανοίγει παρένθεση; οι θετικοί πραγματικοί αριθμοί; κλείνει παρένθεση τέλος δείκτη; συν; νόρμα του άλφα που υψώνεται στη ανοίγει παρένθεση 0 κλείνει παρένθεση τέλος δύναμης, δείκτης; η πρώτη δύναμη του, κεφαλαίο λάμδα; του; ανοίγει παρένθεση; οι θετικοί πραγματικοί αριθμοί; κλείνει παρένθεση τέλος δείκτη; συν; νόρμα του ήτα που υψώνεται στη ανοίγει παρένθεση 0 κλείνει παρένθεση τέλος δύναμης, δείκτης; η πρώτη δύναμη του, κεφαλαίο λάμδα; του; ανοίγει παρένθεση; οι θετικοί πραγματικοί αριθμοί; κλείνει παρένθεση τέλος δείκτη; κλείνει παρένθεση")?;
    return Ok(());
    //theodora. it's read as a matrix because of the mathml, not an actual issues

}

#[test]
fn uoa_corpus_132() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mover accent="true">
     <mi>Β</mi>
     <mo>˜</mo>
    </mover>
    
    <mn>1</mn>
   </msub>
   <mrow><mo>(</mo>
    <mi>Τ</mi>
   <mo>)</mo></mrow><mo>=</mo><munder>
    <mrow>
     <mi>max</mi></mrow>
    <mrow>
     <mi>ι</mi><mo>=</mo><mn>1,</mn><mo>…</mo><mi>ν</mi></mrow>
   </munder>
   <munder>
    <mrow>
     <mi>max</mi></mrow>
    <mrow>
     <mi>ξ</mi><mo>≠</mo><mi>ι</mi></mrow>
   </munder>
   <munder>
    <mrow>
     <mi>sup</mi></mrow>
    <mrow>
     <msub>
      <mi>Γ</mi>
      <mi>ξ</mi>
     </msub>
     </mrow>
   </munder>
   <mstyle displaystyle="true">
    <mrow>
     <munder>
      <mo>∫</mo>
      <mrow>
       <msub>
        <mi>Γ</mi>
        <mi>ξ</mi>
       </msub>
       </mrow>
     </munder>
     <mrow>
      <mrow><mo>|</mo> <mrow>
       <msub>
        <mi>υ</mi>
        <mi>ι</mi>
       </msub>
       </mrow> <mo>|</mo></mrow><mi>d</mi><mi>τ</mi></mrow>
    </mrow>
    
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο βήτα κυματοειδής γραμμή, δείκτης 1; του κεφαλαίο τάαφ; ισούται με; μέγιστο για ιότα ισούται με; 1, αποσιωπητικά, νί; του, μέγιστο για ξ είναι διάφορο του, ιότα; του, σουπρέμουμ για κεφαλαίο γάμμα δείκτης ξ; του; ολοκλήρωμα επί του συνόλου κεφαλαίο γάμμα δείκτης ξ, του; η απόλυτη τιμή του ύψιλον δείκτης ιότα, τέλος απόλυτης τιμής; d τάαφ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_133() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msubsup>
    <mrow>
     <mrow><mo>{</mo> <mrow>
      <msub>
       <mi>ε</mi>
       <mi>λ</mi>
      </msub>
      <msup>
       <mrow/>
       <mrow>
        <mo>′</mo><mtext>​</mtext><mo>′</mo></mrow>
      </msup>
      </mrow> <mo>}</mo></mrow></mrow>
    <mrow>
     <mi>λ</mi><mo>=</mo><mn>1</mn></mrow>
    <mrow>
     <mi>ν</mi><mo>+</mo><mi>μ</mi><mo>−</mo><mn>2</mn><mi>κ</mi></mrow>
   </msubsup>
   <mo>=</mo><mrow><mo>{</mo> <mrow>
    <msub>
     <mrow>
      <mrow><mo>{</mo> <mrow>
       <msub>
        <mi>ε</mi>
        <mi>ι</mi>
       </msub>
       </mrow> <mo>}</mo></mrow></mrow>
     <mrow>
      <mi>ι</mi><mo>∉</mo><mi>Α</mi></mrow>
    </msub>
    <mo>,</mo><msub>
     <mrow>
      <mrow><mo>{</mo> <mrow>
       <msub>
        <mi>ε</mi>
        <mi>ξ</mi>
       </msub>
       <msup>
        <mrow/>
        <mo>′</mo>
       </msup>
       </mrow> <mo>}</mo></mrow></mrow>
     <mrow>
      <mi>ξ</mi><mo>∉</mo><msup>
       <mi>Α</mi>
       <mo>′</mo>
      </msup>
      </mrow>
    </msub>
    </mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει άγκιστρο; έψιλον με 2 μετατεταγμένα μέρη, δείκτης λάμδα εκθέτης τόνος τόνος; κλείνει άγκιστρο δείκτης, λάμδα ισούται με 1 τέλος δείκτη, που υψώνεται στη νί συν μί μείον 2 καπα τέλος δύναμης; ισούται με; το σύνολο ανοίγει άγκιστρο; έψιλον δείκτης ιότα; κλείνει άγκιστρο δείκτης; ιότα δεν ανήκει, κεφαλαίο άλφα τέλος δείκτη; κόμμα; ανοίγει άγκιστρο; έψιλον με 2 μετατεταγμένα μέρη, δείκτης ξ εκθέτης τόνος; κλείνει άγκιστρο δείκτης; ξ δεν ανήκει, κεφαλαίο άλφα τόνος, τέλος δείκτη")?;
    return Ok(());
}

#[test]
fn uoa_corpus_134() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mo>∇</mo><msub>
    <mi>υ</mi>
    <mi>ε</mi>
   </msub>
   <mover>
    <mo>→</mo>
    <mrow>
     <mo stretchy="false">(</mo><mi>ν</mi><mo>+</mo><mn>1,</mn><mi>μ</mi><mo>+</mo><mn>1</mn><mo stretchy="false">)</mo></mrow>
   </mover>
   <mo>∇</mo><mi>υ</mi><mo>+</mo><mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>κ</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>ν</mi>
    </munderover>
    <mrow>
     <msub>
      <mo>∇</mo>
      <mrow>
       <msub>
        <mi>ψ</mi>
        <mi>κ</mi>
       </msub>
       </mrow>
     </msub>
     <msub>
      <mi>υ</mi>
      <mi>κ</mi>
     </msub>
     </mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανάδελτα του ύψιλον δείκτης έψιλον, τέλος ανάδελτα; βέλος προς τα δεξιά με ανοίγει παρένθεση; νί συν 1, μί, συν 1; κλείνει παρένθεση από πάνω; ανάδελτα του ύψιλον, συν; άθροισμα από καπα ισούται με 1 ως νί του; το ανάδελτα δείκτης, ψ δείκτης καπα, τέλος δείκτη; ύψιλον δείκτης καπα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_135() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msubsup>
    <mi>ℑ</mi>
    <mrow>
     <mi>w</mi><mi>s</mi><mi>e</mi><mi>p</mi><mn>,1</mn></mrow>
    <mrow>
     <mi>μ</mi><mo>∼</mo><mi>κ</mi></mrow>
   </msubsup>
   <mo>=</mo><mrow><mo>{</mo> <mrow>
    <mrow><mo>(</mo>
     <mrow>
      <mi>ε</mi><mo>,</mo><msubsup>
       <mrow>
        <mrow><mo>{</mo> <mrow>
         <msub>
          <msup>
           <mi>ε</mi>
           <mo>′</mo>
          </msup>
          
          <mi>ξ</mi>
         </msub>
         </mrow> <mo>}</mo></mrow></mrow>
       <mrow>
        <mi>ξ</mi><mo>=</mo><mn>1</mn></mrow>
       <mi>μ</mi>
      </msubsup>
      </mrow>
    <mo>)</mo></mrow><mo>∈</mo><msubsup>
     <mi>ℑ</mi>
     <mrow>
      <mi>w</mi><mi>s</mi><mi>e</mi><mi>p</mi></mrow>
     <mrow>
      <mi>μ</mi><mo>∼</mo><mi>κ</mi></mrow>
    </msubsup>
    <mo>:</mo><mfrac>
     <mrow>
      <msup>
       <mi>ε</mi>
       <mn>2</mn>
      </msup>
      </mrow>
     <mrow>
      <msub>
       <mi>ε</mi>
       <mi>μ</mi>
      </msub>
      <msup>
       <mrow/>
       <mo>′</mo>
      </msup>
      </mrow>
    </mfrac>
    <mo>→</mo><mn>0</mn></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "φράκτουρ κεφαλαίο i δείκτης wsep,1 τέλος δείκτη, που υψώνεται στη μί κυματοειδής γραμμή, καπα τέλος δύναμης; ισούται με; το σύνολο όλων των ανοίγει παρένθεση; έψιλον κόμμα; ανοίγει άγκιστρο; έψιλον τόνος, δείκτης ξ; κλείνει άγκιστρο δείκτης, ξ ισούται με 1 τέλος δείκτη, στην μί οστή; κλείνει παρένθεση; που ανήκει; φράκτουρ κεφαλαίο i δείκτης wsep τέλος δείκτη, που υψώνεται στη μί κυματοειδής γραμμή, καπα τέλος δύναμης, τέτοια ώστε κλάσμα, έψιλον στο τετράγωνο, προς, έψιλον με 2 μετατεταγμένα μέρη, δείκτης μί εκθέτης τόνος, τέλος κλάσματος; βέλος προς τα δεξιά 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_136() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>ε</mi>
    <mn>1</mn>
   </msub>
   <mo>=</mo><mrow><mo>{</mo> <mrow>
    <mi>ε</mi><mo>,</mo><mrow><mo>{</mo> <mrow>
     <msup>
      <mi>ε</mi>
      <mrow>
       <mn>0,2</mn></mrow>
     </msup>
     <mo>,</mo><msup>
      <mi>ε</mi>
      <mrow>
       <mn>0,5</mn></mrow>
     </msup>
     <mo>,</mo><mi>ε</mi><mo>,</mo><msup>
      <mi>ε</mi>
      <mrow>
       <mn>1,2</mn></mrow>
     </msup>
     <mo>,</mo><mfrac>
      <mrow>
       <msup>
        <mi>ε</mi>
        <mrow>
         <mn>1,5</mn></mrow>
       </msup>
       </mrow>
      <mrow>
       <mrow><mo>|</mo> <mrow>
        <mi>log</mi><mi>ε</mi></mrow> <mo>|</mo></mrow></mrow>
     </mfrac>
     </mrow> <mo>}</mo></mrow></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "έψιλον δείκτης 1; ισούται με; το σύνολο έψιλον κόμμα; το σύνολο έψιλον στην 0,2, κόμμα; έψιλον στην 0,5, κόμμα, έψιλον κόμμα; έψιλον στην 1,2, κόμμα; κλάσμα, έψιλον στην 1,5, προς, η απόλυτη τιμή του ο λογαρίθμου, του έψιλον, τέλος κλάσματος")?;
    return Ok(());
    //theodora. fails with current rules. Should be okay when adding genitive clause
}


#[test]
fn uoa_corpus_137() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msubsup>
    <mi>υ</mi>
    <mi>μ</mi>
    <mi>ν</mi>
   </msubsup>
   <mo>+</mo><msubsup>
    <mi>ν</mi>
    <mi>μ</mi>
    <mi>ν</mi>
   </msubsup>
   <mo>+</mo><mi>κ</mi><mfrac>
    <mrow>
     <mi>d</mi><msubsup>
      <mi>υ</mi>
      <mi>μ</mi>
      <mi>ν</mi>
     </msubsup>
     </mrow>
    <mrow>
     <mi>d</mi><mi>χ</mi></mrow>
   </mfrac>
   <mo>=</mo><mi>κ</mi><mo>×</mo><msubsup>
    <mi>φ</mi>
    <mi>μ</mi>
    <mi>ν</mi>
   </msubsup>
   <mo>+</mo><msubsup>
    <mi>υ</mi>
    <mi>μ</mi>
    <mrow>
     <mi>ν</mi><mo>−</mo><mn>1</mn></mrow>
   </msubsup>
   <mo>+</mo><msubsup>
    <mi>ν</mi>
    <mi>μ</mi>
    <mrow>
     <mi>ν</mi><mo>−</mo><mn>1</mn></mrow>
   </msubsup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ύψιλον δείκτης μί, στην νί οστή; συν, νί δείκτης μί, στην νί οστή; συν; καπα; κλάσμα, d, ύψιλον δείκτης μί, στην νί οστή, προς, d χ, τέλος κλάσματος; ισούται με; καπα επί, φ δείκτης μί, στην νί οστή; συν; ύψιλον δείκτης μί, που υψώνεται στη νί μείον 1 τέλος δύναμης; συν; νί δείκτης μί, που υψώνεται στη νί μείον 1 τέλος δύναμης")?;
    return Ok(());
}

#[test]
fn uoa_corpus_138() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow>
     <munder>
      <mo>∫</mo>
      <mi>Ω</mi>
     </munder>
     <mrow>
      <msup>
       <mrow>
        <mrow><mo>|</mo> <mrow>
         <msubsup>
          <mi>κ</mi>
          <mi>μ</mi>
          <mi>ξ</mi>
         </msubsup>
         </mrow> <mo>|</mo></mrow></mrow>
       <mn>2</mn>
      </msup>
      <mi>d</mi><mi>χ</mi><mo>+</mo><mi>κ</mi><mstyle displaystyle="true">
       <munderover>
        <mo>∑</mo>
        <mrow>
         <mi>ν</mi><mo>=</mo><mn>1</mn></mrow>
        <mi>ξ</mi>
       </munderover>
       <mrow>
        <msup>
         <mrow>
          <mrow><mo>|</mo> <mrow>
           <msubsup>
            <mi>κ</mi>
            <mi>μ</mi>
            <mi>ν</mi>
           </msubsup>
           </mrow> <mo>|</mo></mrow></mrow>
         <mn>2</mn>
        </msup>
        </mrow>
      </mstyle></mrow>
    </mrow>
    
   </mstyle></mrow>
  </math>"#;
    // TODO: add expected speech
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα επί του συνόλου κεφαλαίο ωμέγα της απόλυτης τιμής του καπα δείκτης μί, στην ξ οστή τέλος απόλυτης τιμής στο τετράγωνο; d χ; συν; καπα; άθροισμα από νί ισούται με 1 ως ξ της απόλυτης τιμής του καπα δείκτης μί, στην νί οστή τέλος απόλυτης τιμής στο τετράγωνο")?;
    return Ok(());
    //theodora.  fails with current rules. Should be okay when we add genitive
}

#[test]
fn uoa_corpus_139() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>sgn</mi><mstyle displaystyle="true">
    <munder>
     <mo>∑</mo>
     <mrow>
      <mi>ξ</mi><mo>∈</mo><mi>Ι</mi><mrow><mo>(</mo>
       <mrow>
        <msub>
         <mi>δ</mi>
         <mn>1</mn>
        </msub>
        <mo>,</mo><msub>
         <mi>δ</mi>
         <mn>2</mn>
        </msub>
        </mrow>
      <mo>)</mo></mrow></mrow>
    </munder>
    <mrow>
     <msub>
      <mi>α</mi>
      <mi>ξ</mi>
     </msub>
     <mstyle displaystyle="true">
      <mrow>
       <munder>
        <mo>∫</mo>
        <mrow>
         <msub>
          <mi>Γ</mi>
          <mrow>
           <msub>
            <mi>ι</mi>
            <mn>1</mn>
           </msub>
           </mrow>
         </msub>
         </mrow>
       </munder>
       <mrow>
        <msub>
         <mi>ε</mi>
         <mi>ξ</mi>
        </msub>
        <mi>d</mi><mi>χ</mi></mrow>
      </mrow>
      
     </mstyle></mrow>
   </mstyle><mo>=</mo><mo>−</mo><mi>sgn</mi><mstyle displaystyle="true">
    <munder>
     <mo>∑</mo>
     <mrow>
      <mi>ξ</mi><mo>∈</mo><mi>Ι</mi><mrow><mo>(</mo>
       <mrow>
        <msub>
         <mi>δ</mi>
         <mn>1</mn>
        </msub>
        <mo>,</mo><msub>
         <mi>δ</mi>
         <mn>2</mn>
        </msub>
        </mrow>
      <mo>)</mo></mrow></mrow>
    </munder>
    <mrow>
     <msub>
      <mi>α</mi>
      <mi>ξ</mi>
     </msub>
     <mstyle displaystyle="true">
      <mrow>
       <munder>
        <mo>∫</mo>
        <mrow>
         <msub>
          <mi>Γ</mi>
          <mrow>
           <msub>
            <mi>ι</mi>
            <mn>2</mn>
           </msub>
           </mrow>
         </msub>
         </mrow>
       </munder>
       <mrow>
        <msub>
         <mi>ε</mi>
         <mi>ξ</mi>
        </msub>
        <mi>d</mi><mi>χ</mi></mrow>
      </mrow>
      
     </mstyle></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "πρόσημο; άθροισμα για ξ ανήκει; κεφαλαίο ιότα; ανοίγει παρένθεση; δέλτα δείκτης 1; κόμμα; δέλτα δείκτης 2; κλείνει παρένθεση του; άλφα δείκτης ξ; ολοκλήρωμα επί του συνόλου κεφαλαίο γάμμα δείκτης, ιότα δείκτης 1, τέλος δείκτη, του; έψιλον δείκτης ξ; d χ; ισούται με; μείον πρόσημο; άθροισμα για ξ ανήκει; κεφαλαίο ιότα; ανοίγει παρένθεση; δέλτα δείκτης 1; κόμμα; δέλτα δείκτης 2; κλείνει παρένθεση του; άλφα δείκτης ξ; ολοκλήρωμα επί του συνόλου κεφαλαίο γάμμα δείκτης, ιότα δείκτης 2, τέλος δείκτη, του; έψιλον δείκτης ξ; d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_140() -> Result<()> { //
        let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mi>&#x3C6;</mi><mo>(</mo><msubsup><mi>&#x3A6;</mi><mi>&#x3C3;</mi><mi>&#x3BA;</mi></msubsup><mo>)</mo><mo>=</mo><munder><munder><mrow><mi>&#x3C6;</mi><mo>(</mo><msubsup><mi>&#x3A6;</mi><mi>&#x3C3;</mi><mi>&#x3BA;</mi></msubsup><mo>)</mo><mo>&#x2212;</mo><msub><mi>&#x3C6;</mi><mi>&#x3BA;</mi></msub><mo>(</mo><msubsup><mi>&#x3A6;</mi><mi>&#x3C3;</mi><mi>&#x3BA;</mi></msubsup><mo>)</mo></mrow><mo>&#x23DF;</mo></munder><mpadded lspace="-1px"><mo>&#x21C9;</mo><mn>0</mn></mpadded></munder><mo>+</mo><munder><munder><mrow><msub><mi>&#x3C6;</mi><mi>&#x3BA;</mi></msub><mo>(</mo><msubsup><mi>&#x3A6;</mi><mi>&#x3C3;</mi><mi>&#x3BA;</mi></msubsup><mo>)</mo></mrow><mo>&#x23DF;</mo></munder><msup><mi>&#x3C7;</mi><mi>&#x3BA;</mi></msup></munder><mo>&#x2192;</mo><mi>&#x3C7;</mi></math>"#;
    test("el", "SimpleSpeak", expr, "φ του; ανοίγει παρένθεση; κεφαλαίο φ δείκτης σίγμα, στην καπα οστή; κλείνει παρένθεση; ισούται με; παράσταση παράσταση φ του; ανοίγει παρένθεση; κεφαλαίο φ δείκτης σίγμα, στην καπα οστή; κλείνει παρένθεση; μείον; φ δείκτης καπα; του; ανοίγει παρένθεση; κεφαλαίο φ δείκτης σίγμα, στην καπα οστή; κλείνει παρένθεση με κάτω άγκιστρο από κάτω με ζεύγος βελών προς τα δεξιά 0 από κάτω; συν; παράσταση παράσταση φ δείκτης καπα; του; ανοίγει παρένθεση; κεφαλαίο φ δείκτης σίγμα, στην καπα οστή; κλείνει παρένθεση με κάτω άγκιστρο από κάτω με χ στην καπα οστή από κάτω; βέλος προς τα δεξιά χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_141() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>φ</mi><mo stretchy="false">(</mo><mi>χ</mi><mo stretchy="false">)</mo><mo>=</mo><mi>φ</mi><mo stretchy="false">(</mo><mi>α</mi><mo stretchy="false">)</mo><mo>+</mo><mi>φ</mi><mo>'</mo><mo stretchy="false">(</mo><mi>α</mi><mo stretchy="false">)</mo><mo>×</mo><mo stretchy="false">(</mo><mi>χ</mi><mo>−</mo><mi>α</mi><mo stretchy="false">)</mo><mo>+</mo><mo>…</mo><mo>+</mo><mfrac>
    <mrow>
     <msup>
      <mi>φ</mi>
      <mi>ν</mi>
     </msup>
     <mo stretchy="false">(</mo><mi>α</mi><mo stretchy="false">)</mo></mrow>
    <mi>ν</mi>
   </mfrac>
   <mo>×</mo><msup>
    <mrow>
     <mo stretchy="false">(</mo><mi>χ</mi><mo>−</mo><mi>α</mi><mo stretchy="false">)</mo></mrow>
    <mi>ν</mi>
   </msup>
   <mo>+</mo><mfrac>
    <mrow>
     <msup>
      <mi>φ</mi>
      <mrow>
       <mo stretchy="false">(</mo><mi>ν</mi><mo>+</mo><mn>1</mn><mo stretchy="false">)</mo></mrow>
     </msup>
     <mrow><mo>(</mo>
      <mi>γ</mi>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <mi>ν</mi><mo>+</mo><mn>1</mn></mrow>
   </mfrac>
   <mo>×</mo><msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mi>χ</mi><mo>−</mo><mi>α</mi></mrow>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <mo stretchy="false">(</mo><mi>ν</mi><mo>+</mo><mn>1</mn><mo stretchy="false">)</mo></mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "φ του χ, ισούται με; φ του άλφα, συν; φ τόνος, του άλφα; επί; ανοίγει παρένθεση; χ μείον άλφα; κλείνει παρένθεση; συν αποσιωπητικά, συν; κλάσμα, η νί δύναμη του, φ, του άλφα, προς νί, τέλος κλάσματος; επί; ανοίγει παρένθεση; χ μείον άλφα; κλείνει παρένθεση στην νί οστή; συν; κλάσμα, η ανοίγει παρένθεση, νί συν 1, κλείνει παρένθεση δύναμη του, φ; του γάμμα, προς, νί συν 1, τέλος κλάσματος; επί; ανοίγει παρένθεση; χ μείον άλφα; κλείνει παρένθεση που υψώνεται στη ανοίγει παρένθεση, νί συν 1, κλείνει παρένθεση τέλος δύναμης")?;
    return Ok(());
}

#[test]
fn uoa_corpus_142() -> Result<()> { //
       let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mfrac><mrow><msup><mo>&#x2202;</mo><mn>2</mn></msup><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C7;</mi><mo>&#x2202;</mo><mi>&#x3C8;</mi></mrow></mfrac><mo>=</mo><mrow><mo>(</mo><msup><mi>&#x3C7;</mi><mn>2</mn></msup><mo>&#x2212;</mo><msup><mi>&#x3C8;</mi><mn>2</mn></msup><mo>)</mo></mrow><mo>&#xD7;</mo><mrow><mo>{</mo><mrow><mi>&#x3C4;</mi><mo>&#xD7;</mo><mi>&#x3C6;</mi><mo>''</mo><mo>(</mo><mi>&#x3C4;</mi><mo>)</mo><mo>+</mo><mn>3</mn><mo>&#xD7;</mo><mi>&#x3C6;</mi><mo>'</mo><mo>(</mo><mi>&#x3C4;</mi><mo>)</mo></mrow><mo>}</mo></mrow></math>"#;

    test("el", "SimpleSpeak", expr, "κλάσμα, μερικό διαφορικό στο τετράγωνο; ύψιλον, προς, μερικό διαφορικό χ; μερικό διαφορικό ψ, τέλος κλάσματος; ισούται με; ανοίγει παρένθεση; χ στο τετράγωνο, μείον, ψ στο τετράγωνο; κλείνει παρένθεση; επί; ανοίγει άγκιστρο; τάαφ επί, φ διπλή παράγωγος; του τάαφ; συν, 3 επί, φ τόνος, του τάαφ; κλείνει άγκιστρο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_143() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>log</mi><mrow><mo>(</mo>
    <mrow>
     <mn>1</mn><mo>+</mo><mi>χ</mi></mrow>
   <mo>)</mo></mrow><mo>−</mo><mi>log</mi><mrow><mo>(</mo>
    <mrow>
     <mn>1</mn><mo>−</mo><mi>χ</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mi>log</mi><mfrac>
    <mrow>
     <mn>1</mn><mo>+</mo><mi>χ</mi></mrow>
    <mrow>
     <mn>1</mn><mo>−</mo><mi>χ</mi></mrow>
   </mfrac>
   <mo>=</mo><mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>ι</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>∞</mi>
    </munderover>
    <mrow>
     <mfrac>
      <mrow>
       <msup>
        <mi>χ</mi>
        <mrow>
         <mn>2</mn><mo>⋅</mo><mi>ι</mi><mo>−</mo><mn>1</mn></mrow>
       </msup>
       </mrow>
      <mrow>
       <mn>2</mn><mo>⋅</mo><mi>ι</mi><mo>−</mo><mn>1</mn></mrow>
     </mfrac>
     </mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ο λογάριθμος, του; ανοίγει παρένθεση, 1 συν χ, κλείνει παρένθεση; μείον; ο λογάριθμος, του; ανοίγει παρένθεση, 1 μείον χ, κλείνει παρένθεση; ισούται με; το λογάριθμο, του; κλάσμα, 1 συν χ, προς, 1 μείον χ, τέλος κλάσματος; ισούται με; άθροισμα από ιότα ισούται με 1 ως άπειρο του; κλάσμα, χ που υψώνεται στη 2 φορές ιότα, μείον 1 τέλος δύναμης; προς, 2 φορές ιότα, μείον 1, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_144() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msubsup>
    <mi>Δ</mi>
    <mi>χ</mi>
    <mi>ν</mi>
   </msubsup>
   <mi>ω</mi><mo>=</mo><mstyle displaystyle="true">
    <msub>
     <mo>∑</mo>
     <mrow>
      <mn>0</mn><mo>≤</mo><mi>λ</mi><mo>≤</mo><mi>ν</mi></mrow>
    </msub>
    <mrow>
     <mstyle displaystyle="true">
      <msub>
       <mo>∑</mo>
       <mtable columnalign="left">
        <mtr>
         <mtd>
          <mrow>
           <msub>
            <mi>κ</mi>
            <mn>1</mn>
           </msub>
           <mo>+</mo><mo>…</mo><mo>+</mo><msub>
            <mi>κ</mi>
            <mi>ν</mi>
           </msub>
           <mo>=</mo><mi>λ</mi></mrow>
         </mtd>
        </mtr>
        <mtr>
         <mtd>
          <mrow>
           <msub>
            <mi>κ</mi>
            <mn>1</mn>
           </msub>
           <mo>+</mo><mn>2</mn><msub>
            <mi>κ</mi>
            <mn>2</mn>
           </msub>
           <mo>+</mo><mo>…</mo><mo>+</mo><mi>ν</mi><msub>
            <mi>κ</mi>
            <mi>ν</mi>
           </msub>
           <mo>=</mo><mi>ν</mi></mrow>
         </mtd>
        </mtr>
        <mtr>
         <mtd>
          <mrow>
           <msub>
            <mi>κ</mi>
            <mn>1</mn>
           </msub>
           <mo>,</mo><mo>…</mo><mo>,</mo><msub>
            <mi>κ</mi>
            <mi>ν</mi>
           </msub>
           <mo>≥</mo><mn>0</mn></mrow>
         </mtd>
        </mtr>
       </mtable>
       
      </msub>
      <mrow>
       <msubsup>
        <mi>Δ</mi>
        <mi>υ</mi>
        <mi>λ</mi>
       </msubsup>
       <mi>ω</mi><mo>×</mo><mfrac>
        <mrow>
         <mi>n</mi><mo>!</mo><msup>
          <mrow>
           <mrow><mo>(</mo>
            <mrow>
             <msubsup>
              <mi>Δ</mi>
              <mi>χ</mi>
              <mn>1</mn>
             </msubsup>
             <mi>υ</mi></mrow>
           <mo>)</mo></mrow></mrow>
          <mrow>
           <msup>
            <mi>κ</mi>
            <mn>1</mn>
           </msup>
           </mrow>
         </msup>
         <mo>…</mo><msup>
          <mrow>
           <mrow><mo>(</mo>
            <mrow>
             <msubsup>
              <mi>Δ</mi>
              <mi>χ</mi>
              <mi>ν</mi>
             </msubsup>
             <mi>υ</mi></mrow>
           <mo>)</mo></mrow></mrow>
          <mrow>
           <msub>
            <mi>κ</mi>
            <mi>ν</mi>
           </msub>
           </mrow>
         </msup>
         </mrow>
        <mrow>
         <msub>
          <mi>κ</mi>
          <mn>1</mn>
         </msub>
         <mo>!</mo><msup>
          <mrow>
           <mo stretchy="false">(</mo><mn>1</mn><mo>!</mo><mo stretchy="false">)</mo></mrow>
          <mrow>
           <msub>
            <mi>κ</mi>
            <mn>1</mn>
           </msub>
           </mrow>
         </msup>
         <mo>…</mo><msub>
          <mi>κ</mi>
          <mi>ν</mi>
         </msub>
         <mo>!</mo><msup>
          <mrow>
           <mrow><mo>(</mo>
            <mrow>
             <mi>ν</mi><mo>!</mo></mrow>
           <mo>)</mo></mrow></mrow>
          <mrow>
           <msub>
            <mi>κ</mi>
            <mi>ν</mi>
           </msub>
           </mrow>
         </msup>
         </mrow>
       </mfrac>
       </mrow>
     </mstyle></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο δέλτα δείκτης χ, στην νί οστή; ωμέγα; ισούται με; άθροισμα για 0, είναι μικρότερο από ή ίσο με; λάμδα, είναι μικρότερο από ή ίσο με; νί του; άθροισμα για 3 εξισώσεις; εξίσωση 1; καπα δείκτης 1; συν αποσιωπητικά, συν, καπα δείκτης νί; ισούται με, λάμδα; εξίσωση 2; καπα δείκτης 1; συν, 2 καπα δείκτης 2; συν αποσιωπητικά, συν, νί, καπα δείκτης νί; ισούται με νί; εξίσωση 3; καπα δείκτης 1; κόμμα; αποσιωπητικά, κόμμα; καπα δείκτης νί; είναι μεγαλύτερο από ή ίσο με 0; του; κεφαλαίο δέλτα δείκτης ύψιλον, στην λάμδα οστή; ωμέγα επί; κλάσμα, n; παραγοντικό; ανοίγει παρένθεση; κεφαλαίο δέλτα δείκτης χ, στην πρώτη; ύψιλον; κλείνει παρένθεση που υψώνεται στη καπα στην πρώτη τέλος δύναμης; αποσιωπητικά; ανοίγει παρένθεση; κεφαλαίο δέλτα δείκτης χ, στην νί οστή; ύψιλον; κλείνει παρένθεση που υψώνεται στη καπα δείκτης νί, τέλος δύναμης; προς, καπα δείκτης 1; παραγοντικό; ανοίγει παρένθεση; 1 παραγοντικό, κλείνει παρένθεση που υψώνεται στη καπα δείκτης 1, τέλος δύναμης; αποσιωπητικά; καπα δείκτης νί; παραγοντικό; ανοίγει παρένθεση; νί παραγοντικό; κλείνει παρένθεση που υψώνεται στη καπα δείκτης νί, τέλος δύναμης; τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_145() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <msub>
     <mo>∑</mo>
     <mrow>
      <mn>1</mn><mo>≤</mo><mi>ι</mi><mo>≤</mo><mi>ν</mi></mrow>
    </msub>
    <mrow>
     <msub>
      <mi>α</mi>
      <mi>ι</mi>
     </msub>
     </mrow>
   </mstyle><mo>=</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άθροισμα για 1, είναι μικρότερο από ή ίσο με; ιότα, είναι μικρότερο από ή ίσο με; νί του; άλφα δείκτης ιότα; ισούται με 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_146() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>κ</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>∞</mi>
    </munderover>
    <mrow>
     <mfrac>
      <mrow>
       <msup>
        <mrow>
         <mrow><mo>(</mo>
          <mrow>
           <mo>−</mo><mn>1</mn></mrow>
         <mo>)</mo></mrow></mrow>
        <mrow>
         <mi>κ</mi><mo>+</mo><mn>1</mn></mrow>
       </msup>
       </mrow>
      <mrow>
       <mrow><mo>(</mo>
        <mrow>
         <mi>κ</mi><mo>+</mo><mn>1</mn></mrow>
       <mo>)</mo></mrow><mo>×</mo><mi>ln</mi><mo stretchy="false">(</mo><mi>κ</mi><mo>+</mo><mn>1</mn><mo stretchy="false">)</mo></mrow>
     </mfrac>
     </mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άθροισμα από καπα ισούται με 1 ως άπειρο του; κλάσματος, ανοίγει παρένθεση, μείον 1, κλείνει παρένθεση που υψώνεται στη καπα συν 1 τέλος δύναμης; προς, ανοίγει παρένθεση, καπα συν 1, κλείνει παρένθεση; επί; ο φυσικός λογάριθμος, του; ανοίγει παρένθεση, καπα συν 1, κλείνει παρένθεση, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_147() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>κ</mi><mo>=</mo><mn>11</mn></mrow>
     <mrow>
      <mn>28</mn></mrow>
    </munderover>
    <mrow>
     <mo stretchy="false">(</mo><mi>κ</mi><mo>−</mo><mn>10</mn><mo stretchy="false">)</mo><mo>×</mo><mi>sin</mi><mrow><mo>[</mo> <mrow>
      <mfrac>
       <mi>π</mi>
       <mrow>
        <mi>κ</mi><mo>−</mo><mn>10</mn></mrow>
      </mfrac>
      </mrow> <mo>]</mo></mrow></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άθροισμα από καπα ισούται με 11 ως 28 του; ανοίγει παρένθεση; καπα μείον 10; κλείνει παρένθεση; επί; ημίτονο του; ανοίγει αγκύλη; κλάσμα, π προς, καπα μείον 10, τέλος κλάσματος; κλείνει αγκύλη")?;
    return Ok(());
}

#[test]
fn uoa_corpus_148() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mi>Τ</mi>
    </munderover>
    <mrow>
     <msub>
      <mi>Φ</mi>
      <mn>2</mn>
     </msub>
     <mrow><mo>(</mo>
      <mrow>
       <mi>τ</mi><mo>,</mo><mi>χ</mi></mrow>
     <mo>)</mo></mrow><mi>d</mi><mi>τ</mi></mrow>
   </mrow>
   
  </mstyle><mo>≥</mo><mi>Δ</mi></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από 0 ως κεφαλαίο τάαφ του; κεφαλαίο φ δείκτης 2; ανοίγει παρένθεση; τάαφ κόμμα, χ; κλείνει παρένθεση; d τάαφ; είναι μεγαλύτερο από ή ίσο με; κεφαλαίο δέλτα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_149() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mi>Τ</mi>
    </munderover>
    <mrow>
     <mi>η</mi><mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow><mi>d</mi><mi>τ</mi></mrow>
   </mrow>
   
  </mstyle><mo>=</mo><mn>0</mn></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από 0 ως κεφαλαίο τάαφ του; ήτα του τάαφ, d τάαφ; ισούται με 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_150() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mrow>
     <mrow><mo>〈</mo> <mrow>
      <msubsup>
       <mi>υ</mi>
       <mrow>
        <mi>Δ</mi><mo>,</mo><mi>υ</mi></mrow>
       <mn>0</mn>
      </msubsup>
      </mrow> <mo>〉</mo></mrow></mrow>
    <mi>Φ</mi>
   </msub>
   <mo>=</mo><mo>−</mo><mstyle displaystyle="true">
    <munder>
     <mo>∑</mo>
     <mrow>
      <mi>Κ</mi><mo>∈</mo><mi>Μ</mi></mrow>
    </munder>
    <mrow>
     <msub>
      <mi>υ</mi>
      <mi>Κ</mi>
     </msub>
     <mstyle displaystyle="true">
      <mrow>
       <munder>
        <mo>∫</mo>
        <mi>Κ</mi>
       </munder>
       <mrow>
        <mi>Δ</mi><msup>
         <mi>υ</mi>
         <mn>0</mn>
        </msup>
        <mrow><mo>(</mo>
         <mi>χ</mi>
        <mo>)</mo></mrow><mi>d</mi><mi>χ</mi></mrow>
      </mrow>
      
     </mstyle></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "αριστερά γωνιακή αγκύλη; ύψιλον δείκτης; κεφαλαίο δέλτα, κόμμα, ύψιλον τέλος δείκτη, στην 0; δεξιά γωνιακή αγκύλη δείκτης, κεφαλαίο φ; ισούται με; μείον άθροισμα για κεφαλαίο καπα, ανήκει κεφαλαίο μί του; ύψιλον δείκτης, κεφαλαίο καπα; ολοκλήρωμα επί του συνόλου κεφαλαίο καπα του; κεφαλαίο δέλτα; η μηδενική δύναμη του, ύψιλον; του χ; d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_151() -> Result<()> { // same
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mfenced open="&lt;" close="&gt;"><mrow><msub><mi>&#x3C5;</mi><mi>&#x3C4;</mi></msub><mo>,</mo><mi>&#x3C5;</mi></mrow></mfenced><munder><mo>&#x222B;</mo><mi>&#x3A9;</mi></munder><mo>&#x2207;</mo><mi>&#x3C5;</mi><mrow><mo>(</mo><mi>&#x3C7;</mi><mo>,</mo><mi>&#x3C4;</mi><mo>)</mo></mrow><mo>&#xD7;</mo><mo>&#x2207;</mo><mi>&#x3C5;</mi><mrow><mo>(</mo><mi>&#x3C7;</mi><mo>)</mo></mrow><mi>d</mi><mi>&#x3C7;</mi><mo>&#xA0;</mo><mspace linebreak="newline"/><mo>=</mo><munder><mo>&#x222B;</mo><mi>&#x3A9;</mi></munder><mi>&#x3C6;</mi><mrow><mo>(</mo><mi>&#x3C7;</mi><mo>,</mo><mi>&#x3C4;</mi><mo>)</mo></mrow><mi>&#x3C5;</mi><mrow><mo>(</mo><mi>&#x3C7;</mi><mo>)</mo></mrow><mi>d</mi><mi>&#x3C7;</mi></math>"#;
    test("el", "SimpleSpeak", expr, "αριστερή γωνιακή αγκύλη; ύψιλον δείκτης τάαφ; κόμμα, ύψιλον; δεξιά γωνιακή αγκύλη; ολοκλήρωμα επί του συνόλου κεφαλαίο ωμέγα του; ανάδελτα του ύψιλον; ανοίγει παρένθεση; χ κόμμα, τάαφ; κλείνει παρένθεση; επί, ανάδελτα του ύψιλον του χ; d χ; ισούται με; ολοκλήρωμα επί του συνόλου κεφαλαίο ωμέγα του; φ; ανοίγει παρένθεση; χ κόμμα, τάαφ; κλείνει παρένθεση; ύψιλον του χ, d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_152() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>α</mi><mrow><mo>(</mo>
    <mrow>
     <mi>β</mi><mo>,</mo><mi>υ</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mstyle displaystyle="true">
    <mrow>
     <munder>
      <mo>∫</mo>
      <mi>Ω</mi>
     </munder>
     <mrow>
      <mo>∇</mo><mi>β</mi><mrow><mo>(</mo>
       <mi>χ</mi>
      <mo>)</mo></mrow><mo>⋅</mo><mo>∇</mo><mi>υ</mi><mrow><mo>(</mo>
       <mi>χ</mi>
      <mo>)</mo></mrow><mi>d</mi><mi>χ</mi></mrow>
    </mrow>
    
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άλφα; ανοίγει παρένθεση; βήτα κόμμα, ύψιλον; κλείνει παρένθεση; ισούται με; ολοκλήρωμα επί του συνόλου κεφαλαίο ωμέγα του; ανάδελτα του βήτα του χ; εσωτερικό γινόμενο; ανάδελτα του ύψιλον του χ; d χ")?;
    return Ok(());
    //theodora. fails. Can't access the definition rule for dot product? Now reads:άλφα; ανοίγει παρένθεση; βήτα κόμμα, ύψιλον; κλείνει παρένθεση; ισούται με; ολοκλήρωμα επί του συνόλου κεφαλαίο ωμέγα του; ανάδελτα του βήτα του χ; dot product; ανάδελτα του ύψιλον του χ; d χ
}

#[test]
fn uoa_corpus_153() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow>
     <munder>
      <mo>∫</mo>
      <mi>Ω</mi>
     </munder>
     <mrow>
      <mi>φ</mi><mrow><mo>(</mo>
       <mrow>
        <mi>χ</mi><mo>,</mo><mi>τ</mi></mrow>
      <mo>)</mo></mrow><mi>υ</mi><mrow><mo>(</mo>
       <mi>χ</mi>
      <mo>)</mo></mrow><mi>d</mi><mi>χ</mi></mrow>
    </mrow>
    
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα επί του συνόλου κεφαλαίο ωμέγα του; φ; ανοίγει παρένθεση; χ κόμμα, τάαφ; κλείνει παρένθεση; ύψιλον του χ, d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_154() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow>
     <munder>
      <mo>∫</mo>
      <mi>Κ</mi>
     </munder>
     <mrow>
      <mrow><mo>(</mo>
       <mrow>
        <mi>υ</mi><mrow><mo>(</mo>
         <mrow>
          <mi>χ</mi><mo>,</mo><msub>
           <mi>τ</mi>
           <mrow>
            <mi>ν</mi><mo>+</mo><mn>1</mn></mrow>
          </msub>
          </mrow>
        <mo>)</mo></mrow><mo>−</mo><mi>υ</mi><mrow><mo>(</mo>
         <mrow>
          <mi>χ</mi><mo>,</mo><msub>
           <mi>τ</mi>
           <mi>ν</mi>
          </msub>
          </mrow>
        <mo>)</mo></mrow></mrow>
      <mo>)</mo></mrow><mi>d</mi><mi>χ</mi></mrow>
    </mrow>
    
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα επί του συνόλου κεφαλαίο καπα του; ανοίγει παρένθεση; ύψιλον; ανοίγει παρένθεση; χ κόμμα; τάαφ δείκτης, νί συν 1 τέλος δείκτη; κλείνει παρένθεση; μείον; ύψιλον; ανοίγει παρένθεση; χ κόμμα; τάαφ δείκτης νί; κλείνει παρένθεση; κλείνει παρένθεση; d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_155() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mo>−</mo><mstyle displaystyle="true">
    <mrow>
     <munder>
      <mo>∫</mo>
      <mi>Κ</mi>
     </munder>
     <mrow>
      <mi>Δ</mi><msup>
       <mi>υ</mi>
       <mn>0</mn>
      </msup>
      <mrow><mo>(</mo>
       <mi>χ</mi>
      <mo>)</mo></mrow><mi>d</mi><mi>χ</mi></mrow>
    </mrow>
    
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "μείον ολοκλήρωμα επί του συνόλου κεφαλαίο καπα του; κεφαλαίο δέλτα; η μηδενική δύναμη του, ύψιλον; του χ; d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_156() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>Λ</mi>
    <mrow>
     <mi>Κ</mi><mo>,</mo><msup>
      <mi>σ</mi>
      <mo>″</mo>
     </msup>
     </mrow>
   </msub>
   <mo>=</mo><mstyle displaystyle="true">
    <mrow>
     <munder>
      <mo>∫</mo>
      <mrow>
       <mi>Κ</mi><mo>,</mo><msup>
        <mi>σ</mi>
        <mo>″</mo>
       </msup>
       </mrow>
     </munder>
     <mrow>
      <mi>Ι</mi><mi>d</mi><mi>χ</mi></mrow>
    </mrow>
    
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο λάμδα δείκτης; κεφαλαίο καπα, κόμμα; σίγμα διπλή παράγωγος, τέλος δείκτη; ισούται με; ολοκλήρωμα για κεφαλαίο καπα, κόμμα; σίγμα διπλή παράγωγος, του; κεφαλαίο ιότα, d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_157() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mrow>
      <msub>
       <mi>τ</mi>
       <mi>ν</mi>
      </msub>
      </mrow>
     <mrow>
      <msub>
       <mi>τ</mi>
       <mrow>
        <mi>ν</mi><mo>+</mo><mn>1</mn></mrow>
      </msub>
      </mrow>
    </munderover>
    <mrow>
     <mi>Δ</mi><mi>υ</mi><mrow><mo>(</mo>
      <mrow>
       <mi>χ</mi><mo>,</mo><mi>τ</mi></mrow>
     <mo>)</mo></mrow><mi>d</mi><mi>τ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από τάαφ δείκτης νί, ως τάαφ δείκτης, νί συν 1 τέλος δείκτη, του; κεφαλαίο δέλτα, ύψιλον; ανοίγει παρένθεση; χ κόμμα, τάαφ; κλείνει παρένθεση; d τάαφ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_158() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mi>Τ</mi>
    </munderover>
    <mrow>
     <msup>
      <mrow>
       <mrow><mo>|</mo> <mrow>
        <mi>υ</mi><mrow><mo>(</mo>
         <mi>τ</mi>
        <mo>)</mo></mrow></mrow> <mo>|</mo></mrow></mrow>
      <mn>2</mn>
     </msup>
     <mi>d</mi><mi>τ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από 0 ως κεφαλαίο τάαφ της απόλυτης τιμής του ύψιλον του τάαφ στο τετράγωνο; d τάαφ")?;
    return Ok(());
    //theodora. fails. Should be okay when we add genitive
}

#[test]
fn uoa_corpus_159() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mi>Τ</mi>
    </munderover>
    <mrow>
     <msup>
      <mrow>
       <mrow><mo>|</mo> <mrow>
        <mover accent="true">
         <mi>υ</mi>
         <mo>˙</mo>
        </mover>
        <mrow><mo>(</mo>
         <mi>τ</mi>
        <mo>)</mo></mrow></mrow> <mo>|</mo></mrow></mrow>
      <mn>2</mn>
     </msup>
     <mi>d</mi><mi>τ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από 0 ως κεφαλαίο τάαφ της απόλυτης τιμής του ύψιλον τελεία; του τάαφ τέλος απόλυτης τιμής στο τετράγωνο; d τάαφ")?;
    return Ok(());
    //theodora. fails. Should be okay when we add genitive
}

#[test]
fn uoa_corpus_160() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mi>Τ</mi>
    </munderover>
    <mrow>
     <msup>
      <mi>υ</mi>
      <mo>′</mo>
     </msup>
     <mi>φ</mi><mi>d</mi><mi>τ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από 0 ως κεφαλαίο τάαφ του; ύψιλον τόνος; φ d τάαφ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_161() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mi>Τ</mi>
    </munderover>
    <mrow>
     <msup>
      <mrow>
       <mrow><mo>|</mo> <mrow>
        <mover accent="true">
         <mi>υ</mi>
         <mo>˜</mo>
        </mover>
        <mrow><mo>(</mo>
         <mi>τ</mi>
        <mo>)</mo></mrow></mrow> <mo>|</mo></mrow></mrow>
      <mrow>
       <msup>
        <mi>π</mi>
        <mo>−</mo>
       </msup>
       </mrow>
     </msup>
     <mi>d</mi><mi>τ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από 0 ως κεφαλαίο τάαφ της απόλυτης τιμής του ύψιλον κυματοειδής γραμμή; του τάαφ τέλος απόλυτης τιμής που υψώνεται στη π εκθέτης μείον, τέλος δύναμης; d τάαφ")?;
    return Ok(());
        //theodora. fails. Should be okay when we add genitive

}

#[test]
fn uoa_corpus_162() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mi>Τ</mi>
    </munderover>
    <mrow>
     <mi>Φ</mi><mrow><mo>(</mo>
      <mrow>
       <mi>τ</mi><mo>,</mo><mi>χ</mi></mrow>
     <mo>)</mo></mrow><mi>d</mi><mi>τ</mi></mrow>
   </mrow>
   
  </mstyle><mo>→</mo><mi>∞</mi></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από 0 ως κεφαλαίο τάαφ του; κεφαλαίο φ; ανοίγει παρένθεση; τάαφ κόμμα, χ; κλείνει παρένθεση; d τάαφ; βέλος προς τα δεξιά, άπειρο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_163() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mi>Τ</mi>
    </munderover>
    <mrow>
     <mstyle displaystyle="true">
      <mrow><munderover>
       <mo>∫</mo>
       <mn>0</mn>
       <mn>1</mn>
      </munderover>
      <mrow>
       <mi>γ</mi><mrow><mo>(</mo>
        <mi>τ</mi>
       <mo>)</mo></mrow><mrow><mo>|</mo> <mrow>
        <mi>υ</mi><mrow><mo>(</mo>
         <mi>τ</mi>
        <mo>)</mo></mrow></mrow> <mo>|</mo></mrow><mi>d</mi><mi>σ</mi></mrow>
     </mrow>
     
    </mstyle><mi>d</mi><mi>τ</mi></mrow>
  </mrow>
  
 </mstyle></mrow>
</math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από 0 ως κεφαλαίο τάαφ του; ολοκλήρωμα από 0 ως 1 του; γάμμα του τάαφ; η απόλυτη τιμή του ύψιλον του τάαφ; d σίγμα; d τάαφ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_164() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mi>μ</mi>
   </mfrac>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mi>Τ</mi>
    </munderover>
    <mrow>
     <msub>
      <mi>Φ</mi>
      <mn>1</mn>
     </msub>
     <mrow><mo>(</mo>
      <mrow>
       <mi>τ</mi><mo>,</mo><mi>λ</mi><mover accent="true">
        <mi>υ</mi>
        <mo>¯</mo>
       </mover>
       </mrow>
     <mo>)</mo></mrow><mi>d</mi><mi>τ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "1 προς μί; ολοκλήρωμα από 0 ως κεφαλαίο τάαφ του; κεφαλαίο φ δείκτης 1; ανοίγει παρένθεση; τάαφ κόμμα; λάμδα ύψιλον παύλα; κλείνει παρένθεση; d τάαφ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_165() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow>
     <munder>
      <mo>∫</mo>
      <mi>Ω</mi>
     </munder>
     <mrow>
      <msup>
       <mi>ρ</mi>
       <mrow>
        <mo>−</mo><mn>2</mn></mrow>
      </msup>
      <mi>d</mi><mi>χ</mi></mrow>
    </mrow>
    
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα επί του συνόλου κεφαλαίο ωμέγα του; ρ στην μείον 2, d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_166() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mn>6</mn>
   </mfrac>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mi>σ</mi>
     <mn>1</mn>
    </munderover>
    <mrow>
     <mi>σ</mi><mrow><mo>(</mo>
      <mrow>
       <mn>3</mn><mo>−</mo><msup>
        <mi>σ</mi>
        <mn>2</mn>
       </msup>
       </mrow>
     <mo>)</mo></mrow><mi>η</mi><mrow><mo>(</mo>
      <mi>σ</mi>
     <mo>)</mo></mrow><mi>α</mi><mrow><mo>(</mo>
      <mi>σ</mi>
     <mo>)</mo></mrow><mi>d</mi><mi>σ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "1 έκτο; ολοκλήρωμα από σίγμα ως 1 του; σίγμα; ανοίγει παρένθεση; 3 μείον, σίγμα στο τετράγωνο; κλείνει παρένθεση; ήτα του σίγμα; άλφα του σίγμα, d σίγμα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_167() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>λ</mi><munder>
    <mrow>
     <mi>max</mi></mrow>
    <mrow>
     <mn>0</mn><mo>≤</mo><mi>τ</mi><mo>≤</mo><mn>1</mn></mrow>
   </munder>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mn>1</mn>
    </munderover>
    <mrow>
     <mi>Γ</mi><mrow><mo>(</mo>
      <mrow>
       <mi>τ</mi><mo>,</mo><mi>σ</mi></mrow>
     <mo>)</mo></mrow><mi>η</mi><mrow><mo>(</mo>
      <mi>σ</mi>
     <mo>)</mo></mrow><mi>ψ</mi><mrow><mo>(</mo>
      <mrow>
       <mi>σ</mi><mo>,</mo><mi>β</mi></mrow>
     <mo>)</mo></mrow><mi>d</mi><mi>σ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "λάμδα; μέγιστο για 0, είναι μικρότερο από ή ίσο με; τάαφ, είναι μικρότερο από ή ίσο με 1; του; ολοκλήρωμα από 0 ως 1 του; κεφαλαίο γάμμα; ανοίγει παρένθεση; τάαφ κόμμα, σίγμα; κλείνει παρένθεση; ήτα του σίγμα, ψ; ανοίγει παρένθεση; σίγμα κόμμα, βήτα; κλείνει παρένθεση; d σίγμα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_168() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mn>2</mn>
   </mfrac>
   <mi>λ</mi><munder>
    <mrow>
     <mi>sup</mi></mrow>
    <mrow>
     <mi>υ</mi><mo>∈</mo><mi>Υ</mi><mrow><mo>(</mo>
      <mi>ρ</mi>
     <mo>)</mo></mrow></mrow>
   </munder>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mn>1</mn>
    </munderover>
    <mrow>
     <mrow><mo>|</mo> <mrow>
      <mi>υ</mi><mrow><mo>(</mo>
       <mi>σ</mi>
      <mo>)</mo></mrow></mrow> <mo>|</mo></mrow><mi>d</mi><mi>σ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "1 δεύτερο λάμδα; σουπρέμουμ για ύψιλον ανήκει, κεφαλαίο ύψιλον, του ρ; του; ολοκλήρωμα από 0 ως 1 της απόλυτης τιμής του ύψιλον του σίγμα; d σίγμα")?;
    return Ok(());
    //theodora. fails. Should be okay when we add genitive
}

#[test]
fn uoa_corpus_169() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mn>1</mn>
    </munderover>
    <mrow>
     <mfrac>
      <mo>∂</mo>
      <mrow>
       <mo>∂</mo><mi>τ</mi></mrow>
     </mfrac>
     <mi>Ξ</mi><mrow><mo>(</mo>
      <mrow>
       <mi>τ</mi><mo>,</mo><mi>σ</mi></mrow>
     <mo>)</mo></mrow><mi>υ</mi><mrow><mo>(</mo>
      <mi>σ</mi>
     <mo>)</mo></mrow><mi>d</mi><mi>σ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από 0 ως 1 του; κλάσματος, μερικό διαφορικό προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; κεφαλαίο ξ; ανοίγει παρένθεση; τάαφ κόμμα, σίγμα; κλείνει παρένθεση; ύψιλον του σίγμα; d σίγμα")?;
    return Ok(());
        //theodora. fails with current rules. Will probably be ok when we introduce genitive. Now reads: ολοκλήρωμα από 0 ως 1 του; κλάσμα, μερικό διαφορικό προς, μερικό διαφορικό, τάαφ, τέλος κλάσματος; κεφαλαίο ξ; ανοίγει παρένθεση; τάαφ κόμμα, σίγμα; κλείνει παρένθεση; ύψιλον του σίγμα; d σίγμα

}

#[test]
fn uoa_corpus_170() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mi>τ</mi>
     <mn>1</mn>
    </munderover>
    <mrow>
     <mrow><mo>‖</mo> <msup>
      <mi>υ</mi>
      <mo>″</mo>
     </msup>
      <mo>‖</mo></mrow><mi>σ</mi><mi>d</mi><mi>σ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από τάαφ ως 1 του; νόρμα του ύψιλον διπλή παράγωγος; σίγμα d σίγμα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_171() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow>
     <munder>
      <mo>∫</mo>
      <mi>Η</mi>
     </munder>
     <mi>η</mi>
    </mrow>
    
   </mstyle><mrow><mo>(</mo>
    <mrow>
     <mi>χ</mi><mo>+</mo><mn>2</mn></mrow>
   <mo>)</mo></mrow><mi>d</mi><mi>χ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα επί του συνόλου κεφαλαίο ήτα του; ήτα; ανοίγει παρένθεση, χ συν 2, κλείνει παρένθεση; d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_172() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow>
     <msubsup>
      <mo>∫</mo>
      <mn>1</mn>
      <mi>∞</mi>
     </msubsup>
     <mrow>
      <msup>
       <mi>ε</mi>
       <mrow>
        <msup>
         <mi>χ</mi>
         <mn>2</mn>
        </msup>
        <mo>−</mo><mi>χ</mi><mo>−</mo><mn>1</mn></mrow>
      </msup>
      <mi>d</mi><mi>χ</mi></mrow>
    </mrow>
    
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από 1 ως άπειρο του; έψιλον που υψώνεται στη χ στο τετράγωνο, μείον χ μείον 1 τέλος δύναμης; d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_173() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow><mo>∫</mo>
     <mrow>
      <mfrac>
       <mrow>
        <mi>d</mi><mi>χ</mi></mrow>
       <mi>χ</mi>
      </mfrac>
      </mrow>
    </mrow>
    
   </mstyle><mo>=</mo><mi>log</mi><mi>χ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα του κλάσματος, d χ, προς χ, τέλος κλάσματος; ισούται με, το λογάριθμο, του χ")?;
    return Ok(());
//theodora. fails with current rules. needs genitive clause
  }

#[test]
fn uoa_corpus_174() -> Result<()> { // same
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><munderover><mo>&#x222B;</mo><msub><mi>&#x3C4;</mi><mi>&#x3BD;</mi></msub><msub><mi>&#x3C4;</mi><mrow><mi>&#x3BD;</mi><mo>+</mo><mn>1</mn></mrow></msub></munderover><munder><mo>&#x222B;</mo><mi>&#x39A;</mi></munder><msub><mi>&#x3C5;</mi><mi>&#x3C4;</mi></msub><mrow><mo>(</mo><mi>&#x3C7;</mi><mo>,</mo><mi>&#x3C4;</mi><mo>)</mo></mrow><mi>d</mi><mi>&#x3C7;</mi><mi>d</mi><mi>&#x3C4;</mi><mo>&#x2212;</mo><mspace linebreak="newline"/><mspace width="0em"/><munder><mo>&#x2211;</mo><mrow><mi>&#x3C3;</mi><mo>&#x2208;</mo><msub><mi>&#x395;</mi><mi>&#x39A;</mi></msub></mrow></munder><munderover><mo>&#x222B;</mo><msub><mi>&#x3C4;</mi><mi>&#x3BD;</mi></msub><msub><mi>&#x3C4;</mi><mrow><mi>&#x3BD;</mi><mo>+</mo><mn>1</mn></mrow></msub></munderover><munder><mo>&#x222B;</mo><mi>&#x3C3;</mi></munder><mo>&#x2207;</mo><mi>&#x3C5;</mi><mrow><mo>(</mo><mi>&#x3C7;</mi><mo>,</mo><mi>&#x3C4;</mi><mo>)</mo></mrow><mo>&#xD7;</mo><msub><mi>&#x3BD;</mi><mrow><mi>&#x39A;</mi><mo>,</mo><mi>&#x3C3;</mi></mrow></msub><mrow><mo>(</mo><mi>&#x3C7;</mi><mo>)</mo></mrow><msub><mi>&#x3B4;</mi><mi>&#x3B3;</mi></msub><mrow><mo>(</mo><mi>&#x3C7;</mi><mo>)</mo></mrow><mi>d</mi><mi>&#x3C4;</mi><mo>=</mo><mspace linebreak="newline"/><mspace width="0em"/><munderover><mo>&#x222B;</mo><msub><mi>&#x3C4;</mi><mi>&#x3BD;</mi></msub><msub><mi>&#x3C4;</mi><mrow><mi>&#x3BD;</mi><mo>+</mo><mn>1</mn></mrow></msub></munderover><munder><mo>&#x222B;</mo><mi>&#x39A;</mi></munder><mi>&#x3C6;</mi><mrow><mo>(</mo><mi>&#x3C7;</mi><mo>,</mo><mi>&#x3C4;</mi><mo>)</mo></mrow><mi>d</mi><mi>&#x3C7;</mi><mi>d</mi><mi>&#x3C4;</mi></math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από τάαφ δείκτης νί, ως τάαφ δείκτης, νί συν 1 τέλος δείκτη, του; ολοκλήρωμα επί του συνόλου κεφαλαίο καπα του; ύψιλον δείκτης τάαφ; ανοίγει παρένθεση; χ κόμμα, τάαφ; κλείνει παρένθεση; d χ d τάαφ; μείον; άθροισμα για σίγμα ανήκει; κεφαλαίο έψιλον δείκτης, κεφαλαίο καπα, του; ολοκλήρωμα από τάαφ δείκτης νί, ως τάαφ δείκτης, νί συν 1 τέλος δείκτη, του; ολοκλήρωμα επί του συνόλου σίγμα του; ανάδελτα του ύψιλον; ανοίγει παρένθεση; χ κόμμα, τάαφ; κλείνει παρένθεση; επί; νί δείκτης; κεφαλαίο καπα, κόμμα, σίγμα τέλος δείκτη; του χ; δέλτα δείκτης γάμμα; του χ; d τάαφ; ισούται με; ολοκλήρωμα από τάαφ δείκτης νί, ως τάαφ δείκτης, νί συν 1 τέλος δείκτη, του; ολοκλήρωμα επί του συνόλου κεφαλαίο καπα του; φ; ανοίγει παρένθεση; χ κόμμα, τάαφ; κλείνει παρένθεση; d χ d τάαφ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_175() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mo>∂</mo>
    <mn>2</mn>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <msup>
      <mi>Δ</mi>
      <mi>α</mi>
     </msup>
     <mi>υ</mi><mrow><mo>(</mo>
      <mrow>
       <mi>χ</mi><mo>,</mo><msub>
        <mi>τ</mi>
        <mi>ν</mi>
       </msub>
       </mrow>
     <mo>)</mo></mrow></mrow>
   <mo>)</mo></mrow><mo>=</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mi>κ</mi>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mrow>
      <msub>
       <mi>τ</mi>
       <mrow>
        <mi>ν</mi><mo>−</mo><mn>1</mn></mrow>
      </msub>
      </mrow>
     <mrow>
      <msub>
       <mi>τ</mi>
       <mi>ν</mi>
      </msub>
      </mrow>
    </munderover>
    <mrow>
     <mstyle displaystyle="true">
      <mrow><munderover>
       <mo>∫</mo>
       <mrow>
        <mi>τ</mi><mo>−</mo><mi>η</mi></mrow>
       <mi>τ</mi>
      </munderover>
      <mrow>
       <msub>
        <mrow>
         <mrow><mo>(</mo>
          <mrow>
           <msup>
            <mi>Δ</mi>
            <mi>α</mi>
           </msup>
           <mi>υ</mi></mrow>
         <mo>)</mo></mrow></mrow>
        <mrow>
         <mi>τ</mi><mi>τ</mi></mrow>
       </msub>
       <mrow><mo>(</mo>
        <mrow>
         <mi>χ</mi><mo>,</mo><mi>τ</mi></mrow>
       <mo>)</mo></mrow><mi>d</mi><mi>σ</mi></mrow>
     </mrow>
     
    </mstyle><mi>d</mi><mi>τ</mi></mrow>
  </mrow>
  
 </mstyle></mrow>
</math>"#;
    test("el", "SimpleSpeak", expr, "μερικό διαφορικό στο τετράγωνο; ανοίγει παρένθεση; κεφαλαίο δέλτα στην άλφα οστή; ύψιλον; ανοίγει παρένθεση; χ κόμμα; τάαφ δείκτης νί; κλείνει παρένθεση; κλείνει παρένθεση; ισούται με; κλάσμα, 1 προς, καπα στο τετράγωνο, τέλος κλάσματος; ολοκλήρωμα από τάαφ δείκτης, νί μείον 1 τέλος δείκτη, ως τάαφ δείκτης νί, του; ολοκλήρωμα από τάαφ μείον ήτα ως τάαφ του; ανοίγει παρένθεση; κεφαλαίο δέλτα στην άλφα οστή; ύψιλον; κλείνει παρένθεση δείκτης, τάαφ τάαφ τέλος δείκτη; ανοίγει παρένθεση; χ κόμμα, τάαφ; κλείνει παρένθεση; d σίγμα; d τάαφ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_176() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mi>κ</mi>
   </mfrac>
   <mstyle displaystyle="true">
    <mrow>
     <munder>
      <mo>∫</mo>
      <mi>Κ</mi>
     </munder>
     <mrow>
      <msup>
       <mo>∂</mo>
       <mn>1</mn>
      </msup>
      <mrow><mo>(</mo>
       <mrow>
        <mstyle displaystyle="true">
         <mrow><munderover>
          <mo>∫</mo>
          <mrow>
           <msub>
            <mi>τ</mi>
            <mi>ν</mi>
           </msub>
           </mrow>
          <mrow>
           <msub>
            <mi>τ</mi>
            <mrow>
             <mi>ν</mi><mo>+</mo><mn>1</mn></mrow>
           </msub>
           </mrow>
         </munderover>
         <mrow>
          <mi>Δ</mi><mi>υ</mi><mrow><mo>(</mo>
           <mrow>
            <mi>χ</mi><mo>,</mo><mi>τ</mi></mrow>
          <mo>)</mo></mrow><mi>d</mi><mi>τ</mi></mrow>
        </mrow>
        
       </mstyle></mrow>
     <mo>)</mo></mrow><mi>d</mi><mi>χ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "1 προς καπα; ολοκλήρωμα επί του συνόλου κεφαλαίο καπα του; μερικό διαφορικό στην πρώτη; ανοίγει παρένθεση; ολοκλήρωμα από τάαφ δείκτης νί, ως τάαφ δείκτης, νί συν 1 τέλος δείκτη, του; κεφαλαίο δέλτα, ύψιλον; ανοίγει παρένθεση; χ κόμμα, τάαφ; κλείνει παρένθεση; d τάαφ; κλείνει παρένθεση; d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_177() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mn>1</mn>
    <mrow>
     <msup>
      <mi>κ</mi>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   <mstyle displaystyle="true">
    <mrow>
     <munder>
      <mo>∫</mo>
      <mi>Κ</mi>
     </munder>
     <mrow>
      <mstyle displaystyle="true">
       <mrow><munderover>
        <mo>∫</mo>
        <mrow>
         <msub>
          <mi>τ</mi>
          <mrow>
           <mi>ν</mi><mo>−</mo><mn>1</mn></mrow>
         </msub>
         </mrow>
        <mrow>
         <msub>
          <mi>τ</mi>
          <mi>ν</mi>
         </msub>
         </mrow>
       </munderover>
       <mrow>
        <mstyle displaystyle="true">
         <mrow><munderover>
          <mo>∫</mo>
          <mrow>
           <msub>
            <mi>τ</mi>
            <mrow>
             <mi>ν</mi><mo>−</mo><mn>1</mn></mrow>
           </msub>
           </mrow>
          <mi>τ</mi>
         </munderover>
         <mrow>
          <mstyle displaystyle="true">
           <mrow><munderover>
            <mo>∫</mo>
            <mi>σ</mi>
            <mrow>
             <mi>σ</mi><mo>+</mo><mi>κ</mi></mrow>
           </munderover>
           <mrow>
            <mi>Δ</mi><mfrac>
             <mrow>
              <msup>
               <mi>d</mi>
               <mn>2</mn>
              </msup>
              <mi>υ</mi></mrow>
             <mrow>
              <mi>d</mi><msup>
               <mi>λ</mi>
               <mn>2</mn>
              </msup>
              </mrow>
            </mfrac>
            <mrow><mo>(</mo>
             <mrow>
              <mi>χ</mi><mo>,</mo><mi>λ</mi></mrow>
            <mo>)</mo></mrow><mi>d</mi><mi>λ</mi></mrow>
          </mrow>
          
         </mstyle><mi>d</mi><mi>σ</mi></mrow>
       </mrow>
       
      </mstyle><mi>d</mi><mi>τ</mi></mrow>
    </mrow>
    
   </mstyle><mi>d</mi><mi>χ</mi></mrow>
 </mrow>
 
</mstyle></mrow>
</math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, 1 προς, καπα στο τετράγωνο, τέλος κλάσματος; ολοκλήρωμα επί του συνόλου κεφαλαίο καπα του; ολοκλήρωμα από τάαφ δείκτης, νί μείον 1 τέλος δείκτη, ως τάαφ δείκτης νί, του; ολοκλήρωμα από τάαφ δείκτης, νί μείον 1 τέλος δείκτη, ως τάαφ του; ολοκλήρωμα από σίγμα ως σίγμα συν καπα του; κεφαλαίο δέλτα; κλάσμα, d στο τετράγωνο, ύψιλον, προς, d, λάμδα στο τετράγωνο, τέλος κλάσματος; ανοίγει παρένθεση; χ κόμμα, λάμδα; κλείνει παρένθεση; d λάμδα; d σίγμα; d τάαφ; d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_178() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>‖</mo> <mi>υ</mi> <mo>‖</mo></mrow><mo>≤</mo><msub>
    <mi>Γ</mi>
    <mn>1</mn>
   </msub>
   <mrow><mo>(</mo>
    <mrow>
     <msup>
      <mrow>
       <mrow><mo>(</mo>
        <mrow>
         <mstyle displaystyle="true">
          <mrow><munderover>
           <mo>∫</mo>
           <mn>0</mn>
           <mi>Τ</mi>
          </munderover>
          <mrow>
           <msup>
            <mrow>
             <mrow><mo>|</mo> <mover accent="true">
              <mi>υ</mi>
              <mo>˙</mo>
             </mover>
              <mo>|</mo></mrow></mrow>
            <mrow>
             <mi>π</mi><mrow><mo>(</mo>
              <mi>τ</mi>
             <mo>)</mo></mrow></mrow>
           </msup>
           <mi>d</mi><mi>τ</mi></mrow>
         </mrow>
         
        </mstyle></mrow>
      <mo>)</mo></mrow></mrow>
     <mrow>
      <mfrac>
       <mn>1</mn>
       <mrow>
        <msup>
         <mi>π</mi>
         <mo>−</mo>
        </msup>
        </mrow>
      </mfrac>
      </mrow>
    </msup>
    <mo>+</mo><mn>1</mn><mo>+</mo><mrow><mo>|</mo> <mover accent="true">
     <mi>υ</mi>
     <mo>¯</mo>
    </mover>
     <mo>|</mo></mrow></mrow>
  <mo>)</mo></mrow></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "νόρμα του ύψιλον; είναι μικρότερο από ή ίσο με; κεφαλαίο γάμμα δείκτης 1; ανοίγει παρένθεση; ανοίγει παρένθεση; ολοκλήρωμα από 0 ως κεφαλαίο τάαφ της απόλυτης τιμής του ύψιλον τελεία, τέλος απόλυτης τιμής που υψώνεται στη π του τάαφ τέλος δύναμης; d τάαφ; κλείνει παρένθεση που υψώνεται στη κλάσμα, 1 προς, π εκθέτης μείον; τέλος κλάσματος; τέλος δύναμης; συν 1 συν; η απόλυτη τιμή του ύψιλον παύλα, τέλος απόλυτης τιμής; κλείνει παρένθεση")?;
    return Ok(());
    //theodora. fails. Should be okay when we add genitive
}

#[test]
fn uoa_corpus_179() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>〈</mo> <mrow>
    <msup>
     <mi>Ξ</mi>
     <mo>′</mo>
    </msup>
    <mrow><mo>(</mo>
     <mi>υ</mi>
    <mo>)</mo></mrow><mo>,</mo><mi>ν</mi></mrow> <mo>〉</mo></mrow><mo>=</mo><mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mi>Τ</mi>
    </munderover>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <msup>
        <mrow>
         <mrow><mo>|</mo> <mrow>
          <mover accent="true">
           <mi>υ</mi>
           <mo>˙</mo>
          </mover>
          <mrow><mo>(</mo>
           <mi>τ</mi>
          <mo>)</mo></mrow></mrow> <mo>|</mo></mrow></mrow>
        <mrow>
         <mi>π</mi><mrow><mo>(</mo>
          <mi>τ</mi>
         <mo>)</mo></mrow><mo>−</mo><mn>2</mn></mrow>
       </msup>
       <mover accent="true">
        <mi>υ</mi>
        <mo>˙</mo>
       </mover>
       <mrow><mo>(</mo>
        <mi>τ</mi>
       <mo>)</mo></mrow><mo>,</mo><mover accent="true">
        <mi>υ</mi>
        <mo>˙</mo>
       </mover>
       <mrow><mo>(</mo>
        <mi>τ</mi>
       <mo>)</mo></mrow></mrow>
     <mo>)</mo></mrow><mi>d</mi><mi>τ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "αριστερά γωνιακή αγκύλη; κεφαλαίο ξ τόνος; του ύψιλον; κόμμα, νί; δεξιά γωνιακή αγκύλη; ισούται με; ολοκλήρωμα από 0 ως κεφαλαίο τάαφ του; ανοίγει παρένθεση; η απόλυτη τιμή του ύψιλον τελεία; του τάαφ τέλος απόλυτης τιμής που υψώνεται στη π του τάαφ, μείον 2 τέλος δύναμης; ύψιλον τελεία; του τάαφ; κόμμα; ύψιλον τελεία; του τάαφ; κλείνει παρένθεση; d τάαφ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_180() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>Γ</mi>
    <mn>5</mn>
   </msub>
   <msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mstyle displaystyle="true">
        <mrow><munderover>
         <mo>∫</mo>
         <mn>0</mn>
         <mi>Τ</mi>
        </munderover>
        <mrow>
         <msup>
          <mrow>
           <mrow><mo>|</mo> <mrow>
            <mover accent="true">
             <mi>υ</mi>
             <mo>˙</mo>
            </mover>
            <mrow><mo>(</mo>
             <mi>τ</mi>
            <mo>)</mo></mrow></mrow> <mo>|</mo></mrow></mrow>
          <mrow>
           <mi>π</mi><mrow><mo>(</mo>
            <mi>τ</mi>
           <mo>)</mo></mrow></mrow>
         </msup>
         <mi>d</mi><mi>τ</mi></mrow>
       </mrow>
       
      </mstyle></mrow>
    <mo>)</mo></mrow></mrow>
   <mrow>
    <mfrac>
     <mrow>
      <mi>α</mi><mo>+</mo><mn>1</mn></mrow>
     <mrow>
      <msup>
       <mi>π</mi>
       <mo>−</mo>
      </msup>
      </mrow>
    </mfrac>
    </mrow>
  </msup>
  </mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο γάμμα δείκτης 5; ανοίγει παρένθεση; ολοκλήρωμα από 0 ως κεφαλαίο τάαφ της απόλυτης τιμής του ύψιλον τελεία; του τάαφ τέλος απόλυτης τιμής που υψώνεται στη π του τάαφ τέλος δύναμης; d τάαφ; κλείνει παρένθεση που υψώνεται στη κλάσμα, άλφα συν 1, προς, π εκθέτης μείον; τέλος κλάσματος; τέλος δύναμης")?;
    return Ok(());
    //theodora. fails. Should be okay when we add genitive
}

#[test]
fn uoa_corpus_181() -> Result<()> { //
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mrow><mo>&#x27E8;</mo><msup><mi>&#x3C6;</mi><mo>'</mo></msup><mrow><mo>(</mo><msub><mi>&#x3C5;</mi><mi>&#x3BD;</mi></msub><mo>)</mo></mrow><mo>,</mo><msub><mi>&#x3C5;</mi><mi>&#x3BD;</mi></msub><mo>&#x2212;</mo><mi>&#x3C5;</mi><mo>&#x27E9;</mo></mrow><mo>=</mo><munderover><mo>&#x222B;</mo><mn>0</mn><mi>&#x3A4;</mi></munderover><mrow><mo>[</mo><mrow><mo>(</mo><msup><mrow><mo>|</mo><msub><mover accent="true"><mi>&#x3C5;</mi><mo>&#x2D9;</mo></mover><mi>&#x3BD;</mi></msub><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>)</mo></mrow><mo>|</mo></mrow><mrow><mi>&#x3C0;</mi><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>)</mo></mrow><mo>&#x2212;</mo><mn>2</mn></mrow></msup><msub><mover accent="true"><mi>&#x3C5;</mi><mo>&#x2D9;</mo></mover><mi>&#x3BD;</mi></msub><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>)</mo></mrow><mo>,</mo><msub><mover accent="true"><mi>&#x3C5;</mi><mo>&#x2D9;</mo></mover><mi>&#x3BD;</mi></msub><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>)</mo></mrow><mo>&#x2212;</mo><mover accent="true"><mi>&#x3C5;</mi><mo>&#x2D9;</mo></mover><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>)</mo></mrow><mo>)</mo></mrow><mspace linebreak="newline"/><mrow><mo>+</mo><mo>(</mo><mo>&#x2207;</mo><mi>&#x3A6;</mi><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>,</mo><msub><mi>&#x3C5;</mi><mi>&#x3BD;</mi></msub><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>)</mo></mrow><mo>)</mo></mrow><mo>,</mo><msub><mi>&#x3C5;</mi><mi>&#x3BD;</mi></msub><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>)</mo></mrow><mo>&#x2212;</mo><mi>&#x3C5;</mi><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>)</mo></mrow><mo>)</mo></mrow><mo>]</mo></mrow><mi>d</mi><mi>&#x3C4;</mi></math>"#;
    test("el", "SimpleSpeak", expr, "αριστερή γωνιακή αγκύλη; φ τόνος, του; ανοίγει παρένθεση; ύψιλον δείκτης νί; κλείνει παρένθεση; κόμμα; ύψιλον δείκτης νί; μείον ύψιλον; δεξιά γωνιακή αγκύλη; ισούται με; ολοκλήρωμα από 0 ως κεφαλαίο τάαφ του; ανοίγει αγκύλη; ανοίγει παρένθεση; η απόλυτη τιμή του ύψιλον τελεία, δείκτης νί; του τάαφ τέλος απόλυτης τιμής που υψώνεται στη π του τάαφ, μείον 2 τέλος δύναμης; ύψιλον τελεία, δείκτης νί; του τάαφ; κόμμα; ύψιλον τελεία, δείκτης νί; του τάαφ; μείον, ύψιλον τελεία; του τάαφ; κλείνει παρένθεση; συν ανοίγει παρένθεση; ανάδελτα του κεφαλαίο φ; ανοίγει παρένθεση; τάαφ κόμμα; ύψιλον δείκτης νί; του τάαφ; κλείνει παρένθεση; κόμμα; ύψιλον δείκτης νί; του τάαφ; μείον, ύψιλον του τάαφ; κλείνει παρένθεση; κλείνει αγκύλη; d τάαφ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_182() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mi>Τ</mi>
    </munderover>
    <mrow>
     <mstyle displaystyle="true">
      <mrow><munderover>
       <mo>∫</mo>
       <mn>0</mn>
       <mn>1</mn>
      </munderover>
      <mrow>
       <mrow><mo>(</mo>
        <mrow>
         <mo>∇</mo><mi>Γ</mi><mrow><mo>(</mo>
          <mrow>
           <mi>σ</mi><mi>υ</mi><mrow><mo>(</mo>
            <mi>τ</mi>
           <mo>)</mo></mrow></mrow>
         <mo>)</mo></mrow><mo>−</mo><mo>∇</mo><mi>Γ</mi><mrow><mo>(</mo>
          <mn>0</mn>
         <mo>)</mo></mrow><mo>,</mo><mi>υ</mi><mrow><mo>(</mo>
          <mi>τ</mi>
         <mo>)</mo></mrow></mrow>
       <mo>)</mo></mrow><mi>d</mi><mi>σ</mi></mrow>
     </mrow>
     
    </mstyle><mi>d</mi><mi>τ</mi></mrow>
  </mrow>
  
 </mstyle></mrow>
</math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από 0 ως κεφαλαίο τάαφ του; ολοκλήρωμα από 0 ως 1 του; ανοίγει παρένθεση; ανάδελτα του κεφαλαίο γάμμα; ανοίγει παρένθεση; σίγμα, ύψιλον του τάαφ; κλείνει παρένθεση; μείον; ανάδελτα του κεφαλαίο γάμμα, του 0; κόμμα; ύψιλον του τάαφ; κλείνει παρένθεση; d σίγμα; d τάαφ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_183() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mstyle displaystyle="true">
        <mrow>
         <munder>
          <mo>∫</mo>
          <mrow>
           <msub>
            <mi>Ω</mi>
            <mi>∞</mi>
           </msub>
           </mrow>
         </munder>
         <mrow>
          <mrow><mo>(</mo>
           <mrow>
            <mstyle displaystyle="true">
             <munderover>
              <mo>∑</mo>
              <mrow>
               <mrow><mo>|</mo> <mi>α</mi> <mo>|</mo></mrow><mo>+</mo><mi>ξ</mi><mo>=</mo><mn>1</mn></mrow>
              <mi>λ</mi>
             </munderover>
             <mrow>
              <msup>
               <mi>ρ</mi>
               <mrow>
                <mn>2</mn><mrow><mo>(</mo>
                 <mrow>
                  <mi>β</mi><mo>+</mo><mrow><mo>|</mo> <mi>α</mi> <mo>|</mo></mrow><mo>+</mo><mi>ξ</mi><mo>−</mo><mi>λ</mi></mrow>
                <mo>)</mo></mrow></mrow>
              </msup>
              <msup>
               <mrow>
                <mrow><mo>|</mo> <mrow>
                 <msup>
                  <mi>Δ</mi>
                  <mi>α</mi>
                 </msup>
                 <mi>λ</mi><msub>
                  <mi>υ</mi>
                  <mrow>
                   <msup>
                    <mi>τ</mi>
                    <mi>ξ</mi>
                   </msup>
                   </mrow>
                 </msub>
                 </mrow> <mo>|</mo></mrow></mrow>
               <mn>2</mn>
              </msup>
              <mo>+</mo><msup>
               <mrow>
                <mrow><mo>|</mo> <mi>υ</mi> <mo>|</mo></mrow></mrow>
               <mn>2</mn>
              </msup>
              </mrow>
            </mstyle></mrow>
          <mo>)</mo></mrow><msup>
           <mi>ε</mi>
           <mrow>
            <mo>−</mo><mn>2</mn><mi>γ</mi><mi>τ</mi></mrow>
          </msup>
          <mi>d</mi><mi>χ</mi><mi>d</mi><mi>τ</mi></mrow>
        </mrow>
        
       </mstyle></mrow>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <mfrac>
      <mn>1</mn>
      <mn>2</mn>
     </mfrac>
     </mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση; ολοκλήρωμα επί του συνόλου κεφαλαίο ωμέγα δείκτης άπειρο, του; ανοίγει παρένθεση; άθροισμα από η απόλυτη τιμή του άλφα; συν ξ; ισούται με 1 ως λάμδα του; ρ που υψώνεται στη 2; ανοίγει παρένθεση; βήτα συν, η απόλυτη τιμή του άλφα; συν ξ μείον λάμδα; κλείνει παρένθεση τέλος δύναμης; η απόλυτη τιμή του κεφαλαίο δέλτα στην άλφα οστή; λάμδα; ύψιλον δείκτης, τάαφ στην ξ οστή τέλος δείκτη, τέλος απόλυτης τιμής στο τετράγωνο; συν, η απόλυτη τιμή του ύψιλον στο τετράγωνο; κλείνει παρένθεση; έψιλον που υψώνεται στη μείον 2 γάμμα τάαφ τέλος δύναμης; d χ d τάαφ; κλείνει παρένθεση που υψώνεται στη 1 δεύτερο τέλος δύναμης")?;
    return Ok(());
}

#[test]
fn uoa_corpus_184() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mrow><mo>|</mo> <mi>π</mi> <mo>|</mo></mrow><mo>,</mo><mrow><mo>|</mo> <mi>ρ</mi> <mo>|</mo></mrow><mo>=</mo><mn>0</mn></mrow>
     <mi>μ</mi>
    </munderover>
    <mrow>
     <msup>
      <mrow>
       <mrow><mo>(</mo>
        <mrow>
         <mo>−</mo><mn>1</mn></mrow>
       <mo>)</mo></mrow></mrow>
      <mrow>
       <mrow><mo>|</mo> <mi>π</mi> <mo>|</mo></mrow></mrow>
     </msup>
     <mstyle displaystyle="true">
      <mrow>
       <munder>
        <mo>∫</mo>
        <mi>Ω</mi>
       </munder>
       <mrow>
        <msub>
         <mi>α</mi>
         <mrow>
          <mi>π</mi><mi>ρ</mi></mrow>
        </msub>
        <mrow><mo>(</mo>
         <mrow>
          <mo>·</mo><mo>,</mo><mi>τ</mi></mrow>
        <mo>)</mo></mrow><msup>
         <mi>Δ</mi>
         <mi>ρ</mi>
        </msup>
        <mi>υ</mi><mover accent="true">
         <mrow>
          <msup>
           <mi>Δ</mi>
           <mi>Π</mi>
          </msup>
          <mi>υ</mi></mrow>
         <mo stretchy="true">¯</mo>
        </mover>
        <mi>d</mi><mi>χ</mi></mrow>
      </mrow>
      
     </mstyle></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άθροισμα από η απόλυτη τιμή του π; κόμμα; η απόλυτη τιμή του ρ; ισούται με 0 ως μί του; ανοίγει παρένθεση, μείον 1, κλείνει παρένθεση που υψώνεται στη η απόλυτη τιμή του π τέλος δύναμης; ολοκλήρωμα επί του συνόλου κεφαλαίο ωμέγα του; άλφα δείκτης π ρ τέλος δείκτη; ανοίγει παρένθεση; τελεία κόμμα, τάαφ; κλείνει παρένθεση; κεφαλαίο δέλτα στην ρ οστή; ύψιλον; παράσταση κεφαλαίο δέλτα στην κεφαλαίο π οστή; ύψιλον με γραμμή από πάνω; d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_185() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>ξ</mi><mo>=</mo><mn>0</mn></mrow>
     <mrow>
      <mi>μ</mi><mo>−</mo><mn>1</mn></mrow>
    </munderover>
    <mrow>
     <mstyle displaystyle="true">
      <mrow>
       <munder>
        <mo>∫</mo>
        <mi>Γ</mi>
       </munder>
       <mrow>
        <msub>
         <mi>Β</mi>
         <mi>ξ</mi>
        </msub>
        <mi>υ</mi><mfrac>
         <mrow>
          <msup>
           <mo>∂</mo>
           <mi>ξ</mi>
          </msup>
          <mover accent="true">
           <mi>υ</mi>
           <mo>¯</mo>
          </mover>
          </mrow>
         <mrow>
          <mo>∂</mo><msup>
           <mi>ν</mi>
           <mi>ξ</mi>
          </msup>
          </mrow>
        </mfrac>
        <mi>d</mi><mi>σ</mi></mrow>
      </mrow>
      
     </mstyle></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άθροισμα από ξ ισούται με 0 ως μί μείον 1 του; ολοκλήρωμα επί του συνόλου κεφαλαίο γάμμα του; κεφαλαίο βήτα δείκτης ξ; ύψιλον; κλάσμα, μερικό διαφορικό στην ξ οστή; ύψιλον παύλα; προς, μερικό διαφορικό, νί στην ξ οστή, τέλος κλάσματος; d σίγμα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_186() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>Γ</mi>
    <mn>1</mn>
   </msub>
   <mstyle displaystyle="true">
    <mrow>
     <munder>
      <mo>∫</mo>
      <msup>
       <mi>Κ</mi>
       <mo>′</mo>
      </msup>
      
     </munder>
     <mrow>
      <mrow><mo>[</mo> <mrow>
       <msup>
        <mi>υ</mi>
        <mn>2</mn>
       </msup>
       <mo>+</mo><msup>
        <mrow>
         <mrow><mo>|</mo> <mrow>
          <mi>g</mi><mi>r</mi><mi>a</mi><mi>d</mi><mi>υ</mi></mrow> <mo>|</mo></mrow></mrow>
        <mn>2</mn>
       </msup>
       <mo>+</mo><mstyle displaystyle="true">
        <munder>
         <mo>∑</mo>
         <mrow>
          <mrow><mo>|</mo> <mi>α</mi> <mo>|</mo></mrow><mo>=</mo><mn>2</mn></mrow>
        </munder>
        <mrow>
         <msup>
          <mrow>
           <mrow><mo>|</mo> <mrow>
            <msup>
             <mi>Δ</mi>
             <mi>α</mi>
            </msup>
            <msub>
             <mi>υ</mi>
             <mn>1</mn>
            </msub>
            </mrow> <mo>|</mo></mrow></mrow>
          <mn>2</mn>
         </msup>
         </mrow>
       </mstyle></mrow> <mo>]</mo></mrow><mi>d</mi><msup>
       <mi>χ</mi>
       <mo>′</mo>
      </msup>
      </mrow>
    </mrow>
    
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο γάμμα δείκτης 1; ολοκλήρωμα επί του συνόλου κεφαλαίο καπα τόνος, του; ανοίγει αγκύλη; ύψιλον στο τετράγωνο, συν; η απόλυτη τιμή του κλίση του ύψιλον στο τετράγωνο; συν; άθροισμα για η απόλυτη τιμή του άλφα; ισούται με 2 της απόλυτης τιμής του κεφαλαίο δέλτα στην άλφα οστή; ύψιλον δείκτης 1, τέλος απόλυτης τιμής στο τετράγωνο; κλείνει αγκύλη; d χ τόνος")?;
    return Ok(());
    //theodoa. fails. Should be okay when we add genitive
}

#[test]
fn uoa_corpus_187() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow>
     <msubsup>
      <mo>∫</mo>
      <mn>0</mn>
      <mn>1</mn>
     </msubsup>
     <mrow>
      <mrow><mo>(</mo>
       <mrow>
        <msup>
         <mi>χ</mi>
         <mi>ψ</mi>
        </msup>
        <mo>−</mo><mfrac>
         <mi>ζ</mi>
         <mrow>
          <mn>10</mn></mrow>
        </mfrac>
        </mrow>
      <mo>)</mo></mrow><mi>d</mi><mi>ζ</mi></mrow>
    </mrow>
    
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από 0 ως 1 του; ανοίγει παρένθεση; χ στην ψ οστή, μείον ζήτα προς 10; κλείνει παρένθεση; d ζήτα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_188() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow>
     <msubsup>
      <mo>∫</mo>
      <mn>0</mn>
      <mn>1</mn>
     </msubsup>
     <mrow>
      <mstyle displaystyle="true">
       <mrow>
        <msubsup>
         <mo>∫</mo>
         <mn>0</mn>
         <mrow>
          <msqrt>
           <mrow>
            <mn>1</mn><mo>−</mo><msup>
             <mi>ψ</mi>
             <mn>2</mn>
            </msup>
            </mrow>
          </msqrt>
          </mrow>
        </msubsup>
        <mrow>
         <mn>1</mn><mi>d</mi><mi>χ</mi><mi>d</mi><mi>ψ</mi></mrow>
       </mrow>
       
      </mstyle></mrow>
    </mrow>
    
   </mstyle><mo>=</mo><mstyle displaystyle="true">
    <mrow>
     <msubsup>
      <mo>∫</mo>
      <mn>0</mn>
      <mrow>
       <mfrac>
        <mi>π</mi>
        <mn>2</mn>
       </mfrac>
       </mrow>
     </msubsup>
     <mrow>
      <mstyle displaystyle="true">
       <mrow>
        <msubsup>
         <mo>∫</mo>
         <mn>0</mn>
         <mn>1</mn>
        </msubsup>
        <mrow>
         <mi>ρ</mi><mi>d</mi><mi>ρ</mi><mi>d</mi><mi>θ</mi></mrow>
       </mrow>
       
      </mstyle></mrow>
    </mrow>
    
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα από 0 ως 1 του; ολοκλήρωμα από 0 ως η τετραγωνική ρίζα του 1 μείον, ψ στο τετράγωνο, τέλος ρίζας; του; 1 d χ d ψ; ισούται με; ολοκλήρωμα από 0 ως π προς 2, του; ολοκλήρωμα από 0 ως 1 του; ρ d ρ d θήτα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_189() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <mrow><mo>∫</mo>
     <mrow>
      <mrow><mo>[</mo> <mrow>
       <mfrac>
        <mrow>
         <mi>cos</mi><msqrt>
          <mi>χ</mi>
         </msqrt>
         </mrow>
        <mrow>
         <mn>2</mn><mo>×</mo><msqrt>
          <mi>χ</mi>
         </msqrt>
         </mrow>
       </mfrac>
       </mrow> <mo>]</mo></mrow></mrow>
    </mrow>
    
   </mstyle><mi>d</mi><mi>χ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ολοκλήρωμα του ανοίγει αγκύλη; κλάσμα, συνημίτονο του, η τετραγωνική ρίζα του χ; προς, 2 επί, η τετραγωνική ρίζα του χ; τέλος κλάσματος; κλείνει αγκύλη; d χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_190() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>ν</mi><mo>→</mo><mi>∞</mi></mrow>
   </munder>
   <msub>
    <mrow>
     <mrow><mo>|</mo> <mrow>
      <msub>
       <mi>υ</mi>
       <mi>ν</mi>
      </msub>
      <mo>−</mo><mi>υ</mi></mrow> <mo>|</mo></mrow></mrow>
    <mrow>
     <mi>π</mi><mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow></mrow>
   </msub>
   <mo>=</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν νί προσεγγίζει, άπειρο; της απόλυτης τιμής του ύψιλον δείκτης νί; μείον ύψιλον τέλος απόλυτης τιμής δείκτης, π του τάαφ; ισούται με 0")?;
    return Ok(());
    //theodora. fails. Should be okay when we add genitive
}

#[test]
fn uoa_corpus_191() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>ν</mi><mo>→</mo><mi>∞</mi></mrow>
   </munder>
   <mi>ρ</mi><mrow><mo>(</mo>
    <mrow>
     <msub>
      <mi>υ</mi>
      <mi>ν</mi>
     </msub>
     <mo>−</mo><mi>υ</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν νί προσεγγίζει, άπειρο; του ρ; ανοίγει παρένθεση; ύψιλον δείκτης νί; μείον ύψιλον; κλείνει παρένθεση; ισούται με 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_192() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>ν</mi><mo>→</mo><mi>∞</mi></mrow>
   </munder>
   <mi>ρ</mi><mrow><mo>(</mo>
    <mrow>
     <msub>
      <mi>υ</mi>
      <mi>ν</mi>
     </msub>
     </mrow>
   <mo>)</mo></mrow><mo>=</mo><mi>ρ</mi><mrow><mo>(</mo>
    <mrow>
     <msub>
      <mi>υ</mi>
      <mi>ν</mi>
     </msub>
     </mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν νί προσεγγίζει, άπειρο; του ρ του; ανοίγει παρένθεση; ύψιλον δείκτης νί; κλείνει παρένθεση; ισούται με; ρ του; ανοίγει παρένθεση; ύψιλον δείκτης νί; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_193() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi><mi>sup</mi></mrow>
    <mrow>
     <mi>ν</mi><mo>→</mo><mi>∞</mi></mrow>
   </munder>
   <mrow><mo>〈</mo> <mrow>
    <msup>
     <mi>Ξ</mi>
     <mo>′</mo>
    </msup>
    <mrow><mo>(</mo>
     <mrow>
      <msub>
       <mi>υ</mi>
       <mi>ν</mi>
      </msub>
      </mrow>
    <mo>)</mo></mrow><mo>−</mo><msup>
     <mi>Ξ</mi>
     <mo>′</mo>
    </msup>
    <mrow><mo>(</mo>
     <mi>υ</mi>
    <mo>)</mo></mrow><mo>,</mo><msub>
     <mi>υ</mi>
     <mi>ν</mi>
    </msub>
    <mo>−</mo><mi>υ</mi></mrow> <mo>〉</mo></mrow><mo>=</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανώτατο όριο για νί βέλος προς τα δεξιά, άπειρο; αριστερά γωνιακή αγκύλη; κεφαλαίο ξ τόνος; του; ανοίγει παρένθεση; ύψιλον δείκτης νί; κλείνει παρένθεση; μείον, κεφαλαίο ξ τόνος; του ύψιλον; κόμμα; ύψιλον δείκτης νί; μείον ύψιλον; δεξιά γωνιακή αγκύλη; ισούται με 0")?;
    return Ok(());
    //theodora. Now reads: παράσταση lim του σουπρέμουμ με νί βέλος προς τα δεξιά, άπειρο από κάτω; αριστερά γωνιακή αγκύλη; κεφαλαίο ξ τόνος; του; ανοίγει παρένθεση; ύψιλον δείκτης νί; κλείνει παρένθεση; μείον, κεφαλαίο ξ τόνος; του ύψιλον; κόμμα; ύψιλον δείκτης νί; μείον ύψιλον; δεξιά γωνιακή αγκύλη; ισούται με 0
}

#[test]
fn uoa_corpus_194() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mrow><mo>[</mo> <mrow>
      <mi>Α</mi><munder>
       <mrow>
        <mi>lim</mi></mrow>
       <mrow>
        <mi>χ</mi><mo>→</mo><mo>+</mo><mn>0</mn></mrow>
      </munder>
      <mfrac>
       <mrow>
        <mi>φ</mi><mrow><mo>(</mo>
         <mi>χ</mi>
        <mo>)</mo></mrow></mrow>
       <mi>χ</mi>
      </mfrac>
      </mrow> <mo>]</mo></mrow></mrow>
    <mrow>
     <mo>−</mo><mn>1</mn></mrow>
   </msup>
   <mo>&lt;</mo><mi>λ</mi><mo>&lt;</mo><msup>
    <mrow>
     <mrow><mo>[</mo> <mrow>
      <mi>Β</mi><munder>
       <mrow>
        <mi>lim</mi></mrow>
       <mrow>
        <mi>χ</mi><mo>→</mo><mi>∞</mi></mrow>
      </munder>
      <mfrac>
       <mrow>
        <mi>φ</mi><mrow><mo>(</mo>
         <mi>χ</mi>
        <mo>)</mo></mrow></mrow>
       <mi>χ</mi>
      </mfrac>
      </mrow> <mo>]</mo></mrow></mrow>
    <mrow>
     <mo>−</mo><mn>1</mn></mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει αγκύλη; κεφαλαίο άλφα; το όριο όταν χ προσεγγίζει, συν 0; του; κλάσμα, φ του χ, προς χ, τέλος κλάσματος; κλείνει αγκύλη στην μείον 1; είναι μικρότερο από, λάμδα, είναι μικρότερο από; ανοίγει αγκύλη; κεφαλαίο βήτα; το όριο όταν χ προσεγγίζει, άπειρο; του; κλάσμα, φ του χ, προς χ, τέλος κλάσματος; κλείνει αγκύλη στην μείον 1")?;
    return Ok(());
   
}

#[test]
fn uoa_corpus_195() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mrow><mo>[</mo> <mrow>
      <mi>Α</mi><munder>
       <mrow>
        <mi>lim</mi></mrow>
       <mrow>
        <mi>χ</mi><mo>→</mo><mi>∞</mi></mrow>
      </munder>
      <mfrac>
       <mrow>
        <mi>φ</mi><mrow><mo>(</mo>
         <mi>χ</mi>
        <mo>)</mo></mrow></mrow>
       <mi>χ</mi>
      </mfrac>
      </mrow> <mo>]</mo></mrow></mrow>
    <mrow>
     <mo>−</mo><mn>1</mn></mrow>
   </msup>
   <mo>&lt;</mo><mi>λ</mi><mo>&lt;</mo><msup>
    <mrow>
     <mrow><mo>[</mo> <mrow>
      <mi>Β</mi><munder>
       <mrow>
        <mi>lim</mi></mrow>
       <mrow>
        <mi>χ</mi><mo>→</mo><mo>+</mo><mn>0</mn></mrow>
      </munder>
      <mfrac>
       <mrow>
        <mi>φ</mi><mrow><mo>(</mo>
         <mi>χ</mi>
        <mo>)</mo></mrow></mrow>
       <mi>χ</mi>
      </mfrac>
      </mrow> <mo>]</mo></mrow></mrow>
    <mrow>
     <mo>−</mo><mn>1</mn></mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει αγκύλη; κεφαλαίο άλφα; το όριο όταν χ προσεγγίζει, άπειρο; του; κλάσμα, φ του χ, προς χ, τέλος κλάσματος; κλείνει αγκύλη στην μείον 1; είναι μικρότερο από, λάμδα, είναι μικρότερο από; ανοίγει αγκύλη; κεφαλαίο βήτα; το όριο όταν χ προσεγγίζει, συν 0; του; κλάσμα, φ του χ, προς χ, τέλος κλάσματος; κλείνει αγκύλη στην μείον 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_196() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>λ</mi><munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>Δ</mi><mi>τ</mi><mo>→</mo><mn>0</mn></mrow>
   </munder>
   <mfrac>
    <mn>1</mn>
    <mrow>
     <mi>Δ</mi><mi>τ</mi></mrow>
   </mfrac>
   <mrow><mo>[</mo> <mrow>
    <mrow><mo>(</mo>
     <mrow>
      <mi>Τ</mi><mi>υ</mi></mrow>
    <mo>)</mo></mrow><mrow><mo>(</mo>
     <mrow>
      <mi>τ</mi><mo>+</mo><mi>Δ</mi><mi>τ</mi></mrow>
    <mo>)</mo></mrow><mo>−</mo><mrow><mo>(</mo>
     <mrow>
      <mi>Τ</mi><mi>υ</mi></mrow>
    <mo>)</mo></mrow><mrow><mo>(</mo>
     <mi>τ</mi>
    <mo>)</mo></mrow></mrow> <mo>]</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "λάμδα; το όριο όταν κεφαλαίο δέλτα, τάαφ; προσεγγίζει 0; του; κλάσμα, 1 προς, κεφαλαίο δέλτα, τάαφ, τέλος κλάσματος; ανοίγει αγκύλη; ανοίγει παρένθεση; κεφαλαίο τάαφ, ύψιλον; κλείνει παρένθεση; ανοίγει παρένθεση; τάαφ συν, κεφαλαίο δέλτα, τάαφ; κλείνει παρένθεση; μείον; ανοίγει παρένθεση; κεφαλαίο τάαφ, ύψιλον; κλείνει παρένθεση; τάαφ; κλείνει αγκύλη")?;
    return Ok(());
}

#[test]
fn uoa_corpus_197() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>Δ</mi><mi>τ</mi><mo>→</mo><mn>0</mn></mrow>
   </munder>
   <mfrac>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <msub>
        <mi>Τ</mi>
        <mn>2</mn>
       </msub>
       <mi>υ</mi></mrow>
     <mo>)</mo></mrow><mrow><mo>(</mo>
      <mrow>
       <mi>τ</mi><mo>+</mo><mi>Δ</mi><mi>τ</mi></mrow>
     <mo>)</mo></mrow><mo>−</mo><mrow><mo>(</mo>
      <mrow>
       <msub>
        <mi>Τ</mi>
        <mn>2</mn>
       </msub>
       <mi>υ</mi></mrow>
     <mo>)</mo></mrow><mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <mi>Δ</mi><mi>τ</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν κεφαλαίο δέλτα, τάαφ; προσεγγίζει 0; του; κλάσματος, ανοίγει παρένθεση; κεφαλαίο τάαφ δείκτης 2; ύψιλον; κλείνει παρένθεση; του τάαφ συν, κεφαλαίο δέλτα, τάαφ; κλείνει παρένθεση; μείον; ανοίγει παρένθεση; κεφαλαίο τάαφ δείκτης 2; ύψιλον; κλείνει παρένθεση; του τάαφ, προς, κεφαλαίο δέλτα, τάαφ, τέλος κλάσματος")?;
    return Ok(());
    //theodora. fails. Now reads: το όριο όταν κεφαλαίο δέλτα, τάαφ; προσεγγίζει 0; του; κλάσμα, ανοίγει παρένθεση; κεφαλαίο τάαφ δείκτης 2; ύψιλον; κλείνει παρένθεση; ανοίγει παρένθεση; τάαφ συν, κεφαλαίο δέλτα, τάαφ; κλείνει παρένθεση; μείον; ανοίγει παρένθεση; κεφαλαίο τάαφ δείκτης 2; ύψιλον; κλείνει παρένθεση; τάαφ, προς, κεφαλαίο δέλτα, τάαφ, τέλος κλάσματος

  }

#[test]
fn uoa_corpus_198() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>ν</mi><mo>→</mo><mi>∞</mi></mrow>
   </munder>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mn>1</mn>
    </munderover>
    <mrow>
     <mrow><mo>|</mo> <mrow>
      <mrow><mo>(</mo>
       <mrow>
        <msub>
         <mi>Τ</mi>
         <mn>1</mn>
        </msub>
        <msub>
         <mi>υ</mi>
         <mi>ν</mi>
        </msub>
        </mrow>
      <mo>)</mo></mrow><mrow><mo>(</mo>
       <mi>τ</mi>
      <mo>)</mo></mrow><mo>−</mo><mrow><mo>(</mo>
       <mrow>
        <msub>
         <mi>Τ</mi>
         <mn>1</mn>
        </msub>
        <msub>
         <mi>υ</mi>
         <mn>0</mn>
        </msub>
        </mrow>
      <mo>)</mo></mrow><mrow><mo>(</mo>
       <mi>τ</mi>
      <mo>)</mo></mrow></mrow> <mo>|</mo></mrow><mi>d</mi><mi>τ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν νί προσεγγίζει, άπειρο; του; ολοκλήρωμα από 0 ως 1 της απόλυτης τιμής του ανοίγει παρένθεση; κεφαλαίο τάαφ δείκτης 1; ύψιλον δείκτης νί; κλείνει παρένθεση; του τάαφ; μείον; ανοίγει παρένθεση; κεφαλαίο τάαφ δείκτης 1; ύψιλον δείκτης 0; κλείνει παρένθεση; του τάαφ τέλος απόλυτης τιμής; d τάαφ")?;
    return Ok(());
    //theodora Since T_1υ_ν and T_1υ_0 are functions or operators it would be best to read "Tυ of τ" 
    //genitive clause also needed here
  }

#[test]
fn uoa_corpus_199() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>χ</mi><mo>→</mo><mo>+</mo><mn>0</mn></mrow>
   </munder>
   <mfrac>
    <mrow>
     <mi>φ</mi><mrow><mo>(</mo>
      <mi>χ</mi>
     <mo>)</mo></mrow></mrow>
    <mi>χ</mi>
   </mfrac>
   <mo>&lt;</mo><msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mi>λ</mi><mi>Β</mi></mrow>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <mo>−</mo><mn>1</mn></mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν χ προσεγγίζει, συν 0; του; κλάσμα, φ του χ, προς χ, τέλος κλάσματος; είναι μικρότερο από; ανοίγει παρένθεση; λάμδα κεφαλαίο βήτα; κλείνει παρένθεση στην μείον 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_200() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>λ</mi><mi>Α</mi><munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>χ</mi><mo>→</mo><mi>∞</mi></mrow>
   </munder>
   <mfrac>
    <mrow>
     <mi>φ</mi><mrow><mo>(</mo>
      <mi>χ</mi>
     <mo>)</mo></mrow></mrow>
    <mi>χ</mi>
   </mfrac>
   <mo>&gt;</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "λάμδα κεφαλαίο άλφα; το όριο όταν χ προσεγγίζει, άπειρο; του; κλάσμα, φ του χ, προς χ, τέλος κλάσματος; είναι μεγαλύτερο από 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_201() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>χ</mi><mo>→</mo><mi>∞</mi></mrow>
   </munder>
   <mfrac>
    <mrow>
     <mi>φ</mi><mrow><mo>(</mo>
      <mi>χ</mi>
     <mo>)</mo></mrow></mrow>
    <mi>χ</mi>
   </mfrac>
   <mo>≥</mo><msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mi>λ</mi><mover accent="true">
        <mi>Α</mi>
        <mo>¯</mo>
       </mover>
       </mrow>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <mo>−</mo><mn>1</mn></mrow>
   </msup>
   <mo>+</mo><mi>ε</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν χ προσεγγίζει, άπειρο; του; κλάσμα, φ του χ, προς χ, τέλος κλάσματος; είναι μεγαλύτερο από ή ίσο με; ανοίγει παρένθεση; λάμδα, κεφαλαίο άλφα παύλα; κλείνει παρένθεση στην μείον 1; συν έψιλον")?;
    return Ok(());
}

#[test]
fn uoa_corpus_202() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>χ</mi><mo>→</mo><msup>
      <mn>0</mn>
      <mo>−</mo>
     </msup>
     </mrow>
   </munder>
   <mfrac>
    <mn>1</mn>
    <mi>χ</mi>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν χ προσεγγίζει, 0 εκθέτης μείον; του 1 προς χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_203() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>x</mi><mo>→</mo><msup>
      <mn>0</mn>
      <mo>−</mo>
     </msup>
     </mrow>
   </munder>
   <mrow><mo>(</mo>
    <mrow>
     <mfrac>
      <mn>1</mn>
      <mi>χ</mi>
     </mfrac>
     <mo>+</mo><mfrac>
      <mn>1</mn>
      <mrow>
       <msup>
        <mi>χ</mi>
        <mn>2</mn>
       </msup>
       </mrow>
     </mfrac>
     </mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν x προσεγγίζει, 0 εκθέτης μείον; του; ανοίγει παρένθεση; 1 προς χ, συν; κλάσμα, 1 προς, χ στο τετράγωνο, τέλος κλάσματος; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_204() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>x</mi><mo>→</mo><mn>3</mn></mrow>
   </munder>
   <mfrac>
    <mrow>
     <mn>1</mn><mo>−</mo><msqrt>
      <mrow>
       <mi>χ</mi><mo>−</mo><mn>2</mn></mrow>
     </msqrt>
     </mrow>
    <mrow>
     <mi>χ</mi><mo>−</mo><mn>3</mn></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν x προσεγγίζει 3; του; κλάσμα, 1 μείον; η τετραγωνική ρίζα του χ μείον 2, τέλος ρίζας; προς, χ μείον 3, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_205() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>δ</mi><mi>x</mi><mo>→</mo><mn>0</mn></mrow>
   </munder>
   <mfrac>
    <mrow>
     <mi>φ</mi><mo stretchy="false">(</mo><msub>
      <mi>χ</mi>
      <mn>0</mn>
     </msub>
     <mo>+</mo><mi>δ</mi><mi>χ</mi><mo stretchy="false">)</mo><mo>−</mo><mi>φ</mi><mo stretchy="false">(</mo><msub>
      <mi>χ</mi>
      <mn>0</mn>
     </msub>
     <mo stretchy="false">)</mo></mrow>
    <mrow>
     <mi>δ</mi><mi>χ</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν δέλτα x, προσεγγίζει 0; του; κλάσμα, φ; ανοίγει παρένθεση; χ δείκτης 0; συν δέλτα χ; κλείνει παρένθεση; μείον; φ του; ανοίγει παρένθεση, χ δείκτης 0; κλείνει παρένθεση, προς, δέλτα χ, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_206() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>β</mi><mo>→</mo><mn>1</mn></mrow>
   </munder>
   <mfrac>
    <mi>β</mi>
    <mrow>
     <msup>
      <mrow>
       <mrow><mo>(</mo>
        <mrow>
         <mi>γ</mi><mo>−</mo><mi>δ</mi></mrow>
       <mo>)</mo></mrow></mrow>
      <mn>2</mn>
     </msup>
     </mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν βήτα προσεγγίζει 1; του; κλάσμα, βήτα προς, ανοίγει παρένθεση; γάμμα μείον δέλτα; κλείνει παρένθεση στο τετράγωνο, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_207() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>limsup</mi></mrow>
    <mrow>
     <mi>ν</mi><mo>→</mo><mi>∞</mi></mrow>
   </munder>
   <mrow><mo>(</mo>
    <mrow>
     <msup>
      <mi>Ξ</mi>
      <mo>′</mo>
     </msup>
     <mrow><mo>(</mo>
      <mrow>
       <msub>
        <mi>υ</mi>
        <mi>ν</mi>
       </msub>
       </mrow>
     <mo>)</mo></mrow><mo>−</mo><msup>
      <mi>Ξ</mi>
      <mo>′</mo>
     </msup>
     <mrow><mo>(</mo>
      <mi>υ</mi>
     <mo>)</mo></mrow><mo>,</mo><msub>
      <mi>υ</mi>
      <mi>ν</mi>
     </msub>
     <mo>−</mo><mi>υ</mi></mrow>
   <mo>)</mo></mrow><mo>≤</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανώτατο όριο για νί βέλος προς τα δεξιά, άπειρο; του; ανοίγει παρένθεση; κεφαλαίο ξ τόνος; του; ανοίγει παρένθεση; ύψιλον δείκτης νί; κλείνει παρένθεση; μείον, κεφαλαίο ξ τόνος; του ύψιλον; κόμμα; ύψιλον δείκτης νί; μείον ύψιλον; κλείνει παρένθεση; είναι μικρότερο από ή ίσο με 0")?;
    return Ok(());
    //theodora. fails. Now reads: limit sup νί βέλος προς τα δεξιά, άπειρο; του; ανοίγει παρένθεση; κεφαλαίο ξ τόνος; του; ανοίγει παρένθεση; ύψιλον δείκτης νί; κλείνει παρένθεση; μείον, κεφαλαίο ξ τόνος; του ύψιλον; κόμμα; ύψιλον δείκτης νί; μείον ύψιλον; κλείνει παρένθεση; είναι μικρότερο από ή ίσο με 0
}

#[test]
fn uoa_corpus_208() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>limsup</mi></mrow>
    <mrow>
     <mi>ν</mi><mo>→</mo><mi>∞</mi></mrow>
   </munder>
   <msup>
    <mrow>
     <mrow><mo>|</mo> <mrow>
      <msub>
       <mover accent="true">
        <mi>υ</mi>
        <mo>¯</mo>
       </mover>
       
       <mi>ν</mi>
      </msub>
      </mrow> <mo>|</mo></mrow></mrow>
    <mrow>
     <mo>−</mo><msup>
      <mi>ρ</mi>
      <mo>+</mo>
     </msup>
     <mi>α</mi></mrow>
   </msup>
   <mi>φ</mi><mrow><mo>(</mo>
    <mrow>
     <msub>
      <mi>υ</mi>
      <mi>ν</mi>
     </msub>
     </mrow>
   <mo>)</mo></mrow><mo>=</mo><mo>−</mo><mi>∞</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανώτατο όριο για νί βέλος προς τα δεξιά, άπειρο; της απόλυτης τιμής του ύψιλον παύλα, δείκτης νί, τέλος απόλυτης τιμής που υψώνεται στη μείον ρ εκθέτης συν; άλφα τέλος δύναμης; φ του; ανοίγει παρένθεση; ύψιλον δείκτης νί; κλείνει παρένθεση; ισούται με, μείον άπειρο")?;
    return Ok(());
    //theodora. fails. Now reads: limit sup νί βέλος προς τα δεξιά, άπειρο; του; η απόλυτη τιμή του ύψιλον παύλα, δείκτης νί, τέλος απόλυτης τιμής που υψώνεται στη μείον ρ εκθέτης συν; άλφα τέλος δύναμης; φ του; ανοίγει παρένθεση; ύψιλον δείκτης νί; κλείνει παρένθεση; ισούται με, μείον άπειρο
    //also needs genitive
  }

#[test]
fn uoa_corpus_209() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>Δ</mi><mi>τ</mi><mo>→</mo><mn>0</mn></mrow>
   </munder>
   <mfrac>
    <mi>λ</mi>
    <mrow>
     <mi>Δ</mi><mi>τ</mi></mrow>
   </mfrac>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mn>1</mn>
    </munderover>
    <mrow>
     <mrow><mo>[</mo> <mrow>
      <mi>Ξ</mi><mrow><mo>(</mo>
       <mrow>
        <mi>τ</mi><mo>+</mo><mi>Δ</mi><mi>τ</mi><mo>,</mo><mi>σ</mi></mrow>
      <mo>)</mo></mrow><mo>−</mo><mi>Ξ</mi><mrow><mo>(</mo>
       <mrow>
        <mi>τ</mi><mo>,</mo><mi>σ</mi></mrow>
      <mo>)</mo></mrow></mrow> <mo>]</mo></mrow><mi>υ</mi><mrow><mo>(</mo>
      <mi>σ</mi>
     <mo>)</mo></mrow><mi>d</mi><mi>σ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν κεφαλαίο δέλτα, τάαφ; προσεγγίζει 0; του; κλάσμα, λάμδα προς, κεφαλαίο δέλτα, τάαφ, τέλος κλάσματος; ολοκλήρωμα από 0 ως 1 του; ανοίγει αγκύλη; κεφαλαίο ξ; ανοίγει παρένθεση; τάαφ συν, κεφαλαίο δέλτα, τάαφ; κόμμα, σίγμα; κλείνει παρένθεση; μείον; κεφαλαίο ξ; ανοίγει παρένθεση; τάαφ κόμμα, σίγμα; κλείνει παρένθεση; κλείνει αγκύλη; ύψιλον του σίγμα; d σίγμα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_210() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>ν</mi><mo>→</mo><mi>∞</mi></mrow>
   </munder>
   <mstyle displaystyle="true">
    <mrow><munderover>
     <mo>∫</mo>
     <mn>0</mn>
     <mn>1</mn>
    </munderover>
    <mrow>
     <mrow><mo>|</mo> <mrow>
      <mrow><mo>(</mo>
       <mrow>
        <msub>
         <mi>Τ</mi>
         <mn>1</mn>
        </msub>
        <msub>
         <mi>υ</mi>
         <mi>ν</mi>
        </msub>
        </mrow>
      <mo>)</mo></mrow><mrow><mo>(</mo>
       <mi>τ</mi>
      <mo>)</mo></mrow><mo>−</mo><msub>
       <mi>Τ</mi>
       <mn>1</mn>
      </msub>
      <msub>
       <mi>υ</mi>
       <mn>0</mn>
      </msub>
      <mrow><mo>(</mo>
       <mi>τ</mi>
      <mo>)</mo></mrow></mrow> <mo>|</mo></mrow><mi>d</mi><mi>τ</mi></mrow>
   </mrow>
   
  </mstyle></mrow>
 </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν νί προσεγγίζει, άπειρο; του; ολοκλήρωμα από 0 ως 1 της απόλυτης τιμής του ανοίγει παρένθεση; κεφαλαίο τάαφ δείκτης 1; ύψιλον δείκτης νί; κλείνει παρένθεση; του τάαφ; μείον; κεφαλαίο τάαφ δείκτης 1; ύψιλον δείκτης 0; του τάαφ τέλος απόλυτης τιμής; d τάαφ")?;
    return Ok(());
    //theodora. doesnt recognize the first operator as an operator.
    // genitive clause needed here. 

}

#[test]
fn uoa_corpus_211() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>ν</mi><mo>→</mo><mi>∞</mi></mrow>
   </munder>
   <mi>φ</mi><mrow><mo>(</mo>
    <mrow>
     <mi>τ</mi><mo>,</mo><msub>
      <mi>υ</mi>
      <mi>ν</mi>
     </msub>
     <mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow><mo>,</mo><msub>
      <msup>
       <mi>υ</mi>
       <mo>′</mo>
      </msup>
      
      <mi>ν</mi>
     </msub>
     <mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow><mo>,</mo><msup>
      <mi>υ</mi>
      <mo>″</mo>
     </msup>
     <mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν νί προσεγγίζει, άπειρο; του φ; ανοίγει παρένθεση; τάαφ κόμμα; ύψιλον δείκτης νί; του τάαφ; κόμμα; ύψιλον τόνος, δείκτης νί; του τάαφ; κόμμα; ύψιλον διπλή παράγωγος; του τάαφ; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_212() -> Result<()> { //
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><munder><mi>lim</mi><mrow><mi>&#x394;</mi><mi>&#x3C4;</mi><mo>&#x2192;</mo><mn>0</mn></mrow></munder><munderover><mo>&#x222B;</mo><mn>0</mn><mn>1</mn></munderover><mfrac><mrow><mi>&#x39E;</mi><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>+</mo><mi>&#x394;</mi><mi>&#x3C4;</mi><mo>)</mo></mrow><mo>&#x2212;</mo><mi>&#x39E;</mi><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>,</mo><mi>&#x3C3;</mi><mo>)</mo></mrow></mrow><mrow><mi>&#x394;</mi><mi>&#x3C4;</mi></mrow></mfrac><mi>&#x3B7;</mi><mrow><mo>(</mo><mi>&#x3C3;</mi><mo>)</mo></mrow><mspace linebreak="newline"/><mi>&#x3C6;</mi><mrow><mo>(</mo><mi>&#x3C3;</mi><mo>,</mo><mi>&#x3C5;</mi><mrow><mo>(</mo><mi>&#x3C3;</mi><mo>)</mo></mrow><mo>,</mo><msup><mi>&#x3C5;</mi><mo>'</mo></msup><mrow><mo>(</mo><mi>&#x3C3;</mi><mo>)</mo></mrow><mo>,</mo><msup><mi>&#x3C5;</mi><mo>''</mo></msup><mrow><mo>(</mo><mi>&#x3C3;</mi><mo>)</mo></mrow><mo>)</mo></mrow><mi>d</mi><mi>&#x3C3;</mi></math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν κεφαλαίο δέλτα, τάαφ; προσεγγίζει 0; του; ολοκλήρωμα από 0 ως 1 του; κλάσμα, κεφαλαίο ξ; ανοίγει παρένθεση; τάαφ συν, κεφαλαίο δέλτα, τάαφ; κλείνει παρένθεση; μείον; κεφαλαίο ξ; ανοίγει παρένθεση; τάαφ κόμμα, σίγμα; κλείνει παρένθεση, προς, κεφαλαίο δέλτα, τάαφ, τέλος κλάσματος; ήτα του σίγμα, φ; ανοίγει παρένθεση; σίγμα κόμμα; ύψιλον του σίγμα; κόμμα; ύψιλον τόνος; του σίγμα; κόμμα; ύψιλον διπλή παράγωγος; του σίγμα; κλείνει παρένθεση; d σίγμα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_213() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>Α</mi><mo>=</mo><munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>ν</mi><mo>→</mo><mi>∞</mi></mrow>
   </munder>
   <mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>κ</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>ν</mi>
    </munderover>
    <mrow>
     <mi>φ</mi><mo stretchy="false">(</mo><msub>
      <mi>δ</mi>
      <mi>κ</mi>
     </msub>
     <mo stretchy="false">)</mo><mi>Δ</mi><mi>χ</mi></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο άλφα, ισούται με; το όριο όταν νί προσεγγίζει, άπειρο; του; άθροισμα από καπα ισούται με 1 ως νί του; φ του; ανοίγει παρένθεση; δέλτα δείκτης καπα; κλείνει παρένθεση; κεφαλαίο δέλτα, χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_214() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <munder>
    <mrow>
     <mi>lim</mi></mrow>
    <mrow>
     <mi>χ</mi><mo>→</mo><mi>∞</mi></mrow>
   </munder>
   <mstyle displaystyle="true">
    <mrow>
     <msubsup>
      <mo>∫</mo>
      <mn>0</mn>
      <mi>χ</mi>
     </msubsup>
     <mrow>
      <msup>
       <mi>e</mi>
       <mrow>
        <mo>−</mo><msup>
         <mi>ψ</mi>
         <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      <mi>d</mi><mi>ψ</mi></mrow>
    </mrow>
    
   </mstyle><mo>=</mo><mfrac>
    <mrow>
     <msqrt>
      <mi>π</mi>
     </msqrt>
     </mrow>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το όριο όταν χ προσεγγίζει, άπειρο; του; ολοκλήρωμα από 0 ως χ του; e που υψώνεται στη μείον ψ στο τετράγωνο τέλος δύναμης; d ψ; ισούται με; κλάσμα, η τετραγωνική ρίζα του π; προς 2, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_215() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mrow>
         <msub>
          <mi>β</mi>
          <mn>1</mn>
         </msub>
         </mrow>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mrow>
         <msub>
          <mi>β</mi>
          <mn>2</mn>
         </msub>
         </mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 επί ένα πίνακας-στήλη; γραμμή 1; βήτα δείκτης 1; γραμμή 2; βήτα δείκτης 2")?;
    return Ok(());
}

#[test]
fn uoa_corpus_216() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mrow>
         <msub>
          <mi>Σ</mi>
          <mrow>
           <mn>1,1</mn></mrow>
         </msub>
         </mrow>
       </mtd>
       <mtd>
        <mrow>
         <msub>
          <mi>Σ</mi>
          <mrow>
           <mn>1,2</mn></mrow>
         </msub>
         </mrow>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mrow>
         <msub>
          <mi>Σ</mi>
          <mrow>
           <mn>2,1</mn></mrow>
         </msub>
         </mrow>
       </mtd>
       <mtd>
        <mrow>
         <msub>
          <mi>Σ</mi>
          <mrow>
           <mn>2,2</mn></mrow>
         </msub>
         </mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2")?;
    return Ok(());
}

#[test]
fn uoa_corpus_217() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mrow>
         <msup>
          <mi>Σ</mi>
          <mrow>
           <mn>1,1</mn></mrow>
         </msup>
         </mrow>
       </mtd>
       <mtd>
        <mrow>
         <msup>
          <mi>Σ</mi>
          <mrow>
           <mn>1,2</mn></mrow>
         </msup>
         </mrow>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mrow>
         <msup>
          <mi>Σ</mi>
          <mrow>
           <mn>2,1</mn></mrow>
         </msup>
         </mrow>
       </mtd>
       <mtd>
        <mrow>
         <msup>
          <mi>Σ</mi>
          <mrow>
           <mn>2,2</mn></mrow>
         </msup>
         </mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow><mo>=</mo><msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mtable>
        <mtr>
         <mtd>
          <mrow>
           <msub>
            <mi>Σ</mi>
            <mrow>
             <mn>1,1</mn></mrow>
           </msub>
           </mrow>
         </mtd>
         <mtd>
          <mrow>
           <msub>
            <mi>Σ</mi>
            <mrow>
             <mn>1,2</mn></mrow>
           </msub>
           </mrow>
         </mtd>
        </mtr>
        <mtr>
         <mtd>
          <mrow>
           <msub>
            <mi>Σ</mi>
            <mrow>
             <mn>2,1</mn></mrow>
           </msub>
           </mrow>
         </mtd>
         <mtd>
          <mrow>
           <msub>
            <mi>Σ</mi>
            <mrow>
             <mn>2,2</mn></mrow>
           </msub>
           </mrow>
         </mtd>
        </mtr>
        
       </mtable></mrow>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <mo>−</mo><mn>1</mn></mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα στην 1,1, στήλη 2; κεφαλαίο σίγμα στην 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα στην 2,1, στήλη 2; κεφαλαίο σίγμα στην 2,2; ισούται με; τον 2 επί 2 πίνακα; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2; στην μείον 1")?;
    return Ok(());
    //theodora. fails. Now reads: 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα στην 1,1, στήλη 2; κεφαλαίο σίγμα στην 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα στην 2,1, στήλη 2; κεφαλαίο σίγμα στην 2,2; ισούται με; TEMP NAME του 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2; στην μείον 1
}

#[test]
fn uoa_corpus_218() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mtable>
        <mtr>
         <mtd>
          <mi>Α</mi>
         </mtd>
         <mtd>
          <mi>Β</mi>
         </mtd>
        </mtr>
        <mtr>
         <mtd>
          <msup>
           <mi>Β</mi>
           <mo>′</mo>
          </msup>
          
         </mtd>
         <mtd>
          <mi>Γ</mi>
         </mtd>
        </mtr>
        
       </mtable></mrow>
     <mo>)</mo></mrow></mrow>
    <mrow>
     <mo>−</mo><mn>1</mn></mrow>
   </msup>
   <mo>=</mo><mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mrow>
         <mi>α</mi><mi>α</mi></mrow>
       </mtd>
       <mtd>
        <mrow>
         <mi>α</mi><mi>β</mi></mrow>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mrow>
         <mi>β</mi><mi>α</mi></mrow>
       </mtd>
       <mtd>
        <mrow>
         <mi>β</mi><mi>β</mi></mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο άλφα, στήλη 2; κεφαλαίο βήτα; γραμμή 2; στήλη 1; κεφαλαίο βήτα τόνος; στήλη 2; κεφαλαίο γάμμα; στην μείον 1; ισούται με; 2 επί 2 πίνακα; γραμμή 1; στήλη 1; άλφα άλφα, στήλη 2; άλφα βήτα; γραμμή 2; στήλη 1; βήτα άλφα, στήλη 2; βήτα βήτα")?;
    return Ok(());
    //theodora. fails. Now reads: TEMP NAME του 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο άλφα, στήλη 2; κεφαλαίο βήτα; γραμμή 2; στήλη 1; κεφαλαίο βήτα τόνος; στήλη 2; κεφαλαίο γάμμα; στην μείον 1; ισούται με; 2 επί 2 πίνακα; γραμμή 1; στήλη 1; άλφα άλφα, στήλη 2; άλφα βήτα; γραμμή 2; στήλη 1; βήτα άλφα, στήλη 2; βήτα βήτα
}

#[test]
fn uoa_corpus_219() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mrow>
         <msub>
          <mi>Υ</mi>
          <mn>1</mn>
         </msub>
         </mrow>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mrow>
         <msub>
          <mi>Υ</mi>
          <mn>2</mn>
         </msub>
         </mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 επί ένα πίνακας-στήλη; γραμμή 1; κεφαλαίο ύψιλον δείκτης 1; γραμμή 2; κεφαλαίο ύψιλον δείκτης 2")?;
    return Ok(());
}

#[test]
fn uoa_corpus_220() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mrow>
         <msubsup>
          <mi>σ</mi>
          <mn>1</mn>
          <mn>2</mn>
         </msubsup>
         </mrow>
       </mtd>
       <mtd>
        <mi>γ</mi>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mi>γ</mi>
       </mtd>
       <mtd>
        <mrow>
         <msubsup>
          <mi>σ</mi>
          <mn>2</mn>
          <mn>2</mn>
         </msubsup>
         </mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow><msub>
    <mi>Ι</mi>
    <mi>ν</mi>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 επί 2 πίνακας; γραμμή 1; στήλη 1; σίγμα δείκτης 1, στο τετράγωνο, στήλη 2; γάμμα; γραμμή 2; στήλη 1; γάμμα, στήλη 2; σίγμα δείκτης 2, στο τετράγωνο; κεφαλαίο ιότα δείκτης νί")?;
    return Ok(());
}

#[test]
fn uoa_corpus_221() -> Result<()> { // same
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msub><mi>&#x392;</mi><mn>2</mn></msub><mo>=</mo><mfenced><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr></mtable></mfenced><mo>&#x2297;</mo><msub><mi>&#x399;</mi><mi>&#x3BD;</mi></msub></math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο βήτα δείκτης 2; ισούται με; τον 2 επί 2 διαγώνιο πίνακα; στήλη 1; 1; κυκλωμένο επί; κεφαλαίο ιότα δείκτης νί")?;
    return Ok(());
    //theodora. fails. Now reads. κεφαλαίο βήτα δείκτης 2; ισούται με; ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; 1; κυκλωμένο επί; κεφαλαίο ιότα δείκτης νί
}

#[test]
fn uoa_corpus_222() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>Β</mi>
    <mn>2</mn>
   </msub>
   <mo>=</mo><mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mn>0</mn>
       </mtd>
       <mtd>
        <mn>0</mn>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mn>0</mn>
       </mtd>
       <mtd>
        <mn>1</mn>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow><mo>⊗</mo><msub>
    <mi>Ι</mi>
    <mi>ν</mi>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο βήτα δείκτης 2; ισούται με; τον 2 επί 2 διαγώνιο πίνακα; στήλη 2; 1; κυκλωμένο επί; κεφαλαίο ιότα δείκτης νί")?;
    return Ok(());
    //theodora. fails accusative. Now reads: κεφαλαίο βήτα δείκτης 2; ισούται με; ο 2 επί 2 διαγώνιος πίνακας; στήλη 2; 1; κυκλωμένο επί; κεφαλαίο ιότα δείκτης νί
}

#[test]
fn uoa_corpus_223() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>Β</mi>
    <mn>3</mn>
   </msub>
   <mo>=</mo><mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mn>0</mn>
       </mtd>
       <mtd>
        <mn>1</mn>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mn>1</mn>
       </mtd>
       <mtd>
        <mn>0</mn>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow><mo>⊗</mo><msub>
    <mi>Ι</mi>
    <mi>ν</mi>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο βήτα δείκτης 3; ισούται με; 2 επί 2 πίνακα; γραμμή 1; 0, 1; γραμμή 2; 1, 0; κυκλωμένο επί; κεφαλαίο ιότα δείκτης νί")?;
    return Ok(());
    //theodora. fails accusative rule.
}

#[test]
fn uoa_corpus_224() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>Μ</mi>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mtable>
        <mtr>
         <mtd>
          <mi>Χ</mi>
         </mtd>
         <mtd>
          <mn>0</mn>
         </mtd>
        </mtr>
        <mtr>
         <mtd>
          <mn>0</mn>
         </mtd>
         <mtd>
          <mi>Χ</mi>
         </mtd>
        </mtr>
        
       </mtable></mrow>
     <mo>)</mo></mrow></mrow>
   </msub>
   <mo>=</mo><msub>
    <mi>Ι</mi>
    <mn>2</mn>
   </msub>
   <mo>⊗</mo><msub>
    <mi>Μ</mi>
    <mi>Χ</mi>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο μί δείκτης; ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; τέλος δείκτη; ισούται με; κεφαλαίο ιότα δείκτης 2; κυκλωμένο επί; κεφαλαίο μί δείκτης, κεφαλαίο χ")?;
    return Ok(());
    //theodora. fails. Now reads: κεφαλαίο μί δείκτης; TEMP NAME του ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; τέλος δείκτη; ισούται με; κεφαλαίο ιότα δείκτης 2; κυκλωμένο επί; κεφαλαίο μί δείκτης, κεφαλαίο χ
}

#[test]
fn uoa_corpus_225() -> Result<()> { // same
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mi>var</mi><mrow><mo>(</mo><mtable><mtr><mtd><mi>vec</mi><mrow><mo>(</mo><msub><mover accent="true"><munder accentunder="true"><mi>&#x3A5;</mi><mo>_</mo></munder><mo>^</mo></mover><mn>1</mn></msub><mo>)</mo></mrow></mtd></mtr><mtr><mtd><mi>vec</mi><mrow><mo>(</mo><msub><mover accent="true"><munder accentunder="true"><mi>&#x3A5;</mi><mo>_</mo></munder><mo>^</mo></mover><mn>2</mn></msub><mo>)</mo></mrow></mtd></mtr></mtable><mo>)</mo></mrow><mo>=</mo><mrow><mo>(</mo><mtable><mtr><mtd><msub><mi>&#x3A3;</mi><mn>1,1</mn></msub></mtd><mtd><msub><mi>&#x3A3;</mi><mn>1,2</mn></msub></mtd></mtr><mtr><mtd><msub><mi>&#x3A3;</mi><mn>2,1</mn></msub></mtd><mtd><msub><mi>&#x3A3;</mi><mn>2,2</mn></msub></mtd></mtr></mtable><mo>)</mo></mrow><mo>&#x2297;</mo><msub><mi>&#x399;</mi><mi>&#x3BD;</mi></msub></math>"#;
    test("el", "SimpleSpeak", expr, "var του; 2 επί ένα πίνακα-στήλη; γραμμή 1; vec του; ανοίγει παρένθεση; παράσταση κεφαλαίο ύψιλον με γραμμή από κάτω με καπέλο από πάνω δείκτης 1; κλείνει παρένθεση; γραμμή 2; vec του; ανοίγει παρένθεση; παράσταση κεφαλαίο ύψιλον με γραμμή από κάτω με καπέλο από πάνω δείκτης 2; κλείνει παρένθεση; ισούται με; 2 επί 2 πίνακα; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2; κυκλωμένο επί; κεφαλαίο ιότα δείκτης νί")?;
    return Ok(());
    //theodora. fails. should be fixed with genitive. some accusatives fail
    //Now reads: var του; 2 επί ένα πίνακας-στήλη; γραμμή 1; vec του; ανοίγει παρένθεση; παράσταση κεφαλαίο ύψιλον με γραμμή από κάτω με καπέλο από πάνω δείκτης 1; κλείνει παρένθεση; γραμμή 2; vec του; ανοίγει παρένθεση; παράσταση κεφαλαίο ύψιλον με γραμμή από κάτω με καπέλο από πάνω δείκτης 2; κλείνει παρένθεση; ισούται με; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2; κυκλωμένο επί; κεφαλαίο ιότα δείκτης νί
}

#[test]
fn uoa_corpus_226() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>Ι</mi><mo>⊗</mo><mi>ν</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο ιότα, κυκλωμένο επί, νί")?;
    return Ok(());
}

#[test]
fn uoa_corpus_227() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mrow>
         <msub>
          <mi>Υ</mi>
          <mn>1</mn>
         </msub>
         </mrow>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mrow>
         <msub>
          <mi>Υ</mi>
          <mn>2</mn>
         </msub>
         </mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow><mo>∼</mo><mrow><mo>[</mo> <mrow>
    <mrow><mo>(</mo>
     <mrow>
      <mtable>
       <mtr>
        <mtd>
         <mi>Χ</mi>
        </mtd>
        <mtd>
         <mn>0</mn>
        </mtd>
       </mtr>
       <mtr>
        <mtd>
         <mn>0</mn>
        </mtd>
        <mtd>
         <mi>Χ</mi>
        </mtd>
       </mtr>
       
      </mtable></mrow>
    <mo>)</mo></mrow><mrow><mo>(</mo>
     <mrow>
      <mtable>
       <mtr>
        <mtd>
         <mrow>
          <msub>
           <mi>β</mi>
           <mn>1</mn>
          </msub>
          </mrow>
        </mtd>
       </mtr>
       <mtr>
        <mtd>
         <mrow>
          <msub>
           <mi>β</mi>
           <mn>2</mn>
          </msub>
          </mrow>
        </mtd>
       </mtr>
       
      </mtable></mrow>
    <mo>)</mo></mrow><mo>,</mo><mrow><mo>(</mo>
     <mrow>
      <mtable>
       <mtr>
        <mtd>
         <mrow>
          <msub>
           <mi>Σ</mi>
           <mrow>
            <mn>1,1</mn></mrow>
          </msub>
          </mrow>
        </mtd>
        <mtd>
         <mrow>
          <msub>
           <mi>Σ</mi>
           <mrow>
            <mn>1,2</mn></mrow>
          </msub>
          </mrow>
        </mtd>
       </mtr>
       <mtr>
        <mtd>
         <mrow>
          <msub>
           <mi>Σ</mi>
           <mrow>
            <mn>2,1</mn></mrow>
          </msub>
          </mrow>
        </mtd>
        <mtd>
         <mrow>
          <msub>
           <mi>Σ</mi>
           <mrow>
            <mn>2,2</mn></mrow>
          </msub>
          </mrow>
        </mtd>
       </mtr>
       
      </mtable></mrow>
    <mo>)</mo></mrow></mrow> <mo>]</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 επί ένα πίνακας-στήλη; γραμμή 1; κεφαλαίο ύψιλον δείκτης 1; γραμμή 2; κεφαλαίο ύψιλον δείκτης 2; κυματοειδής γραμμή; ανοίγει αγκύλη; ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; 2 επί ένα πίνακας-στήλη; γραμμή 1; βήτα δείκτης 1; γραμμή 2; βήτα δείκτης 2; κόμμα; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2; κλείνει αγκύλη")?;
    return Ok(());
}

#[test]
fn uoa_corpus_228() -> Result<()> { //
    let expr = r#"<math>
 <mtable columnalign="left">
   <mtr>
    <mtd>
     <mrow><mo>(</mo>
      <mrow>
       <mtable>
        <mtr>
         <mtd>
          <mrow>
           <msub>
            <mover accent="true">
             <mi>β</mi>
             <mo>^</mo>
            </mover>
            
            <mn>1</mn>
           </msub>
           </mrow>
         </mtd>
        </mtr>
        <mtr>
         <mtd>
          <mrow>
           <msub>
            <mover accent="true">
             <mi>β</mi>
             <mo>^</mo>
            </mover>
            
            <mn>2</mn>
           </msub>
           </mrow>
         </mtd>
        </mtr>
        
       </mtable></mrow>
     <mo>)</mo></mrow><mo>=</mo><msup>
      <mrow><mo>[</mo> <mrow>
       <mrow><mo>(</mo>
        <mrow>
         <mtable>
          <mtr>
           <mtd>
            <mrow>
             <msub>
              <msup>
               <mi>Χ</mi>
               <mo>′</mo>
              </msup>
              
              <mn>1</mn>
             </msub>
             </mrow>
           </mtd>
           <mtd>
            <mn>0</mn>
           </mtd>
          </mtr>
          <mtr>
           <mtd>
            <mn>0</mn>
           </mtd>
           <mtd>
            <mrow>
             <msub>
              <msup>
               <mi>Χ</mi>
               <mo>′</mo>
              </msup>
              
              <mn>2</mn>
             </msub>
             </mrow>
           </mtd>
          </mtr>
          
         </mtable></mrow>
       <mo>)</mo></mrow><mrow><mo>(</mo>
        <mrow>
         <mtable>
          <mtr>
           <mtd>
            <mrow>
             <msup>
              <mi>Σ</mi>
              <mrow>
               <mn>1,1</mn></mrow>
             </msup>
             </mrow>
           </mtd>
           <mtd>
            <mrow>
             <msup>
              <mi>Σ</mi>
              <mrow>
               <mn>1,2</mn></mrow>
             </msup>
             </mrow>
           </mtd>
          </mtr>
          <mtr>
           <mtd>
            <mrow>
             <msup>
              <mi>Σ</mi>
              <mrow>
               <mn>2,1</mn></mrow>
             </msup>
             </mrow>
           </mtd>
           <mtd>
            <mrow>
             <msup>
              <mi>Σ</mi>
              <mrow>
               <mn>2,1</mn></mrow>
             </msup>
             </mrow>
           </mtd>
          </mtr>
          
         </mtable></mrow>
       <mo>)</mo></mrow><mrow><mo>(</mo>
        <mrow>
         <mtable>
          <mtr>
           <mtd>
            <mrow>
             <msub>
              <mi>Χ</mi>
              <mn>1</mn>
             </msub>
             </mrow>
           </mtd>
           <mtd>
            <mn>0</mn>
           </mtd>
          </mtr>
          <mtr>
           <mtd>
            <mn>0</mn>
           </mtd>
           <mtd>
            <mrow>
             <msub>
              <mi>Χ</mi>
              <mn>2</mn>
             </msub>
             </mrow>
           </mtd>
          </mtr>
          
         </mtable></mrow>
       <mo>)</mo></mrow></mrow> <mo>]</mo></mrow>
      <mrow>
       <mo>−</mo><mn>1</mn></mrow>
     </msup>
     
    </mtd>
   </mtr>
   <mtr>
    <mtd>
     <mo>×</mo><mrow><mo>(</mo>
      <mrow>
       <mtable>
        <mtr>
         <mtd>
          <mrow>
           <msub>
            <msup>
             <mi>Χ</mi>
             <mo>′</mo>
            </msup>
            
            <mn>1</mn>
           </msub>
           </mrow>
         </mtd>
         <mtd>
          <mn>0</mn>
         </mtd>
        </mtr>
        <mtr>
         <mtd>
          <mn>0</mn>
         </mtd>
         <mtd>
          <mrow>
           <msub>
            <msup>
             <mi>Χ</mi>
             <mo>′</mo>
            </msup>
            
            <mn>2</mn>
           </msub>
           </mrow>
         </mtd>
        </mtr>
        
       </mtable></mrow>
     <mo>)</mo></mrow><mrow><mo>(</mo>
      <mrow>
       <mtable>
        <mtr>
         <mtd>
          <mrow>
           <msup>
            <mi>Σ</mi>
            <mrow>
             <mn>1,1</mn></mrow>
           </msup>
           </mrow>
         </mtd>
         <mtd>
          <mrow>
           <msup>
            <mi>Σ</mi>
            <mrow>
             <mn>1,2</mn></mrow>
           </msup>
           </mrow>
         </mtd>
        </mtr>
        <mtr>
         <mtd>
          <mrow>
           <msup>
            <mi>Σ</mi>
            <mrow>
             <mn>2,1</mn></mrow>
           </msup>
           </mrow>
         </mtd>
         <mtd>
          <mrow>
           <msup>
            <mi>Σ</mi>
            <mrow>
             <mn>2,1</mn></mrow>
           </msup>
           </mrow>
         </mtd>
        </mtr>
        
       </mtable></mrow>
     <mo>)</mo></mrow><mrow><mo>(</mo>
      <mrow>
       <mtable>
        <mtr>
         <mtd>
          <mrow>
           <msub>
            <mi>Υ</mi>
            <mn>1</mn>
           </msub>
           </mrow>
         </mtd>
        </mtr>
        <mtr>
         <mtd>
          <mrow>
           <msub>
            <mi>Υ</mi>
            <mn>2</mn>
           </msub>
           </mrow>
         </mtd>
        </mtr>
        
       </mtable></mrow>
     <mo>)</mo></mrow>
    </mtd>
   </mtr>
  </mtable>
  
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 εξισώσεις; εξίσωση 1; 2 επί ένα πίνακας-στήλη; γραμμή 1; βήτα καπέλο, δείκτης 1; γραμμή 2; βήτα καπέλο, δείκτης 2; ισούται με; ανοίγει αγκύλη; ο 2 επί 2 διαγώνιος πίνακας; κεφαλαίο χ τόνος, δείκτης 1; κεφαλαίο χ τόνος, δείκτης 2; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα στην 1,1, στήλη 2; κεφαλαίο σίγμα στην 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα στην 2,1, στήλη 2; κεφαλαίο σίγμα στην 2,1; ο 2 επί 2 διαγώνιος πίνακας; κεφαλαίο χ δείκτης 1; κεφαλαίο χ δείκτης 2; κλείνει αγκύλη στην μείον 1; εξίσωση 2; επί; ο 2 επί 2 διαγώνιος πίνακας; κεφαλαίο χ τόνος, δείκτης 1; κεφαλαίο χ τόνος, δείκτης 2; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα στην 1,1, στήλη 2; κεφαλαίο σίγμα στην 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα στην 2,1, στήλη 2; κεφαλαίο σίγμα στην 2,1; 2 επί ένα πίνακας-στήλη; γραμμή 1; κεφαλαίο ύψιλον δείκτης 1; γραμμή 2; κεφαλαίο ύψιλον δείκτης 2")?;
    return Ok(());
}

#[test]
fn uoa_corpus_229() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>[</mo> <mrow>
    <mrow><mo>(</mo>
     <mrow>
      <mtable>
       <mtr>
        <mtd>
         <mrow>
          <msub>
           <mi>Χ</mi>
           <mn>1</mn>
          </msub>
          </mrow>
        </mtd>
        <mtd>
         <mn>0</mn>
        </mtd>
       </mtr>
       <mtr>
        <mtd>
         <mrow>
          <mo>−</mo><msub>
           <mi>Σ</mi>
           <mrow>
            <mn>2,1</mn></mrow>
          </msub>
          <msubsup>
           <mi>Σ</mi>
           <mrow>
            <mn>1,1</mn></mrow>
           <mrow>
            <mo>−</mo><mn>1</mn></mrow>
          </msubsup>
          <msub>
           <mi>Χ</mi>
           <mn>1</mn>
          </msub>
          </mrow>
        </mtd>
        <mtd>
         <mrow>
          <msub>
           <mi>Χ</mi>
           <mn>2</mn>
          </msub>
          </mrow>
        </mtd>
       </mtr>
       
      </mtable></mrow>
    <mo>)</mo></mrow><mrow><mo>(</mo>
     <mrow>
      <mtable>
       <mtr>
        <mtd>
         <mrow>
          <msub>
           <mi>β</mi>
           <mn>1</mn>
          </msub>
          </mrow>
        </mtd>
       </mtr>
       <mtr>
        <mtd>
         <mrow>
          <msub>
           <mi>β</mi>
           <mn>2</mn>
          </msub>
          </mrow>
        </mtd>
       </mtr>
       
      </mtable></mrow>
    <mo>)</mo></mrow><mo>,</mo><mrow><mo>(</mo>
     <mrow>
      <mtable>
       <mtr>
        <mtd>
         <mrow>
          <msub>
           <mi>Σ</mi>
           <mrow>
            <mn>1,1</mn></mrow>
          </msub>
          </mrow>
        </mtd>
        <mtd>
         <mn>0</mn>
        </mtd>
       </mtr>
       <mtr>
        <mtd>
         <mn>0</mn>
        </mtd>
        <mtd>
         <mrow>
          <msub>
           <mi>Σ</mi>
           <mrow>
            <mn>22.1</mn></mrow>
          </msub>
          </mrow>
        </mtd>
       </mtr>
       
      </mtable></mrow>
    <mo>)</mo></mrow></mrow> <mo>]</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει αγκύλη; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο χ δείκτης 1; στήλη 2; 0; γραμμή 2; στήλη 1; μείον κεφαλαίο σίγμα δείκτης 2,1; κεφαλαίο σίγμα δείκτης 1,1, στην μείον 1; κεφαλαίο χ δείκτης 1; στήλη 2; κεφαλαίο χ δείκτης 2; 2 επί ένα πίνακας-στήλη; γραμμή 1; βήτα δείκτης 1; γραμμή 2; βήτα δείκτης 2; κόμμα; ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 22.1; κλείνει αγκύλη")?;
    return Ok(());
    //theodora. fails. converts 22.1-->221
}

#[test]
fn uoa_corpus_230() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mi>Ι</mi><mn>,0</mn></mrow>
   <mo>)</mo></mrow><mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mrow>
         <msub>
          <msup>
           <mi>Χ</mi>
           <mo>′</mo>
          </msup>
          
          <mn>1</mn>
         </msub>
         <msup>
          <mi>Σ</mi>
          <mrow>
           <mn>1,1</mn></mrow>
         </msup>
         <msub>
          <mi>Χ</mi>
          <mn>1</mn>
         </msub>
         </mrow>
       </mtd>
       <mtd>
        <mrow>
         <msub>
          <msup>
           <mi>Χ</mi>
           <mo>′</mo>
          </msup>
          
          <mn>1</mn>
         </msub>
         <msup>
          <mi>Σ</mi>
          <mrow>
           <mn>1,2</mn></mrow>
         </msup>
         <msub>
          <mi>Χ</mi>
          <mn>2</mn>
         </msub>
         </mrow>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mrow>
         <msub>
          <msup>
           <mi>Χ</mi>
           <mo>′</mo>
          </msup>
          
          <mn>2</mn>
         </msub>
         <msup>
          <mi>Σ</mi>
          <mrow>
           <mn>2,1</mn></mrow>
         </msup>
         <msub>
          <mi>Χ</mi>
          <mn>1</mn>
         </msub>
         </mrow>
       </mtd>
       <mtd>
        <mrow>
         <msub>
          <msup>
           <mi>Χ</mi>
           <mo>′</mo>
          </msup>
          
          <mn>2</mn>
         </msub>
         <msup>
          <mi>Σ</mi>
          <mrow>
           <mn>2,2</mn></mrow>
         </msup>
         <msub>
          <mi>Χ</mi>
          <mn>2</mn>
         </msub>
         </mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow><mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mn>0</mn>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mi>Ι</mi>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση; κεφαλαίο ιότα;0; κλείνει παρένθεση; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο χ τόνος, δείκτης 1; κεφαλαίο σίγμα στην 1,1; κεφαλαίο χ δείκτης 1; στήλη 2; κεφαλαίο χ τόνος, δείκτης 1; κεφαλαίο σίγμα στην 1,2; κεφαλαίο χ δείκτης 2; γραμμή 2; στήλη 1; κεφαλαίο χ τόνος, δείκτης 2; κεφαλαίο σίγμα στην 2,1; κεφαλαίο χ δείκτης 1; στήλη 2; κεφαλαίο χ τόνος, δείκτης 2; κεφαλαίο σίγμα στην 2,2; κεφαλαίο χ δείκτης 2; 2 επί ένα πίνακας-στήλη; 0; κεφαλαίο ιότα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_231() -> Result<()> { //
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML">
  <mrow>
    <mo>(</mo>
    <mtable>
      <mtr>
        <mtd>
          <msub>
            <mover accent="true">
              <mi>β</mi>
              <mo>~</mo>
            </mover>
            <mn>1</mn>
          </msub>
        </mtd>
      </mtr>
      <mtr>
        <mtd>
          <msub>
            <mover accent="true">
              <mi>β</mi>
              <mo>~</mo>
            </mover>
            <mn>2</mn>
          </msub>
        </mtd>
      </mtr>
    </mtable>
    <mo>)</mo>
  </mrow>

  <mo>~</mo>

  <msub>
    <mi>Ν</mi>
    <msup>
      <mn>2</mn>
      <mi>κ</mi>
    </msup>
  </msub>

  <mrow>
    <mo>[</mo>

    <mrow>
      <mo>(</mo>
      <mtable>
        <mtr>
          <mtd>
            <msub>
              <mi>β</mi>
              <mn>1</mn>
            </msub>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
            <msub>
              <mi>β</mi>
              <mn>2</mn>
            </msub>
          </mtd>
        </mtr>
      </mtable>
      <mo>)</mo>
    </mrow>

    <mo>,</mo>

    <mrow>
      <mo>(</mo>
      <mtable>
        <mtr>
          <mtd>
            <mi mathvariant="normal">Var</mi>
            <mrow>
              <mo>(</mo>
              <msub>
                <mover accent="true">
                  <mi>β</mi>
                  <mo>~</mo>
                </mover>
                <mn>1</mn>
              </msub>
              <mo>)</mo>
            </mrow>
          </mtd>

          <mtd>
            <mi mathvariant="normal">cov</mi>
            <mrow>
              <mo>(</mo>
              <msub>
                <mover accent="true">
                  <mi>β</mi>
                  <mo>~</mo>
                </mover>
                <mn>1</mn>
              </msub>
              <mo>,</mo>
              <msub>
                <mover accent="true">
                  <mi>β</mi>
                  <mo>~</mo>
                </mover>
                <mn>2</mn>
              </msub>
              <mo>)</mo>
            </mrow>
          </mtd>
        </mtr>

        <mtr>
          <mtd>
            <mi mathvariant="normal">cov</mi>
            <mrow>
              <mo>(</mo>
              <msub>
                <mover accent="true">
                  <mi>β</mi>
                  <mo>~</mo>
                </mover>
                <mn>2</mn>
              </msub>
              <mo>,</mo>
              <msub>
                <mover accent="true">
                  <mi>β</mi>
                  <mo>~</mo>
                </mover>
                <mn>1</mn>
              </msub>
              <mo>)</mo>
            </mrow>
          </mtd>

          <mtd>
            <mi mathvariant="normal">Var</mi>
            <mrow>
              <mo>(</mo>
              <msub>
                <mover accent="true">
                  <mi>β</mi>
                  <mo>~</mo>
                </mover>
                <mn>2</mn>
              </msub>
              <mo>)</mo>
            </mrow>
          </mtd>
        </mtr>
      </mtable>
      <mo>)</mo>
    </mrow>

    <mo>]</mo>
  </mrow>
</math>"#;
    test("el", "SimpleSpeak", expr, "2 επί ένα πίνακας-στήλη; γραμμή 1; βήτα κυματοειδής γραμμή, δείκτης 1; γραμμή 2; βήτα κυματοειδής γραμμή, δείκτης 2; κυματοειδής γραμμή; κεφαλαίο νί δείκτης, 2 στην καπα οστή τέλος δείκτη; ανοίγει αγκύλη; 2 επί ένα πίνακας-στήλη; γραμμή 1; βήτα δείκτης 1; γραμμή 2; βήτα δείκτης 2; κόμμα; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; Var του; ανοίγει παρένθεση; βήτα κυματοειδής γραμμή, δείκτης 1; κλείνει παρένθεση, στήλη 2; cov; ανοίγει παρένθεση; βήτα κυματοειδής γραμμή, δείκτης 1; κόμμα; βήτα κυματοειδής γραμμή, δείκτης 2; κλείνει παρένθεση; γραμμή 2; στήλη 1; cov; ανοίγει παρένθεση; βήτα κυματοειδής γραμμή, δείκτης 2; κόμμα; βήτα κυματοειδής γραμμή, δείκτης 1; κλείνει παρένθεση, στήλη 2; Var του; ανοίγει παρένθεση; βήτα κυματοειδής γραμμή, δείκτης 2; κλείνει παρένθεση; κλείνει αγκύλη")?;
    return Ok(());
}

#[test]
fn uoa_corpus_232() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mrow>
         <msub>
          <mi>Υ</mi>
          <mn>1</mn>
         </msub>
         </mrow>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mrow>
         <msub>
          <mi>Υ</mi>
          <mn>2</mn>
         </msub>
         </mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow><mo>∼</mo><msub>
    <mi>Ν</mi>
    <mrow>
     <mn>2</mn><mi>ν</mi></mrow>
   </msub>
   <mrow><mo>[</mo> <mrow>
    <mrow><mo>(</mo>
     <mrow>
      <msub>
       <mi>Ι</mi>
       <mn>2</mn>
      </msub>
      <mo>⊗</mo><mi>Χ</mi></mrow>
    <mo>)</mo></mrow><mrow><mo>(</mo>
     <mrow>
      <mtable>
       <mtr>
        <mtd>
         <mrow>
          <msub>
           <mi>β</mi>
           <mn>1</mn>
          </msub>
          </mrow>
        </mtd>
       </mtr>
       <mtr>
        <mtd>
         <mrow>
          <msub>
           <mi>β</mi>
           <mn>2</mn>
          </msub>
          </mrow>
        </mtd>
       </mtr>
       
      </mtable></mrow>
    <mo>)</mo></mrow><mo>,</mo><mrow><mo>(</mo>
     <mrow>
      <mtable>
       <mtr>
        <mtd>
         <mrow>
          <msubsup>
           <mi>σ</mi>
           <mn>1</mn>
           <mn>2</mn>
          </msubsup>
          </mrow>
        </mtd>
        <mtd>
         <mi>γ</mi>
        </mtd>
       </mtr>
       <mtr>
        <mtd>
         <mi>γ</mi>
        </mtd>
        <mtd>
         <mrow>
          <msubsup>
           <mi>σ</mi>
           <mn>2</mn>
           <mn>2</mn>
          </msubsup>
          </mrow>
        </mtd>
       </mtr>
       
      </mtable></mrow>
    <mo>)</mo></mrow><mo>⊗</mo><msub>
     <mi>Ι</mi>
     <mi>ν</mi>
    </msub>
    </mrow> <mo>]</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 επί ένα πίνακας-στήλη; γραμμή 1; κεφαλαίο ύψιλον δείκτης 1; γραμμή 2; κεφαλαίο ύψιλον δείκτης 2; κυματοειδής γραμμή; κεφαλαίο νί δείκτης, 2 νί τέλος δείκτη; ανοίγει αγκύλη; ανοίγει παρένθεση; κεφαλαίο ιότα δείκτης 2; κυκλωμένο επί, κεφαλαίο χ; κλείνει παρένθεση; 2 επί ένα πίνακας-στήλη; γραμμή 1; βήτα δείκτης 1; γραμμή 2; βήτα δείκτης 2; κόμμα; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; σίγμα δείκτης 1, στο τετράγωνο, στήλη 2; γάμμα; γραμμή 2; στήλη 1; γάμμα, στήλη 2; σίγμα δείκτης 2, στο τετράγωνο; κυκλωμένο επί; κεφαλαίο ιότα δείκτης νί; κλείνει αγκύλη")?;
    return Ok(());
}

#[test]
fn uoa_corpus_233() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mrow>
         <msubsup>
          <mi>σ</mi>
          <mrow>
           <mn>2,0</mn></mrow>
          <mn>4</mn>
         </msubsup>
         </mrow>
       </mtd>
       <mtd>
        <mrow>
         <msubsup>
          <mi>γ</mi>
          <mn>0</mn>
          <mn>2</mn>
         </msubsup>
         </mrow>
       </mtd>
       <mtd>
        <mrow>
         <mo>−</mo><mn>2</mn><msub>
          <mi>γ</mi>
          <mn>0</mn>
         </msub>
         <msubsup>
          <mi>σ</mi>
          <mrow>
           <mn>2,0</mn></mrow>
          <mn>2</mn>
         </msubsup>
         </mrow>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mrow>
         <msubsup>
          <mi>γ</mi>
          <mn>0</mn>
          <mn>2</mn>
         </msubsup>
         </mrow>
       </mtd>
       <mtd>
        <mrow>
         <msubsup>
          <mi>σ</mi>
          <mrow>
           <mn>1,0</mn></mrow>
          <mn>4</mn>
         </msubsup>
         </mrow>
       </mtd>
       <mtd>
        <mrow>
         <mo>−</mo><mn>2</mn><msub>
          <mi>γ</mi>
          <mn>0</mn>
         </msub>
         <msubsup>
          <mi>σ</mi>
          <mrow>
           <mn>1,0</mn></mrow>
          <mn>2</mn>
         </msubsup>
         </mrow>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mrow>
         <mo>−</mo><mn>2</mn><msub>
          <mi>γ</mi>
          <mn>0</mn>
         </msub>
         <msubsup>
          <mi>σ</mi>
          <mrow>
           <mn>2,0</mn></mrow>
          <mn>2</mn>
         </msubsup>
         </mrow>
       </mtd>
       <mtd>
        <mrow>
         <mo>−</mo><mn>2</mn><msub>
          <mi>γ</mi>
          <mn>0</mn>
         </msub>
         <msubsup>
          <mi>σ</mi>
          <mrow>
           <mn>1,0</mn></mrow>
          <mn>2</mn>
         </msubsup>
         </mrow>
       </mtd>
       <mtd>
        <mrow>
         <mn>2</mn><mrow><mo>(</mo>
          <mrow>
           <msubsup>
            <mi>σ</mi>
            <mrow>
             <mn>1,0</mn></mrow>
            <mn>2</mn>
           </msubsup>
           <mo>,</mo><msubsup>
            <mi>σ</mi>
            <mrow>
             <mn>2,0</mn></mrow>
            <mn>2</mn>
           </msubsup>
           <mo>+</mo><msubsup>
            <mi>γ</mi>
            <mn>0</mn>
            <mn>2</mn>
           </msubsup>
           </mrow>
         <mo>)</mo></mrow></mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "3 επί 3 πίνακας; γραμμή 1; στήλη 1; σίγμα δείκτης 2,0, στην τέταρτη, στήλη 2; γάμμα δείκτης 0, στο τετράγωνο, στήλη 3; μείον 2, γάμμα δείκτης 0; σίγμα δείκτης 2,0, στο τετράγωνο; γραμμή 2; στήλη 1; γάμμα δείκτης 0, στο τετράγωνο, στήλη 2; σίγμα δείκτης 1,0, στην τέταρτη, στήλη 3; μείον 2, γάμμα δείκτης 0; σίγμα δείκτης 1,0, στο τετράγωνο; γραμμή 3; στήλη 1; μείον 2, γάμμα δείκτης 0; σίγμα δείκτης 2,0, στο τετράγωνο, στήλη 2; μείον 2, γάμμα δείκτης 0; σίγμα δείκτης 1,0, στο τετράγωνο, στήλη 3; 2; ανοίγει παρένθεση; σίγμα δείκτης 1,0, στο τετράγωνο; κόμμα; σίγμα δείκτης 2,0, στο τετράγωνο; συν, γάμμα δείκτης 0, στο τετράγωνο; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_234() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mrow><mo>{</mo> <mrow>
      <msub>
       <mi>Μ</mi>
       <mrow>
        <mrow><mo>(</mo>
         <mrow>
          <mtable>
           <mtr>
            <mtd>
             <mi>Χ</mi>
            </mtd>
            <mtd>
             <mn>0</mn>
            </mtd>
           </mtr>
           <mtr>
            <mtd>
             <mn>0</mn>
            </mtd>
            <mtd>
             <mi>Χ</mi>
            </mtd>
           </mtr>
           
          </mtable></mrow>
        <mo>)</mo></mrow></mrow>
      </msub>
      <mrow><mo>[</mo> <mrow>
       <mrow><mo>(</mo>
        <mrow>
         <mtable>
          <mtr>
           <mtd>
            <mrow>
             <msubsup>
              <mi>σ</mi>
              <mrow>
               <mn>1,0</mn></mrow>
              <mn>2</mn>
             </msubsup>
             </mrow>
           </mtd>
           <mtd>
            <mrow>
             <msub>
              <mi>γ</mi>
              <mn>0</mn>
             </msub>
             </mrow>
           </mtd>
          </mtr>
          <mtr>
           <mtd>
            <mrow>
             <msub>
              <mi>γ</mi>
              <mn>0</mn>
             </msub>
             </mrow>
           </mtd>
           <mtd>
            <mrow>
             <msubsup>
              <mi>σ</mi>
              <mrow>
               <mn>2,0</mn></mrow>
              <mn>2</mn>
             </msubsup>
             </mrow>
           </mtd>
          </mtr>
          
         </mtable></mrow>
       <mo>)</mo></mrow><mo>⊗</mo><msub>
        <mi>Ι</mi>
        <mi>ν</mi>
       </msub>
       </mrow> <mo>]</mo></mrow><msub>
       <mi>Μ</mi>
       <mrow>
        <mrow><mo>(</mo>
         <mrow>
          <mtable>
           <mtr>
            <mtd>
             <mi>Χ</mi>
            </mtd>
            <mtd>
             <mn>0</mn>
            </mtd>
           </mtr>
           <mtr>
            <mtd>
             <mn>0</mn>
            </mtd>
            <mtd>
             <mi>Χ</mi>
            </mtd>
           </mtr>
           
          </mtable></mrow>
        <mo>)</mo></mrow></mrow>
      </msub>
      </mrow> <mo>}</mo></mrow></mrow>
    <mo>+</mo>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει άγκιστρο ο κεφαλαίο μί δείκτης; ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; τέλος δείκτη; ανοίγει αγκύλη; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; σίγμα δείκτης 1,0, στο τετράγωνο, στήλη 2; γάμμα δείκτης 0; γραμμή 2; στήλη 1; γάμμα δείκτης 0; στήλη 2; σίγμα δείκτης 2,0, στο τετράγωνο; κυκλωμένο επί; κεφαλαίο ιότα δείκτης νί; κλείνει αγκύλη; κεφαλαίο μί δείκτης; ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; τέλος δείκτη, κλείνει άγκιστρο εκθέτης συν")?;
    return Ok(());
    //theodora. fails. Now reads: το σύνολο κεφαλαίο μί δείκτης; TEMP NAME του ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; τέλος δείκτη; ανοίγει αγκύλη; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; σίγμα δείκτης 1,0, στο τετράγωνο, στήλη 2; γάμμα δείκτης 0; γραμμή 2; στήλη 1; γάμμα δείκτης 0; στήλη 2; σίγμα δείκτης 2,0, στο τετράγωνο; κυκλωμένο επί; κεφαλαίο ιότα δείκτης νί; κλείνει αγκύλη; κεφαλαίο μί δείκτης; TEMP NAME του ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; τέλος δείκτη, εκθέτης συν
}

#[test]
fn uoa_corpus_235() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mtable columnalign="left">
     <mtr>
      <mtd>
       <msup>
        <mi>η</mi>
        <mo>′</mo>
       </msup>
       <msup>
        <mrow><mo>(</mo>
         <mrow>
          <msub>
           <mi>Μ</mi>
           <mi>Α</mi>
          </msub>
          <msub>
           <mi>Σ</mi>
           <mn>0</mn>
          </msub>
          <msub>
           <mi>Μ</mi>
           <mi>Α</mi>
          </msub>
          </mrow>
        <mo>)</mo></mrow>
        <mo>+</mo>
       </msup>
       <msub>
        <mi>Β</mi>
        <mn>1</mn>
       </msub>
       <msup>
        <mrow><mo>(</mo>
         <mrow>
          <msub>
           <mi>Μ</mi>
           <mi>Α</mi>
          </msub>
          <msub>
           <mi>Σ</mi>
           <mn>0</mn>
          </msub>
          <msub>
           <mi>Μ</mi>
           <mi>Α</mi>
          </msub>
          </mrow>
        <mo>)</mo></mrow>
        <mo>+</mo>
       </msup>
       <mi>η</mi>
      </mtd>
     </mtr>
     <mtr>
      <mtd>
       <mo>⋮</mo>
      </mtd>
     </mtr>
     <mtr>
      <mtd>
       <msup>
        <mi>η</mi>
        <mo>′</mo>
       </msup>
       <msup>
        <mrow><mo>(</mo>
         <mrow>
          <msub>
           <mi>Μ</mi>
           <mi>Α</mi>
          </msub>
          <msub>
           <mi>Σ</mi>
           <mn>0</mn>
          </msub>
          <msub>
           <mi>Μ</mi>
           <mi>Α</mi>
          </msub>
          </mrow>
        <mo>)</mo></mrow>
        <mo>+</mo>
       </msup>
       <msub>
        <mi>Β</mi>
        <mi>π</mi>
       </msub>
       <msup>
        <mrow><mo>(</mo>
         <mrow>
          <msub>
           <mi>Μ</mi>
           <mi>Α</mi>
          </msub>
          <msub>
           <mi>Σ</mi>
           <mn>0</mn>
          </msub>
          <msub>
           <mi>Μ</mi>
           <mi>Α</mi>
          </msub>
          </mrow>
        <mo>)</mo></mrow>
        <mo>+</mo>
       </msup>
       <mi>η</mi>
      </mtd>
     </mtr>
    </mtable>
    
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "3 επί ένα πίνακας-στήλη; γραμμή 1; ήτα τόνος; ανοίγει παρένθεση; κεφαλαίο μί δείκτης, κεφαλαίο άλφα; κεφαλαίο σίγμα δείκτης 0; κεφαλαίο μί δείκτης, κεφαλαίο άλφα; κλείνει παρένθεση εκθέτης συν; κεφαλαίο βήτα δείκτης 1; ανοίγει παρένθεση; κεφαλαίο μί δείκτης, κεφαλαίο άλφα; κεφαλαίο σίγμα δείκτης 0; κεφαλαίο μί δείκτης, κεφαλαίο άλφα; κλείνει παρένθεση εκθέτης συν; ήτα; γραμμή 2; κατακόρυφα αποσιωπητικά; γραμμή 3; ήτα τόνος; ανοίγει παρένθεση; κεφαλαίο μί δείκτης, κεφαλαίο άλφα; κεφαλαίο σίγμα δείκτης 0; κεφαλαίο μί δείκτης, κεφαλαίο άλφα; κλείνει παρένθεση εκθέτης συν; κεφαλαίο βήτα δείκτης π; ανοίγει παρένθεση; κεφαλαίο μί δείκτης, κεφαλαίο άλφα; κεφαλαίο σίγμα δείκτης 0; κεφαλαίο μί δείκτης, κεφαλαίο άλφα; κλείνει παρένθεση εκθέτης συν; ήτα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_236() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>Κ</mi><mo>=</mo><mrow><mo>(</mo>
    <mrow>
     <msub>
      <mi>φ</mi>
      <mn>1</mn>
     </msub>
     <mo>,</mo><mo>…</mo><mo>,</mo><msub>
      <mi>φ</mi>
      <mrow>
       <mi>ρ</mi><mrow><mo>(</mo>
        <mi>Τ</mi>
       <mo>)</mo></mrow></mrow>
     </msub>
     </mrow>
   <mo>)</mo></mrow><mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mrow>
         <mfrac>
          <mn>1</mn>
          <mrow>
           <msqrt>
            <mrow>
             <msub>
              <mi>λ</mi>
              <mn>1</mn>
             </msub>
             </mrow>
           </msqrt>
           </mrow>
         </mfrac>
         </mrow>
       </mtd>
       <mtd>
        <mo>…</mo>
       </mtd>
       <mtd>
        <mn>0</mn>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mo>⋮</mo>
       </mtd>
       <mtd>
        <mo>⋱</mo>
       </mtd>
       <mtd>
        <mo>⋮</mo>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mn>0</mn>
       </mtd>
       <mtd>
        <mo>⋯</mo>
       </mtd>
       <mtd>
        <mrow>
         <mfrac>
          <mn>1</mn>
          <mrow>
           <msqrt>
            <mrow>
             <msub>
              <mi>λ</mi>
              <mrow>
               <mi>ρ</mi><mrow><mo>(</mo>
                <mi>Τ</mi>
               <mo>)</mo></mrow></mrow>
             </msub>
             </mrow>
           </msqrt>
           </mrow>
         </mfrac>
         </mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο καπα, ισούται με; ανοίγει παρένθεση; φ δείκτης 1; κόμμα; αποσιωπητικά, κόμμα; φ δείκτης, ρ του κεφαλαίο τάαφ; κλείνει παρένθεση; 3 επί 3 πίνακας; γραμμή 1; στήλη 1; κλάσμα, 1 προς, η τετραγωνική ρίζα του λάμδα δείκτης 1; τέλος ρίζας; τέλος κλάσματος; στήλη 2; αποσιωπητικά, στήλη 3; 0; γραμμή 2; στήλη 1; κατακόρυφα αποσιωπητικά, στήλη 2; διαγώνια αποσιωπητικά προς τα κάτω δεξιά, στήλη 3; κατακόρυφα αποσιωπητικά; γραμμή 3; στήλη 1; 0, στήλη 2; αποσιωπητικά, στήλη 3; κλάσμα, 1 προς, η τετραγωνική ρίζα του λάμδα δείκτης, ρ του κεφαλαίο τάαφ; τέλος ρίζας; τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_237() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mi>Σ</mi>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mi>γ</mi>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mrow>
         <mi>τ</mi><mo stretchy="false">(</mo><mi>γ</mi><mo stretchy="false">)</mo></mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow><mo>=</mo><mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mn>1</mn>
       </mtd>
       <mtd>
        <mn>1</mn>
       </mtd>
       <mtd>
        <mn>1</mn>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mn>1</mn>
       </mtd>
       <mtd>
        <mrow>
         <msup>
          <mi>ζ</mi>
          <mn>2</mn>
         </msup>
         </mrow>
       </mtd>
       <mtd>
        <mi>ζ</mi>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mn>1</mn>
       </mtd>
       <mtd>
        <mi>ζ</mi>
       </mtd>
       <mtd>
        <mrow>
         <msup>
          <mi>ζ</mi>
          <mn>2</mn>
         </msup>
         </mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mi>θ</mi>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mrow>
         <mi>σ</mi><mo stretchy="false">(</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mrow>
         <msup>
          <mi>σ</mi>
          <mn>2</mn>
         </msup>
         <mo stretchy="false">(</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "3 επί ένα πίνακας-στήλη; κεφαλαίο σίγμα; γάμμα; τάαφ του γάμμα; ισούται με; 3 επί 3 πίνακας; γραμμή 1; στήλη 1; 1, στήλη 2; 1, στήλη 3; 1; γραμμή 2; στήλη 1; 1, στήλη 2; ζήτα στο τετράγωνο, στήλη 3; ζήτα; γραμμή 3; στήλη 1; 1, στήλη 2; ζήτα, στήλη 3; ζήτα στο τετράγωνο; επί; 3 επί ένα πίνακας-στήλη; γραμμή 1; θήτα; γραμμή 2; σίγμα του θήτα; γραμμή 3; σίγμα στο τετράγωνο, του θήτα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_238() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>Τ</mi><mo>∈</mo><mrow><mo>(</mo>
    <mrow>
     <mn>0,</mn><mi>∞</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο τάαφ, ανήκει; ανοίγει παρένθεση, 0, άπειρο, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_239() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>ω</mi><mo>=</mo><mrow><mo>(</mo>
    <mrow>
     <msub>
      <mi>ω</mi>
      <mn>1</mn>
     </msub>
     <mo>,</mo><mo>…</mo><mo>,</mo><msub>
      <mi>ω</mi>
      <mrow>
       <mi>ν</mi><mo>−</mo><mn>1</mn></mrow>
     </msub>
     </mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ωμέγα ισούται με; ανοίγει παρένθεση; ωμέγα δείκτης 1; κόμμα; αποσιωπητικά, κόμμα; ωμέγα δείκτης, νί μείον 1 τέλος δείκτη; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_240() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>Α</mi>
    <mi>∞</mi>
   </msub>
   <mo>=</mo><mi>Α</mi><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>0,</mn><mi>∞</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο άλφα δείκτης άπειρο; ισούται με; κεφαλαίο άλφα, επί; ανοίγει παρένθεση, 0, άπειρο, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_241() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>Β</mi>
    <mi>χ</mi>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <mn>0,</mn><mi>∞</mi><mo>;</mo><mi>ℂ</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο βήτα στην χ οστή; ανοίγει παρένθεση; 0, άπειρο; ελληνικό ερωτηματικό; οι μιγαδικοί αριθμοί; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_242() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>ℝ</mi>
    <mi>δ</mi>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "οι πραγματικοί αριθμοί στην δέλτα οστή")?;
    return Ok(());
}

#[test]
fn uoa_corpus_243() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>δ</mi><mo>∈</mo><mi>ℕ</mi><mo>\</mo><mrow><mo>{</mo> <mn>0</mn> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "δέλτα ανήκει; στους φυσικούς αριθμούς; ανάστροφη κάθετος, το σύνολο 0")?;
    return Ok(());
    //theodora. fails. Now reads: δέλτα ανήκει; οι φυσικοί αριθμοί; ανάστροφη κάθετος, το σύνολο 0

}

#[test]
fn uoa_corpus_244() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>σ</mi><mo>⊂</mo><mo>∂</mo><mi>Ω</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "σίγμα, είναι ένα υποσύνολο του; μερικό διαφορικό, κεφαλαίο ωμέγα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_245() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>[</mo> <mrow>
    <msub>
     <mi>χ</mi>
     <mi>κ</mi>
    </msub>
    <mo>,</mo><mi>χ</mi></mrow> <mo>]</mo></mrow><mo>⊂</mo><mi>Κ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το κλειστό διάστημα από χ δείκτης καπα, ως χ; είναι ένα υποσύνολο του; κεφαλαίο καπα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_246() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>χ</mi><mo>∈</mo><mover accent="true">
    <mi>Ω</mi>
    <mo>¯</mo>
   </mover>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ ανήκει, κεφαλαίο ωμέγα παύλα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_247() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>Μ</mi>
    <mi>σ</mi>
   </msub>
   <mo>=</mo><mrow><mo>{</mo> <mrow>
    <mi>Κ</mi><mo>,</mo><mi>Λ</mi></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο μί δείκτης σίγμα; ισούται με; το σύνολο κεφαλαίο καπα, κόμμα; κεφαλαίο λάμδα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_248() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>Γ</mi>
    <mrow>
     <mi>∞</mi><mo>,</mo><mi>α</mi></mrow>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <mo>∂</mo><mi>Ω</mi><mo>×</mo><mrow><mo>[</mo> <mrow>
      <mn>0,</mn><mi>∞</mi></mrow> <mo>]</mo></mrow></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο γάμμα που υψώνεται στη άπειρο κόμμα, άλφα τέλος δύναμης; ανοίγει παρένθεση; μερικό διαφορικό, κεφαλαίο ωμέγα; επί; ανοίγει αγκύλη, 0, άπειρο, κλείνει αγκύλη; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_249() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>χ</mi><mo>∈</mo><mo>∂</mo><mi>Ω</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ ανήκει, μερικό διαφορικό, κεφαλαίο ωμέγα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_250() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>{</mo> <mrow>
    <msubsup>
     <mi>β</mi>
     <mi>σ</mi>
     <mi>Κ</mi>
    </msubsup>
    <mo>:</mo><mi>σ</mi><mo>∈</mo><mi>Β</mi><mo>,</mo><mi>Κ</mi><mo>∈</mo><mi>Μ</mi></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το σύνολο βήτα δείκτης σίγμα, στην κεφαλαίο καπα οστή; άνω κάτω τελεία; σίγμα που ανήκει, κεφαλαίο βήτα; κόμμα; κεφαλαίο καπα, που ανήκει, κεφαλαίο μί")?;
    return Ok(());
}

#[test]
fn uoa_corpus_251() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>{</mo> <mrow>
    <msubsup>
     <mi>Κ</mi>
     <mi>Κ</mi>
     <mi>ν</mi>
    </msubsup>
    <mo>:</mo><mi>ν</mi><mo>∈</mo><mrow><mo>〚</mo><mrow>
     <mn>0,</mn><mi>Ν</mi><mo>+</mo><mn>1</mn></mrow><mo>〛</mo></mrow><mo>,</mo><mi>Κ</mi><mo>∈</mo><mi>Μ</mi></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "το σύνολο κεφαλαίο καπα δείκτης, κεφαλαίο καπα, στην νί οστή; άνω κάτω τελεία; νί που ανήκει; αριστερή λευκή τετραγωνική αγκύλη; 0, κεφαλαίο νί, συν 1; δεξιά λευκή τετραγωνική αγκύλη; κόμμα; κεφαλαίο καπα, που ανήκει, κεφαλαίο μί")?;
    return Ok(());
}

#[test]
fn uoa_corpus_252() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>τ</mi>
    <mn>0</mn>
   </msub>
   <mo>∈</mo><mrow><mo>[</mo> <mrow>
    <mn>0,</mn><mi>∞</mi></mrow> <mo>]</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "τάαφ δείκτης 0; ανήκει; ανοίγει αγκύλη, 0, άπειρο, κλείνει αγκύλη")?;
    return Ok(());
}

#[test]
fn uoa_corpus_253() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>υ</mi>
    <mn>0</mn>
   </msub>
   <mo>∈</mo><msup>
    <mi>Λ</mi>
    <mn>2</mn>
   </msup>
   <mrow><mo>(</mo>
    <mi>Ω</mi>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ύψιλον δείκτης 0; ανήκει; κεφαλαίο λάμδα στο τετράγωνο; του κεφαλαίο ωμέγα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_254() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>φ</mi>
    <mn>0</mn>
   </msub>
   <mo>∈</mo><msup>
    <mover accent="true">
     <mi>Γ</mi>
     <mo>˙</mo>
    </mover>
    
    <mi>∞</mi>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <msub>
      <mi>Υ</mi>
      <mn>0</mn>
     </msub>
     </mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "φ δείκτης 0; ανήκει; η άπειρο δύναμη του, κεφαλαίο γάμμα τελεία; του; ανοίγει παρένθεση; κεφαλαίο ύψιλον δείκτης 0; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_255() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>Η</mi>
    <mn>2</mn>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <msup>
      <mi>ε</mi>
      <mrow>
       <mo>−</mo><mi>γ</mi><mo>×</mo><mi>τ</mi></mrow>
     </msup>
     <mo>,</mo><msub>
      <mi>Ω</mi>
      <mi>∞</mi>
     </msub>
     </mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο ήτα στο τετράγωνο; ανοίγει παρένθεση, έψιλον που υψώνεται στη μείον γάμμα, επί τάαφ τέλος δύναμης, κόμμα, κεφαλαίο ωμέγα δείκτης άπειρο, κλείνει παρένθεση")?;
    return Ok(());
    //theodora. fails with current rules. Now reads: κεφαλαίο ήτα στο τετράγωνο; το ανοιχτό διάστημα από έψιλον που υψώνεται στη μείον γάμμα, επί τάαφ τέλος δύναμης, ως κεφαλαίο ωμέγα δείκτης άπειρο
}

#[test]
fn uoa_corpus_256() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>Ω</mi>
    <mi>ρ</mi>
   </msup>
   <mo>:</mo><mo>=</mo><mrow><mo>{</mo> <mrow>
    <mi>χ</mi><mo>∈</mo><mi>Ω</mi><mo>:</mo><mfrac>
     <mi>ρ</mi>
     <mn>2</mn>
    </mfrac>
    <mo>&lt;</mo><mrow><mo>|</mo> <mi>χ</mi> <mo>|</mo></mrow><mo>&lt;</mo><mn>2</mn><mi>ρ</mi><mo>,</mo><mi>ρ</mi><mo>&gt;</mo><mn>0</mn></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο ωμέγα στην ρ οστή; άνω κάτω τελεία; ισούται με; το σύνολο χ που ανήκει, κεφαλαίο ωμέγα; άνω κάτω τελεία; ρ προς 2; είναι μικρότερο από; την απόλυτη τιμή του χ; είναι μικρότερο από, 2 ρ; κόμμα; ρ είναι μεγαλύτερο από 0")?;
    return Ok(());
    //theodora. fails. 
}

#[test]
fn uoa_corpus_257() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>Κ</mi><mo>=</mo><mrow><mo>{</mo> <mrow>
    <mi>χ</mi><mo>=</mo><mrow><mo>(</mo>
     <mrow>
      <msub>
       <mi>χ</mi>
       <mn>1</mn>
      </msub>
      <mo>,</mo><msub>
       <mi>χ</mi>
       <mn>2</mn>
      </msub>
      </mrow>
    <mo>)</mo></mrow><mo>∈</mo><msup>
     <mi>ℝ</mi>
     <mn>2</mn>
    </msup>
    <mo>:</mo><mi>ρ</mi><mo>&gt;</mo><mn>0,0</mn><mo>&lt;</mo><mi>ω</mi><mo>&lt;</mo><msub>
     <mi>ω</mi>
     <mn>0</mn>
    </msub>
    </mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο καπα, ισούται με; το σύνολο όλων των χ ισούται με; ανοίγει παρένθεση; χ δείκτης 1; κόμμα; χ δείκτης 2; κλείνει παρένθεση; που ανήκει r 2 τέτοια ώστε ρ είναι μεγαλύτερο από; 0,0 είναι μικρότερο από, ωμέγα, είναι μικρότερο από; ωμέγα δείκτης 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_258() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>Α</mi><mo>=</mo><mrow><mo>{</mo> <mrow>
    <mi>τ</mi><mo>∈</mo><mrow><mo>[</mo> <mrow>
     <mn>0,</mn><mi>Τ</mi></mrow> <mo>]</mo></mrow><mo>:</mo><mrow><mo>|</mo> <mrow>
     <mover accent="true">
      <mi>υ</mi>
      <mo>˙</mo>
     </mover>
     <mrow><mo>(</mo>
      <mi>τ</mi>
     <mo>)</mo></mrow></mrow> <mo>|</mo></mrow><mo>≥</mo><mn>1</mn></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο άλφα, ισούται με; το σύνολο όλων των τάαφ που ανήκει; ανοίγει αγκύλη; 0, κεφαλαίο τάαφ; κλείνει αγκύλη τέτοια ώστε η απόλυτη τιμή του ύψιλον τελεία; του τάαφ τέλος απόλυτης τιμής; είναι μεγαλύτερο από ή ίσο με 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_259() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>η</mi><mo>:</mo><mrow><mo>(</mo>
    <mrow>
     <mn>0,1</mn></mrow>
   <mo>)</mo></mrow><mo>→</mo><mrow><mo>[</mo> <mrow>
    <mn>0,</mn><mi>∞</mi></mrow> <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ήτα άνω κάτω τελεία; 0,1 βέλος προς τα δεξιά; ανοίγει αγκύλη, 0, άπειρο, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_260() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>φ</mi><mo>:</mo><mrow><mo>(</mo>
    <mrow>
     <mn>0,1</mn></mrow>
   <mo>)</mo></mrow><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>0,</mn><mi>∞</mi></mrow>
   <mo>)</mo></mrow><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>0,</mn><mi>∞</mi></mrow>
   <mo>)</mo></mrow><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mo>−</mo><mi>∞</mi><mn>,0</mn></mrow>
   <mo>)</mo></mrow><mo>→</mo><mrow><mo>[</mo> <mrow>
    <mn>0,</mn><mi>∞</mi></mrow> <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "φ άνω κάτω τελεία; 0,1 επί; ανοίγει παρένθεση, 0, άπειρο, κλείνει παρένθεση; επί; ανοίγει παρένθεση, 0, άπειρο, κλείνει παρένθεση; επί; ανοίγει παρένθεση; μείον άπειρο;0; κλείνει παρένθεση; βέλος προς τα δεξιά; ανοίγει αγκύλη, 0, άπειρο, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_261() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>Γ</mi>
    <mn>2</mn>
   </msup>
   <mrow><mo>[</mo> <mrow>
    <mn>0,1</mn></mrow> <mo>]</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο γάμμα στο τετράγωνο; του 0,1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_262() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>Κ</mi><mrow><mo>(</mo>
    <mi>ρ</mi>
   <mo>)</mo></mrow><mo>=</mo><mrow><mo>{</mo> <mrow>
    <mi>υ</mi><mo>∈</mo><mi>Κ</mi><mo>:</mo><mrow><mo>|</mo> <mrow>
     <mrow><mo>‖</mo> <mi>υ</mi> <mo>‖</mo></mrow></mrow> <mo>|</mo></mrow><mo>&lt;</mo><mi>ρ</mi></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο καπα, του ρ; ισούται με; το σύνολο όλων των ύψιλον που ανήκει, κεφαλαίο καπα τέτοια ώστε η απόλυτη τιμή του νόρμα του ύψιλον τέλος απόλυτης τιμής; είναι μικρότερο από ρ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_263() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>χ</mi><mo>∈</mo><msup>
    <mi>ℝ</mi>
    <mi>ν</mi>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ ανήκει, στους πραγματικούς αριθμούς στην νί οστή")?;
    return Ok(());
  //theodora. fails with current rules. Should be fixed with genitive
}

#[test]
fn uoa_corpus_264() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>α</mi><mo>∈</mo><mrow><mo>[</mo> <mrow>
    <mn>0,</mn><msup>
     <mi>π</mi>
     <mo>−</mo>
    </msup>
    <mo>−</mo><mn>1</mn></mrow> <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άλφα ανήκει; ανοίγει αγκύλη; 0; π εκθέτης μείον; μείον 1; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_265() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>υ</mi><mo>∈</mo><mover accent="true">
    <mrow>
     <mi>Κ</mi><mrow><mo>(</mo>
      <mrow>
       <msub>
        <mi>ρ</mi>
        <mn>2</mn>
       </msub>
       </mrow>
     <mo>)</mo></mrow></mrow>
    <mo stretchy="true">¯</mo>
   </mover>
   <mo>\</mo><mi>Κ</mi><mrow><mo>(</mo>
    <mrow>
     <msub>
      <mi>ρ</mi>
      <mn>1</mn>
     </msub>
     </mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ύψιλον ανήκει; παράσταση κεφαλαίο καπα, του; ανοίγει παρένθεση, ρ δείκτης 2; κλείνει παρένθεση με γραμμή από πάνω; ανάστροφη κάθετος; κεφαλαίο καπα, του; ανοίγει παρένθεση, ρ δείκτης 1; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_266() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>Κ</mi>
    <mi>β</mi>
   </msup>
   <mrow><mo>(</mo>
    <mi>ρ</mi>
   <mo>)</mo></mrow><mo>=</mo><mrow><mo>{</mo> <mrow>
    <mi>υ</mi><mo>∈</mo><msup>
     <mi>Κ</mi>
     <mi>β</mi>
    </msup>
    <mo>:</mo><mrow><mo>‖</mo> <mi>υ</mi> <mo>‖</mo></mrow><mo>&lt;</mo><mi>ρ</mi></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η βήτα δύναμη του, κεφαλαίο καπα; του ρ; ισούται με; το σύνολο όλων των ύψιλον που ανήκει, κεφαλαίο καπα στην βήτα οστή τέτοια ώστε νόρμα του ύψιλον; είναι μικρότερο από ρ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_267() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>0</mn><mo>∈</mo><msub>
    <mi>Ω</mi>
    <mn>1</mn>
   </msub>
   <mo>⊂</mo><msub>
    <mover accent="true">
     <mi>Ω</mi>
     <mo>¯</mo>
    </mover>
    
    <mn>1</mn>
   </msub>
   <mo>⊂</mo><msub>
    <mi>Ω</mi>
    <mn>2</mn>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "0 ανήκει; κεφαλαίο ωμέγα δείκτης 1; είναι ένα υποσύνολο του; κεφαλαίο ωμέγα παύλα, δείκτης 1; είναι ένα υποσύνολο του; κεφαλαίο ωμέγα δείκτης 2")?;
    return Ok(());
}

#[test]
fn uoa_corpus_268() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mi>υ</mi>
    <mo>*</mo>
   </msup>
   <mo>∈</mo><mi>Κ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ύψιλον αστερίσκος; ανήκει, κεφαλαίο καπα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_269() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>Γ</mi><mrow><mo>(</mo>
    <mrow>
     <mn>0,</mn><mi>σ</mi></mrow>
   <mo>)</mo></mrow><mo>≡</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο γάμμα; ανοίγει παρένθεση, 0, σίγμα, κλείνει παρένθεση; είναι ταυτόσημο με 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_270() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>Υ</mi>
    <mn>1</mn>
   </msub>
   <mo>∼</mo><mrow><mo>(</mo>
    <mrow>
     <msub>
      <mi>Χ</mi>
      <mrow>
       <mi>β</mi><mn>1</mn></mrow>
     </msub>
     <mo>,</mo><msub>
      <mi>Σ</mi>
      <mrow>
       <mn>1,1</mn></mrow>
     </msub>
     </mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο ύψιλον δείκτης 1; κυματοειδής γραμμή; ανοίγει παρένθεση; κεφαλαίο χ δείκτης, βήτα 1 τέλος δείκτη; κόμμα; κεφαλαίο σίγμα δείκτης 1,1; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_271() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <msub>
      <mi>Υ</mi>
      <mn>1</mn>
     </msub>
     <mo>,</mo><msub>
      <mi>Υ</mi>
      <mn>2</mn>
     </msub>
     </mrow>
   <mo>)</mo></mrow><mo>=</mo><msub>
    <mi>Σ</mi>
    <mrow>
     <mn>1,2</mn></mrow>
   </msub>
   <mo>≠</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση κεφαλαίο ύψιλον δείκτης 1, κόμμα, κεφαλαίο ύψιλον δείκτης 2; κλείνει παρένθεση, ισούται με; κεφαλαίο σίγμα δείκτης 1,2; είναι διάφορο του 0")?;
    return Ok(());
    //theodora. fails. recognizes as a set. Now reads: το ανοιχτό διάστημα από κεφαλαίο ύψιλον δείκτης 1, ως κεφαλαίο ύψιλον δείκτης 2; ισούται με; κεφαλαίο σίγμα δείκτης 1,2; είναι διάφορο του 0
}

#[test]
fn uoa_corpus_272() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mover accent="true">
    <mi>Ω</mi>
    <mo>¯</mo>
   </mover>
   <mo>=</mo><mstyle displaystyle="true">
    <munder>
     <mo>∪</mo>
     <mrow>
      <mi>Κ</mi><mo>∈</mo><mi>Μ</mi></mrow>
    </munder>
    <mover accent="true">
     <mi>Κ</mi>
     <mo>¯</mo>
    </mover>
    
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο ωμέγα παύλα; ισούται με; ένωση για κεφαλαίο καπα, ανήκει κεφαλαίο μί; κεφαλαίο καπα παύλα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_273() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>max</mi><mrow><mo>{</mo> <mrow>
    <mn>1,2,3</mn></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "μέγιστο του συνόλου 1,2,3")?;
    return Ok(());
}

#[test]
fn uoa_corpus_274() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>sup</mi><mrow><mo>{</mo> <mrow>
    <mn>1,2,3</mn></mrow> <mo>}</mo></mrow><mo>=</mo><mn>3</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "σουπρέμουμ του συνόλου 1,2,3; ισούται με 3")?;
    return Ok(());
}

#[test]
fn uoa_corpus_275() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>Α</mi><mo>⊂</mo><mi>Β</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο άλφα, είναι ένα υποσύνολο του; κεφαλαίο βήτα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_276() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>inf</mi><mrow><mo>{</mo> <mrow>
    <mn>1,2,3</mn></mrow> <mo>}</mo></mrow><mo>=</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ινφίμουμ του συνόλου 1,2,3; ισούται με 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_277() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <munder>
     <mo>∪</mo>
     <mi>α</mi>
    </munder>
    <mi>Α</mi>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ένωση για άλφα, κεφαλαίο άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_278() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mstyle displaystyle="true">
    <munder>
     <mo>∏</mo>
     <mi>α</mi>
    </munder>
    <mi>Α</mi>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "γινόμενο για άλφα του; κεφαλαίο άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_279() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>α</mi><mo>∪</mo><mi>β</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άλφα ένωση βήτα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_280() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>Α</mi><mo>=</mo><mo>∅</mo></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο άλφα, ισούται με, κενό σύνολο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_281() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>α</mi><mo>∪</mo><mi>β</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άλφα ένωση βήτα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_282() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cov</mi><mrow><mo>(</mo>
    <mrow>
     <msub>
      <mi>β</mi>
      <mn>1</mn>
     </msub>
     <mo>,</mo><msub>
      <mi>β</mi>
      <mn>2</mn>
     </msub>
     </mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "cov; ανοίγει παρένθεση; βήτα δείκτης 1; κόμμα; βήτα δείκτης 2; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_283() -> Result<()> { // same
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mi>Var</mi><mrow><mo>(</mo><msub><mi>&#x3B2;</mi><mn>1</mn></msub><mo>)</mo></mrow></math>"#;
    // TODO: add expected speech
    test("el", "SimpleSpeak", expr, "Var του; ανοίγει παρένθεση; βήτα δείκτης 1; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_284() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mo>∀</mo><mi>χ</mi><mo>∈</mo><mi>Χ</mi><mo>:</mo><mo>∃</mo><mi>ψ</mi><mo>∈</mo><mi>Ψ</mi><mo>:</mo><mi>χ</mi><mo>=</mo><mi>ψ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "για κάθε, χ ανήκει κεφαλαίο χ; άνω κάτω τελεία; υπάρχει, ψ ανήκει κεφαλαίο ψ; άνω κάτω τελεία, χ ισούται με ψ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_285() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msubsup>
    <mi>Γ</mi>
    <mn>0</mn>
    <mn>2</mn>
   </msubsup>
   <mrow><mo>[</mo> <mrow>
    <mn>0,1</mn></mrow> <mo>]</mo></mrow><mo>=</mo><mrow><mo>{</mo> <mrow>
    <mi>υ</mi><mo>∈</mo><msup>
     <mi>Γ</mi>
     <mn>2</mn>
    </msup>
    <mrow><mo>[</mo> <mrow>
     <mn>0,1</mn></mrow> <mo>]</mo></mrow><mo>:</mo><mi>υ</mi><mrow><mo>(</mo>
     <mn>0</mn>
    <mo>)</mo></mrow><mo>=</mo><msup>
     <mi>υ</mi>
     <mo>′</mo>
    </msup>
    <mrow><mo>(</mo>
     <mn>1</mn>
    <mo>)</mo></mrow><mo>=</mo><mn>0</mn></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο γάμμα δείκτης 0, στο τετράγωνο; του 0,1; ισούται με; το σύνολο όλων των ύψιλον που ανήκει; κεφαλαίο γάμμα στο τετράγωνο; του 0,1 τέτοια ώστε ύψιλον του 0, ισούται με, ύψιλον τόνος; του 1; ισούται με 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_286() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>max</mi><mrow><mo>{</mo> <mrow>
    <mfrac>
     <mn>1</mn>
     <mn>5</mn>
    </mfrac>
    <msup>
     <mi>χ</mi>
     <mn>2</mn>
    </msup>
    <mo>+</mo><mfrac>
     <mn>1</mn>
     <mrow>
      <msqrt>
       <mi>χ</mi>
      </msqrt>
      </mrow>
    </mfrac>
    <mo>:</mo><mfrac>
     <mn>1</mn>
     <mn>2</mn>
    </mfrac>
    <mi>ρ</mi><mi>τ</mi><mrow><mo>(</mo>
     <mrow>
      <mn>3</mn><mo>−</mo><msup>
       <mi>τ</mi>
       <mn>2</mn>
      </msup>
      </mrow>
    <mo>)</mo></mrow><mo>≤</mo><mi>χ</mi><mo>≤</mo><mi>ρ</mi><mi>τ</mi><mrow><mo>(</mo>
     <mrow>
      <mn>2</mn><mo>−</mo><mi>τ</mi></mrow>
    <mo>)</mo></mrow></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "μέγιστο του; συνόλου όλων των 1 πέμπτο χ στο τετράγωνο; συν; κλάσμα, 1 προς, η τετραγωνική ρίζα του χ; τέλος κλάσματος; τέτοια ώστε 1 δεύτερο ρ τάαφ; ανοίγει παρένθεση; 3 μείον, τάαφ στο τετράγωνο; κλείνει παρένθεση; είναι μικρότερο από ή ίσο με χ, είναι μικρότερο από ή ίσο με; ρ τάαφ; ανοίγει παρένθεση; 2 μείον τάαφ; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_287() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>min</mi><mrow><mo>{</mo> <mrow>
    <mfrac>
     <mn>1</mn>
     <mn>5</mn>
    </mfrac>
    <msup>
     <mi>χ</mi>
     <mn>2</mn>
    </msup>
    <mo>+</mo><mfrac>
     <mn>1</mn>
     <mrow>
      <msqrt>
       <mi>χ</mi>
      </msqrt>
      </mrow>
    </mfrac>
    <mo>:</mo><mfrac>
     <mn>1</mn>
     <mn>2</mn>
    </mfrac>
    <mi>ρ</mi><mi>τ</mi><mrow><mo>(</mo>
     <mrow>
      <mn>3</mn><mo>−</mo><msup>
       <mi>τ</mi>
       <mn>2</mn>
      </msup>
      </mrow>
    <mo>)</mo></mrow><mo>≤</mo><mi>χ</mi><mo>≤</mo><mi>ρ</mi><mi>τ</mi><mrow><mo>(</mo>
     <mrow>
      <mn>2</mn><mo>−</mo><mi>τ</mi></mrow>
    <mo>)</mo></mrow></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ελάχιστο του; συνόλου όλων των 1 πέμπτο χ στο τετράγωνο; συν; κλάσμα, 1 προς, η τετραγωνική ρίζα του χ; τέλος κλάσματος; τέτοια ώστε 1 δεύτερο ρ τάαφ; ανοίγει παρένθεση; 3 μείον, τάαφ στο τετράγωνο; κλείνει παρένθεση; είναι μικρότερο από ή ίσο με χ, είναι μικρότερο από ή ίσο με; ρ τάαφ; ανοίγει παρένθεση; 2 μείον τάαφ; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_288() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>υ</mi>
    <mrow>
     <msup>
      <mi>τ</mi>
      <mi>ξ</mi>
     </msup>
     </mrow>
   </msub>
   <mo>=</mo><mrow><mo>{</mo> <mrow>
    <mfrac>
     <mrow>
      <msup>
       <mo>∂</mo>
       <mi>ξ</mi>
      </msup>
      <msub>
       <mi>υ</mi>
       <mn>1</mn>
      </msub>
      </mrow>
     <mrow>
      <mo>∂</mo><msup>
       <mi>τ</mi>
       <mi>ξ</mi>
      </msup>
      </mrow>
    </mfrac>
    <mo>,</mo><mo>…</mo><mo>,</mo><mfrac>
     <mrow>
      <msup>
       <mo>∂</mo>
       <mi>ξ</mi>
      </msup>
      <msub>
       <mi>υ</mi>
       <mi>σ</mi>
      </msub>
      </mrow>
     <mrow>
      <mo>∂</mo><msup>
       <mi>τ</mi>
       <mi>ξ</mi>
      </msup>
      </mrow>
    </mfrac>
    </mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ύψιλον δείκτης, τάαφ στην ξ οστή τέλος δείκτη; ισούται με; το σύνολο κλάσμα, μερικό διαφορικό στην ξ οστή; ύψιλον δείκτης 1; προς, μερικό διαφορικό, τάαφ στην ξ οστή, τέλος κλάσματος; κόμμα; αποσιωπητικά, κόμμα; κλάσμα, μερικό διαφορικό στην ξ οστή; ύψιλον δείκτης σίγμα; προς, μερικό διαφορικό, τάαφ στην ξ οστή, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_289() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>inf</mi><mrow><mo>{</mo> <mrow>
    <msub>
     <mrow>
      <mrow><mo>‖</mo> <mi>β</mi> <mo>‖</mo></mrow></mrow>
     <mrow>
      <msubsup>
       <mi>Η</mi>
       <mi>β</mi>
       <mi>λ</mi>
      </msubsup>
      <mrow><mo>(</mo>
       <mi>Ω</mi>
      <mo>)</mo></mrow></mrow>
    </msub>
    <mo>:</mo><mi>β</mi><mo>∈</mo><msubsup>
     <mi>Η</mi>
     <mi>β</mi>
     <mi>λ</mi>
    </msubsup>
    <mrow><mo>(</mo>
     <mi>Ω</mi>
    <mo>)</mo></mrow><mo>,</mo><mi>β</mi><mrow><mo>|</mo><mi>Γ</mi></mrow><mo>=</mo><mi>υ</mi></mrow> <mo>}</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ινφίμουμ του; συνόλου νόρμα του βήτα δείκτης; η λάμδα δύναμη του, κεφαλαίο ήτα δείκτης βήτα; του κεφαλαίο ωμέγα τέλος δείκτη; άνω κάτω τελεία; βήτα που ανήκει; η λάμδα δύναμη του, κεφαλαίο ήτα δείκτης βήτα; του κεφαλαίο ωμέγα; κόμμα; βήτα, κάθετη γραμμη, κεφαλαίο γάμμα; ισούται με, ύψιλον")?;
    return Ok(());
}

#[test]
fn uoa_corpus_290() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msubsup>
    <mi>Β</mi>
    <mrow>
     <msub>
      <mi>δ</mi>
      <mn>0</mn>
     </msub>
     <mo>−</mo><mn>1,</mn><mi>η</mi></mrow>
    <mrow>
     <msub>
      <mi>λ</mi>
      <mn>2</mn>
     </msub>
     <mo>−</mo><mn>2</mn><mi>μ</mi><mn>,0</mn></mrow>
   </msubsup>
   <mrow><mo>(</mo>
    <mrow>
     <msup>
      <mi>ε</mi>
      <mrow>
       <mo>−</mo><mi>γ</mi><mi>τ</mi></mrow>
     </msup>
     <mo>,</mo><msub>
      <mi>Κ</mi>
      <mi>∞</mi>
     </msub>
     </mrow>
   <mo>)</mo></mrow><mo>⊂</mo><msubsup>
    <mi>Β</mi>
    <mrow>
     <msub>
      <mi>δ</mi>
      <mn>1</mn>
     </msub>
     <mo>,</mo><mi>η</mi></mrow>
    <mrow>
     <msub>
      <mi>λ</mi>
      <mn>2</mn>
     </msub>
     <mo>−</mo><mn>2</mn><mi>μ</mi><mn>,0</mn></mrow>
   </msubsup>
   <mrow><mo>(</mo>
    <mrow>
     <msup>
      <mi>ε</mi>
      <mrow>
       <mo>−</mo><mi>γ</mi><mi>τ</mi></mrow>
     </msup>
     <mo>,</mo><msub>
      <mi>Κ</mi>
      <mi>∞</mi>
     </msub>
     </mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο βήτα δείκτης; δέλτα δείκτης 0; μείον 1, ήτα τέλος δείκτη, που υψώνεται στη λάμδα δείκτης 2; μείον, 2 μί,0 τέλος δύναμης; ανοίγει παρένθεση έψιλον που υψώνεται στη μείον γάμμα, τάαφ τέλος δύναμης, κόμμα, κεφαλαίο καπα δείκτης άπειρο κλείνει παρένθεση; είναι ένα υποσύνολο του; κεφαλαίο βήτα δείκτης; δέλτα δείκτης 1; κόμμα, ήτα τέλος δείκτη, που υψώνεται στη λάμδα δείκτης 2; μείον, 2 μί,0 τέλος δύναμης; ανοίγει παρένθεση, έψιλον που υψώνεται στη μείον γάμμα, τάαφ τέλος δύναμης, κόμμα, κεφαλαίο καπα δείκτης άπειρο κλείνει παρένθεση")?;
    return Ok(());
    //theodora. fails Now reads: κεφαλαίο βήτα δείκτης; δέλτα δείκτης 0; μείον 1, ήτα τέλος δείκτη, που υψώνεται στη λάμδα δείκτης 2; μείον, 2 μί,0 τέλος δύναμης; το ανοιχτό διάστημα από έψιλον που υψώνεται στη μείον γάμμα, τάαφ τέλος δύναμης, ως κεφαλαίο καπα δείκτης άπειρο; είναι ένα υποσύνολο του; κεφαλαίο βήτα δείκτης; δέλτα δείκτης 1; κόμμα, ήτα τέλος δείκτη, που υψώνεται στη λάμδα δείκτης 2; μείον, 2 μί,0 τέλος δύναμης; το ανοιχτό διάστημα από έψιλον που υψώνεται στη μείον γάμμα, τάαφ τέλος δύναμης, ως κεφαλαίο καπα δείκτης άπειρο
}

#[test]
fn uoa_corpus_291() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>υ</mi><mrow><mo>(</mo>
    <mrow>
     <mo>·</mo><mo>,</mo><mi>τ</mi></mrow>
   <mo>)</mo></mrow><mo>∈</mo><msubsup>
    <mi>Β</mi>
    <mi>μ</mi>
    <mrow>
     <mn>2</mn><mi>μ</mi></mrow>
   </msubsup>
   <mrow><mo>(</mo>
    <mi>Κ</mi>
   <mo>)</mo></mrow><mo>⊂</mo><msubsup>
    <mi>Β</mi>
    <mi>β</mi>
    <mrow>
     <mn>2</mn><mi>μ</mi></mrow>
   </msubsup>
   <mrow><mo>(</mo>
    <mi>Κ</mi>
   <mo>)</mo></mrow><mo>⊂</mo><msubsup>
    <mi>Β</mi>
    <mrow>
     <mi>β</mi><mo>+</mo><mi>ε</mi></mrow>
    <mrow>
     <mn>2</mn><mi>μ</mi></mrow>
   </msubsup>
   <mrow><mo>(</mo>
    <mi>Κ</mi>
   <mo>)</mo></mrow><mo>≡</mo><msubsup>
    <mi>Η</mi>
    <mrow>
     <mi>β</mi><mo>+</mo><mi>ε</mi></mrow>
    <mrow>
     <mn>2</mn><mi>μ</mi></mrow>
   </msubsup>
   <mrow><mo>(</mo>
    <mi>Κ</mi>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ύψιλον; ανοίγει παρένθεση; τελεία κόμμα, τάαφ; κλείνει παρένθεση; ανήκει; η 2 μί δύναμη του, κεφαλαίο βήτα δείκτης μί; του κεφαλαίο καπα; είναι ένα υποσύνολο του; η 2 μί δύναμη του, κεφαλαίο βήτα δείκτης βήτα; του κεφαλαίο καπα; είναι ένα υποσύνολο του; η 2 μί δύναμη του, κεφαλαίο βήτα δείκτης, βήτα συν έψιλον τέλος δείκτη; του κεφαλαίο καπα; είναι ταυτόσημο με; η 2 μί δύναμη του, κεφαλαίο ήτα δείκτης, βήτα συν έψιλον τέλος δείκτη; του κεφαλαίο καπα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_292() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>tan</mi><mi>Α</mi><mo>=</mo><mfrac>
    <mrow>
     <mi>sin</mi><mi>Α</mi></mrow>
    <mrow>
     <mi>cos</mi><mi>Α</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "εφαπτομένη του κεφαλαίο άλφα; ισούται με; κλάσμα, ημίτονο του κεφαλαίο άλφα, προς, συνημίτονο του κεφαλαίο άλφα, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_293() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cot</mi><mi>Α</mi><mo>=</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <mi>tan</mi><mi>Α</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    // TODO: add expected speech
    test("el", "SimpleSpeak", expr, "συνεφαπτομένη, του κεφαλαίο άλφα; ισούται με; κλάσμα, 1 προς, εφαπτομένη του κεφαλαίο άλφα, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_294() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>sec</mi><mi>Α</mi><mo>=</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <mi>cos</mi><mi>Α</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "τέμνουσα του κεφαλαίο άλφα; ισούται με; κλάσμα, 1 προς, συνημίτονο του κεφαλαίο άλφα, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_295() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>csc</mi><mi>Α</mi><mo>=</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <mi>sin</mi><mi>Α</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συντέμνουσα, του κεφαλαίο άλφα; ισούται με; κλάσμα, 1 προς, ημίτονο του κεφαλαίο άλφα, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_296() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>sec</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>Α</mi><mo>−</mo><msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>Α</mi><mo>=</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "τέμνουσα στο τετράγωνο, του κεφαλαίο άλφα; μείον; εφαπτομένη στο τετράγωνο; του κεφαλαίο άλφα; ισούται με 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_297() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>tan</mi><mo stretchy="false">(</mo><mo>−</mo><mi>Α</mi><mo stretchy="false">)</mo><mo>=</mo><mo>−</mo><mi>tan</mi><mi>Α</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "εφαπτομένη του, μείον κεφαλαίο άλφα; ισούται με; μείον εφαπτομένη του κεφαλαίο άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_298() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mo stretchy="false">(</mo><mo>−</mo><mi>Α</mi><mo stretchy="false">)</mo><mo>=</mo><mi>cos</mi><mi>Α</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του, μείον κεφαλαίο άλφα; ισούται με; συνημίτονο του κεφαλαίο άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_299() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>sin</mi><mn>2</mn><mi>Α</mi><mo>=</mo><mn>2</mn><mi>sin</mi><mi>Α</mi><mi>cos</mi><mi>Α</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο του, 2 κεφαλαίο άλφα; ισούται με; 2, ημίτονο του κεφαλαίο άλφα; συνημίτονο του κεφαλαίο άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_300() -> Result<()> { //
    let expr = r#"<math>
 <mtable columnalign="left">
   <mtr>
    <mtd>
     <mi>cos</mi><mn>2</mn><mi>Α</mi><mo>=</mo><msup>
      <mi>cos</mi>
      <mn>2</mn>
     </msup>
     <mi>Α</mi><mo>−</mo><msup>
      <mi>sin</mi>
      <mn>2</mn>
     </msup>
     <mi>Α</mi>
    </mtd>
   </mtr>
   <mtr>
    <mtd>
     <mo>=</mo><mn>1</mn><mo>−</mo><mn>2</mn><msup>
      <mi>sin</mi>
      <mn>2</mn>
     </msup>
     <mi>Α</mi><mo>=</mo><mn>2</mn><msup>
      <mi>cos</mi>
      <mn>2</mn>
     </msup>
     <mi>Α</mi><mo>−</mo><mn>1</mn>
    </mtd>
   </mtr>
  </mtable>
  
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 σειρές; σειρά 1; συνημίτονο του, 2 κεφαλαίο άλφα; ισούται με; συνημίτονο στο τετράγωνο; του κεφαλαίο άλφα; μείον; ημίτονο στο τετράγωνο, του κεφαλαίο άλφα; εξίσωση 2; ισούται με; 1 μείον; 2; ημίτονο στο τετράγωνο, του κεφαλαίο άλφα; ισούται με; 2; συνημίτονο στο τετράγωνο; του κεφαλαίο άλφα; μείον 1")?;
    return Ok(());
    //theodora. fails. In Clearspeak it correctly recognizes 2 rows not 2 equations
    //Now reads. 2 εξισώσεις; εξίσωση 1; συνημίτονο του, 2 κεφαλαίο άλφα; ισούται με; συνημίτονο στο τετράγωνο; του κεφαλαίο άλφα; μείον; ημίτονο στο τετράγωνο, του κεφαλαίο άλφα; εξίσωση 2; ισούται με; 1 μείον; 2; ημίτονο στο τετράγωνο, του κεφαλαίο άλφα; ισούται με; 2; συνημίτονο στο τετράγωνο; του κεφαλαίο άλφα; μείον 1
}

#[test]
fn uoa_corpus_301() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>sin</mi><mn>3</mn><mi>Α</mi><mo>=</mo><mn>3</mn><mi>sin</mi><mi>Α</mi><mo>−</mo><mn>4</mn><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>3</mn>
   </msup>
   <mi>Α</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο του, 3 κεφαλαίο άλφα; ισούται με; 3, ημίτονο του κεφαλαίο άλφα; μείον; 4, ημίτονο στον κύβο, του κεφαλαίο άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_302() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>Α</mi><mo>=</mo><mfrac>
    <mn>1</mn>
    <mn>2</mn>
   </mfrac>
   <mo>−</mo><mfrac>
    <mn>1</mn>
    <mn>2</mn>
   </mfrac>
   <mi>cos</mi><mn>2</mn><mi>Α</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο στο τετράγωνο, του κεφαλαίο άλφα; ισούται με; 1 δεύτερο μείον; 1 δεύτερο; συνημίτονο του, 2 κεφαλαίο άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_303() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>4</mn>
   </msup>
   <mi>Α</mi><mo>=</mo><mfrac>
    <mn>3</mn>
    <mn>8</mn>
   </mfrac>
   <mo>−</mo><mfrac>
    <mn>1</mn>
    <mn>2</mn>
   </mfrac>
   <mi>cos</mi><mn>2</mn><mi>Α</mi><mo>+</mo><mfrac>
    <mn>1</mn>
    <mn>8</mn>
   </mfrac>
   <mi>cos</mi><mn>4</mn><mi>Α</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η τέταρτη δύναμη του, ημίτονο; του κεφαλαίο άλφα; ισούται με; 3 όγδοα μείον; 1 δεύτερο; συνημίτονο του, 2 κεφαλαίο άλφα; συν; 1 όγδοο; συνημίτονο του, 4 κεφαλαίο άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_304() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>5</mn>
   </msup>
   <mi>Α</mi><mo>=</mo><mfrac>
    <mn>5</mn>
    <mn>8</mn>
   </mfrac>
   <mi>cos</mi><mi>Α</mi><mo>+</mo><mfrac>
    <mn>5</mn>
    <mrow>
     <mn>16</mn></mrow>
   </mfrac>
   <mi>cos</mi><mn>3</mn><mi>Α</mi><mo>+</mo><mfrac>
    <mn>1</mn>
    <mrow>
     <mn>16</mn></mrow>
   </mfrac>
   <mi>cos</mi><mn>5</mn><mi>Α</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η πέμπτη δύναμη του, συνημίτονο; του κεφαλαίο άλφα; ισούται με; 5 όγδοα, συνημίτονο του κεφαλαίο άλφα; συν; 5 προς 16; συνημίτονο του, 3 κεφαλαίο άλφα; συν; 1 προς 16; συνημίτονο του, 5 κεφαλαίο άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_305() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>sin</mi><mi>Α</mi><mo>+</mo><mi>sin</mi><mi>Β</mi><mo>=</mo><mn>2</mn><mi>sin</mi><mfrac>
    <mn>1</mn>
    <mn>2</mn>
   </mfrac>
   <mrow><mo>(</mo>
    <mrow>
     <mi>Α</mi><mo>+</mo><mi>Β</mi></mrow>
   <mo>)</mo></mrow><mi>cos</mi><mfrac>
    <mn>1</mn>
    <mn>2</mn>
   </mfrac>
   <mrow><mo>(</mo>
    <mrow>
     <mi>Α</mi><mo>−</mo><mi>Β</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο του κεφαλαίο άλφα; συν, ημίτονο του κεφαλαίο βήτα; ισούται με; 2, ημίτονο του 1 δεύτερο; ανοίγει παρένθεση; κεφαλαίο άλφα, συν κεφαλαίο βήτα; κλείνει παρένθεση; συνημίτονο του 1 δεύτερο; ανοίγει παρένθεση; κεφαλαίο άλφα, μείον κεφαλαίο βήτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_306() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mrow>
     <mo>−</mo><mn>1</mn></mrow>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <mo>−</mo><mi>χ</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mi>π</mi><mo>−</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mrow>
     <mo>−</mo><mn>1</mn></mrow>
   </msup>
   <mi>χ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "αντίστροφη συνημίτονο, του μείον χ; ισούται με; π μείον, αντίστροφη συνημίτονο, του χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_307() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mi>χ</mi><mo>×</mo><mi>ψ</mi></mrow>
   <mo>)</mo></mrow><mo>+</mo><msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>χ</mi><mo>+</mo><mi>ψ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του; ανοίγει παρένθεση, χ επί ψ, κλείνει παρένθεση; συν, εφαπτομένη στο τετράγωνο; του χ; συν ψ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_308() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mi>χ</mi><mo>×</mo><mi>ψ</mi></mrow>
   <mo>)</mo></mrow><mo>+</mo><msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <mi>χ</mi><mo>+</mo><mi>ψ</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του; ανοίγει παρένθεση, χ επί ψ, κλείνει παρένθεση; συν; εφαπτομένη στο τετράγωνο; του; ανοίγει παρένθεση, χ συν ψ, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_309() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mi>χ</mi><mo>×</mo><mi>ψ</mi><mo>+</mo><msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>χ</mi><mo>+</mo><mi>ψ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του χ, επί ψ; συν, εφαπτομένη στο τετράγωνο; του χ; συν ψ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_310() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mi>χ</mi><mo>×</mo><mi>ψ</mi><mo>+</mo><msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <mi>χ</mi><mo>+</mo><mi>ψ</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του χ, επί ψ; συν; εφαπτομένη στο τετράγωνο; του; ανοίγει παρένθεση, χ συν ψ, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_311() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>tan</mi><msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mi>χ</mi><mo>+</mo><mn>2</mn><mo>×</mo><mi>ψ</mi></mrow>
     <mo>)</mo></mrow></mrow>
    <mn>2</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "εφαπτομένη του; ανοίγει παρένθεση; χ συν 2 επί ψ; κλείνει παρένθεση στο τετράγωνο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_312() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mo stretchy="false">(</mo><mi>χ</mi><mo>+</mo><mi>ψ</mi><mo stretchy="false">)</mo><mo>+</mo><mi>cos</mi><mi>χ</mi><mo>+</mo><msup>
    <mi>ψ</mi>
    <mn>2</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο στο τετράγωνο, του; ανοίγει παρένθεση, χ συν ψ, κλείνει παρένθεση; συν, συνημίτονο του χ, συν ψ στο τετράγωνο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_313() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mi>x</mi><mi>ψ</mi></mrow>
   <mo>)</mo></mrow><mo>+</mo><msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>x</mi><mo>+</mo><mi>ψ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του; ανοίγει παρένθεση, x ψ, κλείνει παρένθεση; συν, εφαπτομένη στο τετράγωνο; του x; συν ψ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_314() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mi>x</mi><mi>ψ</mi></mrow>
   <mo>)</mo></mrow><mo>+</mo><msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <mi>χ</mi><mo>+</mo><mi>y</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του; ανοίγει παρένθεση, x ψ, κλείνει παρένθεση; συν; εφαπτομένη στο τετράγωνο; του; ανοίγει παρένθεση, χ συν y, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_315() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mi>χ</mi><mi>y</mi><mo>+</mo><msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>χ</mi><mo>+</mo><mi>y</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του χ y; συν, εφαπτομένη στο τετράγωνο; του χ; συν y")?;
    return Ok(());
}

#[test]
fn uoa_corpus_316() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mi>χ</mi><mi>y</mi><mo>+</mo><msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <mi>x</mi><mo>+</mo><mi>ψ</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του χ y; συν; εφαπτομένη στο τετράγωνο; του; ανοίγει παρένθεση, x συν ψ, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_317() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>cosh</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>χ</mi><mo>−</mo><msup>
    <mrow>
     <mi>sinh</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>χ</mi><mo>=</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "υπερβολικό συνημίτονο στο τετράγωνο; του χ; μείον; υπερβολικό ημίτονο στο τετράγωνο; του χ; ισούται με 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_318() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>χ</mi><mo>+</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>χ</mi><mo>=</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο στο τετράγωνο, του χ; συν, συνημίτονο στο τετράγωνο; του χ; ισούται με 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_319() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mrow>
     <mo>−</mo><mn>1</mn></mrow>
   </msup>
   <mi>χ</mi><mo>≠</mo><mi>sin</mi><msup>
    <mi>χ</mi>
    <mrow>
     <mo>−</mo><mn>1</mn></mrow>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "αντίστροφη ημίτονο, του χ; είναι διάφορο του; ημίτονο του χ στην μείον 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_320() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mi>χ</mi><mo>+</mo><mi>ψ</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mi>cos</mi><mi>χ</mi><mo>⋅</mo><mi>cos</mi><mi>ψ</mi><mo>−</mo><mi>sin</mi><mi>χ</mi><mo>⋅</mo><mi>sin</mi><mi>ψ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του; ανοίγει παρένθεση, χ συν ψ, κλείνει παρένθεση; ισούται με; συνημίτονο του χ, φορές, συνημίτονο του ψ; μείον; ημίτονο του χ, φορές, ημίτονο του ψ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_321() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mn>2</mn><mo>⋅</mo><mi>χ</mi><mo>=</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>χ</mi><mo>−</mo><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>χ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του 2, φορές χ; ισούται με; συνημίτονο στο τετράγωνο; του χ; μείον, ημίτονο στο τετράγωνο, του χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_322() -> Result<()> { //
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mi>sin</mi><mi>&#x3BD;</mi><mi>&#x391;</mi><mo>=</mo><mi>sin</mi><mi>&#x391;</mi><mrow><mo>{</mo><msup><mrow><mo>(</mo><mn>2</mn><mi>cos</mi><mi>&#x391;</mi><mo>)</mo></mrow><mrow><mi>&#x3BD;</mi><mo>&#x2212;</mo><mn>1</mn></mrow></msup><mo>&#x2212;</mo><mrow><mo>(</mo><mfrac linethickness="0"><mrow><mi>&#x3BD;</mi><mo>&#x2212;</mo><mn>2</mn></mrow><mn>1</mn></mfrac><mo>)</mo></mrow><msup><mrow><mo>(</mo><mn>2</mn><mi>cos</mi><mi>&#x391;</mi><mo>)</mo></mrow><mrow><mi>&#x3BD;</mi><mo>&#x2212;</mo><mn>3</mn></mrow></msup><mspace linebreak="newline"/><mo>+</mo><mrow><mo>(</mo><mfrac linethickness="0"><mrow><mi>&#x3BD;</mi><mo>&#x2212;</mo><mn>3</mn></mrow><mn>2</mn></mfrac><mo>)</mo></mrow><msup><mrow><mo>(</mo><mn>2</mn><mi>cos</mi><mi>&#x391;</mi><mo>)</mo></mrow><mrow><mi>&#x3BD;</mi><mo>&#x2212;</mo><mn>5</mn></mrow></msup><mo>&#x2212;</mo><mo>&#x22EF;</mo><mo>}</mo></mrow></math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο του, νί κεφαλαίο άλφα; ισούται με; ημίτονο του κεφαλαίο άλφα; ανοίγει άγκιστρο; ανοίγει παρένθεση; 2, συνημίτονο του κεφαλαίο άλφα; κλείνει παρένθεση που υψώνεται στη νί μείον 1 τέλος δύναμης; μείον; διωνυμικό νί μείον 2 ανά 1; ανοίγει παρένθεση; 2, συνημίτονο του κεφαλαίο άλφα; κλείνει παρένθεση που υψώνεται στη νί μείον 3 τέλος δύναμης; συν; διωνυμικό νί μείον 3 ανά 2; ανοίγει παρένθεση; 2, συνημίτονο του κεφαλαίο άλφα; κλείνει παρένθεση που υψώνεται στη νί μείον 5 τέλος δύναμης; μείον αποσιωπητικά; κλείνει άγκιστρο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_323() -> Result<()> { //
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
<mstyle displaystyle="true">
<mrow>
<mi>cos</mi><mi>ν</mi><mi>Α</mi>
<mo>=</mo>
<mfrac><mn>1</mn><mn>2</mn></mfrac>
<mrow><mo>{</mo>
<mrow>
<msup><mrow><mo>(</mo><mrow><mn>2</mn><mi>cos</mi><mi>Α</mi></mrow><mo>)</mo></mrow><mi>ν</mi></msup>
<mo>−</mo>
<mfrac><mi>ν</mi><mn>1</mn></mfrac>
<msup><mrow><mo>(</mo><mrow><mn>2</mn><mi>cos</mi><mi>Α</mi></mrow><mo>)</mo></mrow><mrow><mi>ν</mi><mo>−</mo><mn>2</mn></mrow></msup>
<mo>+</mo>
<mfrac><mi>ν</mi><mn>2</mn></mfrac>
<mrow><mo>(</mo>
<mfrac linethickness="0"><mrow><mi>ν</mi><mo>−</mo><mn>3</mn></mrow><mn>1</mn></mfrac>
<mo>)</mo></mrow>
<msup><mrow><mo>(</mo><mrow><mn>2</mn><mi>cos</mi><mi>Α</mi></mrow><mo>)</mo></mrow><mrow><mi>ν</mi><mo>−</mo><mn>4</mn></mrow></msup>
<mspace linebreak="newline" indentalign="left"/>
<mo>−</mo>
<mfrac><mi>ν</mi><mn>3</mn></mfrac>
<mrow><mo>(</mo>
<mfrac linethickness="0"><mrow><mi>ν</mi><mo>−</mo><mn>4</mn></mrow><mn>2</mn></mfrac>
<mo>)</mo></mrow>
<msup><mrow><mo>(</mo><mrow><mn>2</mn><mi>cos</mi><mi>Α</mi></mrow><mo>)</mo></mrow><mrow><mi>ν</mi><mo>−</mo><mn>6</mn></mrow></msup>
<mo>+</mo><mo>⋯</mo>
</mrow>
<mo>}</mo></mrow>
</mrow>
</mstyle>
</math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του, νί κεφαλαίο άλφα; ισούται με; 1 δεύτερο; ανοίγει άγκιστρο; ανοίγει παρένθεση; 2, συνημίτονο του κεφαλαίο άλφα; κλείνει παρένθεση στην νί οστή; μείον; νί προς 1; ανοίγει παρένθεση; 2, συνημίτονο του κεφαλαίο άλφα; κλείνει παρένθεση που υψώνεται στη νί μείον 2 τέλος δύναμης; συν; νί προς 2; διωνυμικό νί μείον 3 ανά 1; ανοίγει παρένθεση; 2, συνημίτονο του κεφαλαίο άλφα; κλείνει παρένθεση που υψώνεται στη νί μείον 4 τέλος δύναμης; μείον; νί προς 3; διωνυμικό νί μείον 4 ανά 2; ανοίγει παρένθεση; 2, συνημίτονο του κεφαλαίο άλφα; κλείνει παρένθεση που υψώνεται στη νί μείον 6 τέλος δύναμης; συν αποσιωπητικά; κλείνει άγκιστρο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_324() -> Result<()> { //
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
<mstyle displaystyle="true">
<mrow>
<msup><mi>sin</mi><mrow><mn>2</mn><mi>ν</mi><mo>−</mo><mn>1</mn></mrow></msup><mi>Α</mi>
<mo>=</mo>
<mfrac>
<msup><mrow><mo>(</mo><mo>−</mo><mn>1</mn><mo>)</mo></mrow><mrow><mi>ν</mi><mo>−</mo><mn>1</mn></mrow></msup>
<msup><mn>2</mn><mrow><mn>2</mn><mi>ν</mi><mo>−</mo><mn>2</mn></mrow></msup>
</mfrac>
<mrow><mo>{</mo>
<mrow>
<mi>sin</mi><mrow><mo>(</mo><mn>2</mn><mi>ν</mi><mo>−</mo><mn>1</mn><mo>)</mo></mrow><mi>Α</mi>
<mo>−</mo>
<mrow><mo>(</mo>
<mfrac linethickness="0"><mrow><mn>2</mn><mi>ν</mi><mo>−</mo><mn>1</mn></mrow><mn>1</mn></mfrac>
<mo>)</mo></mrow>
<mi>sin</mi><mrow><mo>(</mo><mn>2</mn><mi>ν</mi><mo>−</mo><mn>3</mn><mo>)</mo></mrow><mi>Α</mi>
<mspace linebreak="newline" indentalign="left"/>
<mo>+</mo><mo>…</mo>
<msup><mrow><mo>(</mo><mo>−</mo><mn>1</mn><mo>)</mo></mrow><mrow><mi>ν</mi><mo>−</mo><mn>1</mn></mrow></msup>
<mrow><mo>(</mo>
<mfrac linethickness="0"><mrow><mn>2</mn><mi>ν</mi><mo>−</mo><mn>1</mn></mrow><mrow><mi>ν</mi><mo>−</mo><mn>1</mn></mrow></mfrac>
<mo>)</mo></mrow>
<mi>sin</mi><mi>Α</mi>
</mrow>
<mo>}</mo></mrow>
</mrow>
</mstyle>
</math>"#;
    test("el", "SimpleSpeak", expr, "η 2 νί, μείον 1 δύναμη του, ημίτονο; του κεφαλαίο άλφα; ισούται με; κλάσμα, ανοίγει παρένθεση, μείον 1, κλείνει παρένθεση που υψώνεται στη νί μείον 1 τέλος δύναμης; προς, 2 που υψώνεται στη 2 νί, μείον 2 τέλος δύναμης; τέλος κλάσματος; ανοίγει άγκιστρο; ημίτονο του; ανοίγει παρένθεση; 2 νί, μείον 1; κλείνει παρένθεση; κεφαλαίο άλφα; μείον; διωνυμικό 2 νί, μείον 1 ανά 1; ημίτονο του; ανοίγει παρένθεση; 2 νί, μείον 3; κλείνει παρένθεση; κεφαλαίο άλφα; συν; αποσιωπητικά; ανοίγει παρένθεση, μείον 1, κλείνει παρένθεση που υψώνεται στη νί μείον 1 τέλος δύναμης; διωνυμικό 2 νί, μείον 1 ανά νί μείον 1 τέλος διωνυμικού; ημίτονο του κεφαλαίο άλφα; κλείνει άγκιστρο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_325() -> Result<()> { //
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
<mstyle displaystyle="true">
<mrow>
<msup><mi>cos</mi><mrow><mn>2</mn><mi>ν</mi><mo>−</mo><mn>1</mn></mrow></msup><mi>Α</mi>
<mo>=</mo>
<mfrac>
<mn>1</mn>
<msup><mn>2</mn><mrow><mn>2</mn><mi>ν</mi><mo>−</mo><mn>2</mn></mrow></msup>
</mfrac>
<mrow><mo>{</mo>
<mrow>
<mi>cos</mi><mrow><mo>(</mo><mn>2</mn><mi>ν</mi><mo>−</mo><mn>1</mn><mo>)</mo></mrow><mi>Α</mi>
<mo>+</mo>
<mrow><mo>(</mo>
<mfrac linethickness="0"><mrow><mn>2</mn><mi>ν</mi><mo>−</mo><mn>1</mn></mrow><mn>1</mn></mfrac>
<mo>)</mo></mrow>
<mi>cos</mi><mrow><mo>(</mo><mn>2</mn><mi>ν</mi><mo>−</mo><mn>3</mn><mo>)</mo></mrow><mi>Α</mi>
<mspace linebreak="newline" indentalign="left"/>
<mo>+</mo><mo>⋯</mo><mo>+</mo>
<mrow><mo>(</mo>
<mfrac linethickness="0"><mrow><mn>2</mn><mi>ν</mi><mo>−</mo><mn>1</mn></mrow><mrow><mi>ν</mi><mo>−</mo><mn>1</mn></mrow></mfrac>
<mo>)</mo></mrow>
<mi>cos</mi><mi>Α</mi>
</mrow>
<mo>}</mo></mrow>
</mrow>
</mstyle>
</math>"#;
    test("el", "SimpleSpeak", expr, "η 2 νί, μείον 1 δύναμη του, συνημίτονο; του κεφαλαίο άλφα; ισούται με; κλάσμα, 1 προς, 2 που υψώνεται στη 2 νί, μείον 2 τέλος δύναμης; τέλος κλάσματος; ανοίγει άγκιστρο; συνημίτονο του; ανοίγει παρένθεση; 2 νί, μείον 1; κλείνει παρένθεση; κεφαλαίο άλφα; συν; διωνυμικό 2 νί, μείον 1 ανά 1; συνημίτονο του; ανοίγει παρένθεση; 2 νί, μείον 3; κλείνει παρένθεση; κεφαλαίο άλφα; συν αποσιωπητικά, συν; διωνυμικό 2 νί, μείον 1 ανά νί μείον 1 τέλος διωνυμικού; συνημίτονο του κεφαλαίο άλφα; κλείνει άγκιστρο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_326() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cosh</mi><mi>χ</mi><mo>=</mo><mfrac>
    <mrow>
     <msup>
      <mi>e</mi>
      <mi>χ</mi>
     </msup>
     <mo>+</mo><msup>
      <mi>e</mi>
      <mrow>
       <mo>−</mo><mi>χ</mi></mrow>
     </msup>
     </mrow>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "υπερβολικό συνημίτονο, του χ; ισούται με; κλάσμα, e στην χ οστή, συν, e που υψώνεται στη μείον χ τέλος δύναμης; προς 2, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_327() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mi>x</mi><mo>+</mo><mi>ψ</mi><mo>+</mo><mi>z</mi></mrow>
     <mo>)</mo></mrow></mrow>
    <mn>2</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση; x συν ψ συν z; κλείνει παρένθεση στο τετράγωνο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_328() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mi>χ</mi><mo>+</mo><mi>y</mi></mrow>
     <mo>)</mo></mrow></mrow>
    <mn>2</mn>
   </msup>
   <mo>+</mo><mi>z</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση, χ συν y, κλείνει παρένθεση στο τετράγωνο; συν z")?;
    return Ok(());
}

#[test]
fn uoa_corpus_329() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>x</mi><mo>+</mo><msup>
    <mrow>
     <mrow><mo>(</mo>
      <mrow>
       <mi>ψ</mi><mo>+</mo><mi>ζ</mi></mrow>
     <mo>)</mo></mrow></mrow>
    <mn>2</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "x συν; ανοίγει παρένθεση; ψ συν ζήτα; κλείνει παρένθεση στο τετράγωνο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_330() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>x</mi><mo>+</mo><mi>ψ</mi><mo>+</mo><msup>
    <mi>z</mi>
    <mn>2</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "x συν ψ συν z στο τετράγωνο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_331() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msqrt>
    <mrow>
     <mfrac>
      <mi>b</mi>
      <mi>c</mi>
     </mfrac>
     </mrow>
   </msqrt>
   <mo>+</mo><mi>δ</mi><mo>−</mo><mi>ε</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η τετραγωνική ρίζα του b προς c; τέλος ρίζας; συν δέλτα μείον έψιλον")?;
    return Ok(());
}

#[test]
fn uoa_corpus_332() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msqrt>
    <mrow>
     <mfrac>
      <mi>β</mi>
      <mrow>
       <mi>c</mi><mo>+</mo><mi>d</mi></mrow>
     </mfrac>
     </mrow>
   </msqrt>
   <mo>−</mo><mi>ε</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η τετραγωνική ρίζα του κλάσματος, βήτα προς, c συν d, τέλος κλάσματος; τέλος ρίζας; μείον έψιλον")?;
    return Ok(());
}


#[test]
fn uoa_corpus_333() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msqrt>
    <mrow>
     <mfrac>
      <mi>β</mi>
      <mi>c</mi>
     </mfrac>
     <mo>−</mo><mi>e</mi></mrow>
   </msqrt>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "η τετραγωνική ρίζα του βήτα προς c; μείον e, τέλος ρίζας")?;
    return Ok(());
}

#[test]
fn uoa_corpus_334() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mi>x</mi><mi>ψ</mi></mrow>
   <mo>)</mo></mrow><mo>+</mo><msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>x</mi><mo>+</mo><mi>ψ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του; ανοίγει παρένθεση, x ψ, κλείνει παρένθεση; συν, εφαπτομένη στο τετράγωνο; του x; συν ψ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_335() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mi>x</mi><mi>ψ</mi></mrow>
   <mo>)</mo></mrow><mo>+</mo><msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <mi>χ</mi><mo>+</mo><mi>y</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του; ανοίγει παρένθεση, x ψ, κλείνει παρένθεση; συν; εφαπτομένη στο τετράγωνο; του; ανοίγει παρένθεση, χ συν y, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_336() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mi>χ</mi><mi>y</mi><mo>+</mo><msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>χ</mi><mo>+</mo><mi>y</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του χ y; συν, εφαπτομένη στο τετράγωνο; του χ; συν y")?;
    return Ok(());
}

#[test]
fn uoa_corpus_337() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mi>χ</mi><mi>y</mi><mo>+</mo><msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <mi>x</mi><mo>+</mo><mi>ψ</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του χ y; συν; εφαπτομένη στο τετράγωνο; του; ανοίγει παρένθεση, x συν ψ, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_338() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>α</mi><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mi>β</mi><mo>+</mo><mi>γ</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "άλφα επί; ανοίγει παρένθεση; βήτα συν γάμμα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_339() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>3</mn><mo>×</mo><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><msup>
      <mi>χ</mi>
      <mn>3</mn>
     </msup>
     <mo>+</mo><mn>6</mn><mi>χ</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "3 επί χ στο τετράγωνο, επί; ανοίγει παρένθεση; 2 χ στον κύβο, συν 6 χ; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_340() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mi>α</mi><mo>+</mo><mi>β</mi></mrow>
   <mo>)</mo></mrow><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mi>γ</mi><mo>+</mo><mi>δ</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση; άλφα συν βήτα; κλείνει παρένθεση; επί; ανοίγει παρένθεση; γάμμα συν δέλτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_341() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mn>3</mn><msup>
      <mi>χ</mi>
      <mn>2</mn>
     </msup>
     <mi>ψ</mi><mo>+</mo><mn>2</mn><mi>ψ</mi></mrow>
   <mo>)</mo></mrow><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><msup>
      <mi>χ</mi>
      <mn>2</mn>
     </msup>
     <mo>+</mo><mn>5</mn></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση; 3 χ στο τετράγωνο, ψ; συν 2 ψ; κλείνει παρένθεση; επί; ανοίγει παρένθεση; 2 χ στο τετράγωνο, συν 5; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_342() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>3</mn><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><msup>
      <mi>χ</mi>
      <mn>3</mn>
     </msup>
     <mo>+</mo><mn>6</mn><mi>χ</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "3 χ στο τετράγωνο, επί; ανοίγει παρένθεση; 2 χ στον κύβο, συν 6 χ; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_343() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>3</mn><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "3 χ στο τετράγωνο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_344() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>2</mn><msup>
    <mi>χ</mi>
    <mn>3</mn>
   </msup>
   <mo>+</mo><mn>6</mn><mi>χ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 χ στον κύβο, συν 6 χ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_345() -> Result<()> { // 
    let expr = r#"<math>
 <mrow>
   <mn>3</mn><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><msup>
      <mi>χ</mi>
      <mn>3</mn>
     </msup>
     <mo>+</mo><mn>6</mn><mi>χ</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mn>3</mn><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mo>×</mo><mn>2</mn><msup>
    <mi>χ</mi>
    <mn>3</mn>
   </msup>
   <mo>+</mo><mn>3</mn><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mo>×</mo><mn>6</mn><mi>χ</mi><mo>=</mo><mn>6</mn><msup>
    <mi>χ</mi>
    <mn>5</mn>
   </msup>
   <mo>+</mo><mn>18</mn><msup>
    <mi>χ</mi>
    <mn>3</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "3 χ στο τετράγωνο, επί; ανοίγει παρένθεση; 2 χ στον κύβο, συν 6 χ; κλείνει παρένθεση; ισούται με; 3 χ στο τετράγωνο, επί 2 χ στον κύβο; συν, 3 χ στο τετράγωνο, επί 6 χ; ισούται με; 6 χ στην πέμπτη, συν, 18 χ στον κύβο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_346() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mn>3</mn><msup>
      <mi>χ</mi>
      <mn>2</mn>
     </msup>
     <mi>ψ</mi><mo>+</mo><mn>2</mn><mi>ψ</mi></mrow>
   <mo>)</mo></mrow><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><msup>
      <mi>χ</mi>
      <mn>2</mn>
     </msup>
     <mo>+</mo><mn>5</mn></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση; 3 χ στο τετράγωνο, ψ; συν 2 ψ; κλείνει παρένθεση; επί; ανοίγει παρένθεση; 2 χ στο τετράγωνο, συν 5; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_347() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>3</mn><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mi>ψ</mi><mo>+</mo><mn>2</mn><mi>ψ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "3 χ στο τετράγωνο, ψ; συν 2 ψ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_348() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>2</mn><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mo>+</mo><mn>5</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 χ στο τετράγωνο, συν 5")?;
    return Ok(());
}

#[test]
fn uoa_corpus_349() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mn>3</mn><msup>
      <mi>χ</mi>
      <mn>2</mn>
     </msup>
     <mi>ψ</mi><mo>+</mo><mn>2</mn><mi>ψ</mi></mrow>
   <mo>)</mo></mrow><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><msup>
      <mi>χ</mi>
      <mn>2</mn>
     </msup>
     <mo>+</mo><mn>5</mn></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση; 3 χ στο τετράγωνο, ψ; συν 2 ψ; κλείνει παρένθεση; επί; ανοίγει παρένθεση; 2 χ στο τετράγωνο, συν 5; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_350() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mo>=</mo><mn>3</mn><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mi>ψ</mi><mo>×</mo><mn>2</mn><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mo>+</mo><mn>3</mn><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mi>ψ</mi><mo>×</mo><mn>5</mn><mo>+</mo><mn>2</mn><mi>ψ</mi><mo>×</mo><mn>2</mn><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mo>+</mo><mn>2</mn><mi>ψ</mi><mo>×</mo><mn>5</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ισούται με; 3 χ στο τετράγωνο, ψ επί 2 χ στο τετράγωνο; συν, 3 χ στο τετράγωνο, ψ επί 5; συν, 2 ψ επί 2 χ στο τετράγωνο; συν 2 ψ επί 5")?;
    return Ok(());
}

#[test]
fn uoa_corpus_351() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mo>=</mo><mn>6</mn><msup>
    <mi>χ</mi>
    <mn>4</mn>
   </msup>
   <mi>ψ</mi><mo>+</mo><mn>15</mn><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mi>ψ</mi><mo>+</mo><mn>4</mn><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mi>ψ</mi><mo>+</mo><mn>10</mn><mi>ψ</mi><mo>=</mo><mn>6</mn><msup>
    <mi>χ</mi>
    <mn>4</mn>
   </msup>
   <mi>ψ</mi><mo>+</mo><mn>19</mn><msup>
    <mi>χ</mi>
    <mn>2</mn>
   </msup>
   <mi>ψ</mi><mo>+</mo><mn>10</mn><mi>ψ</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ισούται με; 6 χ στην τέταρτη, ψ; συν, 15 χ στο τετράγωνο, ψ; συν, 4 χ στο τετράγωνο, ψ; συν 10 ψ; ισούται με; 6 χ στην τέταρτη, ψ; συν, 19 χ στο τετράγωνο, ψ; συν 10 ψ")?;
    return Ok(());
}

#[test]
fn uoa_corpus_352() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>2</mn><mi>α</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_353() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>sin</mi><mrow><mo>(</mo>
    <mrow>
     <mi>α</mi><mo>+</mo><mi>β</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο του; ανοίγει παρένθεση; άλφα συν βήτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_354() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mi>α</mi><mo>+</mo><mi>β</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του; ανοίγει παρένθεση; άλφα συν βήτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_355() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>tan</mi><mrow><mo>(</mo>
    <mrow>
     <mi>α</mi><mo>+</mo><mi>β</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "εφαπτομένη του; ανοίγει παρένθεση; άλφα συν βήτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_356() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>sin</mi><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>α</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mi>sin</mi><mrow><mo>(</mo>
    <mrow>
     <mi>α</mi><mo>+</mo><mi>α</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mi>sin</mi><mrow><mo>(</mo>
    <mi>α</mi>
   <mo>)</mo></mrow><mo>×</mo><mi>cos</mi><mrow><mo>(</mo>
    <mi>α</mi>
   <mo>)</mo></mrow><mo>+</mo><mi>cos</mi><mrow><mo>(</mo>
    <mi>α</mi>
   <mo>)</mo></mrow><mo>×</mo><mi>sin</mi><mrow><mo>(</mo>
    <mi>α</mi>
   <mo>)</mo></mrow><mo>=</mo><mn>2</mn><mo>×</mo><mi>sin</mi><mrow><mo>(</mo>
    <mi>α</mi>
   <mo>)</mo></mrow><mo>×</mo><mi>cos</mi><mrow><mo>(</mo>
    <mi>a</mi>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση; ισούται με; ημίτονο του; ανοίγει παρένθεση; άλφα συν άλφα; κλείνει παρένθεση; ισούται με; ημίτονο του άλφα, επί, συνημίτονο του άλφα; συν; συνημίτονο του άλφα; επί, ημίτονο του άλφα; ισούται με; 2 επί, ημίτονο του άλφα, επί, συνημίτονο του a")?;
    return Ok(());
}

#[test]
fn uoa_corpus_357() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>sin</mi><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>α</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mn>2</mn><mo>×</mo><mi>sin</mi><mrow><mo>(</mo>
    <mi>α</mi>
   <mo>)</mo></mrow><mo>×</mo><mi>cos</mi><mrow><mo>(</mo>
    <mi>α</mi>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση; ισούται με; 2 επί, ημίτονο του άλφα, επί, συνημίτονο του άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_358() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>α</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mi>α</mi><mo>+</mo><mi>α</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mi>cos</mi><mrow><mo>(</mo>
    <mi>α</mi>
   <mo>)</mo></mrow><mo>×</mo><mi>cos</mi><mrow><mo>(</mo>
    <mi>α</mi>
   <mo>)</mo></mrow><mo>−</mo><mi>sin</mi><mrow><mo>(</mo>
    <mi>α</mi>
   <mo>)</mo></mrow><mo>×</mo><mi>sin</mi><mrow><mo>(</mo>
    <mi>α</mi>
   <mo>)</mo></mrow><mo>=</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>−</mo><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση; ισούται με; συνημίτονο του; ανοίγει παρένθεση; άλφα συν άλφα; κλείνει παρένθεση; ισούται με; συνημίτονο του άλφα; επί, συνημίτονο του άλφα; μείον; ημίτονο του άλφα, επί, ημίτονο του άλφα; ισούται με; συνημίτονο στο τετράγωνο; του άλφα; μείον, ημίτονο στο τετράγωνο, του άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_359() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mo>=</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>−</mo><mo stretchy="false">(</mo><mn>1</mn><mo>−</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo stretchy="false">)</mo><mo>=</mo><mn>2</mn><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>−</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ισούται με; συνημίτονο στο τετράγωνο; του άλφα; μείον; ανοίγει παρένθεση; 1 μείον; συνημίτονο στο τετράγωνο; του άλφα; κλείνει παρένθεση; ισούται με; 2, συνημίτονο στο τετράγωνο; του άλφα; μείον 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_360() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mo>=</mo><mo stretchy="false">(</mo><mn>1</mn><mo>−</mo><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo stretchy="false">)</mo><mo>−</mo><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>=</mo><mn>1</mn><mo>−</mo><mn>2</mn><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ισούται με; ανοίγει παρένθεση; 1 μείον, ημίτονο στο τετράγωνο, του άλφα; κλείνει παρένθεση; μείον, ημίτονο στο τετράγωνο, του άλφα; ισούται με; 1 μείον; 2, ημίτονο στο τετράγωνο, του άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_361() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>α</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>−</mo><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση; ισούται με; συνημίτονο στο τετράγωνο; του άλφα; μείον, ημίτονο στο τετράγωνο, του άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_362() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>α</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>−</mo><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>=</mo><mn>2</mn><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>−</mo><mn>1</mn><mo>=</mo><mn>1</mn><mo>−</mo><mn>2</mn><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση; ισούται με; συνημίτονο στο τετράγωνο; του άλφα; μείον, ημίτονο στο τετράγωνο, του άλφα; ισούται με; 2, συνημίτονο στο τετράγωνο; του άλφα; μείον 1; ισούται με; 1 μείον; 2, ημίτονο στο τετράγωνο, του άλφα")?;
    return Ok(());
}

#[test]
fn uoa_corpus_363() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>tan</mi><mo stretchy="false">(</mo><mn>2</mn><mi>α</mi><mo stretchy="false">)</mo><mo>=</mo><mfrac>
    <mrow>
     <mi>tan</mi><mi>α</mi><mo>+</mo><mi>tan</mi><mi>α</mi></mrow>
    <mrow>
     <mn>1</mn><mo>−</mo><mi>tan</mi><mi>α</mi><mi>tan</mi><mi>α</mi></mrow>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mn>2</mn><mi>tan</mi><mi>α</mi></mrow>
    <mrow>
     <mn>1</mn><mo>−</mo><msup>
      <mrow>
       <mi>tan</mi></mrow>
      <mn>2</mn>
     </msup>
     <mi>α</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "εφαπτομένη του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση; ισούται με; κλάσμα, εφαπτομένη του άλφα; συν, εφαπτομένη του άλφα, προς, 1 μείον; εφαπτομένη του άλφα; εφαπτομένη του άλφα, τέλος κλάσματος; ισούται με; κλάσμα, 2, εφαπτομένη του άλφα, προς, 1 μείον; εφαπτομένη στο τετράγωνο; του άλφα, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_364() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>tan</mi><mo stretchy="false">(</mo><mn>2</mn><mi>α</mi><mo stretchy="false">)</mo><mo>=</mo><mfrac>
    <mrow>
     <mn>2</mn><mi>tan</mi><mi>α</mi></mrow>
    <mrow>
     <mn>1</mn><mo>−</mo><msup>
      <mrow>
       <mi>tan</mi></mrow>
      <mn>2</mn>
     </msup>
     <mi>α</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "εφαπτομένη του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση; ισούται με; κλάσμα, 2, εφαπτομένη του άλφα, προς, 1 μείον; εφαπτομένη στο τετράγωνο; του άλφα, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_365() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mo stretchy="false">(</mo><mn>2</mn><mi>α</mi><mo stretchy="false">)</mo><mo>=</mo><mn>2</mn><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>−</mo><mn>1</mn><mo>⇔</mo><mn>1</mn><mo>+</mo><mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>α</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mn>2</mn><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>⇔</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>=</mo><mfrac>
    <mrow>
     <mn>1</mn><mo>+</mo><mi>cos</mi><mn>2</mn><mi>α</mi></mrow>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση; ισούται με; 2, συνημίτονο στο τετράγωνο; του άλφα; μείον 1; αμφίδρομο διπλό βέλος; 1 συν; συνημίτονο του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση; ισούται με; 2, συνημίτονο στο τετράγωνο; του άλφα; αμφίδρομο διπλό βέλος; συνημίτονο στο τετράγωνο; του άλφα; ισούται με; κλάσμα, 1 συν, συνημίτονο του 2 άλφα, προς 2, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_366() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>α</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mn>1</mn><mo>−</mo><mn>2</mn><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>⇔</mo><mn>2</mn><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>=</mo><mn>1</mn><mo>−</mo><mi>cos</mi><mo stretchy="false">(</mo><mn>2</mn><mi>α</mi><mo stretchy="false">)</mo><mo>⇔</mo><mi>s</mi><mi>i</mi><msup>
    <mi>n</mi>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>=</mo><mfrac>
    <mrow>
     <mn>1</mn><mo>−</mo><mi>cos</mi><mrow><mo>(</mo>
      <mrow>
       <mn>2</mn><mi>α</mi></mrow>
     <mo>)</mo></mrow></mrow>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση; ισούται με; 1 μείον; 2, ημίτονο στο τετράγωνο, του άλφα; αμφίδρομο διπλό βέλος; 2, ημίτονο στο τετράγωνο, του άλφα; ισούται με; 1 μείον; συνημίτονο του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση; αμφίδρομο διπλό βέλος; ημίτονο στο τετράγωνο, του άλφα; ισούται με; κλάσμα, 1 μείον; συνημίτονο του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση, προς 2, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_367() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>α</mi><mo>=</mo><mfrac>
    <mrow>
     <msup>
      <mrow>
       <mi>sin</mi></mrow>
      <mn>2</mn>
     </msup>
     <mi>α</mi></mrow>
    <mrow>
     <msup>
      <mrow>
       <mi>cos</mi></mrow>
      <mn>2</mn>
     </msup>
     <mi>α</mi></mrow>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mfrac>
      <mrow>
       <mn>1</mn><mo>−</mo><mi>cos</mi><mo stretchy="false">(</mo><mn>2</mn><mi>α</mi><mo stretchy="false">)</mo></mrow>
      <mn>2</mn>
     </mfrac>
     </mrow>
    <mrow>
     <mfrac>
      <mrow>
       <mn>1</mn><mo>+</mo><mi>cos</mi><mrow><mo>(</mo>
        <mrow>
         <mn>2</mn><mi>α</mi></mrow>
       <mo>)</mo></mrow></mrow>
      <mn>2</mn>
     </mfrac>
     </mrow>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mn>1</mn><mo>−</mo><mi>cos</mi><mo stretchy="false">(</mo><mn>2</mn><mi>α</mi><mo stretchy="false">)</mo></mrow>
    <mrow>
     <mn>1</mn><mo>+</mo><mi>cos</mi><mo stretchy="false">(</mo><mn>2</mn><mi>α</mi><mo stretchy="false">)</mo></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "εφαπτομένη στο τετράγωνο; του άλφα; ισούται με; κλάσμα, ημίτονο στο τετράγωνο, του άλφα, προς, συνημίτονο στο τετράγωνο; του άλφα, τέλος κλάσματος; ισούται με; κλάσμα, κλάσμα, 1 μείον; συνημίτονο του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση, προς 2, τέλος κλάσματος; προς, κλάσμα, 1 συν; συνημίτονο του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση, προς 2, τέλος κλάσματος; τέλος κλάσματος; ισούται με; κλάσμα, 1 μείον; συνημίτονο του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση, προς, 1 συν; συνημίτονο του; ανοίγει παρένθεση, 2 άλφα, κλείνει παρένθεση, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_368() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mn>22.5</mn><mo>°</mo><mo>=</mo><mfrac>
    <mrow>
     <mn>45</mn><mo>°</mo></mrow>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "22.5 μοίρες; ισούται με; κλάσμα, 45 μοίρες; προς 2, τέλος κλάσματος")?;
    return Ok(());
    //theodora. fails. coverts 22.5 --> 225
}

#[test]
fn uoa_corpus_369() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <mn>22.5</mn><mo>°</mo></mrow>
   <mo>)</mo></mrow><mo>=</mo><mfrac>
    <mrow>
     <mn>1</mn><mo>−</mo><mi>cos</mi><mrow><mo>(</mo>
      <mrow>
       <mn>45</mn><mo>°</mo></mrow>
     <mo>)</mo></mrow></mrow>
    <mn>2</mn>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mn>1</mn><mo>−</mo><mfrac>
      <mrow>
       <msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
      <mn>2</mn>
     </mfrac>
     </mrow>
    <mn>2</mn>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mn>2</mn><mo>−</mo><msqrt>
      <mn>2</mn>
     </msqrt>
     </mrow>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο στο τετράγωνο, του; ανοίγει παρένθεση, 22.5 μοίρες; κλείνει παρένθεση; ισούται με; κλάσμα, 1 μείον; συνημίτονο του; ανοίγει παρένθεση, 45 μοίρες; κλείνει παρένθεση, προς 2, τέλος κλάσματος; ισούται με; κλάσμα, 1 μείον; κλάσμα, η τετραγωνική ρίζα του 2; προς 2, τέλος κλάσματος; προς 2, τέλος κλάσματος; ισούται με; κλάσμα, 2 μείον, η τετραγωνική ρίζα του 2; προς 2, τέλος κλάσματος")?;
    return Ok(());
        //theodora. fails. converts 22.5 to 225

}

#[test]
fn uoa_corpus_370() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mo>⇒</mo><mi>sin</mi><mrow><mo>(</mo>
    <mrow>
     <mn>22.5</mn><mo>°</mo></mrow>
   <mo>)</mo></mrow><mo>=</mo><mfrac>
    <mrow>
     <msqrt>
      <mrow>
       <mn>2</mn><mo>−</mo><msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
     </msqrt>
     </mrow>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "διπλό βέλος προς τα δεξιά; ημίτονο του; ανοίγει παρένθεση, 22.5 μοίρες; κλείνει παρένθεση; ισούται με; κλάσμα, η τετραγωνική ρίζα του 2 μείον, η τετραγωνική ρίζα του 2; τέλος ρίζας; προς 2, τέλος κλάσματος")?;
    return Ok(());
  //theodora. fails. converts 22.5 to 225

}

#[test]
fn uoa_corpus_371() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mo stretchy="false">(</mo><mn>22.5</mn><mo>°</mo><mo stretchy="false">)</mo><mo>=</mo><mfrac>
    <mrow>
     <mn>1</mn><mo>+</mo><mi>cos</mi><mrow><mo>(</mo>
      <mrow>
       <mn>45</mn><mo>°</mo></mrow>
     <mo>)</mo></mrow></mrow>
    <mn>2</mn>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mn>1</mn><mo>+</mo><mfrac>
      <mrow>
       <msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
      <mn>2</mn>
     </mfrac>
     </mrow>
    <mn>2</mn>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mn>2</mn><mo>+</mo><msqrt>
      <mn>2</mn>
     </msqrt>
     </mrow>
    <mn>2</mn>
   </mfrac>
   <mo>⇒</mo><mi>cos</mi><mo stretchy="false">(</mo><mn>22.5</mn><mo>°</mo><mo stretchy="false">)</mo><mo>=</mo><mfrac>
    <mrow>
     <msqrt>
      <mrow>
       <mn>2</mn><mo>+</mo><msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
     </msqrt>
     </mrow>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο στο τετράγωνο; του; ανοίγει παρένθεση, 22.5 μοίρες; κλείνει παρένθεση; ισούται με; κλάσμα, 1 συν; συνημίτονο του; ανοίγει παρένθεση, 45 μοίρες; κλείνει παρένθεση, προς 2, τέλος κλάσματος; ισούται με; κλάσμα, 1 συν; κλάσμα, η τετραγωνική ρίζα του 2; προς 2, τέλος κλάσματος; προς 2, τέλος κλάσματος; ισούται με; κλάσμα, 2 συν, η τετραγωνική ρίζα του 2; προς 2, τέλος κλάσματος; διπλό βέλος προς τα δεξιά; συνημίτονο του; ανοίγει παρένθεση, 22.5 μοίρες; κλείνει παρένθεση; ισούται με; κλάσμα, η τετραγωνική ρίζα του 2 συν, η τετραγωνική ρίζα του 2; τέλος ρίζας; προς 2, τέλος κλάσματος")?;
    return Ok(());
  //theodora. fails. converts 22.5 to 225

}

#[test]
fn uoa_corpus_372() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>tan</mi><mo stretchy="false">(</mo><mn>22.5</mn><mo>°</mo><mo stretchy="false">)</mo><mo>=</mo><mfrac>
    <mrow>
     <msqrt>
      <mrow>
       <mn>2</mn><mo>−</mo><msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
     </msqrt>
     </mrow>
    <mrow>
     <msqrt>
      <mrow>
       <mn>2</mn><mo>+</mo><msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
     </msqrt>
     </mrow>
   </mfrac>
   <mo>=</mo><msqrt>
    <mn>2</mn>
   </msqrt>
   <mo>−</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "εφαπτομένη του; ανοίγει παρένθεση, 22.5 μοίρες; κλείνει παρένθεση; ισούται με; κλάσμα, η τετραγωνική ρίζα του 2 μείον, η τετραγωνική ρίζα του 2; τέλος ρίζας; προς, η τετραγωνική ρίζα του 2 συν, η τετραγωνική ρίζα του 2; τέλος ρίζας; τέλος κλάσματος; ισούται με; η τετραγωνική ρίζα του 2; μείον 1")?;
    return Ok(());
    //theodora. fails. converts 22.5 to 225
}

#[test]
fn uoa_corpus_373() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cot</mi><mo stretchy="false">(</mo><mn>22.5</mn><mo>°</mo><mo stretchy="false">)</mo><mo>=</mo><mfrac>
    <mrow>
     <msqrt>
      <mrow>
       <mn>2</mn><mo>+</mo><msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
     </msqrt>
     </mrow>
    <mrow>
     <msqrt>
      <mrow>
       <mn>2</mn><mo>−</mo><msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
     </msqrt>
     </mrow>
   </mfrac>
   <mo>=</mo><msqrt>
    <mn>2</mn>
   </msqrt>
   <mo>+</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνεφαπτομένη, του; ανοίγει παρένθεση, 22.5 μοίρες; κλείνει παρένθεση; ισούται με; κλάσμα, η τετραγωνική ρίζα του 2 συν, η τετραγωνική ρίζα του 2; τέλος ρίζας; προς, η τετραγωνική ρίζα του 2 μείον, η τετραγωνική ρίζα του 2; τέλος ρίζας; τέλος κλάσματος; ισούται με; την τετραγωνική ρίζα του 2; συν 1")?;
    return Ok(());
    //theodora. converts 22.5 to 225. also fails accusative
}

#[test]
fn uoa_corpus_374() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>p</mi><mo stretchy="false">(</mo><mi>χ</mi><mrow><mo>|</mo><mrow>
    <msub>
     <mi>ω</mi>
     <mi>ι</mi>
    </msub>
    <mo stretchy="false">)</mo></mrow></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "p; ανοίγει παρένθεση; χ; κάθετη γραμμη; ωμέγα δείκτης ιότα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_375() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>θ</mi>
    <mi>i</mi>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "θήτα δείκτης i")?;
    return Ok(());
}

#[test]
fn uoa_corpus_376() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>p</mi><mo stretchy="false">(</mo><mi>χ</mi><mrow><mo>|</mo><mrow>
    <msub>
     <mi>ω</mi>
     <mi>ι</mi>
    </msub>
    <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "p; ανοίγει παρένθεση; χ; κάθετη γραμμη; ωμέγα δείκτης ιότα; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_377() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>χ</mi>
    <mn>1</mn>
   </msub>
   <mo>,</mo><mo>…</mo><mo>,</mo><msub>
    <mi>χ</mi>
    <mi>Ν</mi>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "χ δείκτης 1; κόμμα; αποσιωπητικά, κόμμα; χ δείκτης, κεφαλαίο νί")?;
    return Ok(());
}

#[test]
fn uoa_corpus_378() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>p</mi><mo stretchy="false">(</mo><mi>Χ</mi><mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "p; ανοίγει παρένθεση; κεφαλαίο χ, ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_379() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>p</mi><mo stretchy="false">(</mo><mi>Χ</mi><mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo><mo>=</mo><mi>p</mi><mo stretchy="false">(</mo><msub>
    <mi>χ</mi>
    <mn>1</mn>
   </msub>
   <mo>,</mo><msub>
    <mi>χ</mi>
    <mn>2</mn>
   </msub>
   <mo>,</mo><mo>…</mo><mo>,</mo><msub>
    <mi>χ</mi>
    <mi>Ν</mi>
   </msub>
   <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo><mo>=</mo><mstyle displaystyle="true">
    <munderover>
     <mo>∏</mo>
     <mrow>
      <mi>κ</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>Ν</mi>
    </munderover>
    <mrow>
     <mi>p</mi><mo stretchy="false">(</mo><msub>
      <mi>χ</mi>
      <mi>κ</mi>
     </msub>
     <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "p; ανοίγει παρένθεση; κεφαλαίο χ, ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση; ισούται με; p του; ανοίγει παρένθεση; χ δείκτης 1; κόμμα; χ δείκτης 2; κόμμα; αποσιωπητικά, κόμμα; χ δείκτης, κεφαλαίο νί; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση; ισούται με; γινόμενο από καπα ισούται με 1 ως κεφαλαίο νί του; p; ανοίγει παρένθεση; χ δείκτης καπα; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_380() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>θ</mi>
    <mrow>
     <mi>M</mi><mi>L</mi></mrow>
   </msub>
   <mo>=</mo><mi>arg</mi><msub>
    <mrow>
     <mi>max</mi></mrow>
    <mi>θ</mi>
   </msub>
   <mstyle displaystyle="true">
    <munderover>
     <mo>∏</mo>
     <mrow>
      <mi>κ</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>Ν</mi>
    </munderover>
    <mrow>
     <mi>p</mi><mo stretchy="false">(</mo><msub>
      <mi>χ</mi>
      <mi>κ</mi>
     </msub>
     <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "θήτα δείκτης κεφαλαίο m κεφαλαίο l; ισούται με; όρισμα του, μέγιστο δείκτης θήτα; του; γινόμενο από καπα ισούται με 1 ως κεφαλαίο νί του; p; ανοίγει παρένθεση; χ δείκτης καπα; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_381() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <mo>∂</mo><mstyle displaystyle="true">
      <munderover>
       <mo>∏</mo>
       <mrow>
        <mi>κ</mi><mo>=</mo><mn>1</mn></mrow>
       <mi>Ν</mi>
      </munderover>
      <mrow>
       <mi>p</mi><mo stretchy="false">(</mo><msub>
        <mi>χ</mi>
        <mi>κ</mi>
       </msub>
       <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
     </mstyle></mrow>
    <mrow>
     <mo>∂</mo><mi>θ</mi></mrow>
   </mfrac>
   <mo>=</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, μερικό διαφορικό; γινόμενο από καπα ισούται με 1 ως κεφαλαίο νί του; p; ανοίγει παρένθεση; χ δείκτης καπα; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση, προς, μερικό διαφορικό, θήτα, τέλος κλάσματος; ισούται με 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_382() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>L</mi><mo stretchy="false">(</mo><mi>θ</mi><mo stretchy="false">)</mo><mo>=</mo><mi>ln</mi><mstyle displaystyle="true">
    <munderover>
     <mo>∏</mo>
     <mrow>
      <mi>κ</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>Ν</mi>
    </munderover>
    <mrow>
     <mi>p</mi><mo stretchy="false">(</mo><msub>
      <mi>χ</mi>
      <mi>κ</mi>
     </msub>
     <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο l του θήτα; ισούται με; το φυσικό λογάριθμο, του; γινόμενο από καπα ισούται με 1 ως κεφαλαίο νί του; p; ανοίγει παρένθεση; χ δείκτης καπα; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_383() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <mo>∂</mo><mi>L</mi><mo stretchy="false">(</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
    <mrow>
     <mo>∂</mo><mi>θ</mi></mrow>
   </mfrac>
   <mo>=</mo><mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>κ</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>Ν</mi>
    </munderover>
    <mrow>
     <mfrac>
      <mrow>
       <mo>∂</mo><mi>ln</mi><mi>p</mi><mo stretchy="false">(</mo><msub>
        <mi>χ</mi>
        <mi>κ</mi>
       </msub>
       <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
      <mrow>
       <mo>∂</mo><mi>θ</mi></mrow>
     </mfrac>
     </mrow>
   </mstyle><mo>=</mo><mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>κ</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>Ν</mi>
    </munderover>
    <mrow>
     <mfrac>
      <mn>1</mn>
      <mrow>
       <mi>p</mi><mo stretchy="false">(</mo><msub>
        <mi>χ</mi>
        <mi>κ</mi>
       </msub>
       <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
     </mfrac>
     <mo>×</mo><mfrac>
      <mrow>
       <mo>∂</mo><mi>p</mi><mo stretchy="false">(</mo><msub>
        <mi>χ</mi>
        <mi>κ</mi>
       </msub>
       <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
      <mrow>
       <mo>∂</mo><mi>θ</mi></mrow>
     </mfrac>
     </mrow>
   </mstyle><mo>=</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, μερικό διαφορικό; κεφαλαίο l του θήτα, προς, μερικό διαφορικό, θήτα, τέλος κλάσματος; ισούται με; άθροισμα από καπα ισούται με 1 ως κεφαλαίο νί του; κλάσμα, μερικό διαφορικό; ο φυσικός λογάριθμος, του p; ανοίγει παρένθεση; χ δείκτης καπα; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση, προς, μερικό διαφορικό, θήτα, τέλος κλάσματος; ισούται με; άθροισμα από καπα ισούται με 1 ως κεφαλαίο νί του; κλάσμα, 1 προς, p; ανοίγει παρένθεση; χ δείκτης καπα; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση, τέλος κλάσματος; επί; κλάσμα, μερικό διαφορικό p; ανοίγει παρένθεση; χ δείκτης καπα; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση, προς, μερικό διαφορικό, θήτα, τέλος κλάσματος; ισούται με 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_384() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>a</mi><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mi>b</mi><mo>+</mo><mi>c</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "a επί; ανοίγει παρένθεση, b συν c, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_385() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>3</mn><mo>×</mo><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   <mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><msup>
      <mi>x</mi>
      <mn>3</mn>
     </msup>
     <mo>+</mo><mn>6</mn><mi>x</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "3 επί x στο τετράγωνο, επί; ανοίγει παρένθεση; 2 x στον κύβο, συν 6 x; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_386() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mi>a</mi><mo>+</mo><mi>b</mi></mrow>
   <mo>)</mo></mrow><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mi>c</mi><mo>+</mo><mi>d</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση, a συν b, κλείνει παρένθεση; επί; ανοίγει παρένθεση, 100 συν 500, κλείνει παρένθεση")?;
    return Ok(());
    //theodora. fails. For some reason it recognizes c + d as 100 + 500
  //does the same in english
}

#[test]
fn uoa_corpus_387() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mn>3</mn><msup>
      <mi>x</mi>
      <mn>2</mn>
     </msup>
     <mi>ψ</mi><mo>+</mo><mn>2</mn><mi>ψ</mi></mrow>
   <mo>)</mo></mrow><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><msup>
      <mi>x</mi>
      <mn>2</mn>
     </msup>
     <mo>+</mo><mn>5</mn></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση; 3 x στο τετράγωνο, ψ; συν 2 ψ; κλείνει παρένθεση; επί; ανοίγει παρένθεση; 2 x στο τετράγωνο, συν 5; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_388() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>3</mn><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   <mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><msup>
      <mi>x</mi>
      <mn>3</mn>
     </msup>
     <mo>+</mo><mn>6</mn><mi>x</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "3 x στο τετράγωνο, επί; ανοίγει παρένθεση; 2 x στον κύβο, συν 6 x; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_389() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>3</mn><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "3 x στο τετράγωνο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_390() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>2</mn><msup>
    <mi>x</mi>
    <mn>3</mn>
   </msup>
   <mo>+</mo><mn>6</mn><mi>x</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 x στον κύβο, συν 6 x")?;
    return Ok(());
}

#[test]
fn uoa_corpus_391() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mn>3</mn><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   <mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><msup>
      <mi>x</mi>
      <mn>3</mn>
     </msup>
     <mo>+</mo><mn>6</mn><mi>x</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mn>3</mn><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   <mo>×</mo><mn>2</mn><msup>
    <mi>x</mi>
    <mn>3</mn>
   </msup>
   <mo>+</mo><mn>3</mn><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   <mo>×</mo><mn>6</mn><mi>x</mi><mo>=</mo><mn>6</mn><msup>
    <mi>x</mi>
    <mn>5</mn>
   </msup>
   <mo>+</mo><mn>18</mn><msup>
    <mi>x</mi>
    <mn>3</mn>
   </msup>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "3 x στο τετράγωνο, επί; ανοίγει παρένθεση; 2 x στον κύβο, συν 6 x; κλείνει παρένθεση; ισούται με; 3 x στο τετράγωνο, επί 2 x στον κύβο; συν, 3 x στο τετράγωνο, επί 6 x; ισούται με; 6 x στην πέμπτη, συν, 18 x στον κύβο")?;
    return Ok(());
}

#[test]
fn uoa_corpus_392() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mn>3</mn><msup>
      <mi>x</mi>
      <mn>2</mn>
     </msup>
     <mi>y</mi><mo>+</mo><mn>2</mn><mi>y</mi></mrow>
   <mo>)</mo></mrow><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><msup>
      <mi>x</mi>
      <mn>2</mn>
     </msup>
     <mo>+</mo><mn>5</mn></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση; 3 x στο τετράγωνο, y; συν 2 y; κλείνει παρένθεση; επί; ανοίγει παρένθεση; 2 x στο τετράγωνο, συν 5; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_393() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>3</mn><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   <mi>y</mi><mo>+</mo><mn>2</mn><mi>y</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "3 x στο τετράγωνο, y; συν 2 y")?;
    return Ok(());
}

#[test]
fn uoa_corpus_394() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>2</mn><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   <mo>+</mo><mn>5</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 x στο τετράγωνο, συν 5")?;
    return Ok(());
}

#[test]
fn uoa_corpus_395() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mn>3</mn><msup>
      <mi>x</mi>
      <mn>2</mn>
     </msup>
     <mi>y</mi><mo>+</mo><mn>2</mn><mi>y</mi></mrow>
   <mo>)</mo></mrow><mo>×</mo><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><msup>
      <mi>x</mi>
      <mn>2</mn>
     </msup>
     <mo>+</mo><mn>5</mn></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ανοίγει παρένθεση; 3 x στο τετράγωνο, y; συν 2 y; κλείνει παρένθεση; επί; ανοίγει παρένθεση; 2 x στο τετράγωνο, συν 5; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_396() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mo>=</mo><mn>3</mn><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   <mi>y</mi><mo>×</mo><mn>2</mn><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   <mo>+</mo><mn>3</mn><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   <mi>y</mi><mo>×</mo><mn>5</mn><mo>+</mo><mn>2</mn><mi>y</mi><mo>×</mo><mn>2</mn><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   <mo>+</mo><mn>2</mn><mi>y</mi><mo>×</mo><mn>5</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ισούται με; 3 x στο τετράγωνο, y επί 2 x στο τετράγωνο; συν, 3 x στο τετράγωνο, y επί 5; συν, 2 y επί 2 x στο τετράγωνο; συν 2 y επί 5")?;
    return Ok(());
}

#[test]
fn uoa_corpus_397() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mo>=</mo><mn>6</mn><msup>
    <mi>x</mi>
    <mn>4</mn>
   </msup>
   <mi>y</mi><mo>+</mo><mn>15</mn><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   <mi>y</mi><mo>+</mo><mn>4</mn><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   <mi>y</mi><mo>+</mo><mn>10</mn><mi>y</mi><mo>=</mo><mn>6</mn><msup>
    <mi>x</mi>
    <mn>4</mn>
   </msup>
   <mi>y</mi><mo>+</mo><mn>19</mn><msup>
    <mi>x</mi>
    <mn>2</mn>
   </msup>
   <mi>y</mi><mo>+</mo><mn>10</mn><mi>y</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ισούται με; 6 x στην τέταρτη, y; συν, 15 x στο τετράγωνο, y; συν, 4 x στο τετράγωνο, y; συν 10 y; ισούται με; 6 x στην τέταρτη, y; συν, 19 x στο τετράγωνο, y; συν 10 y")?;
    return Ok(());
}

#[test]
fn uoa_corpus_398() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mn>2</mn><mi>a</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "2 a")?;
    return Ok(());
}

#[test]
fn uoa_corpus_399() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>sin</mi><mrow><mo>(</mo>
    <mrow>
     <mi>a</mi><mo>+</mo><mi>b</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο του; ανοίγει παρένθεση, a συν b, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_400() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mi>a</mi><mo>+</mo><mi>b</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του; ανοίγει παρένθεση, a συν b, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_401() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>tan</mi><mrow><mo>(</mo>
    <mrow>
     <mi>a</mi><mo>+</mo><mi>b</mi></mrow>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "εφαπτομένη του; ανοίγει παρένθεση, a συν b, κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_402() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>sin</mi><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>a</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mi>sin</mi><mrow><mo>(</mo>
    <mrow>
     <mi>a</mi><mo>+</mo><mi>a</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mi>sin</mi><mrow><mo>(</mo>
    <mi>a</mi>
   <mo>)</mo></mrow><mo>×</mo><mi>cos</mi><mrow><mo>(</mo>
    <mi>a</mi>
   <mo>)</mo></mrow><mo>+</mo><mi>cos</mi><mrow><mo>(</mo>
    <mi>a</mi>
   <mo>)</mo></mrow><mo>×</mo><mi>sin</mi><mrow><mo>(</mo>
    <mi>a</mi>
   <mo>)</mo></mrow><mo>=</mo><mn>2</mn><mo>×</mo><mi>sin</mi><mrow><mo>(</mo>
    <mi>a</mi>
   <mo>)</mo></mrow><mo>×</mo><mi>cos</mi><mrow><mo>(</mo>
    <mi>a</mi>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο του 2 a; ισούται με; ημίτονο του; ανοίγει παρένθεση, a συν a, κλείνει παρένθεση; ισούται με; ημίτονο του a, επί, συνημίτονο του a; συν; συνημίτονο του a, επί ημίτονο του a; ισούται με; 2 επί ημίτονο του a, επί, συνημίτονο του a")?;
    return Ok(());
}

#[test]
fn uoa_corpus_403() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>sin</mi><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>a</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mn>2</mn><mo>×</mo><mi>sin</mi><mrow><mo>(</mo>
    <mi>a</mi>
   <mo>)</mo></mrow><mo>×</mo><mi>cos</mi><mrow><mo>(</mo>
    <mi>a</mi>
   <mo>)</mo></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο του 2 a; ισούται με; 2 επί ημίτονο του a, επί, συνημίτονο του a")?;
    return Ok(());
}

#[test]
fn uoa_corpus_404() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>a</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mi>a</mi><mo>+</mo><mi>a</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mi>cos</mi><mrow><mo>(</mo>
    <mi>a</mi>
   <mo>)</mo></mrow><mo>×</mo><mi>cos</mi><mrow><mo>(</mo>
    <mi>a</mi>
   <mo>)</mo></mrow><mo>−</mo><mi>sin</mi><mrow><mo>(</mo>
    <mi>a</mi>
   <mo>)</mo></mrow><mo>×</mo><mi>sin</mi><mrow><mo>(</mo>
    <mi>a</mi>
   <mo>)</mo></mrow><mo>=</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>−</mo><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του 2 a; ισούται με; συνημίτονο του; ανοίγει παρένθεση, a συν a, κλείνει παρένθεση; ισούται με; συνημίτονο του a, επί, συνημίτονο του a; μείον; ημίτονο του a, επί ημίτονο του a; ισούται με; συνημίτονο στο τετράγωνο; του a; μείον, ημίτονο στο τετράγωνο, του a")?;
    return Ok(());
}

#[test]
fn uoa_corpus_405() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mo>=</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>−</mo><mo stretchy="false">(</mo><mn>1</mn><mo>−</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo stretchy="false">)</mo><mo>=</mo><mn>2</mn><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>−</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ισούται με; συνημίτονο στο τετράγωνο; του a; μείον; ανοίγει παρένθεση; 1 μείον, συνημίτονο στο τετράγωνο; του a; κλείνει παρένθεση; ισούται με; 2, συνημίτονο στο τετράγωνο; του a; μείον 1")?;
    return Ok(());
}

#[test]
fn uoa_corpus_406() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mo>=</mo><mo stretchy="false">(</mo><mn>1</mn><mo>−</mo><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo stretchy="false">)</mo><mo>−</mo><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>=</mo><mn>1</mn><mo>−</mo><mn>2</mn><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ισούται με; ανοίγει παρένθεση; 1 μείον, ημίτονο στο τετράγωνο, του a; κλείνει παρένθεση; μείον, ημίτονο στο τετράγωνο, του a; ισούται με; 1 μείον; 2, ημίτονο στο τετράγωνο, του a")?;
    return Ok(());
}

#[test]
fn uoa_corpus_407() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>a</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>−</mo><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του 2 a; ισούται με; συνημίτονο στο τετράγωνο; του a; μείον, ημίτονο στο τετράγωνο, του a")?;
    return Ok(());
}

#[test]
fn uoa_corpus_408() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>a</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>−</mo><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>=</mo><mn>2</mn><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>−</mo><mn>1</mn><mo>=</mo><mn>1</mn><mo>−</mo><mn>2</mn><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του 2 a; ισούται με; συνημίτονο στο τετράγωνο; του a; μείον, ημίτονο στο τετράγωνο, του a; ισούται με; 2, συνημίτονο στο τετράγωνο; του a; μείον 1; ισούται με; 1 μείον; 2, ημίτονο στο τετράγωνο, του a")?;
    return Ok(());
}

#[test]
fn uoa_corpus_409() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>tan</mi><mo stretchy="false">(</mo><mn>2</mn><mi>a</mi><mo stretchy="false">)</mo><mo>=</mo><mfrac>
    <mrow>
     <mi>tan</mi><mi>a</mi><mo>+</mo><mi>tan</mi><mi>a</mi></mrow>
    <mrow>
     <mn>1</mn><mo>−</mo><mi>tan</mi><mi>a</mi><mi>tan</mi><mi>a</mi></mrow>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mn>2</mn><mi>tan</mi><mi>a</mi></mrow>
    <mrow>
     <mn>1</mn><mo>−</mo><msup>
      <mrow>
       <mi>tan</mi></mrow>
      <mn>2</mn>
     </msup>
     <mi>a</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "εφαπτομένη του 2 a; ισούται με; κλάσμα, εφαπτομένη του a, συν, εφαπτομένη του a, προς, 1 μείον; εφαπτομένη του a; εφαπτομένη του a, τέλος κλάσματος; ισούται με; κλάσμα, 2, εφαπτομένη του a, προς, 1 μείον, εφαπτομένη στο τετράγωνο; του a, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_410() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>tan</mi><mo stretchy="false">(</mo><mn>2</mn><mi>a</mi><mo stretchy="false">)</mo><mo>=</mo><mfrac>
    <mrow>
     <mn>2</mn><mi>tan</mi><mi>a</mi></mrow>
    <mrow>
     <mn>1</mn><mo>−</mo><msup>
      <mrow>
       <mi>tan</mi></mrow>
      <mn>2</mn>
     </msup>
     <mi>a</mi></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "εφαπτομένη του 2 a; ισούται με; κλάσμα, 2, εφαπτομένη του a, προς, 1 μείον, εφαπτομένη στο τετράγωνο; του a, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_411() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mo stretchy="false">(</mo><mn>2</mn><mi>a</mi><mo stretchy="false">)</mo><mo>=</mo><mn>2</mn><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>−</mo><mn>1</mn><mo>⇔</mo><mn>1</mn><mo>+</mo><mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>a</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mn>2</mn><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>⇔</mo><msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>=</mo><mfrac>
    <mrow>
     <mn>1</mn><mo>+</mo><mi>cos</mi><mn>2</mn><mi>a</mi></mrow>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του 2 a; ισούται με; 2, συνημίτονο στο τετράγωνο; του a; μείον 1; αμφίδρομο διπλό βέλος; 1 συν, συνημίτονο του 2 a; ισούται με; 2, συνημίτονο στο τετράγωνο; του a; αμφίδρομο διπλό βέλος; συνημίτονο στο τετράγωνο; του a; ισούται με; κλάσμα, 1 συν, συνημίτονο του 2 a, προς 2, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_412() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cos</mi><mrow><mo>(</mo>
    <mrow>
     <mn>2</mn><mi>a</mi></mrow>
   <mo>)</mo></mrow><mo>=</mo><mn>1</mn><mo>−</mo><mn>2</mn><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>⇔</mo><mn>2</mn><msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>=</mo><mn>1</mn><mo>−</mo><mi>cos</mi><mo stretchy="false">(</mo><mn>2</mn><mi>a</mi><mo stretchy="false">)</mo><mo>⇔</mo><mi>s</mi><mi>i</mi><msup>
    <mi>n</mi>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>=</mo><mfrac>
    <mrow>
     <mn>1</mn><mo>−</mo><mi>cos</mi><mrow><mo>(</mo>
      <mrow>
       <mn>2</mn><mi>a</mi></mrow>
     <mo>)</mo></mrow></mrow>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο του 2 a; ισούται με; 1 μείον; 2, ημίτονο στο τετράγωνο, του a; αμφίδρομο διπλό βέλος; 2, ημίτονο στο τετράγωνο, του a; ισούται με; 1 μείον, συνημίτονο του 2 a; αμφίδρομο διπλό βέλος; ημίτονο στο τετράγωνο, του a; ισούται με; κλάσμα, 1 μείον, συνημίτονο του 2 a, προς 2, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_413() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>tan</mi></mrow>
    <mn>2</mn>
   </msup>
   <mi>a</mi><mo>=</mo><mfrac>
    <mrow>
     <msup>
      <mrow>
       <mi>sin</mi></mrow>
      <mn>2</mn>
     </msup>
     <mi>a</mi></mrow>
    <mrow>
     <msup>
      <mrow>
       <mi>cos</mi></mrow>
      <mn>2</mn>
     </msup>
     <mi>a</mi></mrow>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mfrac>
      <mrow>
       <mn>1</mn><mo>−</mo><mi>cos</mi><mo stretchy="false">(</mo><mn>2</mn><mi>a</mi><mo stretchy="false">)</mo></mrow>
      <mn>2</mn>
     </mfrac>
     </mrow>
    <mrow>
     <mfrac>
      <mrow>
       <mn>1</mn><mo>+</mo><mi>cos</mi><mrow><mo>(</mo>
        <mrow>
         <mn>2</mn><mi>a</mi></mrow>
       <mo>)</mo></mrow></mrow>
      <mn>2</mn>
     </mfrac>
     </mrow>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mn>1</mn><mo>−</mo><mi>cos</mi><mo stretchy="false">(</mo><mn>2</mn><mi>a</mi><mo stretchy="false">)</mo></mrow>
    <mrow>
     <mn>1</mn><mo>+</mo><mi>cos</mi><mo stretchy="false">(</mo><mn>2</mn><mi>a</mi><mo stretchy="false">)</mo></mrow>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "εφαπτομένη στο τετράγωνο; του a; ισούται με; κλάσμα, ημίτονο στο τετράγωνο, του a, προς, συνημίτονο στο τετράγωνο; του a, τέλος κλάσματος; ισούται με; κλάσμα, κλάσμα, 1 μείον, συνημίτονο του 2 a, προς 2, τέλος κλάσματος; προς, κλάσμα, 1 συν, συνημίτονο του 2 a, προς 2, τέλος κλάσματος; τέλος κλάσματος; ισούται με; κλάσμα, 1 μείον, συνημίτονο του 2 a, προς, 1 συν, συνημίτονο του 2 a, τέλος κλάσματος")?;
    return Ok(());
}

#[test]
fn uoa_corpus_414() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mn>22.5</mn><mo>°</mo><mo>=</mo><mfrac>
    <mrow>
     <mn>45</mn><mo>°</mo></mrow>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    // TODO: add expected speech
    test("el", "SimpleSpeak", expr, "22.5 μοίρες; ισούται με; κλάσμα, 45 μοίρες; προς 2, τέλος κλάσματος")?;
    return Ok(());
    //theodora. fails. converts 22.5 to 225
}

#[test]
fn uoa_corpus_415() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>sin</mi></mrow>
    <mn>2</mn>
   </msup>
   <mrow><mo>(</mo>
    <mrow>
     <mn>22.5</mn><mo>°</mo></mrow>
   <mo>)</mo></mrow><mo>=</mo><mfrac>
    <mrow>
     <mn>1</mn><mo>−</mo><mi>cos</mi><mrow><mo>(</mo>
      <mrow>
       <mn>45</mn><mo>°</mo></mrow>
     <mo>)</mo></mrow></mrow>
    <mn>2</mn>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mn>1</mn><mo>−</mo><mfrac>
      <mrow>
       <msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
      <mn>2</mn>
     </mfrac>
     </mrow>
    <mn>2</mn>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mn>2</mn><mo>−</mo><msqrt>
      <mn>2</mn>
     </msqrt>
     </mrow>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "ημίτονο στο τετράγωνο, του; ανοίγει παρένθεση, 22.5 μοίρες; κλείνει παρένθεση; ισούται με; κλάσμα, 1 μείον; συνημίτονο του; ανοίγει παρένθεση, 45 μοίρες; κλείνει παρένθεση, προς 2, τέλος κλάσματος; ισούται με; κλάσμα, 1 μείον; κλάσμα, η τετραγωνική ρίζα του 2; προς 2, τέλος κλάσματος; προς 2, τέλος κλάσματος; ισούται με; κλάσμα, 2 μείον, η τετραγωνική ρίζα του 2; προς 2, τέλος κλάσματος")?;
    return Ok(());
    //theodora. fails. converts 225 --> 225
}

#[test]
fn uoa_corpus_416() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mo>⇒</mo><mi>sin</mi><mrow><mo>(</mo>
    <mrow>
     <mn>22.5</mn><mo>°</mo></mrow>
   <mo>)</mo></mrow><mo>=</mo><mfrac>
    <mrow>
     <msqrt>
      <mrow>
       <mn>2</mn><mo>−</mo><msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
     </msqrt>
     </mrow>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "διπλό βέλος προς τα δεξιά; ημίτονο του; ανοίγει παρένθεση, 22.5 μοίρες; κλείνει παρένθεση; ισούται με; κλάσμα, η τετραγωνική ρίζα του 2 μείον, η τετραγωνική ρίζα του 2; τέλος ρίζας; προς 2, τέλος κλάσματος")?;
    return Ok(());
    //theodora. fails. converts 22.5 -->225
}


#[test]
fn uoa_corpus_417() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msup>
    <mrow>
     <mi>cos</mi></mrow>
    <mn>2</mn>
   </msup>
   <mo stretchy="false">(</mo><mn>22.5</mn><mo>°</mo><mo stretchy="false">)</mo><mo>=</mo><mfrac>
    <mrow>
     <mn>1</mn><mo>+</mo><mi>cos</mi><mrow><mo>(</mo>
      <mrow>
       <mn>45</mn><mo>°</mo></mrow>
     <mo>)</mo></mrow></mrow>
    <mn>2</mn>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mn>1</mn><mo>+</mo><mfrac>
      <mrow>
       <msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
      <mn>2</mn>
     </mfrac>
     </mrow>
    <mn>2</mn>
   </mfrac>
   <mo>=</mo><mfrac>
    <mrow>
     <mn>2</mn><mo>+</mo><msqrt>
      <mn>2</mn>
     </msqrt>
     </mrow>
    <mn>2</mn>
   </mfrac>
   <mo>⇒</mo><mi>cos</mi><mo stretchy="false">(</mo><mn>22.5</mn><mo>°</mo><mo stretchy="false">)</mo><mo>=</mo><mfrac>
    <mrow>
     <msqrt>
      <mrow>
       <mn>2</mn><mo>+</mo><msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
     </msqrt>
     </mrow>
    <mn>2</mn>
   </mfrac>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνημίτονο στο τετράγωνο; του; ανοίγει παρένθεση, 22.5 μοίρες; κλείνει παρένθεση; ισούται με; κλάσμα, 1 συν; συνημίτονο του; ανοίγει παρένθεση, 45 μοίρες; κλείνει παρένθεση, προς 2, τέλος κλάσματος; ισούται με; κλάσμα, 1 συν; κλάσμα, η τετραγωνική ρίζα του 2; προς 2, τέλος κλάσματος; προς 2, τέλος κλάσματος; ισούται με; κλάσμα, 2 συν, η τετραγωνική ρίζα του 2; προς 2, τέλος κλάσματος; διπλό βέλος προς τα δεξιά; συνημίτονο του; ανοίγει παρένθεση, 225 μοίρες; κλείνει παρένθεση; ισούται με; κλάσμα, η τετραγωνική ρίζα του 2 συν, η τετραγωνική ρίζα του 2; τέλος ρίζας; προς 2, τέλος κλάσματος")?;
    return Ok(());
    //theodora. fails. converts 22.5 -->225

}

#[test]
fn uoa_corpus_418() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>tan</mi><mo stretchy="false">(</mo><mn>22.5</mn><mo>°</mo><mo stretchy="false">)</mo><mo>=</mo><mfrac>
    <mrow>
     <msqrt>
      <mrow>
       <mn>2</mn><mo>−</mo><msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
     </msqrt>
     </mrow>
    <mrow>
     <msqrt>
      <mrow>
       <mn>2</mn><mo>+</mo><msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
     </msqrt>
     </mrow>
   </mfrac>
   <mo>=</mo><msqrt>
    <mn>2</mn>
   </msqrt>
   <mo>−</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "εφαπτομένη του; ανοίγει παρένθεση, 22.5 μοίρες; κλείνει παρένθεση; ισούται με; κλάσμα, η τετραγωνική ρίζα του 2 μείον, η τετραγωνική ρίζα του 2; τέλος ρίζας; προς, η τετραγωνική ρίζα του 2 συν, η τετραγωνική ρίζα του 2; τέλος ρίζας; τέλος κλάσματος; ισούται με; την τετραγωνική ρίζα του 2; μείον 1")?;
    return Ok(());
    //theodora. fails. converts 22.5 --> 225. fails accusative rule
}

#[test]
fn uoa_corpus_419() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>cot</mi><mo stretchy="false">(</mo><mn>22.5</mn><mo>°</mo><mo stretchy="false">)</mo><mo>=</mo><mfrac>
    <mrow>
     <msqrt>
      <mrow>
       <mn>2</mn><mo>+</mo><msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
     </msqrt>
     </mrow>
    <mrow>
     <msqrt>
      <mrow>
       <mn>2</mn><mo>−</mo><msqrt>
        <mn>2</mn>
       </msqrt>
       </mrow>
     </msqrt>
     </mrow>
   </mfrac>
   <mo>=</mo><msqrt>
    <mn>2</mn>
   </msqrt>
   <mo>+</mo><mn>1</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "συνεφαπτομένη, του; ανοίγει παρένθεση, 22.5 μοίρες; κλείνει παρένθεση; ισούται με; κλάσμα, η τετραγωνική ρίζα του 2 συν, η τετραγωνική ρίζα του 2; τέλος ρίζας; προς, η τετραγωνική ρίζα του 2 μείον, η τετραγωνική ρίζα του 2; τέλος ρίζας; τέλος κλάσματος; ισούται με; την τετραγωνική ρίζα του 2; συν 1")?;
    return Ok(());
 //theodora. fails. converts 22.5 to 225. also fails accusative rule

}

#[test]
fn uoa_corpus_420() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>p</mi><mo stretchy="false">(</mo><mi>x</mi><mrow><mo>|</mo><mrow>
    <msub>
     <mi>ω</mi>
     <mi>i</mi>
    </msub>
    <mo stretchy="false">)</mo></mrow></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "p; ανοίγει παρένθεση; x; κάθετη γραμμη; ωμέγα δείκτης i; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_421() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>θ</mi>
    <mi>i</mi>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "θήτα δείκτης i")?;
    return Ok(());
}

#[test]
fn uoa_corpus_422() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>p</mi><mo stretchy="false">(</mo><mi>x</mi><mrow><mo>|</mo><mrow>
    <msub>
     <mi>ω</mi>
     <mi>i</mi>
    </msub>
    <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow></mrow></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "p; ανοίγει παρένθεση; x; κάθετη γραμμη; ωμέγα δείκτης i; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_423() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>x</mi>
    <mn>1</mn>
   </msub>
   <mo>,</mo><mo>…</mo><mo>,</mo><msub>
    <mi>x</mi>
    <mi>Ν</mi>
   </msub>
   </mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "x δείκτης 1; κόμμα; αποσιωπητικά, κόμμα; x δείκτης, κεφαλαίο νί")?;
    return Ok(());
}

#[test]
fn uoa_corpus_424() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>p</mi><mo stretchy="false">(</mo><mi>X</mi><mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "p; ανοίγει παρένθεση; κεφαλαίο x, ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_425() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <mi>p</mi><mo stretchy="false">(</mo><mi>X</mi><mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo><mo>=</mo><mi>p</mi><mo stretchy="false">(</mo><msub>
    <mi>x</mi>
    <mn>1</mn>
   </msub>
   <mo>,</mo><msub>
    <mi>x</mi>
    <mn>2</mn>
   </msub>
   <mo>,</mo><mo>…</mo><mo>,</mo><msub>
    <mi>x</mi>
    <mi>N</mi>
   </msub>
   <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo><mo>=</mo><mstyle displaystyle="true">
    <munderover>
     <mo>∏</mo>
     <mrow>
      <mi>k</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>N</mi>
    </munderover>
    <mrow>
     <mi>p</mi><mo stretchy="false">(</mo><msub>
      <mi>x</mi>
      <mi>k</mi>
     </msub>
     <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "p; ανοίγει παρένθεση; κεφαλαίο x, ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση; ισούται με; p του; ανοίγει παρένθεση; x δείκτης 1; κόμμα; x δείκτης 2; κόμμα; αποσιωπητικά, κόμμα; x δείκτης κεφαλαίο n; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση; ισούται με; γινόμενο από k ισούται με 1 ως κεφαλαίο n του; p; ανοίγει παρένθεση; x δείκτης k; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_426() -> Result<()> { // same
    let expr = r#"<math>
 <mrow>
   <msub>
    <mi>θ</mi>
    <mrow>
     <mi>M</mi><mi>L</mi></mrow>
   </msub>
   <mo>=</mo><mi>arg</mi><msub>
    <mrow>
     <mi>max</mi></mrow>
    <mi>θ</mi>
   </msub>
   <mstyle displaystyle="true">
    <munderover>
     <mo>∏</mo>
     <mrow>
      <mi>k</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>N</mi>
    </munderover>
    <mrow>
     <mi>p</mi><mo stretchy="false">(</mo><msub>
      <mi>x</mi>
      <mi>k</mi>
     </msub>
     <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "θήτα δείκτης κεφαλαίο m κεφαλαίο l; ισούται με; όρισμα του, μέγιστο δείκτης θήτα; του; γινόμενο από k ισούται με 1 ως κεφαλαίο n του; p; ανοίγει παρένθεση; x δείκτης k; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_427() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <mo>∂</mo><mstyle displaystyle="true">
      <munderover>
       <mo>∏</mo>
       <mrow>
        <mi>k</mi><mo>=</mo><mn>1</mn></mrow>
       <mi>N</mi>
      </munderover>
      <mrow>
       <mi>p</mi><mo stretchy="false">(</mo><msub>
        <mi>x</mi>
        <mi>k</mi>
       </msub>
       <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
     </mstyle></mrow>
    <mrow>
     <mo>∂</mo><mi>θ</mi></mrow>
   </mfrac>
   <mo>=</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, μερικό διαφορικό; γινόμενο από k ισούται με 1 ως κεφαλαίο n του; p; ανοίγει παρένθεση; x δείκτης k; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση, προς, μερικό διαφορικό, θήτα, τέλος κλάσματος; ισούται με 0")?;
    return Ok(());
}

#[test]
fn uoa_corpus_428() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mi>L</mi><mo stretchy="false">(</mo><mi>θ</mi><mo stretchy="false">)</mo><mo>=</mo><mi>ln</mi><mstyle displaystyle="true">
    <munderover>
     <mo>∏</mo>
     <mrow>
      <mi>k</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>N</mi>
    </munderover>
    <mrow>
     <mi>p</mi><mo stretchy="false">(</mo><msub>
      <mi>x</mi>
      <mi>k</mi>
     </msub>
     <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
   </mstyle></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κεφαλαίο l του θήτα; ισούται με; το φυσικό λογάριθμο, του; γινόμενο από k ισούται με 1 ως κεφαλαίο n του; p; ανοίγει παρένθεση; x δείκτης k; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση")?;
    return Ok(());
}

#[test]
fn uoa_corpus_429() -> Result<()> { //
    let expr = r#"<math>
 <mrow>
   <mfrac>
    <mrow>
     <mo>∂</mo><mi>L</mi><mo stretchy="false">(</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
    <mrow>
     <mo>∂</mo><mi>θ</mi></mrow>
   </mfrac>
   <mo>=</mo><mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>k</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>N</mi>
    </munderover>
    <mrow>
     <mfrac>
      <mrow>
       <mo>∂</mo><mi>ln</mi><mi>p</mi><mo stretchy="false">(</mo><msub>
        <mi>x</mi>
        <mi>k</mi>
       </msub>
       <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
      <mrow>
       <mo>∂</mo><mi>θ</mi></mrow>
     </mfrac>
     </mrow>
   </mstyle><mo>=</mo><mstyle displaystyle="true">
    <munderover>
     <mo>∑</mo>
     <mrow>
      <mi>k</mi><mo>=</mo><mn>1</mn></mrow>
     <mi>N</mi>
    </munderover>
    <mrow>
     <mfrac>
      <mn>1</mn>
      <mrow>
       <mi>p</mi><mo stretchy="false">(</mo><msub>
        <mi>x</mi>
        <mi>k</mi>
       </msub>
       <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
     </mfrac>
     <mo>×</mo><mfrac>
      <mrow>
       <mo>∂</mo><mi>p</mi><mo stretchy="false">(</mo><msub>
        <mi>x</mi>
        <mi>k</mi>
       </msub>
       <mo>;</mo><mi>θ</mi><mo stretchy="false">)</mo></mrow>
      <mrow>
       <mo>∂</mo><mi>θ</mi></mrow>
     </mfrac>
     </mrow>
   </mstyle><mo>=</mo><mn>0</mn></mrow>
  </math>"#;
    test("el", "SimpleSpeak", expr, "κλάσμα, μερικό διαφορικό; κεφαλαίο l του θήτα, προς, μερικό διαφορικό, θήτα, τέλος κλάσματος; ισούται με; άθροισμα από k ισούται με 1 ως κεφαλαίο n του; κλάσμα, μερικό διαφορικό; ο φυσικός λογάριθμος, του p; ανοίγει παρένθεση; x δείκτης k; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση, προς, μερικό διαφορικό, θήτα, τέλος κλάσματος; ισούται με; άθροισμα από k ισούται με 1 ως κεφαλαίο n του; κλάσμα, 1 προς, p; ανοίγει παρένθεση; x δείκτης k; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση, τέλος κλάσματος; επί; κλάσμα, μερικό διαφορικό p; ανοίγει παρένθεση; x δείκτης k; ελληνικό ερωτηματικό, θήτα; κλείνει παρένθεση, προς, μερικό διαφορικό, θήτα, τέλος κλάσματος; ισούται με 0")?;
    return Ok(());
}

