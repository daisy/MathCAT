#![allow(non_snake_case)]

use crate::common::*;
use anyhow::Result;

#[test]
fn binomial_theorem_simple_speak() -> Result<()> {
    let expr = r#"
        <math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
            <msup>
                <mrow><mo>(</mo><mi>x</mi><mo>+</mo><mi>a</mi><mo>)</mo></mrow>
                <mi>n</mi>
            </msup>
            <mo>=</mo>
            <munderover>
                <mo stretchy="false">&#x2211;</mo>
                <mrow><mi>k</mi><mo>=</mo><mn>0</mn></mrow>
                <mi>n</mi>
            </munderover>
            <mrow>
                <mo>(</mo>
                <mfrac linethickness="0"><mi>n</mi><mi>k</mi></mfrac>
                <mo>)</mo>
                <msup><mi>x</mi><mi>k</mi></msup>
                <msup><mi>a</mi><mrow><mi>n</mi><mo>&#x2212;</mo><mi>k</mi></mrow></msup>
            </mrow>
        </math>
    "#;
    test(
        "pl",
        "SimpleSpeak",
        expr,
        "nawias otwierający, x plus a, nawias zamykający do potęgi n; równa się; suma od k równa się 0, do n; n po k razy x do potęgi k razy a do potęgi n minus k",
    )?;
    Ok(())
}

#[test]
fn variable_power_simple_speak() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mi>n</mi></msup></math>";
    test("pl", "SimpleSpeak", expr, "x do potęgi n")?;
    Ok(())
}

#[test]
fn binomial_simple_speak() -> Result<()> {
    let expr = r#"
        <math>
            <mo>(</mo>
            <mfrac linethickness="0"><mi>n</mi><mi>k</mi></mfrac>
            <mo>)</mo>
        </math>
    "#;
    test("pl", "SimpleSpeak", expr, "n po k")?;
    Ok(())
}

#[test]
fn clearspeak_absolute_value_and_equivalence() -> Result<()> {
    let expr = r#"
        <math>
            <mrow><mo>|</mo><mi>x</mi><mo>&#x2212;</mo><mn>3</mn><mo>|</mo></mrow>
            <mo>&#x21d4;</mo>
            <mn>1</mn><mo>&lt;</mo><mi>x</mi><mo>&lt;</mo><mn>5</mn>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "wartość bezwzględna z x minus 3; wtedy i tylko wtedy 1; jest mniejsze niż x jest mniejsze niż 5",
    )?;
    Ok(())
}

#[test]
fn clearspeak_closed_interval() -> Result<()> {
    let expr = r#"
        <math>
            <mi>x</mi><mo>&#x2208;</mo>
            <mrow><mo>[</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo>]</mo></mrow>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "x jest elementem zbioru, przedział domknięty od a do b",
    )?;
    Ok(())
}

#[test]
fn clearspeak_limit_arrow() -> Result<()> {
    let expr = r#"
        <math>
            <munder>
                <mo>lim</mo>
                <mrow><mi>x</mi><mo>&#x2192;</mo><mi>a</mi></mrow>
            </munder>
            <mfrac>
                <mrow><mi>f</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow>
                <mrow><mi>g</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow>
            </mfrac>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "granica gdy x dąży do a, z f z x przez g z x",
    )?;
    Ok(())
}

#[test]
fn clearspeak_multiline_plural() -> Result<()> {
    let expr = r#"
        <math>
            <mi>f</mi>
            <mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            <mo>=</mo>
            <mrow>
                <mo stretchy="true">{</mo>
                <mtable>
                    <mtr><mtd><mo>-</mo><mn>1</mn></mtd><mtd><mtext>dla</mtext></mtd><mtd><mi>x</mi><mo>&lt;</mo><mn>0</mn></mtd></mtr>
                    <mtr><mtd><mn>0</mn></mtd><mtd><mtext>dla</mtext></mtd><mtd><mi>x</mi><mo>=</mo><mn>0</mn></mtd></mtr>
                    <mtr><mtd><mn>1</mn></mtd><mtd><mtext>dla</mtext></mtd><mtd><mi>x</mi><mo>&gt;</mo><mn>0</mn></mtd></mtr>
                </mtable>
            </mrow>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "f z x równa się; 3 przypadki; przypadek 1; minus 1 dla x jest mniejsze niż 0; przypadek 2; 0 dla x równa się 0; przypadek 3; 1 dla x jest większe niż 0",
    )?;
    Ok(())
}

#[test]
fn clearspeak_vector_product_names() -> Result<()> {
    let expr = r#"
        <math>
            <mover><mi>a</mi><mo>&#x2192;</mo></mover>
            <mo>&#x22c5;</mo>
            <mover><mi>b</mi><mo>&#x2192;</mo></mover>
            <mo>=</mo>
            <mover><mi>a</mi><mo>&#x2192;</mo></mover>
            <mo>&#xd7;</mo>
            <mover><mi>b</mi><mo>&#x2192;</mo></mover>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "wektor a, iloczyn skalarny wektor b; równa się, wektor a, iloczyn wektorowy wektor b",
    )?;
    Ok(())
}

