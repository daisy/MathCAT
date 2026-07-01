use crate::common::*;
use anyhow::Result;

#[test]
fn complex() -> Result<()> {
    let expr = "<math>
                    <mi>ℂ</mi>
                </math>";
    test("fr", "ClearSpeak", expr, "les nombres complexes")?;
    return Ok(());

}

#[test]
fn natural() -> Result<()> {
    let expr = "<math>
                    <mi>ℕ</mi>
                </math>";
    test("fr", "ClearSpeak", expr, "les nombres naturels")?;
    return Ok(());

}

#[test]
fn rationals() -> Result<()> {
    let expr = "<math>
                    <mi>ℚ</mi>
                </math>";
    test("fr", "ClearSpeak", expr, "les nombres rationnels")?;
    return Ok(());

}

#[test]
fn reals() -> Result<()> {
    let expr = "<math>
                    <mi>ℝ</mi>
                </math>";
    test("fr", "ClearSpeak", expr, "les nombres réels")?;
    return Ok(());

}

#[test]
fn integers() -> Result<()> {
    let expr = "<math>
                    <mi>ℤ</mi>
                </math>";
    test("fr", "ClearSpeak", expr, "les nombres entiers")?;
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
    test("fr", "ClearSpeak", expr, "C 2")?;
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
    test("fr", "ClearSpeak", expr, "N 2")?;
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
    test("fr", "ClearSpeak", expr, "Q 2")?;
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
    test("fr", "ClearSpeak", expr, "R 3")?;
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
    test("fr", "ClearSpeak", expr, "Z 4")?;
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
    test("fr", "ClearSpeak", expr, "les nombres entiers positifs")?;
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
    test("fr", "ClearSpeak", expr, "les nombres entiers négatifs")?;
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
    test("fr", "ClearSpeak", expr, "les nombres rationnels positifs")?;
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
    test("fr", "ClearSpeak", expr, "les nombres rationnels négatifs")?;
    return Ok(());

}

#[test]
fn empty_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mo>}</mo>
            </math>";
    test("fr", "ClearSpeak", expr, "l'ensemble vide")?;
    return Ok(());

}

#[test]
fn single_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>12</mn><mo>}</mo>
            </math>";
    test("fr", "ClearSpeak", expr, "l'ensemble 12")?;
    return Ok(());

}

#[test]
fn multiple_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test("fr", "ClearSpeak", expr, "l'ensemble 5 virgule, 10 virgule, 15")?;
    return Ok(());

}

#[test]
fn set_with_colon() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>:</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("fr", "ClearSpeak", expr, "l'ensemble de tous x tel que x est supérieur à 2")?;
    return Ok(());

}

#[test]
fn set_with_bar() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("fr", "ClearSpeak", expr, "l'ensemble de tous x tel que x est supérieur à 2")?;
    return Ok(());

}

#[test]
fn element_alone() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test("fr", "ClearSpeak", expr, "3 plus 2 i, n'est pas membre de, les nombres réels")?;
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
    test("fr", "ClearSpeak", expr,
                    "la sommation sur i est membre de les nombres entiers de; la fraction avec numérateur 1; et dénominateur i au carré")?;
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
    test("fr", "ClearSpeak", expr, "l'ensemble de tous x dans les nombres entiers tel que 2 est inférieur à x est inférieur à 7")?;
    return Ok(());

}

#[test]
fn complicated_set_with_mtext() -> Result<()> {
    // as of 8/5/21, parsing of "|" is problematic in the example, so <mrows> are needed for this test
    let expr = "<math>
        <mo>{</mo>
        <mrow> <mi>x</mi><mo>∈</mo><mi>ℕ</mi></mrow>
        <mo>|</mo>
        <mrow><mi>x</mi> <mtext>est un nombre pair</mtext> </mrow>
        <mo>}</mo>
        </math>";
    test("fr", "ClearSpeak", expr, 
            "l'ensemble de tous x dans les nombres naturels tel que x est un nombre pair")?;
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
    test_ClearSpeak("fr", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "l'ensemble de tous x membre de les nombres entiers tel que x est supérieur à 5")?;
                return Ok(());

}

#[test]
fn element_alone_member() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("fr", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "3 plus 2 i, n'est pas membre de, les nombres réels")?;
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
    test_ClearSpeak("fr", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "la sommation sur i est membre de les nombres entiers de; la fraction avec numérateur 1; et dénominateur i au carré")?;
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
    test_ClearSpeak("fr", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "l'ensemble de tous x élément de les nombres entiers tel que x est supérieur à 5")?;
                return Ok(());

}

#[test]
fn element_alone_element() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("fr", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "3 plus 2 i, n'est pas un élément de, les nombres réels")?;
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
    test_ClearSpeak("fr", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "la sommation sur i est un élément de, les nombres entiers de; la fraction avec numérateur 1; et dénominateur i au carré")?;
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
    test_ClearSpeak("fr", "ClearSpeak_SetMemberSymbol", "In",
                expr, "l'ensemble de tous x dans les nombres entiers tel que x est supérieur à 5")?;
                return Ok(());

}

#[test]
fn element_alone_in() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("fr", "ClearSpeak_SetMemberSymbol", "In",
                expr, "3 plus 2 i, n'est pas dans les nombres réels")?;
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
    test_ClearSpeak("fr", "ClearSpeak_SetMemberSymbol", "In",
                expr, "la sommation sur i est dans les nombres entiers de; la fraction avec numérateur 1; et dénominateur i au carré")?;
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
    test_ClearSpeak("fr", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "l'ensemble de tous x appartenant à les nombres entiers tel que x est supérieur à 5")?;
                return Ok(());

}

#[test]
fn element_alone_belongs() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("fr", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "3 plus 2 i, n'appartient pas à, les nombres réels")?;
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
    test_ClearSpeak("fr", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "la sommation sur i appartient à les nombres entiers de; la fraction avec numérateur 1; et dénominateur i au carré")?;
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
            test_ClearSpeak_prefs("fr", vec![("ClearSpeak_SetMemberSymbol", "Member"), ("ClearSpeak_Sets", "woAll")],
                expr, "l'ensemble de x membre de les nombres entiers tel que x est supérieur à 5")?;
                return Ok(());

}

#[test]
fn multiple_element_set_woall() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test_ClearSpeak("fr", "ClearSpeak_Sets", "woAll", expr, "l'ensemble 5 virgule, 10 virgule, 15")?;
    return Ok(());

}

#[test]
fn multiple_element_set_silent_bracket() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
            test_ClearSpeak("fr", "ClearSpeak_Sets", "SilentBracket", expr, "5 virgule, 10 virgule, 15")?;
            return Ok(());

        }

#[test]
fn silent_bracket() -> Result<()> {
    let expr = "<math>
                <mo>{</mo><mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow><mo>}</mo>
            </math>";
            test_ClearSpeak("fr", "ClearSpeak_Sets", "SilentBracket", expr,
                    "l'ensemble de tous x tel que x est supérieur à 2")?;
                    return Ok(());

        }

