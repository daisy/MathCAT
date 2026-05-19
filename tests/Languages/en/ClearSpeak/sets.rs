use crate::common::*;
use anyhow::Result;

#[test]
fn complex() -> Result<()> {
    let expr = "<math>
                    <mi>ℂ</mi>
                </math>";
    test("en", "ClearSpeak", expr, "the complex numbers")?;
    return Ok(());

}

#[test]
fn natural() -> Result<()> {
    let expr = "<math>
                    <mi>ℕ</mi>
                </math>";
    test("en", "ClearSpeak", expr, "the natural numbers")?;
    return Ok(());

}

#[test]
fn rationals() -> Result<()> {
    let expr = "<math>
                    <mi>ℚ</mi>
                </math>";
    test("en", "ClearSpeak", expr, "the rational numbers")?;
    return Ok(());

}

#[test]
fn reals() -> Result<()> {
    let expr = "<math>
                    <mi>ℝ</mi>
                </math>";
    test("en", "ClearSpeak", expr, "the real numbers")?;
    return Ok(());

}

#[test]
fn integers() -> Result<()> {
    let expr = "<math>
                    <mi>ℤ</mi>
                </math>";
    test("en", "ClearSpeak", expr, "the integers")?;
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
    test("en", "ClearSpeak", expr, "C 2")?;
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
    test("en", "ClearSpeak", expr, "N 2")?;
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
    test("en", "ClearSpeak", expr, "Q 2")?;
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
    test("en", "ClearSpeak", expr, "R 3")?;
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
    test("en", "ClearSpeak", expr, "Z 4")?;
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
    test("en", "ClearSpeak", expr, "the positive integers")?;
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
    test("en", "ClearSpeak", expr, "the negative integers")?;
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
    test("en", "ClearSpeak", expr, "the positive rational numbers")?;
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
    test("en", "ClearSpeak", expr, "the negative rational numbers")?;
    return Ok(());

}

#[test]
fn empty_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mo>}</mo>
            </math>";
    test("en", "ClearSpeak", expr, "the empty set")?;
    return Ok(());

}

#[test]
fn single_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>12</mn><mo>}</mo>
            </math>";
    test("en", "ClearSpeak", expr, "the set 12")?;
    return Ok(());

}

#[test]
fn multiple_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test("en", "ClearSpeak", expr, "the set 5 comma, 10 comma, 15")?;
    return Ok(());

}

#[test]
fn set_with_colon() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>:</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("en", "ClearSpeak", expr, "the set of all x such that x is greater than 2")?;
    return Ok(());

}

#[test]
fn set_with_bar() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("en", "ClearSpeak", expr, "the set of all x such that x is greater than 2")?;
    return Ok(());

}

#[test]
fn element_alone() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test("en", "ClearSpeak", expr, "3 plus 2 i, is not a member of, the real numbers")?;
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
    test("en", "ClearSpeak", expr,
                    "the sum over i is a member of the integers of; the fraction with numerator 1; and denominator i squared")?;
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
    test("en", "ClearSpeak", expr, "the set of all x in the integers such that 2 is less than x is less than 7")?;
    return Ok(());

}

#[test]
fn complicated_set_with_mtext() -> Result<()> {
    // as of 8/5/21, parsing of "|" is problematic in the example, so <mrows> are needed for this test
    let expr = "<math>
        <mo>{</mo>
        <mrow> <mi>x</mi><mo>∈</mo><mi>ℕ</mi></mrow>
        <mo>|</mo>
        <mrow><mi>x</mi> <mtext>is an even number</mtext> </mrow>
        <mo>}</mo>
        </math>";
    test("en", "ClearSpeak", expr, 
            "the set of all x in the natural numbers such that x is an even number")?;
            return Ok(());

}


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
    test_ClearSpeak("en", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "the set of all x member of the integers such that x is greater than 5")?;
                return Ok(());

}

#[test]
fn element_alone_member() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("en", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "3 plus 2 i, is not a member of, the real numbers")?;
                return Ok(());

}

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
    test_ClearSpeak("en", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "the sum over i is a member of the integers of; the fraction with numerator 1; and denominator i squared")?;
                return Ok(());

}


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
    test_ClearSpeak("en", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "the set of all x element of the integers such that x is greater than 5")?;
                return Ok(());

}

#[test]
fn element_alone_element() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("en", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "3 plus 2 i, is not an element of, the real numbers")?;
                return Ok(());

}

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
    test_ClearSpeak("en", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "the sum over i is an element of the integers of; the fraction with numerator 1; and denominator i squared")?;
                return Ok(());

}

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
    test_ClearSpeak("en", "ClearSpeak_SetMemberSymbol", "In",
                expr, "the set of all x in the integers such that x is greater than 5")?;
                return Ok(());

}

#[test]
fn element_alone_in() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("en", "ClearSpeak_SetMemberSymbol", "In",
                expr, "3 plus 2 i, is not in the real numbers")?;
                return Ok(());

}

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
    test_ClearSpeak("en", "ClearSpeak_SetMemberSymbol", "In",
                expr, "the sum over i is in the integers of; the fraction with numerator 1; and denominator i squared")?;
                return Ok(());

}

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
    test_ClearSpeak("en", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "the set of all x belonging to the integers such that x is greater than 5")?;
                return Ok(());

}

#[test]
fn element_alone_belongs() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("en", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "3 plus 2 i, does not belong to, the real numbers")?;
                return Ok(());

}

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
    test_ClearSpeak("en", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "the sum over i belongs to the integers of; the fraction with numerator 1; and denominator i squared")?;
                return Ok(());

}


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
            test_ClearSpeak_prefs("en", vec![("ClearSpeak_SetMemberSymbol", "Member"), ("ClearSpeak_Sets", "woAll")],
                expr, "the set of x member of the integers such that x is greater than 5")?;
                return Ok(());

}

#[test]
fn multiple_element_set_woall() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test_ClearSpeak("en", "ClearSpeak_Sets", "woAll", expr, "the set 5 comma, 10 comma, 15")?;
    return Ok(());

}

#[test]
fn multiple_element_set_silent_bracket() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
            test_ClearSpeak("en", "ClearSpeak_Sets", "SilentBracket", expr, "5 comma, 10 comma, 15")?;
            return Ok(());

        }

#[test]
fn silent_bracket() -> Result<()> {
    let expr = "<math>
                <mo>{</mo><mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow><mo>}</mo>
            </math>";
            test_ClearSpeak("en", "ClearSpeak_Sets", "SilentBracket", expr,
                    "the set of all x such that x is greater than 2")?;
                    return Ok(());

        }