#[test]
fn clearspeak_limsup_and_tilde() -> Result<()> {
    let expr = r#"
        <math>
            <munder>
                <mi>limsup</mi>
                <mrow><mi>n</mi><mo>&#x2192;</mo><mi>&#x221e;</mi></mrow>
            </munder>
            <msub><mi>a</mi><mi>n</mi></msub>
            <mo>&#x223c;</mo>
            <mn>0</mn>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "granica górna gdy n dąży do nieskończoności, z a indeks dolny n; tylda 0",
    )?;
    Ok(())
}

#[test]
fn clearspeak_max_min_and_infinity_norm_are_polish() -> Result<()> {
    let expr = r#"
        <math>
            <mrow intent="max($x)">
                <mi arg="x">x</mi>
            </mrow>
            <mo>,</mo>
            <mrow intent="min($x)">
                <mi arg="x">x</mi>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mo>∥</mo><mi>x</mi><msub><mo>∥</mo><mi>∞</mi></msub>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mo>∥</mo><mi>x</mi><msubsup><mo>∥</mo><mi>∞</mi><mn>2</mn></msubsup>
            </mrow>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "maksimum z x, przecinek, minimum z x, przecinek; norma nieskończoności z x, przecinek; norma nieskończoności z x do potęgi 2",
    )?;
    test(
        "pl",
        "SimpleSpeak",
        expr,
        "maksimum z x, przecinek, minimum z x, przecinek; norma nieskończoności z x, przecinek; norma nieskończoności z x do kwadratu",
    )?;
    Ok(())
}

#[test]
fn clearspeak_named_functions_without_intent_are_polish() -> Result<()> {
    let expr = r#"
        <math>
            <mi>max</mi><mo>&#x2061;</mo><mi>x</mi>
            <mo>,</mo>
            <mi>min</mi><mo>&#x2061;</mo><mi>x</mi>
            <mo>,</mo>
            <mi>sup</mi><mo>&#x2061;</mo><mi>x</mi>
            <mo>,</mo>
            <mi>inf</mi><mo>&#x2061;</mo><mi>x</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "maksimum z x, przecinek, minimum z x, przecinek; supremum z x, przecinek, infimum z x",
    )?;
    test(
        "pl",
        "SimpleSpeak",
        expr,
        "maksimum z x, przecinek, minimum z x, przecinek; supremum z x, przecinek, infimum z x",
    )?;
    Ok(())
}

#[test]
fn clearspeak_under_named_functions_from_pandoc_are_polish() -> Result<()> {
    let expr = r#"
        <math>
            <munder><mo>max</mo><mi>i</mi></munder>
            <mrow><mo>|</mo><msub><mi>x</mi><mi>i</mi></msub><mo>|</mo></mrow>
            <mo>=</mo>
            <msup>
                <mrow>
                    <mo>(</mo>
                    <munder><mo>min</mo><mi>i</mi></munder>
                    <mrow><mo>|</mo><msub><mi>x</mi><mi>i</mi></msub><mo>|</mo></mrow>
                    <mo>)</mo>
                </mrow>
                <mn>2</mn>
            </msup>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "maksimum z i pod z, wartość bezwzględna z x indeks dolny i; równa się; nawias otwierający minimum z i pod z, wartość bezwzględna z x indeks dolny i; nawias zamykający do kwadratu",
    )?;
    Ok(())
}

#[test]
fn clearspeak_under_compound_operators_are_polish() -> Result<()> {
    let expr = r#"
        <math>
            <munder>
                <mrow><mi>arg</mi><mi>max</mi></mrow>
                <mrow><mi>x</mi><mo>∈</mo><mi>X</mi></mrow>
            </munder>
            <mi>f</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            <mo>,</mo>
            <munder>
                <mrow><mi>arg</mi><mi>min</mi></mrow>
                <mrow><mi>y</mi><mo>∈</mo><mi>Y</mi></mrow>
            </munder>
            <mi>g</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>y</mi><mo>)</mo></mrow>
            <mo>,</mo>
            <munder>
                <mrow><mi>ess</mi><mi>sup</mi></mrow>
                <mrow><mi>x</mi><mo>∈</mo><mi>Ω</mi></mrow>
            </munder>
            <mrow><mo>|</mo><mi>f</mi><mo>|</mo></mrow>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "arg maksimum z x jest elementem zbioru, wielka x pod; f z x; przecinek; arg minimum z y jest elementem zbioru, wielka y pod; g z y; przecinek; supremum istotne z x jest elementem zbioru, wielka omega pod; wartość bezwzględna z f",
    )?;
    Ok(())
}

#[test]
fn clearspeak_infinity_after_to_uses_genitive() -> Result<()> {
    let expr = r#"
        <math>
            <munder>
                <mo>lim</mo>
                <mrow>
                    <mi>x</mi><mo>&#x2192;</mo>
                    <mrow><mo>-</mo><mi>&#x221e;</mi></mrow>
                </mrow>
            </munder>
            <mi>f</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            <mo>,</mo>
            <munderover>
                <mo>&#x2211;</mo>
                <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
                <mi>&#x221e;</mi>
            </munderover>
            <msup><mn>2</mn><mrow><mo>-</mo><mi>n</mi></mrow></msup>
            <mo>,</mo>
            <munderover>
                <mo>&#x222b;</mo>
                <mn>0</mn>
                <mi>&#x221e;</mi>
            </munderover>
            <msup><mi>e</mi><mrow><mo>-</mo><mi>x</mi></mrow></msup>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "granica gdy x dąży do minus nieskończoności, z f z x; przecinek; suma od n równa się 1, do nieskończoności; 2 do potęgi minus n; przecinek; całka od 0, do nieskończoności; e do potęgi minus x",
    )?;
    Ok(())
}

