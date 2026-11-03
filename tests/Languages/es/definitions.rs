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

    test("es", "ClearSpeak", expr, "la tupla de x coma, y")?;
    test("es", "SimpleSpeak", expr, "la tupla de x coma, y")?;

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

#[test]
fn fixed_test() -> Result<()> {
    let expr = r#"
      <math>
        <mi intent="set-of-reals">R
        </mi>
      </math>
    "#;

    test("es", "ClearSpeak", expr, "el conjunto de los números reales")?;
    Ok(())
}

#[test]
fn i_test() -> Result<()> {
    let expr = r#"
      <math>
        <mi intent="imaginary-i">i
        </mi>
      </math>
    "#;

    test("es", "ClearSpeak", expr, "i")?;
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

    test("es", "ClearSpeak", expr, "el piso de x")?;

    Ok(())
}

#[test]
fn ratio_function_basic() -> Result<()> {
    let expr = r#"
      <math>
        <mrow intent="ratio($x,$y)">
          <mi arg="x">6</mi>
          <mi arg="y">3</mi>
        </mrow>
      </math>
    "#;

    test_prefs("es", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "6 es a 3")?;
    test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "6 es a 3")?;
    test_prefs("es", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "6 es a 3")?;
    test("es", "ClearSpeak", expr, "6 es a 3")?;
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

    test( "es", "ClearSpeak", expr, "diferencia de mayúscula a y mayúscula b")?;

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
            "x transpuesta",
        ),
        (
            "highlight",
            r#"
            <menclose intent="highlight($x)">
                <mi arg="x">x</mi>
            </menclose>
            "#,
            "x resaltado",
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
            "el límite cuando x",
        ),
        (
            "unit-vector",
            r#"
            <mover intent="unit-vector($x)">
                <mi arg="x">x</mi>
                <mo>^</mo>
            </mover>
            "#,
            "vector unitario x",
        ),
        (
            "line-segment",
            r#"
            <mover intent="line-segment($x)">
                <mi arg="x">x</mi>
                <mo>¯</mo>
            </mover>
            "#,
            "segmento x",
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


// REVISAR (-th, inglés: th): «x a la y-th potencia»
// REVISAR (inglés: absolute value): «el absolute value de x»
// REVISAR (inglés: polar): «coordenada polar de x coma, y»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn functions_and_inverses_tests() -> Result<()> {
    let tests = vec![
        //("closed-interval", "closed interval between x and y"),
        //("closed-open-interval", "interval between x included and y"),
        //("open-closed-interval", "interval between x and y included"),
        //("open-interval", "open interval between x and y"),

        ("inverse", "inversa de x"),
        ("domain", "el dominio de x"),
        ("codomain", "el codominio de x"),
        ("image", "la imagen de x"),

        //("fraction", "fraction x over y end fraction"),
        ("mixed-fraction", "x y y"),
        ("quotient", "parte entera de x dividido por y"),
        ("evaluated-at", "x evaluado en y"),
        ("remainder", "el resto de x dividido por y"),

        ("max", "el máximo de x coma, y coma, z"),
        ("min", "el mínimo de x coma, y coma, z"),

        ("power", "x a la y-th potencia"),

        // ("root", ...) skipped: ClearSpeak `tag: root` requires an index
        // (same as the Polish suite).

        ("greatest-common-divisor", "el mcd de x coma, y coma, z"),
        ("least-common-multiple", "el mcm de x coma, y coma, z"),

        ("absolute-value", "el absolute value de x"),

        ("complex-conjugate", "el conjugado complejo de x"),
        ("complex-arg", "el argumento de x"),
        ("real-part", "la parte real"),
        ("imaginary-part", "la parte imaginaria"),

        ("polar-coordinate", "coordenada polar de x coma, y"),
        ("spherical-coordinate", "coordenada esférica de x coma, y coma, z"),
        ("cartesian-coordinate", "coordenada cartesiana de x coma, y coma, z"),
        ("coordinate", "la coordenada de x coma, y coma, z"),

        ("floor", "el piso de x"),
        ("ceiling", "el techo de x"),
        ("round", "el redondeo de x"),
        ("fractional-part", "la parte fraccionaria de x"),

        
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


#[test]
fn calculus_tests() -> Result<()> {
    let tests = vec![
        //("derivative", "the derivative of x with respect to x"),
        //"definite-integral", "integral over x from x to x"),

        // prefix
        ("limit", "el límite cuando x"),

        // infix
        ("tends-to", "x tiende a y"),
        ("tends-to-from-above", "x tiende por la derecha a y"),
        ("tends-to-from-below", "x tiende por la izquierda a y"),
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


#[test]
fn set_tests() -> Result<()> {
    let tests = vec![
        ("set", "el conjunto x"),
        // ("set-difference", "set difference of x and y"),
        ("complement", "el complemento de x"),
        //("empty-set", "empty set"),
        ("cardinality", "el cardinal de x"),
        ("list", "la lista de x"),
        ("tuple", "la tupla de x"),
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
            "la suma de x",
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
            "la suma sobre i de x",
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
            "la suma desde i hasta n de x",
        ),
        (
            "product-1",
            r#"
            <mrow intent="product($x)">
                <mo>&#x220F;</mo>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "el producto de x",
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
            "el producto sobre i de x",
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
            "el producto desde i hasta n de x",
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

#[test]
fn elementary_classical_tests() -> Result<()> {
    let tests = vec![
        // Trig
        ("sine", "seno de x"),
        ("cosine", "coseno de x"),
        ("tangent", "tangente de x"),
        ("secant", "secante de x"),
        ("cosecant", "cosecante de x"),
        ("cotangent", "cotangente de x"),

        // Inverse trig
        ("arcsine", "arcoseno de x"),
        ("arccosine", "arcocoseno de x"),
        ("arctangent", "arcotangente de x"),
        ("arcsecant", "arcosecante de x"),
        ("arccosecant", "arcocosecante de x"),
        ("arccotangent", "arcocotangente de x"),

        // Hyperbolic trig
        ("hyperbolic-sine", "seno hiperbólico de x"),
        ("hyperbolic-cosine", "coseno hiperbólico de x"),
        ("hyperbolic-tangent", "tangente hiperbólica de x"),
        ("hyperbolic-secant", "secante hiperbólica de x"),
        ("hyperbolic-cosecant", "cosecante hiperbólica de x"),
        ("hyperbolic-cotangent", "cotangente hiperbólica de x"),

        // Inverse hyperbolic trig
        ("arc-hyperbolic-sine", "arcoseno hiperbólico de x"),
        ("arc-hyperbolic-cosine", "arcocoseno hiperbólico de x"),
        ("arc-hyperbolic-tangent", "arcotangente hiperbólica de x"),
        ("arc-hyperbolic-secant", "arcosecante hiperbólica de x"),
        ("arc-hyperbolic-cosecant", "arcocosecante hiperbólica de x"),
        ("arc-hyperbolic-cotangent", "arcocotangente hiperbólica de x"),
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

#[test]
fn statistics_and_probability_tests() -> Result<()> {
    let tests = vec![
        ("mean", "la media de x"),
        ("standard-deviation", "la desviación estándar de x"),
        ("variance", "la varianza de x"),
        ("median", "la mediana de x"),
        ("mode", "la moda de x"),

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

#[test]
fn linear_algebra_tests() -> Result<()> {
    let tests = vec![
        ("vector", "vector de x"),
        // ("matrix", "matrix of x"),
        ("determinant", "determinante de x"),
        ("adjugate", "la matriz adjunta de x"),
        ("magnitude", "el módulo de x"),
        ("norm", "norma de x"),
        ("span", "el espacio generado de x"),

        // transpose supports both postfix and function; we test function explicitly
        ("transpose", "transpuesta de x"),

        // dimensional product is infix
        ("dimensional-product", "x por y"),

        // unit-vector is prefix
        ("unit-vector", "vector unitario x")
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

#[test]
fn nofix_set_tests() -> Result<()> {
    let tests = vec![
        ("set-of-integers", "ℤ", "el conjunto de los números enteros"),
        ("set-of-reals", "ℝ", "el conjunto de los números reales"),
        ("set-of-rationals", "ℚ", "el conjunto de los números racionales"),
        ("set-of-natural-numbers", "ℕ", "el conjunto de los números naturales"),
        ("set-of-complex-numbers", "ℂ", "el conjunto de los números complejos"),
        ("set-of-primes", "ℙ", "el conjunto de los números primos"),
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

#[test]
fn geometry_prefix_multi_value_tests() -> Result<()> {
    let tests = vec![
        ("line-segment", "segmento de línea x y"),
        ("directed-line-segment", "segmento dirigido x y"),
        ("line", "recta x y"),
        ("ray", "semirrecta x y"),
        ("arc", "arco x y"),
        ("point", "punto x y z"),
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

#[test]
fn geometry_prefix_tests() -> Result<()> {
    let tests = vec![
        ("length", "la longitud de x"),
        ("area", "el área de x"),
        ("volume", "volumen de x"),
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

#[test]
fn separator_tests() -> Result<()> {
  let expr = "<math>
            <mrow intent='time-separator($x,$y)'>
                <mi arg='x'>x</mi>
                <mi arg='y'>y</mi>
            </mrow>
        </math>".to_string();

    test("es", "ClearSpeak", &expr, "x y")?;
    Ok(())
}

// REVISAR (inglés: fenced group): «fenced-group de x»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn general_concepts_tests() -> Result<()> {
    let tests = vec![
        // Unary structural
        ("fenced-group", "fenced-group de x"),
        ("highlight", "x resaltado"),
        ("least-common-denominator", "el mínimo común denominador de x coma, y coma, z"), // add x, y , z ...
        ("pochhammer", "permutación de x"),
        ("permutation-cycle", "el ciclo de permutación de x"),

        // Binary structural / infix-style
        // ("ordered-pair", "the pair of x and y"),
        ("rate", "x por y"),
        
        ("binomial-coefficient", "x sobre y"),
        ("embellished-name", "x con anotación y"),
        ("indexed-by", "x subíndice y"),
        // ("translation", "translation by of x comma, y"), // "of" after "by" is awkward; needs dedicated rule or of-suppression
        ("constraint", "x con la restricción y"),
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

#[test]
fn grouping_tests() -> Result<()> {
    let tests = vec![
        ("annotation", "x que es y"),
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

#[test]
fn function_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("curl", "rotacional de x"),
        ("divergence", "divergencia de x"),
        ("gradient", "gradiente de x"),
        ("laplacian", "laplaciano de x"),
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

#[test]
fn prefix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("angle", "ángulo x"),
        ("angle-measure", "medida del ángulo x"),
        ("change", "incremento de x"),
        ("for-all", "para todo x"),
        ("measured-angle", "ángulo medido x"),
        ("not", "no x"),
        ("number-of", "número de x"),
        ("partial-derivative", "parcial x"),
        ("right-angle", "ángulo recto x"),
        ("square-root-of", "raíz cuadrada de x"),
        ("there-does-not-exist", "no existe x"),
        ("there-exists", "existe x"),
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

// REVISAR (inglés: divide): «x divide a y»
// REVISAR (inglés: divide): «x no divide a y»
// REVISAR (distinto de): «x distinto de y»
// REVISAR (inglés: tilde): «x tilde y»
#[test]
#[ignore = "lectura en español pendiente de revisión"]
fn infix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("and", "x y y"),
        ("applied-to", "x aplicado a y"),
        ("approximately", "x aproximadamente igual a y"),
        ("congruent", "x congruente con y"),
        ("cartesian-product", "x producto cartesiano y"),
        ("composed-with", "x compuesta con y"),
        ("cross-product", "x producto vectorial y"),
        ("defined-as", "x definido como y"),
        ("divided-by", "x dividido por y"),
        ("divides", "x divide a y"),
        ("does-not-belong-to", "x no pertenece a y"),
        ("does-not-divide", "x no divide a y"),
        ("dot-product", "x producto escalar y"),
        ("downwards-diagonal-ellipsis", "x puntos suspensivos diagonales descendentes y"),
        ("direct-product", "x producto directo y"),
        ("element-of", "x pertenece a y"),
        ("ellipsis", "x puntos suspensivos y"),
        ("equals", "x es igual a y"),
        ("equivalent-to", "x equivalente a y"),
        ("evaluates-to", "x da como resultado y"),
        ("given", "x dado y"),
        ("greater-than", "x mayor que y"),
        ("greater-than-or-equal-to", "x mayor o igual que y"),
        ("identically-equals", "x idénticamente igual a y"),
        ("if-and-only-if", "x si y solo si y"),
        ("implies", "x implica y"),
        ("inner-product", "x producto interior y"),
        ("intersection", "x intersección y"),
        ("less-than", "x menor que y"),
        ("less-than-or-equal-to", "x menor o igual que y"),
        ("list-separator", "x coma y"),
        ("maps-to", "x se asigna a y"),
        ("member-of", "x pertenece a y"),
        ("minus", "x menos y"),
        ("minus-or-plus", "x menos más y"),
        ("not-subset", "x no es subconjunto de y"),
        ("not-superset", "x no es superconjunto de y"),
        ("not-equal-to", "x distinto de y"),
        ("not-member-of", "x no pertenece a y"),
        ("not-parallel-to", "x no es paralela a y"),
        ("obtained-from", "x obtenido de y"),
        ("or", "x o y"),
        ("outer-product", "x producto exterior y"),
        ("parallel-to", "x paralela a y"),
        ("perpendicular", "x perpendicular a y"),
        ("plus", "x más y"),
        ("plus-or-minus", "x más menos y"),
        ("precedes", "x precede a y"),
        ("proportional", "x proporcional a y"),
        ("range-separator", "x a y"),
        ("ratio", "x es a y"),
        ("similar", "x semejante a y"),
        ("subset", "x subconjunto de y"),
        ("subset-or-equal", "x subconjunto o igual a y"),
        ("succeeds", "x sucede a y"),
        ("such-that", "x tal que y"),
        ("superset", "x superconjunto de y"),
        ("superset-or-equal", "x superconjunto o igual a y"),
        ("tilde", "x tilde y"),
        ("times", "x por y"),
        ("union", "x unión y"),
        ("upwards-diagonal-ellipsis", "x puntos suspensivos diagonales ascendentes y"),
        ("vertical-ellipsis", "x puntos suspensivos verticales y"),
        ("xor", "x o exclusivo y"),
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

#[test]
fn postfix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("factorial", "x factorial"),
        ("percent", "x por ciento"),
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

#[test]
fn nofix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("diameter", "d", "diámetro"),
        ("distance", "D", "distancia"),
        ("probability", "P", "probabilidad"),
        ("radius", "r", "radio"),
        ("volume", "V", "volumen"),
        ("exponential-e", "e", "e"),
        ("imaginary-i", "i", "i"),
        ("differential-d", "d", "d"),
        ("golden-ratio", "φ", "número áureo"),
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