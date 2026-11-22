use crate::common::*;
use anyhow::Result;

#[test]
fn complex() -> Result<()> {
    let expr = "<math>
                    <mi>ℂ</mi>
                </math>";
    test("el", "SimpleSpeak", expr, "οι μιγαδικοί αριθμοί")?;
    return Ok(());

}

#[test]
fn natural() -> Result<()> {
    let expr = "<math>
                    <mi>ℕ</mi>
                </math>";
    test("el", "SimpleSpeak", expr, "οι φυσικοί αριθμοί")?;
    return Ok(());

}

#[test]
fn rationals() -> Result<()> {
    let expr = "<math>
                    <mi>ℚ</mi>
                </math>";
    test("el", "SimpleSpeak", expr, "οι ρητοί αριθμοί")?;
    return Ok(());

}

#[test]
fn reals() -> Result<()> {
    let expr = "<math>
                    <mi>ℝ</mi>
                </math>";
    test("el", "SimpleSpeak", expr, "οι πραγματικοί αριθμοί")?;
    return Ok(());

}

#[test]
fn integers() -> Result<()> {
    let expr = "<math>
                    <mi>ℤ</mi>
                </math>";
    test("el", "SimpleSpeak", expr, "οι ακέραιοι αριθμοί")?; 
    return Ok(());

}

#[test]
fn msup_complex() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℂ</mi>
                    <mn>2</mn>
                </msup>
                </math>";
    test("el", "SimpleSpeak", expr, "c 2")?;
    return Ok(());

}

#[test]
fn msup_natural() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℕ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("el", "SimpleSpeak", expr, "n 2")?;
    return Ok(());

}

#[test]
fn msup_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("el", "SimpleSpeak", expr, "q 2")?;
    return Ok(());

}

#[test]
fn msup_reals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℝ</mi>
                    <mn>3</mn>
                </msup>
            </math>";
    test("el", "SimpleSpeak", expr, "r 3")?;
    return Ok(());

}

#[test]
fn msup_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mn>4</mn>
                </msup>
            </math>";
    test("el", "SimpleSpeak", expr, "z 4")?;
    return Ok(());

}

#[test]
fn msup_positive_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("el", "SimpleSpeak", expr, "οι θετικοί ακέραιοι αριθμοί")?; 
    return Ok(());

}

#[test]
fn msup_negative_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("el", "SimpleSpeak", expr, "οι αρνητικοί ακέραιοι αριθμοί")?;
    return Ok(());

}

#[test]
fn msup_positive_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("el", "SimpleSpeak", expr, "οι θετικοί ρητοί αριθμοί")?;
    return Ok(());

}

#[test]
fn msup_negative_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("el", "SimpleSpeak", expr, "οι αρνητικοί ρητοί αριθμοί")?;
    return Ok(());

}

#[test]
fn empty_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mo>}</mo>
            </math>";
    test("el", "SimpleSpeak", expr, "το κενό σύνολο")?;
    return Ok(());

}

#[test]
fn single_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>12</mn><mo>}</mo>
            </math>";
    test("el", "SimpleSpeak", expr, "το σύνολο 12")?;
    return Ok(());

}

#[test]
fn multiple_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test("el", "SimpleSpeak", expr, "το σύνολο 5 κόμμα, 10 κόμμα, 15")?;
    return Ok(());

}

#[test]
fn set_with_colon() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>:</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("el", "SimpleSpeak", expr, "το σύνολο όλων των x τέτοια ώστε x είναι μεγαλύτερο από 2")?;
    return Ok(());

}

#[test]
fn set_with_bar() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("el", "SimpleSpeak", expr, "το σύνολο όλων των x τέτοια ώστε x είναι μεγαλύτερο από 2")?; // το σύνολο όλων των x τέτοια ώστε x είναι μεγαλύτερο από 2
    return Ok(());

}

#[test]
fn element_alone() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test("el", "SimpleSpeak", expr, "3 συν 2 i, δεν ανήκει, στους πραγματικούς αριθμούς")?;
    return Ok(());

} 

#[test] // vivi
fn element_set() -> Result<()> {
    let expr = "<math>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
        </math>";
    test("el", "SimpleSpeak", expr, "x ανήκει, στους ακέραιους αριθμούς")?;
    return Ok(());

}

#[test]
fn element_under_sum() -> Result<()> {
    let expr = "<math>
            <munder>
                <mo>∑</mo>
                <mrow> <mi>i</mi> <mo>∈</mo> <mi>ℤ</mi> </mrow>
            </munder>
            <mfrac>
                <mn>1</mn>
                <mrow> <msup>  <mi>i</mi> <mn>2</mn> </msup> </mrow>
            </mfrac>
        </math>";
    test("el", "SimpleSpeak", expr,
                    "άθροισμα για i ανήκει, στους ακέραιους αριθμούς του; κλάσμα, 1 προς, i στο τετράγωνο, τέλος κλάσματος")?; 
                    return Ok(());

}

#[test]
fn complicated_set_with_colon() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mn>2</mn>
            <mo>&#x003C;</mo>
            <mi>x</mi>
            <mo>&#x003C;</mo>
            <mn>7</mn>
            <mo>}</mo>
        </math>";
    test("el", "SimpleSpeak", expr, "το σύνολο όλων των x που ανήκει, στους ακέραιους αριθμούς τέτοια ώστε 2 είναι μικρότερο από x είναι μικρότερο από 7")?; 
    return Ok(());

}

#[test]
fn complicated_set_with_mtext() -> Result<()> {
    // as of 8/5/21, parsing of "|" is problematic an element of the example, so <mrows> are needed for this test
    let expr = "<math>
        <mo>{</mo>
        <mrow> <mi>x</mi><mo>∈</mo><mi>ℕ</mi></mrow>
        <mo>|</mo>
        <mrow><mi>x</mi> <mtext>&#x00A0;is&#x00A0;an&#x00A0;even&#x00A0;number</mtext> </mrow>
        <mo>}</mo>
        </math>";
    test("el", "SimpleSpeak", expr, 
            "το σύνολο όλων των x που ανήκει, στους φυσικούς αριθμούς τέτοια ώστε x is an even number")?; 
            return Ok(());

}