#[test]
fn common_fraction_numerators_use_polish_feminine_forms() -> Result<()> {
    let expr = r#"
        <math>
            <mfrac><mn>1</mn><mn>2</mn></mfrac>
            <mo>+</mo>
            <mfrac><mn>2</mn><mn>3</mn></mfrac>
            <mo>+</mo>
            <mfrac><mn>3</mn><mn>4</mn></mfrac>
            <mo>+</mo>
            <mfrac><mn>1</mn><mn>10</mn></mfrac>
            <mo>+</mo>
            <mfrac><mn>2</mn><mn>10</mn></mfrac>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "jedna druga plus dwie trzecie plus 3 czwarte plus jedna dziesiąta plus dwie dziesiąte",
    )?;
    test(
        "pl",
        "SimpleSpeak",
        expr,
        "jedna druga plus dwie trzecie plus 3 czwarte plus jedna dziesiąta plus dwie dziesiąte",
    )?;
    Ok(())
}

#[test]
fn clearspeak_approximation_relations() -> Result<()> {
    let expr = r#"
        <math>
            <mi>x</mi><mo>&#x2248;</mo><mi>y</mi><mo>,</mo>
            <mi>y</mi><mo>&#x2243;</mo><mi>z</mi><mo>,</mo>
            <mi>z</mi><mo>&#x2245;</mo><mi>w</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "x jest w przybliżeniu równe y; przecinek; y jest asymptotycznie równe z; przecinek; z jest przystające do w",
    )?;
    Ok(())
}

#[test]
fn clearspeak_precedence_relation_implies() -> Result<()> {
    let expr = r#"
        <math>
            <mi>a</mi><mo>&#x227c;</mo><mi>b</mi>
            <mo>&#x2227;</mo>
            <mi>b</mi><mo>&#x227c;</mo><mi>c</mi>
            <mo>&#x21d2;</mo>
            <mi>a</mi><mo>&#x227c;</mo><mi>c</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "a poprzedza lub jest równe, b i b, poprzedza lub jest równe, c implikuje a, poprzedza lub jest równe c",
    )?;
    Ok(())
}

#[test]
fn clearspeak_square_relations_and_negated_implication() -> Result<()> {
    let expr = r#"
        <math>
            <mi>x</mi><mo>&#x2291;</mo><mi>y</mi><mo>&#x228f;</mo><mi>z</mi>
            <mo>,</mo>
            <mi>u</mi><mo>&#x2280;</mo><mi>v</mi>
            <mo>,</mo>
            <mi>P</mi><mo>&#x21cf;</mo><mi>Q</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "x kwadratowo poprzedza lub jest równe; y kwadratowo poprzedza z; przecinek; u nie poprzedza v, przecinek; wielka p nie implikuje wielka q",
    )?;
    Ok(())
}

#[test]
fn clearspeak_much_less_greater_relations() -> Result<()> {
    let expr = r#"
        <math>
            <mi>x</mi><mo>&#x226a;</mo><mi>y</mi>
            <mo>,</mo>
            <mi>y</mi><mo>&#x226b;</mo><mi>z</mi>
            <mo>,</mo>
            <mi>a</mi><mo>&#x22d8;</mo><mi>b</mi>
            <mo>,</mo>
            <mi>c</mi><mo>&#x22d9;</mo><mi>d</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "x jest znacznie mniejsze niż y; przecinek; y jest znacznie większe niż z; przecinek; a jest bardzo znacznie mniejsze niż b; przecinek; c jest bardzo znacznie większe niż d",
    )?;
    Ok(())
}

#[test]
fn clearspeak_two_headed_arrow() -> Result<()> {
    let expr = r#"
        <math>
            <mi>A</mi><mo>&#x21a0;</mo><mi>B</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "wielka a, strzałka w prawo z dwoma grotami, wielka b",
    )?;
    Ok(())
}

#[test]
fn clearspeak_negated_unicode_relations() -> Result<()> {
    let expr = r#"
        <math>
            <mi>x</mi><mo>&#x2270;</mo><mi>y</mi>
            <mo>,</mo>
            <mi>a</mi><mo>&#x22e0;</mo><mi>b</mi>
            <mo>,</mo>
            <mi>c</mi><mo>&#x22e2;</mo><mi>d</mi>
            <mo>,</mo>
            <mi>A</mi><mo>&#x220c;</mo><mi>x</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "x nie jest mniejsze ani równe y; przecinek; a nie poprzedza ani nie jest równe b; przecinek; c nie poprzedza kwadratowo ani nie jest równe d; przecinek; wielka a nie zawiera elementu x",
    )?;
    Ok(())
}

