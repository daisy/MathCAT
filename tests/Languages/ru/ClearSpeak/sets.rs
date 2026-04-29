use crate::common::*;

use anyhow::Result;

#[test]
fn complex() -> Result<()> {
    let expr = "<math>
                    <mi>ℂ</mi>
                </math>";
    test("ru", "ClearSpeak", expr, "комплексные числа")?;
    return Ok(());
}

#[test]
fn natural() -> Result<()> {
    let expr = "<math>
                    <mi>ℕ</mi>
                </math>";
    test("ru", "ClearSpeak", expr, "натуральные числа")?;
    return Ok(());
}

#[test]
fn rationals() -> Result<()> {
    let expr = "<math>
                    <mi>ℚ</mi>
                </math>";
    test("ru", "ClearSpeak", expr, "рациональные числа")?;
    return Ok(());
}

#[test]
fn reals() -> Result<()> {
    let expr = "<math>
                    <mi>ℝ</mi>
                </math>";
    test("ru", "ClearSpeak", expr, "действительные числа")?;
    return Ok(());
}

#[test]
fn integers() -> Result<()> {
    let expr = "<math>
                    <mi>ℤ</mi>
                </math>";
    test("ru", "ClearSpeak", expr, "целые числа")?;
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
    test("ru", "ClearSpeak", expr, "цэ 2")?;
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
    test("ru", "ClearSpeak", expr, "эн 2")?;
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
    test("ru", "ClearSpeak", expr, "ку 2")?;
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
    test("ru", "ClearSpeak", expr, "эр 3")?;
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
    test("ru", "ClearSpeak", expr, "зэ 4")?;
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
    test("ru", "ClearSpeak", expr, "положительные целые числа")?;
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
    test("ru", "ClearSpeak", expr, "отрицательные целые числа")?;
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
    test("ru", "ClearSpeak", expr, "положительные рациональные числа")?;
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
    test("ru", "ClearSpeak", expr, "отрицательные рациональные числа")?;
    return Ok(());
}

#[test]
fn empty_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mo>}</mo>
            </math>";
    test("ru", "ClearSpeak", expr, "пустое множество")?;
    return Ok(());
}

#[test]
fn single_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>12</mn><mo>}</mo>
            </math>";
    test("ru", "ClearSpeak", expr, "множество 12")?;
    return Ok(());
}

#[test]
fn multiple_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test("ru", "ClearSpeak", expr, "множество 5 запятая, 10 запятая, 15")?;
    return Ok(());
}

#[test]
fn set_with_colon() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>:</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("ru", "ClearSpeak", expr, "множество всех x таких, что x больше 2")?;
    return Ok(());
}

#[test]
fn set_with_bar() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("ru", "ClearSpeak", expr, "множество всех x таких, что x больше 2")?;
    return Ok(());
}

#[test]
fn element_alone() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test("ru", "ClearSpeak", expr, "3 плюс 2 i, не принадлежит действительным числам")?;
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
    test("ru", "ClearSpeak", expr,
                    "сумма по i, принадлежащему целым числам, от; дробь, числитель: 1; знаменатель: i в квадрате")?;
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
    test("ru", "ClearSpeak", expr, "множество всех x из целых чисел, таких что 2 меньше x меньше 7")?;
    return Ok(());
}

#[test]
fn complicated_set_with_mtext() -> Result<()> {
    let expr = "<math>
        <mo>{</mo>
        <mrow> <mi>x</mi><mo>∈</mo><mi>ℕ</mi></mrow>
        <mo>|</mo>
        <mrow><mi>x</mi> <mtext>is an even number</mtext> </mrow>
        <mo>}</mo>
        </math>";
    test("ru", "ClearSpeak", expr, 
            "множество всех x из натуральных чисел, таких что x — чётное число")?;
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
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "множество всех x, принадлежащих целым числам, таких что x больше 5")?;
                return Ok(());
}

#[test]
fn element_alone_member() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "3 плюс 2 i, не принадлежит действительным числам")?;
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
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "сумма по i, принадлежащему целым числам, от; дробь, числитель: 1; знаменатель: i в квадрате")?;
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
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "множество всех x, являющихся элементами целых чисел, таких что x больше 5")?;
                return Ok(());
}

#[test]
fn element_alone_element() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "3 плюс 2 i, не является элементом действительных чисел")?;
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
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "сумма по i, являющемуся элементом целых чисел, от; дробь, числитель: 1; знаменатель: i в квадрате")?;
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
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "In",
                expr, "множество всех x из целых чисел, таких что x больше 5")?;
                return Ok(());
}

#[test]
fn element_alone_in() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "In",
                expr, "3 плюс 2 i, не принадлежит действительным числам")?;
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
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "In",
                expr, "сумма по i из целых чисел, от; дробь, числитель: 1; знаменатель: i в квадрате")?;
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
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "множество всех x, входящих в целые числа, таких что x больше 5")?;
                return Ok(());
}

#[test]
fn element_alone_belongs() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "3 плюс 2 i, не входит в действительные числа")?;
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
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "сумма по i, входящему в целые числа, от; дробь, числитель: 1; знаменатель: i в квадрате")?;
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
    test_ClearSpeak_prefs("ru", vec![("ClearSpeak_SetMemberSymbol", "Member"), ("ClearSpeak_Sets", "woAll")],
                expr, "множество x, принадлежащих целым числам, таких что x больше 5")?;
                return Ok(());
}

#[test]
fn multiple_element_set_woall() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test_ClearSpeak("ru", "ClearSpeak_Sets", "woAll", expr, "множество 5 запятая, 10 запятая, 15")?;
    return Ok(());
}

#[test]
fn multiple_element_set_silent_bracket() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test_ClearSpeak("ru", "ClearSpeak_Sets", "SilentBracket", expr, "5 запятая, 10 запятая, 15")?;
    return Ok(());
}

#[test]
fn silent_bracket() -> Result<()> {
    let expr = "<math>
                <mo>{</mo><mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow><mo>}</mo>
            </math>";
    test_ClearSpeak("ru", "ClearSpeak_Sets", "SilentBracket", expr,
                    "множество всех x таких, что x больше 2")?;
                    return Ok(());
}

