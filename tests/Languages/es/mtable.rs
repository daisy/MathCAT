use crate::common::*;
use anyhow::Result;

#[test]
fn matrix_1x1() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable><mtr><mtd>
        <mn>3</mn>
      </mtd> </mtr></mtable>
        <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("es", "ClearSpeak",  expr, "la matriz 1 por 1 con elemento 3")?;
    test("es", "SimpleSpeak", expr, "la matriz 1 por 1 con elemento 3")?;
    return Ok(());

}

#[test]
fn determinant_1x1() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>|</mo>
        <mtable><mtr><mtd>
        <mn>3</mn>
      </mtd> </mtr></mtable>
        <mo>|</mo></mrow></mrow>
    </math>
                                ";
    test("es", "ClearSpeak",  expr, "el determinante 1 por 1 con elemento 3")?;
    test("es", "SimpleSpeak", expr, "el determinante 1 por 1 con elemento 3")?;
    return Ok(());

}


#[test]
fn matrix_1x2() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>5</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("es", "ClearSpeak",  expr, "la matriz fila 1 por 2; 3, 5")?;
    test("es", "SimpleSpeak", expr, "la matriz fila 1 por 2; 3, 5")?;
    return Ok(());

}


#[test]
fn matrix_1x3() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mrow><mo>-</mo><mi>x</mi></mrow>
          </mtd>
          <mtd>
            <mn>5</mn>
          </mtd>
          <mtd>
            <mn>12</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("es", "ClearSpeak", expr, "la matriz fila 1 por 3; menos x, 5, 12")?;
    test("es", "SimpleSpeak", expr, "la matriz fila 1 por 3; menos x, 5, 12")?;
    return Ok(());

}

#[test]
fn matrix_2x1_not_simple() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mrow>
            <mi>x</mi><mo>+</mo><mn>1</mn>
            </mrow>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mrow>
            <mi>x</mi><mo>-</mo><mn>1</mn></mrow>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("es", "ClearSpeak", expr, "la matriz columna 2 por 1; fila 1; x más 1; fila 2; x menos 1")?;
    test("es", "SimpleSpeak", expr, "la matriz columna 2 por 1; fila 1; x más 1; fila 2; x menos 1")?;
    return Ok(());

}
#[test]
fn matrix_3x1_not_simple() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mrow>
            <mi>x</mi>
            </mrow>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mrow>
            <mi>a</mi>
            </mrow>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mfrac>
              <mi>x</mi>
              <mrow>
                <mi>x</mi><mo>+</mo><mn>1</mn>
              </mrow>
            </mfrac>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>";
    test("es", "SimpleSpeak", expr, "la matriz columna 3 por 1; fila 1; x; fila 2; a; fila 3; fracción, x partido por, x más 1, fin de fracción")?;
    test("es", "ClearSpeak",  expr, "la matriz columna 3 por 1; fila 1; x; fila 2; a; fila 3; la fracción con numerador x; y denominador x más 1")?;
            return Ok(());

}

#[test]
fn determinant_2x2() -> Result<()> {
    let expr = "<math>
      <mrow>
      <mrow><mo>|</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>7</mn>
          </mtd>
          <mtd>
            <mn>5</mn>
          </mtd>
          </mtr>
          
        </mtable>
      <mo>|</mo></mrow></mrow>
                        </math>";
    test("es", "ClearSpeak",  expr, "el determinante 2 por 2; fila 1; 2, 1; fila 2; 7, 5")?;
    test("es", "SimpleSpeak", expr, "el determinante 2 por 2; fila 1; 2, 1; fila 2; 7, 5")?;
    return Ok(());

}

#[test]
fn matrix_2x3() -> Result<()> {
    let expr = "
    <math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>[</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>4</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>6</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>]</mo></mrow></mrow>
    </math>
                                ";
    test("es", "ClearSpeak",  expr, "la matriz 2 por 3; fila 1; 3, 1, 4; fila 2; 0, 2, 6")?;
    test("es", "SimpleSpeak", expr, "la matriz 2 por 3; fila 1; 3, 1, 4; fila 2; 0, 2, 6")?;
    return Ok(());

}

