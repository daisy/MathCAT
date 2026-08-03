/// Tests for rules in definitions:
/// *  modified var
use crate::common::*;
use anyhow::Result;

#[test]
fn tuple_basic() -> Result<()> {
    // function 
    let expr = r#"
      <math>
        <mrow intent="tuple($x,$y)">
          <mi arg="x">x</mi>
          <mi arg="y">y</mi>
        </mrow>
      </math>
    "#;

    test("hu", "ClearSpeak", expr, "a tuple x vessző, y")?;
    test("hu", "SimpleSpeak", expr, "a tuple x vessző, y")?;

    return Ok(());
}

#[test]
fn my_set_basic() -> Result<()> {
    let expr = r#"
      <math>
        <mrow intent="set($x,$y)">
          <mi arg="x">x</mi>
          <mi arg="y">y</mi>
        </mrow>
      </math>
    "#;

    test("hu", "ClearSpeak", expr, "az üres halmaz")?;

    Ok(())
}

#[test]
fn fixed_test() -> Result<()> {
    let expr = r#"
      <math>
        <mi intent="az összes valós szám halmaza">az összes valós szám halmaza
        </mi>
      </math>
    "#;

    test("hu", "ClearSpeak", expr, "az összes valós szám halmaza")?;
    Ok(())
}

#[test]
fn i_test() -> Result<()> {
    let expr = r#"
      <math>
    <mi intent="képzeletbeli én">képzeletbeli én
        </mi>
      </math>
    "#;

    test("hu", "ClearSpeak", expr, "képzeletbeli én")?;
    Ok(())
}


#[test]
fn floor_basic() -> Result<()> {
    let expr = r#"
      <math>
        <mrow intent="floor($x)">
          <mi arg="x">x</mi>
        </mrow>
      </math>
    "#;

    test("hu", "ClearSpeak", expr, "lépcső x")?;

    Ok(())
}

#[test]
fn set_difference_basic() -> Result<()> {
    let expr = r#"
      <math>
        <mrow intent="set-difference($A,$B)">
          <mi arg="A">A</mi>
          <mo>&#x2216;</mo>
          <mi arg="B">B</mi>
        </mrow>
      </math>
    "#;

    test( "hu", "ClearSpeak", expr, "különbséghalmaz nagy a vessző, nagy b")?;

    Ok(())
}

