use crate::common::*;
use anyhow::Result;

#[test]
fn menclose_actuarial() -> Result<()> {
    let expr = "<math>
                    <menclose notation='actuarial'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "αναλογιστικό σύμβολο, περικλείει 3 συν 2 i τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_box() -> Result<()> {
    let expr = "<math>
                    <menclose notation='box circle'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "περίβλημα, κύκλος, περικλείει 3 συν 2 i τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_left() -> Result<()> {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "γραμμή αριστερά, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_right() -> Result<()> {
    let expr = "<math>
                    <menclose notation='right'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "γραμμή δεξιά, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "γραμμή πάνω, κάτω, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_updiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "άνω διαγώνια, διαγραφή, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_downdiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "κάτω διαγώνια, διαγραφή, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_cross_out() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "x, διαγραφή, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_vertical_horizontal_strike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='verticalstrike horizontalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "κάθετη, οριζόντια, διαγραφή, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_leftarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='leftarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "αριστερό βέλος, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_right_up_down_arrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation=' rightarrow downarrow  uparrow  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "βέλος προς τα πάνω, βέλος προς τα κάτω, δεξί βέλος, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_northeastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "βορειοανατολικό βέλος, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_other_single_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestarrow southwestarrow southeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "νοτιοανατολικό βέλος, νοτιοδυτικό βέλος, βορειοδυτικό βέλος, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_northwestsoutheastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestsoutheastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "αμφίδρομο διαγώνιο προς τα κάτω βέλος, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_other_double_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updownarrow leftrightarrow northeastsouthwestarrow'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "αμφίδρομο κάθετο βέλος, αμφίδρομο οριζόντιο βέλος, αμφίδρομο διαγώνιο βέλος, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_madrub() -> Result<()> {
    let expr = "<math>
                    <menclose notation='madrub'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "σύμβολο αραβικού παραγοντικού, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "γωνία φάσης, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_circle_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle circle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "κύκλος, γωνία φάσης, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_longdiv() -> Result<()> {
    let expr = "<math>
                    <menclose notation='longdiv'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "σύμβολο της μακράς διαίρεσης, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_default() -> Result<()> {
    let expr = "<math>
                    <menclose> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "σύμβολο της μακράς διαίρεσης, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_empty_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation=''> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "σύμβολο της μακράς διαίρεσης, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_longdiv_whitespace_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation='  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "σύμβολο της μακράς διαίρεσης, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn menclose_radical() -> Result<()> {
    let expr = "<math>
                    <menclose notation='radical'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "ClearSpeak", expr, "τετραγωνική ρίζα, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}

#[test]
fn simple_speak_menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("el", "SimpleSpeak", expr, "γραμμή πάνω, κάτω, περικλείει 3 δεύτερα τέλος περίκλεισης")?;
    return Ok(());

}