#[test]
fn augmented_matrix_2x3() -> Result<()> {
    let expr = "
    <math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>[</mo>
        <mtable columnlines='none solid'>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>4</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>6</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>]</mo></mrow></mrow>
    </math>
                                ";
    test("es", "ClearSpeak",  expr, "la matriz ampliada 2 por 3; fila 1; 3, 1, separador, 4; fila 2; 0, 2, separador, 6")?;
    test("es", "SimpleSpeak", expr, "la matriz ampliada 2 por 3; fila 1; 3, 1, separador, 4; fila 2; 0, 2, separador, 6")?;
    Ok(())
}

#[test]
fn dashed_augmented_matrix_separator() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow><mo>[</mo>
        <mtable columnlines='dashed'>
          <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr>
        </mtable>
      <mo>]</mo></mrow>
    </math>";
    test("es", "ClearSpeak", expr, "la matriz fila 1 por 3; 1, separador, 2, separador, 3")?;
    test("es", "SimpleSpeak", expr, "la matriz fila 1 por 3; 1, separador, 2, separador, 3")?;
    Ok(())
}

/// A horizontal line is announced once, after the row it separates from the next row.
#[test]
fn matrix_row_separator() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow><mo>[</mo>
        <mtable rowlines='solid'>
          <mtr>
            <mtd><mn>1</mn></mtd>
            <mtd><mn>2</mn></mtd>
          </mtr>
          <mtr>
            <mtd><mn>3</mn></mtd>
            <mtd><mn>4</mn></mtd>
          </mtr>
        </mtable>
      <mo>]</mo></mrow>
    </math>";
    test("es", "ClearSpeak", expr, "la matriz 2 por 2; fila 1; 1, 2, separador de filas; fila 2; 3, 4")?;
    test("es", "SimpleSpeak", expr, "la matriz 2 por 2; fila 1; 1, 2, separador de filas; fila 2; 3, 4")?;
    Ok(())
}

/// Horizontal and vertical lines use distinct announcements at their respective boundaries.
#[test]
fn matrix_row_and_column_separators() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow><mo>[</mo>
        <mtable rowlines='dashed' columnlines='solid'>
          <mtr>
            <mtd><mn>1</mn></mtd>
            <mtd><mn>2</mn></mtd>
          </mtr>
          <mtr>
            <mtd><mn>3</mn></mtd>
            <mtd><mn>4</mn></mtd>
          </mtr>
        </mtable>
      <mo>]</mo></mrow>
    </math>";
    test("es", "ClearSpeak", expr, "la matriz ampliada 2 por 2; fila 1; 1, separador, 2, separador de filas; fila 2; 3, separador, 4")?;
    test("es", "SimpleSpeak", expr, "la matriz ampliada 2 por 2; fila 1; 1, separador, 2, separador de filas; fila 2; 3, separador, 4")?;
    Ok(())
}

#[test]
fn matrix_2x3_labeled() -> Result<()> {
    let expr = "
    <math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>[</mo>
        <mtable>
          <mlabeledtr>
          <mtd>
            <mtext>(3.1)</mtext>
          </mtd>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>4</mn>
          </mtd>
          </mlabeledtr>
          <mtr>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>6</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>]</mo></mrow></mrow>
    </math>
                                ";
    test("es", "ClearSpeak",  expr,
        "la matriz 2 por 3; fila 1 con etiqueta (3.1); columna 1; 3, columna 2; 1, columna 3; 4; fila 2; columna 1; 0, columna 2; 2, columna 3; 6")?;
    test("es", "SimpleSpeak", expr,
        "la matriz 2 por 3; fila 1 con etiqueta (3.1); columna 1; 3, columna 2; 1, columna 3; 4; fila 2; columna 1; 0, columna 2; 2, columna 3; 6")?;
                                   return Ok(());

}

#[test]
fn matrix_3x1() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>[</mo>
        <mtable>
        <mtr>
          <mtd>
          <mn>1</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>2</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>3</mn>
          </mtd>
        </mtr>           
        </mtable> <mo>]</mo></mrow></mrow>
    </math>
                                ";
    test("es", "ClearSpeak",  expr, "la matriz columna 3 por 1; 1; 2; 3")?;
    test("es", "SimpleSpeak", expr, "la matriz columna 3 por 1; 1; 2; 3")?;
    return Ok(());

}