#[test]
fn clearspeak_series3_symbol_words() -> Result<()> {
    let expr = r#"
        <math>
            <mi>p</mi><mo>&#x2234;</mo><mi>q</mi>
            <mo>,</mo>
            <mi>r</mi><mo>&#x2235;</mo><mi>s</mi>
            <mo>,</mo>
            <mi>u</mi><mo>&#x223d;</mo><mi>v</mi>
            <mo>,</mo>
            <mi>g</mi><mo>&#x2240;</mo><mi>h</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "p zatem q, przecinek; r ponieważ s, przecinek; u odwrócona tylda v, przecinek; g iloczyn wieńcowy h",
    )?;
    Ok(())
}

#[test]
fn clearspeak_series3_arrows() -> Result<()> {
    let expr = r#"
        <math>
            <mi>a</mi><mo>&#x219a;</mo><mi>b</mi>
            <mo>,</mo>
            <mi>c</mi><mo>&#x21ae;</mo><mi>d</mi>
            <mo>,</mo>
            <mi>e</mi><mo>&#x21c7;</mo><mi>f</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "a przekreślona strzałka w lewo b; przecinek; c przekreślona strzałka w lewo i w prawo d; przecinek; e podwójne strzałki w lewo f",
    )?;
    Ok(())
}

#[test]
fn clearspeak_series3_delimiters() -> Result<()> {
    let expr = r#"
        <math>
            <mo>&#x298d;</mo><mi>x</mi><mo>&#x2990;</mo>
            <mo>,</mo>
            <mo>&#x3010;</mo><mi>A</mi><mo>&#x3011;</mo>
            <mo>,</mo>
            <mo>&#x3014;</mo><mi>C</mi><mo>&#x3015;</mo>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "lewy nawias kwadratowy z haczykiem w górnym rogu x, prawy nawias kwadratowy z haczykiem w górnym rogu; przecinek; lewy czarny nawias soczewkowy; wielka a, prawy czarny nawias soczewkowy; przecinek; lewy nawias skorupowy, wielka c prawy nawias skorupowy",
    )?;
    Ok(())
}

#[test]
fn clearspeak_series4_bold_math_alphabet() -> Result<()> {
    let expr = r#"
        <math>
            <mi>&#x1d431;</mi><mo>&#x22c5;</mo><mi>&#x1d432;</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "pogrubione x iloczyn skalarny pogrubione y",
    )?;
    Ok(())
}

#[test]
fn clearspeak_series4_arrow_words() -> Result<()> {
    let expr = r#"
        <math>
            <mi>a</mi><mo>&#x21dc;</mo><mi>b</mi>
            <mo>,</mo>
            <mi>c</mi><mo>&#x27ff;</mo><mi>d</mi>
            <mo>,</mo>
            <mi>e</mi><mo>&#x21fb;</mo><mi>f</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "a falista strzałka w lewo b, przecinek; c długa falista strzałka w prawo d; przecinek; e, strzałka w prawo z podwójną pionową kreską f",
    )?;
    Ok(())
}

#[test]
fn clearspeak_series4_relation_words() -> Result<()> {
    let expr = r#"
        <math>
            <mi>x</mi><mo>&#x22eb;</mo><mi>y</mi>
            <mo>,</mo>
            <mi>a</mi><mo>&#x2a7d;</mo><mi>b</mi>
            <mo>,</mo>
            <mi>c</mi><mo>&#x2aaf;</mo><mi>d</mi>
            <mo>,</mo>
            <mi>e</mi><mo>&#x2ac5;</mo><mi>f</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "x nie zawiera jako normalnej podgrupy y; przecinek; a mniejsze niż lub ukośnie równe b; przecinek; c poprzedza nad pojedynczą linią równości d; przecinek; e podzbiór nad znakiem równości f",
    )?;
    Ok(())
}

#[test]
fn clearspeak_series4_logic_operator_words() -> Result<()> {
    let expr = r#"
        <math>
            <mi>A</mi><mo>&#x228d;</mo><mi>B</mi>
            <mo>,</mo>
            <mi>C</mi><mo>&#x22a2;</mo><mi>D</mi>
            <mo>,</mo>
            <mi>E</mi><mo>&#x22a7;</mo><mi>F</mi>
            <mo>,</mo>
            <mi>G</mi><mo>&#x27d7;</mo><mi>H</mi>
            <mo>,</mo>
            <mi>I</mi><mo>&#x2a3f;</mo><mi>J</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "wielka a mnożenie wielozbiorów, wielka b; przecinek; wielka c dowodzi wielka d; przecinek; wielka e modeluje wielka f; przecinek; wielka g, pełne złączenie zewnętrzne, wielka h; przecinek; wielka i amalgamacja lub koprodukt, wielka j",
    )?;
    Ok(())
}

#[test]
fn clearspeak_series5_arrow_symbol_words() -> Result<()> {
    let expr = r#"
        <math>
            <mi>a</mi><mo>&#x2911;</mo><mi>b</mi>
            <mo>,</mo>
            <mi>c</mi><mo>&#x291d;</mo><mi>d</mi>
            <mo>,</mo>
            <mi>e</mi><mo>&#x2938;</mo><mi>f</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "a strzałka w prawo z kropkowanym trzonem b; przecinek; c strzałka w lewo do wypełnionego rombu d; przecinek; e, prawostronna strzałka łukowa zgodna z ruchem wskazówek zegara f",
    )?;
    Ok(())
}

