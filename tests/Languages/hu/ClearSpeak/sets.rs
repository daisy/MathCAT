use crate::common::*;
use anyhow::Result;

// AI generated
#[test]
fn complex() -> Result<()> {
    let expr = "<math>
                    <mi>ℂ</mi>
                </math>";
    test("hu", "ClearSpeak", expr, "a komplex számok halmaza")?;
    return Ok(());

}

// AI generated
#[test]
fn natural() -> Result<()> {
    let expr = "<math>
                    <mi>ℕ</mi>
                </math>";
    test("hu", "ClearSpeak", expr, "a természetes számok halmaza")?;
    return Ok(());

}

// AI generated
#[test]
fn rationals() -> Result<()> {
    let expr = "<math>
                    <mi>ℚ</mi>
                </math>";
    test("hu", "ClearSpeak", expr, "a racionális számok halmaza")?;
    return Ok(());

}

// AI generated
#[test]
fn reals() -> Result<()> {
    let expr = "<math>
                    <mi>ℝ</mi>
                </math>";
    test("hu", "ClearSpeak", expr, "a valós számok halmaza")?;
    return Ok(());

}

// AI generated
#[test]
fn integers() -> Result<()> {
    let expr = "<math>
                    <mi>ℤ</mi>
                </math>";
    test("hu", "ClearSpeak", expr, "az egész számok halmaza")?;
    return Ok(());

}



// AI generated
#[test]
fn msup_complex() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℂ</mi>
                    <mn>2</mn>
                </msup>
                </math>";
    test("hu", "ClearSpeak", expr, "komplex számhalmaz 2")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_natural() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℕ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("hu", "ClearSpeak", expr, "természetes számhalmaz 2")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("hu", "ClearSpeak", expr, "racionális számhalmaz 2")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_reals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℝ</mi>
                    <mn>3</mn>
                </msup>
            </math>";
    test("hu", "ClearSpeak", expr, "valós számhalmaz 3")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mn>4</mn>
                </msup>
            </math>";
    test("hu", "ClearSpeak", expr, "egész számhalmaz 4")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_positive_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("hu", "ClearSpeak", expr, "pozitív egész számok")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_negative_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("hu", "ClearSpeak", expr, "negatív egész számok")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_positive_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("hu", "ClearSpeak", expr, "pozitív racionális számok")?;
    return Ok(());

}

// AI generated
#[test]
fn msup_negative_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("hu", "ClearSpeak", expr, "negatív racionális számok")?;
    return Ok(());

}

// AI generated
#[test]
fn empty_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mo>}</mo>
            </math>";
    test("hu", "ClearSpeak", expr, "az üres halmaz")?;
    return Ok(());

}

// AI generated
#[test]
fn single_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>12</mn><mo>}</mo>
            </math>";
    test("hu", "ClearSpeak", expr, "a halmaz 12")?;
    return Ok(());

}

// AI generated
#[test]
fn multiple_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test("hu", "ClearSpeak", expr, "a halmaz 5 vessző, 10 vessző, 15")?;
    return Ok(());

}

// AI generated
#[test]
fn set_with_colon() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>:</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("hu", "ClearSpeak", expr, "x melyekre teljesül, hogy minden x nagyobb, mint 2")?;
    return Ok(());

}

// AI generated
#[test]
fn set_with_bar() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("hu", "ClearSpeak", expr, "x melyekre teljesül, hogy minden x nagyobb, mint 2")?;
    return Ok(());

}

// AI generated
#[test]
fn element_alone() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test("hu", "ClearSpeak", expr, "3 plusz 2 i; nem tagja az alábbi halmaznak:, a valós számok halmaza")?;
    return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak", expr,
                    "összeg alatta i tagja az alábbi halmaznak:, az egész számok halmaza; a tört, melynek számlálója 1; nevezője pedig i a négyzeten")?;
                    return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak", expr, "x az egész számok halmaza melyekre teljesül, hogy minden 2 kisebb, mint x kisebb, mint 7")?;
    return Ok(());

}

// AI generated
#[test]
fn complicated_set_with_mtext() -> Result<()> {
    // as of 8/5/21, parsing of "|" is problematic in the example, so <mrows> are needed for this test
    let expr = "<math>
        <mo>{</mo>
        <mrow> <mi>x</mi><mo>∈</mo><mi>ℕ</mi></mrow>
        <mo>|</mo>
        <mrow><mi>x</mi> <mtext>páros szám</mtext> </mrow>
        <mo>}</mo>
        </math>";
    test("hu", "ClearSpeak", expr, 
            "x a természetes számok halmaza melyekre teljesül, hogy minden x páros szám")?;
            return Ok(());

}