#[test]
fn matrix_4x1() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>6</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>1</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>2</mn>
          </mtd>
          </mtr>            
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("es", "ClearSpeak",  expr, "la matriz columna 4 por 1; fila 1; 3; fila 2; 6; fila 3; 1; fila 4; 2")?;
    test("es", "SimpleSpeak", expr, "la matriz columna 4 por 1; fila 1; 3; fila 2; 6; fila 3; 1; fila 4; 2")?;
    return Ok(());

}

#[test]
fn matrix_4x1_labeled() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>6</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>1</mn>
          </mtd>
          </mtr>
          <mlabeledtr>
          <mtd>
            <mtext>(3.1)</mtext>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          </mlabeledtr>            
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("es", "ClearSpeak",  expr,
        "la matriz columna 4 por 1; fila 1; 3; fila 2; 6; fila 3; 1; fila 4 con etiqueta (3.1); 2")?;
    test("es", "SimpleSpeak", expr,
        "la matriz columna 4 por 1; fila 1; 3; fila 2; 6; fila 3; 1; fila 4 con etiqueta (3.1); 2")?;
        return Ok(());

}

#[test]
fn matrix_1x4() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>6</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("es", "ClearSpeak",  expr, "la matriz fila 1 por 4; columna 1; 3, columna 2; 6, columna 3; 1, columna 4; 2")?;
    test("es", "SimpleSpeak", expr, "la matriz fila 1 por 4; columna 1; 3, columna 2; 6, columna 3; 1, columna 4; 2")?;
    return Ok(());

}

#[test]
fn matrix_4x4() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>4</mn>
          </mtd>
          <mtd>
            <mn>3</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>9</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>6</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>9</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("es", "ClearSpeak",  expr, "la matriz 4 por 4; fila 1; columna 1; 0, columna 2; 3, columna 3; 4, columna 4; 3; fila 2; columna 1; 2, columna 2; 1, columna 3; 0, columna 4; 9; fila 3; columna 1; 3, columna 2; 0, columna 3; 2, columna 4; 1; fila 4; columna 1; 6, columna 2; 2, columna 3; 9, columna 4; 0")?;
    test("es", "SimpleSpeak", expr, "la matriz 4 por 4; fila 1; columna 1; 0, columna 2; 3, columna 3; 4, columna 4; 3; fila 2; columna 1; 2, columna 2; 1, columna 3; 0, columna 4; 9; fila 3; columna 1; 3, columna 2; 0, columna 3; 2, columna 4; 1; fila 4; columna 1; 6, columna 2; 2, columna 3; 9, columna 4; 0")?;
    return Ok(());
}

#[test]
fn matrix_4x2() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
    <mrow>
      <mrow><mo>(</mo>
        <mtable>
        <mtr>
          <mtd>
          <mn>1</mn>
          </mtd>
          <mtd>
          <mn>3</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>4</mn>
          </mtd>
          <mtd>
          <mn>2</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>2</mn>
          </mtd>
          <mtd>
          <mn>1</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>0</mn>
          </mtd>
          <mtd>
          <mn>5</mn>
          </mtd>
        </mtr>
        
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
      ";
    test("es", "ClearSpeak",  expr, "la matriz 4 por 2; fila 1; columna 1; 1, columna 2; 3; fila 2; columna 1; 4, columna 2; 2; fila 3; columna 1; 2, columna 2; 1; fila 4; columna 1; 0, columna 2; 5")?;
    test("es", "SimpleSpeak", expr, "la matriz 4 por 2; fila 1; columna 1; 1, columna 2; 3; fila 2; columna 1; 4, columna 2; 2; fila 3; columna 1; 2, columna 2; 1; fila 4; columna 1; 0, columna 2; 5")?;
    return Ok(());
}

// put absolute value test here since it is related to determinate and is small for its own file
#[test]
fn simple_absolute_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>x</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test("es", "SimpleSpeak", expr, "el valor absoluto de x")?;
  test("es", "ClearSpeak",  expr, "el valor absoluto de x")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "Auto")], expr, "valor absoluto de x")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "el valor absoluto de x, fin del valor absoluto")?;
  return Ok(());
}
  