#[test]
fn clearspeak_series5_angle_and_operator_words() -> Result<()> {
    let expr = r#"
        <math>
            <mi>u</mi><mo>&#x29a8;</mo><mi>v</mi>
            <mo>,</mo>
            <mi>x</mi><mo>&#x2a11;</mo><mi>y</mi>
            <mo>,</mo>
            <mi>p</mi><mo>&#x2a3d;</mo><mi>q</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "u; kąt mierzony z otwartym ramieniem zakończonym strzałką skierowaną w górę i w prawo v; przecinek; x, całka przeciwna do ruchu wskazówek zegara, y; przecinek; p prawostronny iloczyn wewnętrzny q",
    )?;
    Ok(())
}

#[test]
fn clearspeak_series5_pua_mathjax_words() -> Result<()> {
    let expr = r#"
        <math>
            <mi>p</mi><mo>&#xe916;</mo><mi>q</mi>
            <mo>,</mo>
            <mi>r</mi><mo>&#xe925;</mo><mi>s</mi>
            <mo>,</mo>
            <mi>t</mi><mo>&#xe97a;</mo><mi>u</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "p, nadzbiór z kropką zawiera jako relację indeksu dolnego q; przecinek; r tylda z dwiema kropkami s, przecinek; t sparowane poczwórne pionowe kropki u",
    )?;
    Ok(())
}

#[test]
fn clearspeak_series6_unicode_symbol_words() -> Result<()> {
    let expr = r#"
        <math>
            <mi>a</mi><mo>&#x2975;</mo><mi>b</mi>
            <mo>,</mo>
            <mi>c</mi><mo>&#x29bc;</mo><mi>d</mi>
            <mo>,</mo>
            <mi>e</mi><mo>&#x29d8;</mo><mi>f</mi>
            <mo>,</mo>
            <mi>g</mi><mo>&#x29ee;</mo><mi>h</mi>
            <mo>,</mo>
            <mi>i</mi><mo>&#x2a5a;</mo><mi>j</mi>
            <mo>,</mo>
            <mi>k</mi><mo>&#x2a6f;</mo><mi>l</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "a, strzałka w prawo nad znakiem przybliżonej równości b; przecinek; c, znak dzielenia w kółku obrócony przeciwnie do ruchu wskazówek zegara d; przecinek; e; lewy falisty ogranicznik; f przecinek; g biały kwadrat z przekreśleniem błędu h; przecinek; i logiczne i ze środkowym trzonem j; przecinek; k w przybliżeniu równe z daszkiem l",
    )?;
    Ok(())
}

#[test]
fn clearspeak_series7_unicode_symbol_words() -> Result<()> {
    let expr = r#"
        <math>
            <mi>a</mi><mo>&#x2ab7;</mo><mi>b</mi>
            <mo>,</mo>
            <mi>c</mi><mo>&#x2af7;</mo><mi>d</mi>
            <mo>,</mo>
            <mi>e</mi><mo>&#x2b3f;</mo><mi>f</mi>
            <mo>,</mo>
            <mi>g</mi><mo>&#xe980;</mo><mi>h</mi>
            <mo>,</mo>
            <mi>i</mi><mo>&#xea09;</mo><mi>j</mi>
            <mo>,</mo>
            <mi>k</mi><mo>&#xeb08;</mo><mi>l</mi>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
            "a, poprzedza nad znakiem przybliżonej równości b; przecinek; c potrójnie zagnieżdżone mniejsze niż d; przecinek; e, falista strzałka skierowana bezpośrednio w lewo f; przecinek; g kąt sferyczny otwarty w górę h; przecinek; i ani równe, ani mniejsze niż j; przecinek; k, łuk ze strzałką w lewo przeciwnie do ruchu wskazówek zegara l",
    )?;
    Ok(())
}

#[test]
fn clearspeak_ac_current_symbol() -> Result<()> {
    let expr = r#"
        <math>
            <mi>q</mi><mo>&#x23e6;</mo><mi>q</mi>
        </math>
    "#;
    test("pl", "ClearSpeak", expr, "q prąd przemienny q")?;
    Ok(())
}

#[test]
fn clearspeak_prescript_words_are_polish() -> Result<()> {
    let expr = r#"
        <math>
            <mmultiscripts>
                <mi>x</mi>
                <none/><none/>
                <mprescripts/>
                <none/><mo>&#x2032;</mo>
            </mmultiscripts>
        </math>
    "#;
    test("pl", "ClearSpeak", expr, "x lewy indeks górny prim")?;
    Ok(())
}

#[test]
fn clearspeak_degree_prescript_uses_nominative() -> Result<()> {
    let expr = r#"
        <math>
            <mmultiscripts>
                <mi>x</mi>
                <none/><none/>
                <mprescripts/>
                <none/><mo>&#xb0;</mo>
            </mmultiscripts>
        </math>
    "#;
    test("pl", "ClearSpeak", expr, "x lewy indeks górny stopień")?;
    Ok(())
}

#[test]
fn clearspeak_degree_after_number_stays_plural() -> Result<()> {
    let expr = r#"
        <math>
            <msup><mn>30</mn><mo>&#xb0;</mo></msup>
        </math>
    "#;
    test("pl", "ClearSpeak", expr, "30 stopni")?;
    Ok(())
}

