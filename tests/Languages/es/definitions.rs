/// Tests for rules in definitions:
/// *  modified var
use crate::common::*;
use anyhow::Result;

// REVISAR (inglés: tuple): «tuple de x coma, y»
// REVISAR (inglés: tuple): «tuple de x coma, y»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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

    test("es", "ClearSpeak", expr, "tuple de x coma, y")?;
    test("es", "SimpleSpeak", expr, "tuple de x coma, y")?;

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

    test("es", "ClearSpeak", expr, "el conjunto vacío")?;

    Ok(())
}

// REVISAR (inglés: of set): «set of reals»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn fixed_test() -> Result<()> {
    let expr = r#"
      <math>
        <mi intent="set-of-reals">R
        </mi>
      </math>
    "#;

    test("es", "ClearSpeak", expr, "set of reals")?;
    Ok(())
}

// REVISAR (inglés: imaginary): «imaginary i»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn i_test() -> Result<()> {
    let expr = r#"
      <math>
        <mi intent="imaginary-i">i
        </mi>
      </math>
    "#;

    test("es", "ClearSpeak", expr, "imaginary i")?;
    Ok(())
}


// REVISAR (inglés: floor): «floor de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn floor_basic() -> Result<()> {
    let expr = r#"
      <math>
        <mrow intent="floor($x)">
          <mi arg="x">x</mi>
        </mrow>
      </math>
    "#;

    test("es", "ClearSpeak", expr, "floor de x")?;

    Ok(())
}

// REVISAR (inglés: ratio): «ratio de 6 coma, 3»
// REVISAR (inglés: ratio): «ratio de 6 coma, 3»
// REVISAR (inglés: ratio): «ratio de 6 coma, 3»
// REVISAR (inglés: ratio): «ratio de 6 coma, 3»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn ratio_function_basic() -> Result<()> {
    let expr = r#"
      <math>
        <mrow intent="ratio($x,$y)">
          <mi arg="x">6</mi>
          <mi arg="y">3</mi>
        </mrow>
      </math>
    "#;

    test_prefs("es", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "ratio de 6 coma, 3")?;
    test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "ratio de 6 coma, 3")?;
    test_prefs("es", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "ratio de 6 coma, 3")?;
    test("es", "ClearSpeak", expr, "ratio de 6 coma, 3")?;
    Ok(())
}

// REVISAR (inglés: difference set): «set difference de mayúscula a coma, mayúscula b»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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

    test( "es", "ClearSpeak", expr, "set difference de mayúscula a coma, mayúscula b")?;

    Ok(())
}