#[test]
fn absolute_value_plus_1() -> Result<()> {
let expr = "<math>
    <mrow><mrow><mo>|</mo>
      <mrow><mi>x</mi><mo>+</mo><mn>1</mn> </mrow>
    <mo>|</mo></mrow></mrow>
  </math>";
  test("es", "ClearSpeak", expr, "el valor absoluto de x más 1")?;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "valor absoluto de x más 1, fin del valor absoluto")?;
  return Ok(());
}

#[test]
fn simple_cardinality_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>S</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_AbsoluteValue", "Cardinality")], expr,
             "el cardinal de mayúscula s")?;
    return Ok(());
}
  
// Test preferences
#[test]
fn simple_matrix_speak_col_num() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("es", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "la matriz 2 por 2; fila 1; columna 1; 2, columna 2; 1; fila 2; columna 1; 7, columna 2; 5")?;
    return Ok(());
}

#[test]
fn col_matrix_3x1_speak_col_num() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "la matriz columna 3 por 1; fila 1; 1; fila 2; 2; fila 3; 3")?;
    return Ok(());
}

#[test]
fn row_matrix_1x2_speak_col_num() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "la matriz fila 1 por 2; columna 1; 1, columna 2; 2")?;
    return Ok(());
}

#[test]
fn matrix_2x2_speak_col_num() -> Result<()> {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "la matriz 2 por 2; fila 1; columna 1; b sub 1 1; columna 2; b sub 1 2; fila 2; columna 1; b sub 2 1; columna 2; b sub 2 2")?;
    return Ok(());
}


#[test]
fn simple_matrix_silent_col_num() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("es", "ClearSpeak_Matrix", "SilentColNum",
        expr, "la matriz 2 por 2; fila 1; 2, 1; fila 2; 7, 5")?;
    return Ok(());
}

#[test]
fn col_matrix_3x1_silent_col_num() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "SilentColNum",
        expr, "la matriz columna 3 por 1; 1; 2; 3")?;
    return Ok(());
}

#[test]
fn row_matrix_1x2_silent_col_num() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "SilentColNum",
        expr, "la matriz fila 1 por 2; 1, 2")?;
    return Ok(());
}

#[test]
fn matrix_2x2_silent_col_num() -> Result<()> {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "SilentColNum",
        expr, "la matriz 2 por 2; fila 1; b sub 1 1; b sub 1 2; fila 2; b sub 2 1; b sub 2 2")?;
    return Ok(());
  }


#[test]
fn simple_matrix_end_matrix() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("es", "ClearSpeak_Matrix", "EndMatrix",
        expr, "la matriz 2 por 2; fila 1; 2, 1; fila 2; 7, 5; fin de la matriz")?;
    return Ok(());
  }

#[test]
fn col_matrix_3x1_end_matrix() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "EndMatrix",
        expr, "la matriz columna 3 por 1; 1; 2; 3; fin de la matriz")?;
    return Ok(());
  }

#[test]
fn row_matrix_1x2_end_matrix() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "EndMatrix",
        expr, "la matriz fila 1 por 2; 1, 2; fin de la matriz")?;
    return Ok(());
  }

#[test]
fn matrix_2x2_end_matrix() -> Result<()> {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "EndMatrix",
        expr, "la matriz 2 por 2; fila 1; columna 1; b sub 1 1; columna 2; b sub 1 2; fila 2; columna 1; b sub 2 1; columna 2; b sub 2 2; fin de la matriz")?;
    return Ok(());
  }

#[test]
fn augmented_matrix_3x4_end_matrix() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
      <mtable columnalign='right right right right' columnlines='none none solid'>
        <mtr>
          <mtd><mn>1</mn></mtd>
          <mtd><mn>2</mn></mtd>
          <mtd><mrow><mo>-</mo><mn>1</mn></mrow></mtd>
          <mtd><mn>3</mn></mtd>
        </mtr>
        <mtr>
          <mtd><mrow><mo>-</mo><mn>3</mn></mrow></mtd>
          <mtd><mn>3</mn></mtd>
          <mtd><mrow><mo>-</mo><mn>1</mn></mrow></mtd>
          <mtd><mn>2</mn></mtd>
        </mtr>
        <mtr>
          <mtd><mn>2</mn></mtd>
          <mtd><mn>3</mn></mtd>
          <mtd><mn>2</mn></mtd>
          <mtd><mrow><mo>-</mo><mn>1</mn></mrow></mtd>
        </mtr>
      </mtable>
    <mo>]</mo></mrow>
  </mrow>