#[test]
fn simplespeak_degree_unit_plural() -> Result<()> {
    let expr = r#"
        <math>
            <mn>1</mn><mi intent=":unit">&#xb0;</mi><mo>,</mo>
            <mn>2</mn><mi intent=":unit">&#xb0;</mi>
        </math>
    "#;
    test("pl", "SimpleSpeak", expr, "1 stopień, przecinek; 2 stopnie")?;
    Ok(())
}

#[test]
fn clearspeak_word_degree_celsius_prescript_is_polish() -> Result<()> {
    let expr = r#"
        <math>
            <mrow><mi>T</mi><mo>=</mo><mn>1</mn><msup><mtext> </mtext><mo>∘</mo></msup><mi>C</mi></mrow>
            <mo>,</mo>
            <mrow><mi>T</mi><mo>=</mo><mn>2</mn><msup><mtext> </mtext><mo>∘</mo></msup><mi>C</mi></mrow>
            <mo>,</mo>
            <mrow><mi>T</mi><mo>=</mo><mn>20</mn><msup><mtext> </mtext><mo>∘</mo></msup><mi>C</mi></mrow>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "wielka t równa się 1 stopień Celsjusza; przecinek; wielka t równa się 2 stopnie Celsjusza; przecinek; wielka t równa się 20 stopni Celsjusza",
    )?;
    Ok(())
}

#[test]
fn clearspeak_vector_calculus_intents_are_polish() -> Result<()> {
    let expr = r#"
        <math>
            <mrow intent="gradient($f)"><mi arg="f">f</mi></mrow>
            <mo>,</mo>
            <mrow intent="divergence($g)"><mi arg="g">g</mi></mrow>
            <mo>,</mo>
            <mrow intent="curl($h)"><mi arg="h">h</mi></mrow>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "gradient z f, przecinek; dywergencja z g, przecinek, rotacja z h",
    )?;
    Ok(())
}

#[test]
fn clearspeak_array_intent_is_polish() -> Result<()> {
    let expr = r#"
        <math>
            <mtable intent=":array">
                <mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr>
                <mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr>
            </mtable>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "tablica z; wiersz 1; kolumna 1; a, kolumna 2; b; przecinek; wiersz 2; kolumna 1; c, kolumna 2; d",
    )?;
    Ok(())
}

#[test]
fn clearspeak_covariance_and_subscripted_norm_are_polish() -> Result<()> {
    let expr = r#"
        <math>
            <mi>Cov</mi>
            <mo>,</mo>
            <mrow intent="subscripted-norm($x,$p)">
                <mi arg="x">x</mi>
                <mi arg="p">p</mi>
            </mrow>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "kowariancja przecinek, norma p z x",
    )?;
    Ok(())
}

#[test]
fn clearspeak_word_subscripted_norm_is_polish() -> Result<()> {
    let expr = r#"
        <math>
            <mrow>
                <mo>∥</mo>
                <mi>x</mi>
                <msub><mo>∥</mo><mi>p</mi></msub>
            </mrow>
        </math>
    "#;
    test("pl", "ClearSpeak", expr, "norma p z x")?;
    Ok(())
}

#[test]
fn clearspeak_star_subscripted_norm_is_polish() -> Result<()> {
    let expr = r#"
        <math>
            <mrow>
                <mo>∥</mo><mi>a</mi><msub><mo>∥</mo><mo>*</mo></msub>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mo>∥</mo><mi>a</mi><msub><mo>∥</mo><mo>*</mo></msub><mo>=</mo><mn>1</mn>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mo>∥</mo><mi>a</mi><msubsup><mo>∥</mo><mo>*</mo><mn>2</mn></msubsup>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mo>∥</mo><mi>b</mi><msub><mo>∥</mo><mo>∗</mo></msub>
            </mrow>
            <mo>,</mo>
            <mrow intent="subscripted-norm($x,$s)">
                <mi arg="x">a</mi>
                <mo arg="s">*</mo>
            </mrow>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "norma gwiazdkowa z a, przecinek; norma gwiazdkowa z a, równa się 1; przecinek; norma gwiazdkowa z a do potęgi 2; przecinek; norma gwiazdkowa z b, przecinek; norma gwiazdkowa z a",
    )?;
    Ok(())
}

#[test]
fn clearspeak_word_leading_norms_are_polish() -> Result<()> {
    let expr = r#"
        <math>
            <mrow>
                <mo>∥</mo><mi>x</mi><msub><mo>∥</mo><mi>p</mi></msub><mo>=</mo><mn>1</mn>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mo>∥</mo><mi>x</mi><msub><mo>∥</mo><mn>2</mn></msub>
                <mo>≤</mo>
                <mo>∥</mo><mi>y</mi><msub><mo>∥</mo><mn>1</mn></msub>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mo>∥</mo><mi>z</mi><mo>∥</mo><mo>=</mo><mn>1</mn>
            </mrow>
            <mo>,</mo>
            <mrow>
                <msqrt><mi>n</mi></msqrt><mo>&#x2062;</mo>
                <mo>∥</mo><mi>x</mi><msub><mo>∥</mo><mn>2</mn></msub>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mi>x</mi><mo>+</mo><mo>∥</mo><mi>y</mi><msub><mo>∥</mo><mi>p</mi></msub>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mi>p</mi><mo>∥</mo><mi>q</mi>
            </mrow>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "norma p z x równa się 1, przecinek; norma 2 z x, jest mniejsze lub równe, norma 1 z y; przecinek; norma z z, równa się 1; przecinek; pierwiastek kwadratowy z n; norma 2 z x; przecinek; x plus norma p z y, przecinek; p jest równoległe do q",
    )?;
    Ok(())
}