// REVISAR (intercambiar): «x intercambiar»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
            "x intercambiar",
        ),
        (
            "highlight",
            r#"
            <menclose intent="highlight($x)">
                <mi arg="x">x</mi>
            </menclose>
            "#,
            "highlight de x",
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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

// REVISAR (inglés: unit): «unit vector de x»
// REVISAR (inglés: line segment): «line segment de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
            "límite cuando x",
        ),
        (
            "unit-vector",
            r#"
            <mover intent="unit-vector($x)">
                <mi arg="x">x</mi>
                <mo>^</mo>
            </mover>
            "#,
            "unit vector de x",
        ),
        (
            "line-segment",
            r#"
            <mover intent="line-segment($x)">
                <mi arg="x">x</mi>
                <mo>¯</mo>
            </mover>
            "#,
            "line segment de x",
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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


// REVISAR (inglés: inverse): «inverse de x»
// REVISAR (inglés: domain): «domain de x»
// REVISAR (inglés: codomain): «codomain de x»
// REVISAR (inglés: image): «image de x»
// REVISAR (inglés: fraction): «mixed fraction de x coma, y»
// REVISAR (inglés: at evaluated): «evaluated at de x coma, y»
// REVISAR (inglés: remainder): «remainder de x coma, y»
// REVISAR (-th, inglés: th): «x a la y-th potencia»
// REVISAR (inglés: common): «greatest common divisor de x coma, y coma, z»
// REVISAR (inglés: common least): «least common multiple de x coma, y coma, z»
// REVISAR (inglés: absolute value): «el absolute value de x»
// REVISAR (inglés: complex conjugate): «complex conjugate de x»
// REVISAR (inglés: arg complex): «complex arg de x»
// REVISAR (inglés: coordinate polar): «polar coordinate de x coma, y»
// REVISAR (inglés: coordinate spherical): «spherical coordinate de x coma, y coma, z»
// REVISAR (inglés: cartesian coordinate): «cartesian coordinate de x coma, y coma, z»
// REVISAR (inglés: coordinate): «coordinate de x coma, y coma, z»
// REVISAR (inglés: floor): «floor de x»
// REVISAR (inglés: ceiling): «ceiling de x»
// REVISAR (inglés: fractional part): «fractional part de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn functions_and_inverses_tests() -> Result<()> {
    let tests = vec![
        //("closed-interval", "closed interval between x and y"),
        //("closed-open-interval", "interval between x included and y"),
        //("open-closed-interval", "interval between x and y included"),
        //("open-interval", "open interval between x and y"),

        ("inverse", "inverse de x"),
        ("domain", "domain de x"),
        ("codomain", "codomain de x"),
        ("image", "image de x"),

        //("fraction", "fraction x over y end fraction"),
        ("mixed-fraction", "mixed fraction de x coma, y"),
        ("quotient", "quotient de x coma, y"),
        ("evaluated-at", "evaluated at de x coma, y"),
        ("remainder", "remainder de x coma, y"),

        ("max", "max de x coma, y coma, z"),
        ("min", "min de x coma, y coma, z"),

        ("power", "x a la y-th potencia"),

        // ("root", ...) skipped: ClearSpeak `tag: root` requires an index
        // (same as the Polish suite).

        ("greatest-common-divisor", "greatest common divisor de x coma, y coma, z"),
        ("least-common-multiple", "least common multiple de x coma, y coma, z"),

        ("absolute-value", "el absolute value de x"),

        ("complex-conjugate", "complex conjugate de x"),
        ("complex-arg", "complex arg de x"),
        ("real-part", "la parte real"),
        ("imaginary-part", "la parte imaginaria"),

        ("polar-coordinate", "polar coordinate de x coma, y"),
        ("spherical-coordinate", "spherical coordinate de x coma, y coma, z"),
        ("cartesian-coordinate", "cartesian coordinate de x coma, y coma, z"),
        ("coordinate", "coordinate de x coma, y coma, z"),

        ("floor", "floor de x"),
        ("ceiling", "ceiling de x"),
        ("round", "round de x"),
        ("fractional-part", "fractional part de x"),

        
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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


// REVISAR (inglés: tends to): «tends to de x coma, y»
// REVISAR (inglés: above from tends to): «tends to from above de x coma, y»
// REVISAR (inglés: below from tends to): «tends to from below de x coma, y»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn calculus_tests() -> Result<()> {
    let tests = vec![
        //("derivative", "the derivative of x with respect to x"),
        //"definite-integral", "integral over x from x to x"),

        // prefix
        ("limit", "límite cuando x"),

        // infix
        ("tends-to", "tends to de x coma, y"),
        ("tends-to-from-above", "tends to from above de x coma, y"),
        ("tends-to-from-below", "tends to from below de x coma, y"),
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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


// REVISAR (inglés: complement): «complement de x»
// REVISAR (inglés: cardinality): «cardinality de x»
// REVISAR (inglés: list): «list de x»
// REVISAR (inglés: tuple): «tuple de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn set_tests() -> Result<()> {
    let tests = vec![
        ("set", "el conjunto x"),
        // ("set-difference", "set difference of x and y"),
        ("complement", "complement de x"),
        //("empty-set", "empty set"),
        ("cardinality", "cardinality de x"),
        ("list", "list de x"),
        ("tuple", "tuple de x"),
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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


// REVISAR (inglés: sum): «sum de x»
// REVISAR (inglés: sum): «sum de i coma, x»
// REVISAR (inglés: sum): «sum de i coma, n coma, x»
// REVISAR (inglés: product): «product de x»
// REVISAR (inglés: product): «product de i coma, x»
// REVISAR (inglés: product): «product de i coma, n coma, x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
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
            "sum de x",
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
            "sum de i coma, x",
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
            "sum de i coma, n coma, x",
        ),
        (
            "product-1",
            r#"
            <mrow intent="product($x)">
                <mo>&#x220F;</mo>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "product de x",
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
            "product de i coma, x",
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
            "product de i coma, n coma, x",
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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

// REVISAR (inglés: sine): «sine de x»
// REVISAR (inglés: cosine): «cosine de x»
// REVISAR (inglés: tangent): «tangent de x»
// REVISAR (inglés: secant): «secant de x»
// REVISAR (inglés: cosecant): «cosecant de x»
// REVISAR (inglés: cotangent): «cotangent de x»
// REVISAR (inglés: arcsine): «arcsine de x»
// REVISAR (inglés: arccosine): «arccosine de x»
// REVISAR (inglés: arctangent): «arctangent de x»
// REVISAR (inglés: arcsecant): «arcsecant de x»
// REVISAR (inglés: hyperbolic sine): «hyperbolic sine de x»
// REVISAR (inglés: cosine hyperbolic): «hyperbolic cosine de x»
// REVISAR (inglés: hyperbolic tangent): «hyperbolic tangent de x»
// REVISAR (inglés: hyperbolic secant): «hyperbolic secant de x»
// REVISAR (inglés: cosecant hyperbolic): «hyperbolic cosecant de x»
// REVISAR (inglés: cotangent hyperbolic): «hyperbolic cotangent de x»
// REVISAR (inglés: hyperbolic sine): «arc hyperbolic sine de x»
// REVISAR (inglés: cosine hyperbolic): «arc hyperbolic cosine de x»
// REVISAR (inglés: hyperbolic tangent): «arc hyperbolic tangent de x»
// REVISAR (inglés: hyperbolic secant): «arc hyperbolic secant de x»
// REVISAR (inglés: cosecant hyperbolic): «arc hyperbolic cosecant de x»
// REVISAR (inglés: cotangent hyperbolic): «arc hyperbolic cotangent de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn elementary_classical_tests() -> Result<()> {
    let tests = vec![
        // Trig
        ("sine", "sine de x"),
        ("cosine", "cosine de x"),
        ("tangent", "tangent de x"),
        ("secant", "secant de x"),
        ("cosecant", "cosecant de x"),
        ("cotangent", "cotangent de x"),

        // Inverse trig
        ("arcsine", "arcsine de x"),
        ("arccosine", "arccosine de x"),
        ("arctangent", "arctangent de x"),
        ("arcsecant", "arcsecant de x"),
        ("arccosecant", "arccosecant de x"),
        ("arccotangent", "arccotangent de x"),

        // Hyperbolic trig
        ("hyperbolic-sine", "hyperbolic sine de x"),
        ("hyperbolic-cosine", "hyperbolic cosine de x"),
        ("hyperbolic-tangent", "hyperbolic tangent de x"),
        ("hyperbolic-secant", "hyperbolic secant de x"),
        ("hyperbolic-cosecant", "hyperbolic cosecant de x"),
        ("hyperbolic-cotangent", "hyperbolic cotangent de x"),

        // Inverse hyperbolic trig
        ("arc-hyperbolic-sine", "arc hyperbolic sine de x"),
        ("arc-hyperbolic-cosine", "arc hyperbolic cosine de x"),
        ("arc-hyperbolic-tangent", "arc hyperbolic tangent de x"),
        ("arc-hyperbolic-secant", "arc hyperbolic secant de x"),
        ("arc-hyperbolic-cosecant", "arc hyperbolic cosecant de x"),
        ("arc-hyperbolic-cotangent", "arc hyperbolic cotangent de x"),
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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

// REVISAR (inglés: mean): «mean de x»
// REVISAR (inglés: deviation standard): «standard deviation de x»
// REVISAR (inglés: variance): «variance de x»
// REVISAR (inglés: median): «median de x»
// REVISAR (de de, inglés: mode): «mode de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn statistics_and_probability_tests() -> Result<()> {
    let tests = vec![
        ("mean", "mean de x"),
        ("standard-deviation", "standard deviation de x"),
        ("variance", "variance de x"),
        ("median", "median de x"),
        ("mode", "mode de x"),

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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

// REVISAR (inglés: adjugate): «adjugate de x»
// REVISAR (de de, magnitude, inglés: magnitude): «magnitude de x»
// REVISAR (regla): «regla de x»
// REVISAR (inglés: span): «span de x»
// REVISAR (intercambiar): «intercambiar de x»
// REVISAR (inglés: product): «dimensional product de x coma, y»
// REVISAR (inglés: unit): «unit vector de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn linear_algebra_tests() -> Result<()> {
    let tests = vec![
        ("vector", "vector de x"),
        // ("matrix", "matrix of x"),
        ("determinant", "determinante de x"),
        ("adjugate", "adjugate de x"),
        ("magnitude", "magnitude de x"),
        ("norm", "regla de x"),
        ("span", "span de x"),

        // transpose supports both postfix and function; we test function explicitly
        ("transpose", "intercambiar de x"),

        // dimensional product is infix
        ("dimensional-product", "dimensional product de x coma, y"),

        // unit-vector is prefix
        ("unit-vector", "unit vector de x")
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
          "transpose" => "<math>
                  <mrow intent='transpose:function($x)'>
                      <mi arg='x'>x</mi>
                  </mrow>
              </math>".to_string(),
          _ => format!(
              "<math>
                  <mrow intent='{intent}($x)'>
                      <mi arg='x'>x</mi>
                  </mrow>
              </math>"
          ),
      };

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

// REVISAR (inglés: integers of set): «set of integers»
// REVISAR (inglés: of set): «set of reals»
// REVISAR (inglés: of set): «set of rationals»
// REVISAR (inglés: natural numbers of set): «set of natural numbers»
// REVISAR (inglés: complex numbers of set): «set of complex numbers»
// REVISAR (inglés: of set): «set of primes»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn nofix_set_tests() -> Result<()> {
    let tests = vec![
        ("set-of-integers", "ℤ", "set of integers"),
        ("set-of-reals", "ℝ", "set of reals"),
        ("set-of-rationals", "ℚ", "set of rationals"),
        ("set-of-natural-numbers", "ℕ", "set of natural numbers"),
        ("set-of-complex-numbers", "ℂ", "set of complex numbers"),
        ("set-of-primes", "ℙ", "set of primes"),
    ];

    for (intent, symbol, expected) in tests {
        let expr = format!(
            "<math>
                <mi intent='{}'>{}</mi>
            </math>",
            intent,
            symbol
        );

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

// REVISAR (inglés: directed line segment): «directed line segment de x coma, y»
// REVISAR (inglés: line): «line de x coma, y»
// REVISAR (inglés: point): «point de x coma, y coma, z»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn geometry_prefix_multi_value_tests() -> Result<()> {
    let tests = vec![
        ("line-segment", "segmento de línea x y"),
        ("directed-line-segment", "directed line segment de x coma, y"),
        ("line", "line de x coma, y"),
        ("ray", "semirrecta x y"),
        ("arc", "arco x y"),
        ("point", "point de x coma, y coma, z"),
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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

// REVISAR (inglés: length): «length de x»
// REVISAR (inglés: area): «area de x»
// REVISAR (inglés: volume): «volume de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn geometry_prefix_tests() -> Result<()> {
    let tests = vec![
        ("length", "length de x"),
        ("area", "area de x"),
        ("volume", "volume de x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{intent}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>"
        );

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

// REVISAR (inglés: separator): «time separator de x coma, y»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn separator_tests() -> Result<()> {
  let expr = "<math>
            <mrow intent='time-separator($x,$y)'>
                <mi arg='x'>x</mi>
                <mi arg='y'>y</mi>
            </mrow>
        </math>".to_string();

    test("es", "ClearSpeak", &expr, "time separator de x coma, y")?;
    Ok(())
}

// REVISAR (inglés: fenced group): «fenced group de x»
// REVISAR (inglés: common denominator least): «least common denominator de x coma, y coma, z»
// REVISAR (inglés: cycle permutation): «permutation cycle de x»
// REVISAR (inglés: constraint): «constraint de x coma, y»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn general_concepts_tests() -> Result<()> {
    let tests = vec![
        // Unary structural
        ("fenced-group", "fenced group de x"),
        ("highlight", "highlight de x"),
        ("least-common-denominator", "least common denominator de x coma, y coma, z"), // add x, y , z ...
        ("pochhammer", "pochhammer de x"),
        ("permutation-cycle", "permutation cycle de x"),

        // Binary structural / infix-style
        // ("ordered-pair", "the pair of x and y"),
        ("rate", "rate de x coma, y"),
        
        ("binomial-coefficient", "binomial coefficient de x coma, y"),
        ("embellished-name", "embellished name de x coma, y"),
        ("indexed-by", "x subíndice y"),
        // ("translation", "translation by of x comma, y"), // "of" after "by" is awkward; needs dedicated rule or of-suppression
        ("constraint", "constraint de x coma, y"),
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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

// REVISAR (inglés: annotation): «annotation de x coma, y»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn grouping_tests() -> Result<()> {
    let tests = vec![
        ("annotation", "annotation de x coma, y"),
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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

// REVISAR (inglés: curl of): «curl of x»
// REVISAR (inglés: div of): «div of x»
// REVISAR (inglés: gradient of): «gradient of x»
// REVISAR (lahplahsian): «lahplahsian de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn function_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("curl", "curl of x"),
        ("divergence", "div of x"),
        ("gradient", "gradient of x"),
        ("laplacian", "lahplahsian de x"),
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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

// REVISAR (inglés: angle): «angle de x»
// REVISAR (inglés: angle measure): «angle measure de x»
// REVISAR (inglés: change): «change de x»
// REVISAR (inglés: all for): «for all de x»
// REVISAR (inglés: angle measured): «measured angle de x»
// REVISAR (inglés: not): «not de x»
// REVISAR (inglés: number of): «number of de x»
// REVISAR (inglés: derivative partial): «partial derivative de x»
// REVISAR (inglés: angle right): «right angle de x»
// REVISAR (inglés: of root square): «square root of de x»
// REVISAR (inglés: does exist not there): «there does not exist de x»
// REVISAR (inglés: exists there): «there exists de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn prefix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("angle", "angle de x"),
        ("angle-measure", "angle measure de x"),
        ("change", "change de x"),
        ("for-all", "for all de x"),
        ("measured-angle", "measured angle de x"),
        ("not", "not de x"),
        ("number-of", "number of de x"),
        ("partial-derivative", "partial derivative de x"),
        ("right-angle", "right angle de x"),
        ("square-root-of", "square root of de x"),
        ("there-does-not-exist", "there does not exist de x"),
        ("there-exists", "there exists de x"),
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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

// REVISAR (inglés: and): «and de x coma, y»
// REVISAR (inglés: applied to): «applied to de x coma, y»
// REVISAR (inglés: approximately): «approximately de x coma, y»
// REVISAR (inglés: congruent): «congruent de x coma, y»
// REVISAR (inglés: cartesian product): «cartesian product de x coma, y»
// REVISAR (inglés: composed with): «composed with de x coma, y»
// REVISAR (inglés: cross product): «x cross product y»
// REVISAR (inglés: as defined): «defined as de x coma, y»
// REVISAR (inglés: by divided): «divided by de x coma, y»
// REVISAR (inglés: divides): «divides de x coma, y»
// REVISAR (inglés: belong does not to): «does not belong to de x coma, y»
// REVISAR (de de, inglés: divide does not): «does not divide de x coma, y»
// REVISAR (inglés: dot product): «x dot product y»
// REVISAR (inglés: downwards ellipsis): «downwards diagonal ellipsis de x coma, y»
// REVISAR (inglés: direct product): «direct product de x coma, y»
// REVISAR (inglés: element of): «element of de x coma, y»
// REVISAR (inglés: ellipsis): «ellipsis de x coma, y»
// REVISAR (inglés: equals): «equals de x coma, y»
// REVISAR (inglés: equivalent to): «equivalent to de x coma, y»
// REVISAR (inglés: evaluates to): «evaluates to de x coma, y»
// REVISAR (inglés: given): «given de x coma, y»
// REVISAR (inglés: greater than): «greater than de x coma, y»
// REVISAR (inglés: equal greater or than to): «greater than or equal to de x coma, y»
// REVISAR (inglés: equals identically): «identically equals de x coma, y»
// REVISAR (inglés: and if only): «if and only if de x coma, y»
// REVISAR (inglés: implies): «implies de x coma, y»
// REVISAR (inglés: inner product): «inner product de x coma, y»
// REVISAR (inglés: intersection): «intersection de x coma, y»
// REVISAR (inglés: less than): «less than de x coma, y»
// REVISAR (inglés: equal less or than to): «less than or equal to de x coma, y»
// REVISAR (inglés: list separator): «list separator de x coma, y»
// REVISAR (inglés: maps to): «maps to de x coma, y»
// REVISAR (inglés: member of): «member of de x coma, y»
// REVISAR (inglés: minus or plus): «minus or plus de x coma, y»
// REVISAR (inglés: not subset): «not subset de x coma, y»
// REVISAR (inglés: not superset): «not superset de x coma, y»
// REVISAR (inglés: equal not to): «not equal to de x coma, y»
// REVISAR (inglés: member not of): «not member of de x coma, y»
// REVISAR (inglés: not parallel to): «not parallel to de x coma, y»
// REVISAR (inglés: from obtained): «obtained from de x coma, y»
// REVISAR (inglés: or): «or de x coma, y»
// REVISAR (inglés: outer product): «outer product de x coma, y»
// REVISAR (inglés: parallel to): «parallel to de x coma, y»
// REVISAR (inglés: minus or plus): «plus or minus de x coma, y»
// REVISAR (inglés: precedes): «precedes de x coma, y»
// REVISAR (inglés: proportional): «proportional de x coma, y»
// REVISAR (inglés: separator): «range separator de x coma, y»
// REVISAR (inglés: ratio): «ratio de x coma, y»
// REVISAR (inglés: similar): «similar de x coma, y»
// REVISAR (inglés: subset): «subset de x coma, y»
// REVISAR (inglés: equal or subset): «subset or equal de x coma, y»
// REVISAR (inglés: succeeds): «succeeds de x coma, y»
// REVISAR (inglés: such that): «such that de x coma, y»
// REVISAR (inglés: superset): «superset de x coma, y»
// REVISAR (inglés: equal or superset): «superset or equal de x coma, y»
// REVISAR (de de, inglés: tilde): «tilde de x coma, y»
// REVISAR (inglés: times): «times de x coma, y»
// REVISAR (inglés: union): «union de x coma, y»
// REVISAR (inglés: ellipsis upwards): «upwards diagonal ellipsis de x coma, y»
// REVISAR (inglés: ellipsis): «vertical ellipsis de x coma, y»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn infix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("and", "and de x coma, y"),
        ("applied-to", "applied to de x coma, y"),
        ("approximately", "approximately de x coma, y"),
        ("congruent", "congruent de x coma, y"),
        ("cartesian-product", "cartesian product de x coma, y"),
        ("composed-with", "composed with de x coma, y"),
        ("cross-product", "x cross product y"),
        ("defined-as", "defined as de x coma, y"),
        ("divided-by", "divided by de x coma, y"),
        ("divides", "divides de x coma, y"),
        ("does-not-belong-to", "does not belong to de x coma, y"),
        ("does-not-divide", "does not divide de x coma, y"),
        ("dot-product", "x dot product y"),
        ("downwards-diagonal-ellipsis", "downwards diagonal ellipsis de x coma, y"),
        ("direct-product", "direct product de x coma, y"),
        ("element-of", "element of de x coma, y"),
        ("ellipsis", "ellipsis de x coma, y"),
        ("equals", "equals de x coma, y"),
        ("equivalent-to", "equivalent to de x coma, y"),
        ("evaluates-to", "evaluates to de x coma, y"),
        ("given", "given de x coma, y"),
        ("greater-than", "greater than de x coma, y"),
        ("greater-than-or-equal-to", "greater than or equal to de x coma, y"),
        ("identically-equals", "identically equals de x coma, y"),
        ("if-and-only-if", "if and only if de x coma, y"),
        ("implies", "implies de x coma, y"),
        ("inner-product", "inner product de x coma, y"),
        ("intersection", "intersection de x coma, y"),
        ("less-than", "less than de x coma, y"),
        ("less-than-or-equal-to", "less than or equal to de x coma, y"),
        ("list-separator", "list separator de x coma, y"),
        ("maps-to", "maps to de x coma, y"),
        ("member-of", "member of de x coma, y"),
        ("minus", "x menos y"),
        ("minus-or-plus", "minus or plus de x coma, y"),
        ("not-subset", "not subset de x coma, y"),
        ("not-superset", "not superset de x coma, y"),
        ("not-equal-to", "not equal to de x coma, y"),
        ("not-member-of", "not member of de x coma, y"),
        ("not-parallel-to", "not parallel to de x coma, y"),
        ("obtained-from", "obtained from de x coma, y"),
        ("or", "or de x coma, y"),
        ("outer-product", "outer product de x coma, y"),
        ("parallel-to", "parallel to de x coma, y"),
        ("perpendicular", "perpendicular de x coma, y"),
        ("plus", "x más y"),
        ("plus-or-minus", "plus or minus de x coma, y"),
        ("precedes", "precedes de x coma, y"),
        ("proportional", "proportional de x coma, y"),
        ("range-separator", "range separator de x coma, y"),
        ("ratio", "ratio de x coma, y"),
        ("similar", "similar de x coma, y"),
        ("subset", "subset de x coma, y"),
        ("subset-or-equal", "subset or equal de x coma, y"),
        ("succeeds", "succeeds de x coma, y"),
        ("such-that", "such that de x coma, y"),
        ("superset", "superset de x coma, y"),
        ("superset-or-equal", "superset or equal de x coma, y"),
        ("tilde", "tilde de x coma, y"),
        ("times", "times de x coma, y"),
        ("union", "union de x coma, y"),
        ("upwards-diagonal-ellipsis", "upwards diagonal ellipsis de x coma, y"),
        ("vertical-ellipsis", "vertical ellipsis de x coma, y"),
        ("xor", "xor de x coma, y"),
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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

// REVISAR (inglés: percent): «percent de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn postfix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("factorial", "factorial de x"),
        ("percent", "percent de x"),
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

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

// REVISAR (inglés: diameter): «diameter»
// REVISAR (inglés: distance): «distance»
// REVISAR (inglés: probability): «probability»
// REVISAR (inglés: radius): «radius»
// REVISAR (inglés: volume): «volume»
// REVISAR (inglés: exponential): «exponential e»
// REVISAR (inglés: imaginary): «imaginary i»
// REVISAR (inglés: golden ratio): «golden ratio»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn nofix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("diameter", "d", "diameter"),
        ("distance", "D", "distance"),
        ("probability", "P", "probability"),
        ("radius", "r", "radius"),
        ("volume", "V", "volume"),
        ("exponential-e", "e", "exponential e"),
        ("imaginary-i", "i", "imaginary i"),
        ("differential-d", "d", "differential d"),
        ("golden-ratio", "φ", "golden ratio"),
    ];

    for (intent, symbol, expected) in tests {
        let expr = format!(
            "<math>
                <mi intent='{}'>{}</mi>
            </math>",
            intent,
            symbol
        );

        test("es", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}