</math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "EndMatrix",
        expr, "la matriz ampliada 3 por 4; fila 1; columna 1; 1, columna 2; 2, columna 3; menos 1, separador, columna 4; 3; fila 2; columna 1; menos 3, columna 2; 3, columna 3; menos 1, separador, columna 4; 2; fila 3; columna 1; 2, columna 2; 3, columna 3; 2, separador, columna 4; menos 1; fin de la matriz")?;
    test("es", "SimpleSpeak",
        expr, "la matriz ampliada 3 por 4; fila 1; columna 1; 1, columna 2; 2, columna 3; menos 1, separador, columna 4; 3; fila 2; columna 1; menos 3, columna 2; 3, columna 3; menos 1, separador, columna 4; 2; fila 3; columna 1; 2, columna 2; 3, columna 3; 2, separador, columna 4; menos 1; fin de la matriz")?;
    Ok(())
  }


#[test]
fn simple_matrix_vector() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("es", "ClearSpeak_Matrix", "Vector",
        expr, "la matriz 2 por 2; fila 1; 2, 1; fila 2; 7, 5")?;
    return Ok(());
  }

#[test]
fn col_matrix_3x1_vector() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "Vector",
        expr, "el vector columna 3 por 1; 1; 2; 3")?;
    return Ok(());
  }

#[test]
fn row_matrix_1x2_vector() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "Vector",
        expr, "el vector fila 1 por 2; 1, 2")?;
    return Ok(());
  }

#[test]
fn matrix_2x2_vector() -> Result<()> {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "Vector",
        expr, "la matriz 2 por 2; fila 1; columna 1; b sub 1 1; columna 2; b sub 1 2; fila 2; columna 1; b sub 2 1; columna 2; b sub 2 2")?;
    return Ok(());
  }


#[test]
fn simple_matrix_end_vector() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("es", "ClearSpeak_Matrix", "EndVector",
        expr, "la matriz 2 por 2; fila 1; 2, 1; fila 2; 7, 5; fin del vector")?;
    return Ok(());
  }

#[test]
fn col_matrix_3x1_end_vector() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "EndVector",
        expr, "el vector columna 3 por 1; 1; 2; 3; fin del vector")?;
    return Ok(());
  }

#[test]
fn row_matrix_1x2_end_vector() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "EndVector",
        expr, "el vector fila 1 por 2; 1, 2; fin del vector")?;
    return Ok(());
  }

#[test]
fn matrix_2x2_end_vector() -> Result<()> {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("es", "ClearSpeak_Matrix", "EndVector",
        expr, "la matriz 2 por 2; fila 1; columna 1; b sub 1 1; columna 2; b sub 1 2; fila 2; columna 1; b sub 2 1; columna 2; b sub 2 2; fin del vector")?;
  return Ok(());
}


#[test]
fn matrix_binomial() -> Result<()> {
  let expr = "<math>
      <mo>(</mo><mrow>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
      </mrow><mo>)</mo>
    </math>";
  test_ClearSpeak("es", "ClearSpeak_Matrix", "Combinatorics", expr, "3 sobre 2")?;
  return Ok(());
}

#[test]
fn matrix_simple_table() -> Result<()> {
  let expr = "<math>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
    </math>";
  test("es", "ClearSpeak", expr, "arreglo con 2 filas y 1 columna; fila 1; columna 1; 3; fila 2; columna 1; 2")
}