#[test]
fn clearspeak_complex_norm_indices_are_polish() -> Result<()> {
    let expr = r#"
        <math>
            <mrow>
                <mo>∥</mo><mi>a</mi>
                <msub>
                    <mo>∥</mo>
                    <mrow><mn>1</mn><mo>→</mo><mi>∞</mi></mrow>
                </msub>
            </mrow>
            <mo>,</mo>
            <mrow intent="subscripted-norm($x,$p)">
                <mi arg="x">a</mi>
                <mrow arg="p"><mn>1</mn><mo>→</mo><mi>∞</mi></mrow>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mo>∥</mo><mi>f</mi>
                <msub>
                    <mo>∥</mo>
                    <mrow>
                        <msup><mi>L</mi><mi>∞</mi></msup>
                        <mrow><mo>(</mo><mi>Ω</mi><mo>)</mo></mrow>
                    </mrow>
                </msub>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mo>∥</mo><mi>u</mi>
                <msub>
                    <mo>∥</mo>
                    <mrow>
                        <msup><mi>W</mi><mrow><mn>1</mn><mo>,</mo><mi>∞</mi></mrow></msup>
                        <mrow><mo>(</mo><mi>Ω</mi><mo>)</mo></mrow>
                    </mrow>
                </msub>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mo>∥</mo><mi>a</mi><msubsup><mo>∥</mo><mi>F</mi><mn>2</mn></msubsup>
            </mrow>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "norma od 1 do nieskończoności z a, przecinek; norma od 1 do nieskończoności z a, przecinek; norma wielka l do potęgi nieskończoność, z wielka omega z f; przecinek; norma wielka w do potęgi 1 przecinek, nieskończoność, z wielka omega z u; przecinek; norma Frobeniusa z a do potęgi 2",
    )?;
    Ok(())
}

#[test]
fn clearspeak_powered_word_norms_are_polish() -> Result<()> {
    let expr = r#"
        <math>
            <mrow>
                <mfrac><mn>1</mn><mn>2</mn></mfrac>
                <mo>&#x2062;</mo>
                <mo>∥</mo><mi>a</mi><mi>x</mi><mo>-</mo><mi>b</mi>
                <msubsup><mo>∥</mo><mn>2</mn><mn>2</mn></msubsup>
                <mo>+</mo>
                <mi>λ</mi><mo>&#x2062;</mo>
                <mo>∥</mo><mi>w</mi><msub><mo>∥</mo><mn>1</mn></msub>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mi>p</mi><mo>∥</mo><mi>q</mi>
            </mrow>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "jedna druga norma 2 z, a x minus b, do kwadratu plus lambda norma 1 z w; przecinek; p jest równoległe do q",
    )?;
    Ok(())
}

#[test]
fn simplespeak_powered_word_norms_are_polish() -> Result<()> {
    let expr = r#"
        <math>
            <mrow>
                <mfrac><mn>1</mn><mn>2</mn></mfrac>
                <mo>&#x2062;</mo>
                <mo>∥</mo><mi>a</mi><mi>x</mi><mo>-</mo><mi>b</mi>
                <msubsup><mo>∥</mo><mn>2</mn><mn>2</mn></msubsup>
                <mo>+</mo>
                <mi>λ</mi><mo>&#x2062;</mo>
                <mo>∥</mo><mi>w</mi><msub><mo>∥</mo><mn>1</mn></msub>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mi>p</mi><mo>∥</mo><mi>q</mi>
            </mrow>
        </math>
    "#;
    test(
        "pl",
        "SimpleSpeak",
        expr,
        "jedna druga norma 2 z, a x minus b, do kwadratu plus lambda norma 1 z w; przecinek; p jest równoległe do q",
    )?;
    Ok(())
}

#[test]
fn clearspeak_named_math_words() -> Result<()> {
    let expr = r#"
        <math>
            <mi>rank</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>a</mi><mo>)</mo></mrow>
            <mo>+</mo>
            <mi>nullity</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>a</mi><mo>)</mo></mrow>
            <mo>=</mo>
            <mrow><mi>p</mi><mtext> prime</mtext></mrow>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "rząd z a plus wymiar jądra z a; równa się p pierwsze",
    )?;
    Ok(())
}