// AI generated
#[test]
fn set_with_bar_member() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
    test_ClearSpeak("hu", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "x tagja az egész számok halmaza melyekre teljesül, hogy minden x nagyobb, mint 5")?;
                return Ok(());

}

// AI generated
#[test]
fn element_alone_member() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("hu", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "3 plusz 2 i; nem tagja az alábbi halmaznak:, a valós számok halmaza")?;
                return Ok(());

}

// AI generated
#[test]
fn element_under_sum_member() -> Result<()> {
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
    test_ClearSpeak("hu", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "összeg alatta i tagja az alábbi halmaznak:, az egész számok halmaza; a tört, melynek számlálója 1; nevezője pedig i a négyzeten")?;
                return Ok(());

}


// AI generated
#[test]
fn set_with_bar_element() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
    test_ClearSpeak("hu", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "x eleme ennek:, az egész számok halmaza melyekre teljesül, hogy minden x nagyobb, mint 5")?;
                return Ok(());

}

// AI generated
#[test]
fn element_alone_element() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("hu", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "3 plusz 2 i, nem eleme ennek:, a valós számok halmaza")?;
                return Ok(());

}

// AI generated
#[test]
fn element_under_sum_element() -> Result<()> {
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
    test_ClearSpeak("hu", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "összeg alatta i egy eleme ennek:, az egész számok halmaza; a tört, melynek számlálója 1; nevezője pedig i a négyzeten")?;
                return Ok(());

}

// AI generated
#[test]
fn set_with_bar_in() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
    test_ClearSpeak("hu", "ClearSpeak_SetMemberSymbol", "In",
                expr, "x az egész számok halmaza melyekre teljesül, hogy minden x nagyobb, mint 5")?;
                return Ok(());

}

// AI generated
#[test]
fn element_alone_in() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("hu", "ClearSpeak_SetMemberSymbol", "In",
                expr, "3 plusz 2 i; nincs benne az alábbi halmazban:, a valós számok halmaza")?;
                return Ok(());

}

// AI generated
#[test]
fn element_under_sum_in() -> Result<()> {
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
    test_ClearSpeak("hu", "ClearSpeak_SetMemberSymbol", "In",
                expr, "összeg alatta i benne van ebben:, az egész számok halmaza; a tört, melynek számlálója 1; nevezője pedig i a négyzeten")?;
                return Ok(());

}

// AI generated
#[test]
fn set_with_bar_belongs() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
    test_ClearSpeak("hu", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "x hozzátartozó eleme ennek:, az egész számok halmaza melyekre teljesül, hogy minden x nagyobb, mint 5")?;
                return Ok(());

}

// AI generated
#[test]
fn element_alone_belongs() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("hu", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "3 plusz 2 i, nem tartozik ebbe:, a valós számok halmaza")?;
                return Ok(());

}

// AI generated
#[test]
fn element_under_sum_belongs() -> Result<()> {
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
    test_ClearSpeak("hu", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "összeg alatta i ebbe a halmazba tartozik:, az egész számok halmaza; a tört, melynek számlálója 1; nevezője pedig i a négyzeten")?;
                return Ok(());

}


// AI generated
#[test]
fn set_member_woall() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
            test_ClearSpeak_prefs("hu", vec![("ClearSpeak_SetMemberSymbol", "Member"), ("ClearSpeak_Sets", "woAll")],
                expr, "x tagja az egész számok halmaza melyekre teljesül, hogy x nagyobb, mint 5")?;
                return Ok(());

}

// AI generated
#[test]
fn multiple_element_set_woall() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test_ClearSpeak("hu", "ClearSpeak_Sets", "woAll", expr, "a halmaz 5 vessző, 10 vessző, 15")?;
    return Ok(());

}

// AI generated
#[test]
fn multiple_element_set_silent_bracket() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
            test_ClearSpeak("hu", "ClearSpeak_Sets", "SilentBracket", expr, "5 vessző, 10 vessző, 15")?;
            return Ok(());

        }

// AI generated
#[test]
fn silent_bracket() -> Result<()> {
    let expr = "<math>
                <mo>{</mo><mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow><mo>}</mo>
            </math>";
            test_ClearSpeak("hu", "ClearSpeak_Sets", "SilentBracket", expr,
                    "x melyekre teljesül, hogy minden x nagyobb, mint 2")?;
                    return Ok(());

        }