#[test]
fn mtable_prefix_op() -> Result<()>{
    // When a table is prefixed with a non-blank operator, assume it is something besides array intent
    for (op, speech) in [("(", "abrir paréntesis, 2 líneas; línea 1; 3; línea 2; 2"),
			 ("[", "abrir corchetes, 2 líneas; línea 1; 3; línea 2; 2"),
			 ("|", "barra vertical, 2 líneas; línea 1; 3; línea 2; 2"),
			 ("f", "f, 2 líneas; línea 1; 3; línea 2; 2")] {
	let expr = format!("<math>
        <mo>{op}</mo><mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
    </math>");
	test("es", "ClearSpeak", &expr, speech)?;
    }
    Ok(())
}

#[test]
fn mtable_blank_op() -> Result<()>{
    // When a table is prefixed with a blank operator, still assume array intent
    let expr = "<math>
        <mo>\u{2062}</mo><mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
    </math>";
    test("es", "ClearSpeak", expr, "; arreglo con 2 filas y 1 columna; fila 1; columna 1; 3; fila 2; columna 1; 2")
}



#[test]
fn mtable_colspan_table() -> Result<()>{
  let expr = "<math>
        <mtable><mtr><mtd colspan=\"2\"><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable>
    </math>";
  test("es", "ClearSpeak", expr, "arreglo con 2 filas y 1 columna; fila 1; columna 1; 3; fila 2; columna 1; 2, columna 2; 4")
}

#[test]
fn bug_mtable_rowspan_colspan() -> Result<()>{
  // Currently, the code correctly computes the number of rows and
  // columns, but it does not correctly compute the column number whnen
  // colspans are involved.
  let expr = "<math>
        <mtable>
           <mtr>
             <mtd rowspan=\"2\" colspan=\"2\"><mi>a</mi></mtd>
             <mtd rowspan=\"2\"><mi>b</mi></mtd>
             <mtd colspan=\"2\"><mi>c</mi></mtd>
           </mtr>
           <mtr>
             <mtd><mi>d</mi></mtd><mtd><mi>e</mi></mtd>
           </mtr>
        </mtable>
    </math>";
    test("es", "ClearSpeak", expr,
	 "arreglo con 2 filas y 3 columnas; fila 1; columna 1; a, columna 2; b, columna 3; c; fila 2; columna 1; d, columna 2; e")
}

#[test]
fn mtable_array_no_comma_between_rows() -> Result<()> {
  // Issue #890: an unfenced table gets 'array' intent; its rows must not be separated by "comma"
  let expr = "<math display=\"inline\">
      <mtable class=\"menv-arraylike\">
        <mtr>
          <mtd><mi>a</mi></mtd>
          <mtd><mi>b</mi></mtd>
        </mtr>
        <mtr>
          <mtd><mi>c</mi></mtd>
          <mtd><mi>d</mi></mtd>
        </mtr>
      </mtable>
    </math>";
  test("es", "ClearSpeak", expr,
    "arreglo con 2 filas y 2 columnas; fila 1; columna 1; a, columna 2; b; fila 2; columna 1; c, columna 2; d")?;
  test("es", "SimpleSpeak", expr,
    "arreglo con 2 filas y 2 columnas; fila 1; columna 1; a, columna 2; b; fila 2; columna 1; c, columna 2; d")
}



#[test]
fn matrix_times() {
  let expr = "<math>
    <mfenced><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable></mfenced>
    <mfenced><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable></mfenced>
  </math>";
  let _ = test("es", "SimpleSpeak", expr,
    "la matriz 2 por 2; fila 1; 1, 2; fila 2; 3, 4; por, la matriz 2 por 2; fila 1; a, b; fila 2; c, d");
}

#[test]
fn unknown_mtable_property() -> Result<()> {
  let expr = "<math display='block'>
      <mtable intent=':system-of-equations:prefix($e1,$e1x)'>
        <mtr arg='e1'>
        <mtd columnalign='right'>
          <mi>a</mi>
        </mtd>
        <mtd columnalign='center'>
          <mo>=</mo>
        </mtd>
        <mtd intent='_($lhs)' columnalign='left'>
          <mrow arg='lhs'>
          <mi>b</mi>
          <mo>+</mo>
          <mi>c</mi>
          <mo>&#x2212;</mo>
          <mi>d</mi>
        </mrow>
        </mtd>
        </mtr>
        <mtr arg='e1x'>
        <mtd intent='_' columnalign='right'></mtd>
        <mtd intent='_' columnalign='center'></mtd>
        <mtd arg='rhs' columnalign='left'>
          <mo form='infix'>+</mo>
          <mi>e</mi>
          <mo>&#x2212;</mo>
          <mi>f</mi>
        </mtd>
        </mtr>
      </mtable>
    </math>";
    test("es", "ClearSpeak",  expr,
         "2 líneas; línea 1; a es igual a, b más c menos d; línea 2; más e menos f")?;
    return Ok(());
  }


#[test]
fn zero_matrix() -> Result<()> {
  let expr = "<math>
      <mo>[</mo>
      <mtable>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
      </mtable>
      <mo>]</mo>
  </math>";
  test("es", "SimpleSpeak", expr,
    "la matriz nula 2 por 2")?;
    return Ok(());
  }

#[test]
fn identity_matrix() -> Result<()> {
  let expr = "<math>
      <mo>(</mo>
      <mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr>
      </mtable>
      <mo>)</mo>
  </math>";
  test("es", "SimpleSpeak", expr,
    "la matriz identidad 3 por 3")?;
    return Ok(());
  }

#[test]
fn identity_matrix_false_positive_negative_one() -> Result<()> {
  let expr = "<math>
      <mo>[</mo>
      <mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>-1</mn></mtd></mtr>
      </mtable>
      <mo>]</mo>
  </math>";
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "la matriz diagonal 2 por 2; columna 1; 1; columna 2; menos 1")?;
  Ok(())
}