#[test]
fn subscripted_function_application_with_invisible_times_is_polish() -> Result<()> {
    let expr = r#"
        <math>
            <mrow>
                <msub><mi>σ</mi><mi>i</mi></msub>
                <mo>&#x2062;</mo>
                <mrow><mo>(</mo><mi>A</mi><mo>)</mo></mrow>
            </mrow>
            <mo>,</mo>
            <mrow>
                <mn>2</mn>
                <mo>&#x2062;</mo>
                <mrow><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow>
            </mrow>
            <mo>,</mo>
            <mrow>
                <msub><mi>σ</mi><mi>i</mi></msub>
                <mo>&#x2061;</mo>
                <mi>A</mi>
            </mrow>
            <mo>,</mo>
            <mrow>
                <munderover>
                    <mo>∑</mo>
                    <mi>i</mi>
                    <mrow><msub><mi>σ</mi><mi>i</mi></msub></mrow>
                </munderover>
                <mo>&#x2062;</mo>
                <mrow><mo>(</mo><mi>A</mi><mo>)</mo></mrow>
            </mrow>
        </math>
    "#;
    test(
        "pl",
        "ClearSpeak",
        expr,
        "sigma indeks dolny i, z wielka a; przecinek; 2 razy, nawias otwierający, x plus 1, nawias zamykający; przecinek; sigma indeks dolny i, z wielka a; przecinek; suma od i, do sigma indeks dolny i; z wielka a",
    )?;
    test_ClearSpeak_prefs(
        "pl",
        vec![("ClearSpeak_Functions", "None")],
        expr,
        "sigma indeks dolny i, z wielka a; przecinek; 2 razy, nawias otwierający, x plus 1, nawias zamykający; przecinek; sigma indeks dolny i, z wielka a; przecinek; suma od i, do sigma indeks dolny i; z wielka a",
    )?;
    test(
        "pl",
        "SimpleSpeak",
        expr,
        "sigma indeks dolny i, z wielka a; przecinek; 2 razy, nawias otwierający, x plus 1, nawias zamykający; przecinek; sigma indeks dolny i, z wielka a; przecinek; suma od i, do sigma indeks dolny i; z wielka a",
    )?;
    Ok(())
}

#[test]
fn clearspeak_determinant_with_vertical_bars() -> Result<()> {
    let expr = r#"
        <math>
            <mrow>
                <mo>|</mo>
                <mtable>
                    <mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr>
                    <mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr>
                </mtable>
                <mo>|</mo>
            </mrow>
        </math>
    "#;
    test_prefs(
        "pl",
        "ClearSpeak",
        vec![("Verbosity", "Verbose")],
        expr,
        "2 na 2 wyznacznik; wiersz 1; a, b; wiersz 2; c, d; koniec wyznacznika",
    )?;
    Ok(())
}


// =====================================================================
// Testy regresji: normy z podwojna kreska U+2225 generowane przez pandoc
// (oba ograniczniki ∥ jako form="postfix"). Poprawka: ∥ dostal warianty
// LEFT/RIGHT_FENCE w operator-info.in, dzieki czemu norma paruje sie i
// staje sie nawigowalnym intentem 'norm'/'subscripted-norm'. Patrz tez
// usuniecie pustych 'ot: ""' (psuly nawigacje zagniezdzonych wezlow).
// =====================================================================

#[test]
fn norma_pandoc_prosta() -> Result<()> {
    // \|x\| -> oba ∥ jako postfix (jak generuje pandoc)
    let expr = r#"<math><mrow>
        <mo stretchy="false" form="postfix">&#x2225;</mo>
        <mi>x</mi>
        <mo stretchy="false" form="postfix">&#x2225;</mo>
    </mrow></math>"#;
    test("pl", "ClearSpeak", expr, "norma z x")?;
    Ok(())
}

#[test]
fn norma_pandoc_z_indeksem() -> Result<()> {
    // \|x\|_p
    let expr = r#"<math><mrow>
        <mo stretchy="false" form="postfix">&#x2225;</mo>
        <mi>x</mi>
        <msub><mo stretchy="false" form="postfix">&#x2225;</mo><mi>p</mi></msub>
    </mrow></math>"#;
    test("pl", "ClearSpeak", expr, "norma p z x")?;
    Ok(())
}

#[test]
fn norma_pandoc_frobeniusa() -> Result<()> {
    // \|A\|_F
    let expr = r#"<math><mrow>
        <mo stretchy="false" form="postfix">&#x2225;</mo>
        <mi>A</mi>
        <msub><mo stretchy="false" form="postfix">&#x2225;</mo><mi>F</mi></msub>
    </mrow></math>"#;
    test("pl", "ClearSpeak", expr, "norma Frobeniusa z wielka a")?;
    Ok(())
}

#[test]
fn norma_pandoc_gwiazdkowa() -> Result<()> {
    // \|A\|_*
    let expr = r#"<math><mrow>
        <mo stretchy="false" form="postfix">&#x2225;</mo>
        <mi>A</mi>
        <msub><mo stretchy="false" form="postfix">&#x2225;</mo><mo>*</mo></msub>
    </mrow></math>"#;
    test("pl", "ClearSpeak", expr, "norma gwiazdkowa z wielka a")?;
    Ok(())
}

#[test]
fn rownoleglosc_nie_jest_norma() -> Result<()> {
    // TEST NEGATYWNY: a ∥ b (pojedynczy ∥ jako relacja) NIE moze byc norma
    let expr = r#"<math><mrow><mi>a</mi><mo>&#x2225;</mo><mi>b</mi></mrow></math>"#;
    test("pl", "ClearSpeak", expr, "a jest równoległe do b")?;
    Ok(())
}