#[test]
fn postfix_test() -> Result<()> {
    let tests = [
        (
            "transpose",
            r#"
            <msup intent="transpose($x)">
                <mi arg="x">x</mi>
                <mo>T</mo>
            </msup>
            "#,
            "transzponált x",
        ),
        (
            "highlight",
            r#"
            <menclose intent="highlight($x)">
                <mi arg="x">x</mi>
            </menclose>
            "#,
            "x kiemelt",
        ),
    ];

    // Loop through all test cases, name _, body, and expected result
    for (_, body, expected) in tests {
        let expr = format!(
            r#"
            <math>
                {}
            </math>
            "#,
            body
        );

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn prefix_test() -> Result<()> {
    // Prefix test limit, unit-vector, line-segment
    // Directed-line-segment, line, ray, arc
    let tests = [
        (
            "limit",
            r#"
            <mrow intent="limit($x)">
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "határérték x",
        ),
        (
            "unit-vector",
            r#"
            <mover intent="unit-vector($x)">
                <mi arg="x">x</mi>
                <mo>^</mo>
            </mover>
            "#,
            "egységvektor x",
        ),
        (
            "line-segment",
            r#"
            <mover intent="line-segment($x)">
                <mi arg="x">x</mi>
                <mo>¯</mo>
            </mover>
            "#,
            "line segment x",
        ),
    ];

    for (_, body, expected) in tests {
        let expr = format!(
            r#"
            <math>
                {}
            </math>
            "#,
            body
        );

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


#[test]
fn functions_and_inverses_tests() -> Result<()> {
    let tests = vec![
        ("closed-interval", "x től y ig terjedő intervallum, tartalmazza a(z) x elemet és y elemet"),
        ("closed-open-interval", "x től y ig terjedő intervallum, tartalmazza a(z) x elemet de nem tartalmazza a(z) y elemet"),
        ("open-closed-interval", "x től y ig terjedő intervallum, nem tartalmazza a(z) x elemet de tartalmazza a(z) y elemet"),
        ("open-interval", "x től y ig terjedő intervallum, nem tartalmazza a(z) x elemet vagy y elemet"),

        ("inverse", "inverz x"),
        ("domain", "domén x"),
        ("codomain", "kodomán x"),
        ("image", "kép x"),

        //("fraction", "fraction x over y end fraction"),
        ("mixed-fraction", "x és y"),
        ("quotient", "egész rész osztva x vessző, y"),
        ("evaluated-at", "x értékelve y"),
        ("remainder", "a maradék, osztva x vessző, y"),

        ("max", "max x vessző, y vessző, z"),
        ("min", "min x vessző, y vessző, z"),

        ("power", "x a(z) y-edik hatványon"),

        //("root", "gyök x"),

        ("greatest-common-divisor", "a legnagyobb közös osztó x vessző, y vessző, z"),
        ("least-common-multiple", "a legkisebb közös többszörös x vessző, y vessző, z"),

        ("absolute-value", "x abszolút értéke"),

        ("complex-conjugate", "Komplex konjugált x"),
        ("complex-arg", "argumentum x"),
        ("real-part", "valós rész"),
        ("imaginary-part", "képzetes rész"),

        ("polar-coordinate", "poláris koordináta x vessző, y"),
        ("spherical-coordinate", "gömb koordináta x vessző, y vessző, z"),
        ("cartesian-coordinate", "derékszögű koordináta x vessző, y vessző, z"),
        ("coordinate", "pont, x vessző y vessző z"),

        ("floor", "lépcső x"),
        ("ceiling", "mennyezet x"),
        ("round", "kerekített érték x"),
        ("fractional-part", "tört rész x"),

        
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            
            "max"
            | "min"
            | "greatest-common-divisor"
            | "least-common-multiple"
            | "spherical-coordinate"
            | "cartesian-coordinate"
            | "coordinate" => {
                format!(
                    "<math>
                        <mrow intent='{}($a,$b,$c)'>
                            <mi arg='a'>x</mi>
                            <mi arg='b'>y</mi>
                            <mi arg='c'>z</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }

            "power"
            | "mixed-fraction"
            | "quotient"
            | "evaluated-at"
            | "remainder"
            | "closed-interval"
            | "closed-open-interval"
            | "open-closed-interval"
            | "open-interval" 
            | "polar-coordinate" => {
                format!(
                    "<math>
                        <mrow intent='{}($a,$b)'>
                            <mi arg='a'>x</mi>
                            <mi arg='b'>y</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }

            _ => {
                format!(
                    "<math>
                        <mrow intent='{}($x)'>
                            <mi arg='x'>x</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
        };

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


#[test]
fn calculus_tests() -> Result<()> {
    let tests = vec![
        //("derivative", "the derivative of x with respect to x"),
        //"definite-integral", "integral over x from x to x"),

        // prefix
        ("limit", "határérték x"),

        // infix
        ("tends-to", "x ehhez tart y"),
        ("tends-to-from-above", "x általában alulról indul y"),
        ("tends-to-from-below", "x alulról indul y"),
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            "limit" => {
                format!(
                    "<math>
                        <mrow intent='{}($x)'>
                            <mi arg='x'>x</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }

            _ => {
                // infix cases
                format!(
                    "<math>
                        <mrow intent='{}($a,$b)'>
                            <mi arg='a'>x</mi>
                            <mi arg='b'>y</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
        };

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


#[test]
fn set_tests() -> Result<()> {
    let tests = vec![
        ("set", "a halmaz x"),
        //("set-difference", "x és y halmaz különbsége"),
        ("complement", "komplemens x"),
        //("empty-set", "üres halmaz"),
        ("cardinality", "számosság x"),
        ("list", "lista x"),
        ("tuple", "a tuple x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


#[test]
fn sequence_and_series_intents() -> Result<()> {
    let tests = [
        (
            "sum-1",
            r#"
            <mrow intent="sum($x)">
                <mo>&#x2211;</mo>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "összeg x",
        ),
        (
            "sum-2",
            r#"
            <mrow intent="sum($i,$x)">
                <mo>&#x2211;</mo>
                <mi arg="i">i</mi>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "összeg i vessző, x",
        ),
        (
            "sum-3",
            r#"
            <mrow intent="sum($i,$n,$x)">
                <mo>&#x2211;</mo>
                <mi arg="i">i</mi>
                <mi arg="n">n</mi>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "összeg i vessző, n vessző, x",
        ),
        (
            "product-1",
            r#"
            <mrow intent="product($x)">
                <mo>&#x220F;</mo>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "szorzat x",
        ),
        (
            "product-2",
            r#"
            <mrow intent="product($i,$x)">
                <mo>&#x220F;</mo>
                <mi arg="i">i</mi>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "szorzat i vessző, x",
        ),
        (
            "product-3",
            r#"
            <mrow intent="product($i,$n,$x)">
                <mo>&#x220F;</mo>
                <mi arg="i">i</mi>
                <mi arg="n">n</mi>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "szorzat i vessző, n vessző, x",
        ),
    ];

    for (_, body, expected) in tests {
        let expr = format!(
            r#"
            <math>
                {}
            </math>
            "#,
            body
        );

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn elementary_classical_tests() -> Result<()> {
    let tests = vec![
        // Trig
        ("sine", "szinusz x"),
        ("cosine", "koszinusz x"),
        ("tangent", "tangens x"),
        ("secant", "szekáns x"),
        ("cosecant", "koszekáns x"),
        ("cotangent", "kotangens x"),

        // Inverse trig
        ("arcsine", "arkusz szinusz x"),
        ("arccosine", "arkusz koszinusz x"),
        ("arctangent", "arkusz tangens x"),
        ("arcsecant", "arkusz szekáns x"),
        ("arccosecant", "arkusz koszekáns x"),
        ("arccotangent", "arkusz kotangens x"),

        // Hyperbolic trig
        ("hyperbolic-sine", "hiperbolikus szinusz x"),
        ("hyperbolic-cosine", "hiperbolikus koszinusz x"),
        ("hyperbolic-tangent", "hiperbolikus tangens x"),
        ("hyperbolic-secant", "hiperbolikus szekáns x"),
        ("hyperbolic-cosecant", "hiperbolikus koszekáns x"),
        ("hyperbolic-cotangent", "hiperbolikus kotangens x"),

        // Inverse hyperbolic trig
        ("arc-hyperbolic-sine", "arkusz hiperbolikus szinusz x"),
        ("arc-hyperbolic-cosine", "arkusz hiperbolikus koszinusz x"),
        ("arc-hyperbolic-tangent", "arkusz hiperbolikus tangens x"),
        ("arc-hyperbolic-secant", "arkusz hiperbolikus szekáns x"),
        ("arc-hyperbolic-cosecant", "arkusz hiperbolikus koszekáns x"),
        ("arc-hyperbolic-cotangent", "arkusz hiperbolikus kotangens x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn statistics_and_probability_tests() -> Result<()> {
    let tests = vec![
        ("mean", "jelentése x"),
        ("standard-deviation", "szórás x"),
        ("variance", "variencia x"),
        ("median", "medián x"),
        ("mode", "mód x"),

        // conditional probability typically two arguments
        // ("conditional-probability", "probability of x given y"),
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            "conditional-probability" => format!(
                "<math>
                    <mrow intent='{}($A,$B)'>
                        <mi>P</mi>
                        <mo>(</mo>
                        <mrow>
                          <mi arg='x'>x</mi>
                          <mo>|</mo>
                          <mi arg='y'>y</mi>
                        </mrow>
                        <mo>)</mo>
                    </mrow>
                </math>",
                intent
            ),
            _ => format!(
                "<math>
                    <mrow intent='{}($x)'>
                        <mi arg='x'>x</mi>
                    </mrow>
                </math>",
                intent
            ),
        };

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn linear_algebra_tests() -> Result<()> {
    let tests = vec![
        ("vector", "vektor x"),
        // ("matrix", "matrix of x"),
        ("determinant", "determináns a x"),
        ("adjugate", "adjugált x"),
        ("magnitude", "nagyságrend x"),
        ("norm", "normalizálás x"),
        ("span", "fesztávolság x"),

        // transpose supports both postfix and function; we test function explicitly
        ("transpose", "transzponált x"),

        // dimensional product is infix
        ("dimensional-product", "x a(z) y"),

        // unit-vector is prefix
        ("unit-vector", "egységvektor x")
    ];

    for (intent, expected) in tests {
        let expr: String = match intent {
          "dimensional-product" => {
              "<math>
                  <mrow intent='dimensional-product($x,$y)'>
                      <mi arg='x'>x</mi>
                      <mi arg='y'>y</mi>
                  </mrow>
              </math>"
              .to_string()
          }
          _ => format!(
              "<math>
                  <mrow intent='{intent}($x)'>
                      <mi arg='x'>x</mi>
                  </mrow>
              </math>"
          ),
      };

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn nofix_set_tests() -> Result<()> {
    let tests = vec![
        ("set-of-integers", "ℤ", "az összes egész szám halmaza"),
        ("set-of-reals", "valós számok halmaza", "az összes valós szám halmaza"),
        ("set-of-rationals", "ℚ", "az összes racionális szám halmaza"),
        ("set-of-natural-numbers", "ℕ", "az összes természetes szám halmaza"),
        ("set-of-complex-numbers", "ℂ", "az összes komplex szám halmaza"),
        ("set-of-primes", "ℙ", "az összes prímszám halmaza"),
    ];

    for (intent, symbol, expected) in tests {
        let expr = format!(
            "<math>
                <mi intent='{}:nofix'>{}</mi>
            </math>",
            intent,
            symbol
        );

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn geometry_prefix_multi_value_tests() -> Result<()> {
    let tests = vec![
        ("line-segment", "vonalszakasz x y"),
        ("directed-line-segment", "irányított vonalszakasz x y"),
        ("line", "vonal x y"),
        ("ray", "sugár x y"),
        ("arc", "ív x y"),
        ("point", "pont x y z"),
    ];

    for (intent, expected) in tests {
        let expr = match intent{
            "point" => {
                format!(
                    "<math>
                        <mrow intent='{intent}($x,$y,$z)'>
                            <mi arg='x'>x</mi>
                            <mi arg='y'>y</mi>
                            <mi arg='z'>z</mi>
                        </mrow>
                    </math>"
                )
            }

            _ => { 
                format!(
                    "<math>
                        <mrow intent='{intent}($x,$y)'>
                            <mi arg='x'>x</mi>
                            <mi arg='y'>y</mi>
                        </mrow>
                    </math>"
                )
            }
        };

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn geometry_prefix_tests() -> Result<()> {
    let tests = vec![
        ("length", "hossz x"),
        ("area", "terület x"),
        ("volume", "hangerő x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{intent}:function($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>"
        );

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn separator_tests() -> Result<()> {
  let expr = format!(
        "<math>
            <mrow intent='time-separator($x,$y)'>
                <mi arg='x'>x</mi>
                <mi arg='y'>y</mi>
            </mrow>
        </math>"
    );

    test("hu", "ClearSpeak", &expr, "x y")?;
    Ok(())
}

#[test]
fn general_concepts_tests() -> Result<()> {
    let tests = vec![
        // Unary structural
        ("fenced-group", "bekerített csoport x"),
        ("highlight", "x kiemelt"),
        ("least-common-denominator", "legkisebb közös nevező x vessző, y vessző, z"), // add x, y , z ...
        ("pochhammer", "permutáció x"),
        ("permutation-cycle", "permutációs ciklus x"),

        // Binary structural / infix-style
        // ("ordered-pair", "the pair of x and y"),
        ("rate", "x per y"),
        
        ("binomial-coefficient", "x választ y"),
        ("embellished-name", "x megjegyzéssel y"),
        ("indexed-by", "x alsó index y"),
        // ("translation", "translation by x comma, y"), // Changes translation to comma
        ("constraint", "x feltétellel y"),
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            "ordered-pair"
            | "rate"
            | "constraint"
            | "binomial-coefficient"
            | "embellished-name"
            | "translation"
            | "indexed-by" => {
                format!(
                    "<math>
                        <mrow intent='{}($a,$b)'>
                            <mi arg='a'>x</mi>
                            <mi arg='b'>y</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
            "least-common-denominator" => {
                format!(
                    "<math>
                        <mrow intent='{}($x,$y,$z)'>
                            <mi arg='x'>x</mi>
                            <mi arg='y'>y</mi>
                            <mi arg='z'>z</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }

            // unary cases
            _ => {
                format!(
                    "<math>
                        <mrow intent='{}($x)'>
                            <mi arg='x'>x</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
        };

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn grouping_tests() -> Result<()> {
    let tests = vec![
("annotation", "x ami y"),
        // ("braced-group", "grouped x end-grouped"),
        // ("repeating-decimal", "repeating decimal of x"),
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            "annotation" => {
                format!(
                    "<math>
                        <mrow intent='{}($x,$y)'>
                            <mi arg='x'>x</mi>
                            <mi arg='y'>y</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
            _ => {
                format!(
                    "<math>
                        <mrow intent='{}($x)'>
                            <mi arg='x'>x</mi>
                        </mrow>
                    </math>",
                    intent
                )
            } 
        };

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn function_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("curl", "rotáció x"),
        ("divergence", "eltérés x"),
        ("gradient", "gradiens x"),
        ("laplacian", "laplaciánus x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn prefix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("angle", "szög x"),
        ("angle-measure", "szögmérés x"),
        ("change", "változás x"),
        ("for-all", "minden x"),
        ("measured-angle", "mért szög x"),
        ("not", "nem x"),
        ("number-of", "száma x"),
        ("partial-derivative", "parciális derivált x"),
        ("right-angle", "derékszög x"),
        ("square-root-of", "négyzetgyöke x"),
        ("there-does-not-exist", "nem létezik x"),
        ("there-exists", "létezik x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn infix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("and", "x és y"),
        ("applied-to", "x alkalmazva y"),
        ("approximately", "x hozzávetőlegesen y"),
        ("congruent", "x egybeesik y"),
        ("cartesian-product", "x derékszögű szorzat y"),
        ("composed-with", "x komponálva y"),
        ("cross-product", "x kereszt szorzata y"),
        ("defined-as", "x definiálva y"),
        ("divided-by", "x osztva y"),
        ("divides", "x osztja y"),
        ("does-not-belong-to", "x nem tartozik hozzá y"),
        ("does-not-divide", "x nem osztja y"),
        ("dot-product", "x skaláris szorzat y"),
        ("downwards-diagonal-ellipsis", "x lefelé mutató átlós ellipszis y"),
        ("direct-product", "x direkt szorzat y"),
        ("element-of", "x eleme y"),
        ("ellipsis", "x ellipszis y"),
        ("equals", "x egyenlő y"),
        ("equivalent-to", "x egyenértékű ezzel y"),
        ("evaluates-to", "x értéke y"),
        ("given", "x adott y"),
        ("greater-than", "x nagyobb, mint y"),
        ("greater-than-or-equal-to", "x nagyobb vagy egyenlő, mint y"),
        ("identically-equals", "x teljesen egyenlő y"),
        ("if-and-only-if", "x akkor és csak akkor, ha y"),
        ("implies", "x utal y"),
        ("inner-product", "x belső szorzat y"),
        ("intersection", "x keresztezi y"),
        ("less-than", "x kisebb, mint y"),
        ("less-than-or-equal-to", "x kisebb vagy egyenlő, mint y"),
        ("list-separator", "x vessző y"),
        ("maps-to", "x megfelel ennek y"),
        ("member-of", "x tagja y"),
        ("minus", "x mínusz y"),
        ("minus-or-plus", "x mínusz vagy plusz y"),
        ("not-subset", "x nem részhalmaza y"),
        ("not-superset", "x nem szuperhalmaza y"),
        ("not-equal-to", "x nem egyenlő y"),
        ("not-member-of", "x nem tagja y"),
        ("not-parallel-to", "x nem párhuzamos y"),
        ("obtained-from", "x innen származik y"),
        ("or", "x vagy y"),
        ("outer-product", "x külső szorzata y"),
        ("parallel-to", "x párhuzamos y"),
        ("perpendicular", "x merőleges y"),
        ("plus", "x plusz y"),
        ("plus-or-minus", "x plusz vagy mínusz y"),
        ("precedes", "x megelőzi y"),
        ("proportional", "x arányos y"),
        ("range-separator", "x keresztül y"),
        ("ratio", "x arány y"),
        ("similar", "x hasonló y"),
        ("subset", "x részhalmaza y"),
        ("subset-or-equal", "x részhalmaza vagy egyenlő y"),
        ("succeeds", "x követi y"),
        ("such-that", "x úgy, hogy y"),
        ("superset", "x szuperhalmaza y"),
        ("superset-or-equal", "x szuperhalmaza vagy egyenlő y"),
        ("tilde", "x tilde y"),
        ("times", "x szorozva y"),
        ("union", "x unió y"),
        ("upwards-diagonal-ellipsis", "x felfelé átlós ellipszis y"),
        ("vertical-ellipsis", "x függőleges ellipszis y"),
        ("xor", "x kizárólagos vagy y"),
    ];
    
    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($a,$b)'>
                    <mi arg='a'>x</mi>
                    <mi arg='b'>y</mi>
                </mrow>
            </math>",
            intent
        );

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn postfix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("factorial", "x faktoriális"),
        ("percent", "x százalék"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn nofix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("diameter", "átmérő", "átmérő"),
        ("distance", "távolság", "távolság"),
        ("probability", "valószínűség", "valószínűség"),
        ("radius", "sugár", "sugár"),
        ("volume", "hangerő", "hangerő"),
        ("exponential-e", "e", "e"),
        ("imaginary-i", "képzeletbeli én", "képzeletbeli én"),
        ("differential-d", "d", "d"),
        ("golden-ratio", "φ", "aranymetszés"),
    ];

    for (intent, symbol, expected) in tests {
        let expr = format!(
            "<math>
                <mi intent='{}:nofix'>{}</mi>
            </math>",
            intent,
            symbol
        );

        test("hu", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}