#[test]
fn identity_matrix_false_positive_zero_diagonal() -> Result<()> {
  let expr = "<math>
      <mo>[</mo>
      <mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
      </mtable>
      <mo>]</mo>
  </math>";
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "la matriz diagonal 2 por 2; columna 1; 1")?;
  Ok(())
}

#[test]
fn diagonal_matrix() -> Result<()> {
  let expr = "<math>
      <mo>(</mo>
      <mtable>
        <mtr><mtd><mn>2</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd><mtd><msup><mi>x</mi><mn>2</mn></msup></mtd></mtr>
      </mtable>
      <mo>)</mo>
  </math>";
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "la matriz diagonal 3 por 3; columna 1; 2; columna 2; 1; columna 3; x al cuadrado")?;
  // test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Verbose")],
  //     expr, "the 3 by 3 diagonal matrix; row 1, column 1, 2; row 2, column 2, 1; row 3, column 3, x squared");
    return Ok(());
  }

#[test]
fn single_line_with_label() -> Result<()> {
  let expr = r#"<math>
  <mtable class="gather" displaystyle="true" intent=":system-of-equations">
    <mtr>
      <mtd intent=":equation-label"> <mtext>(2)</mtext> </mtd>
      <mtd> <mi>𝑏</mi> <mo>=</mo> <mn>2</mn> </mtd>
    </mtr>
  </mtable>
  </math>"#;
  test_prefs("es", "ClearSpeak", vec![("Verbosity", "Terse")],
      expr, "1 línea, con etiqueta 2; b igual a 2")?;
  test_prefs("es", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "1 ecuación, con etiqueta 2; b igual a 2")?;
    return Ok(());
  }

#[test]
fn matrix_raised_to_power() -> Result<()> {
  // A parenthesized matrix as the base of msup must speak as "the matrix ... squared".
  // Regression for #762: the mrow 'matrix' intent rule returns a bare `x:`, so the intent tree
  // carried a nested TEMP_NAME wrapper into `power` and NVDA read "TEMP NAME of the 2 by 2 matrix ...".
  let expr = "<math><msup>
      <mrow><mo>(</mo>
        <mtable>
          <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr>
          <mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr>
        </mtable>
      <mo>)</mo></mrow>
      <mn>2</mn>
    </msup></math>";
  test("es", "ClearSpeak", expr, "la matriz 2 por 2; fila 1; 1, 2; fila 2; 3, 4; al cuadrado")
}

#[test]
fn matrix_raised_to_power_lualatex_spacing() -> Result<()> {
  // The exact MathML from #762 (LuaLaTeX pmatrix output): negative-width mspace around the
  // table and fence attributes on the parens. Canonicalization must fold the spacing away so
  // the matrix is still recognized as the base of the power.
  let expr = "<math display='block'>
    <msup>
      <mrow>
        <mo fence='true' lspace='0' rspace='0' symmetric='true'>(</mo>
        <mspace width='-4.981pt'/>
        <mrow>
          <mspace width='4.981pt'/>
          <mtable>
            <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr>
            <mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr>
          </mtable>
          <mspace width='4.981pt'/>
        </mrow>
        <mspace width='-4.981pt'/>
        <mo fence='true' lspace='0' rspace='0' symmetric='true'>)</mo>
      </mrow>
      <mn>2</mn>
    </msup>
  </math>";
  test("es", "ClearSpeak", expr, "la matriz 2 por 2; fila 1; 1, 2; fila 2; 3, 4; al cuadrado")
}
