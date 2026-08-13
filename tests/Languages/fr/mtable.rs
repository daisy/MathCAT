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
    test("fr", "ClearSpeak",  expr, "la matrice 1 par 1 avec l'élément 3")?;
    test("fr", "SimpleSpeak", expr, "la matrice 1 par 1 avec l'élément 3")?;
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
    test("fr", "ClearSpeak",  expr, "le déterminant 1 par 1 avec l'élément 3")?;
    test("fr", "SimpleSpeak", expr, "le déterminant 1 par 1 avec l'élément 3")?;
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
    test("fr", "ClearSpeak",  expr, "la matrice 1 par 2 rangée; 3, 5")?;
    test("fr", "SimpleSpeak", expr, "la matrice 1 par 2 rangée; 3, 5")?;
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
    test("fr", "ClearSpeak", expr, "la matrice 1 par 3 rangée; moins x, 5, 12")?;
    test("fr", "SimpleSpeak", expr, "la matrice 1 par 3 rangée; moins x, 5, 12")?;
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
    test("fr", "ClearSpeak", expr, "la matrice 2 par 1 colonne; rangée 1; x plus 1; rangée 2; x moins 1")?;
    test("fr", "SimpleSpeak", expr, "la matrice 2 par 1 colonne; rangée 1; x plus 1; rangée 2; x moins 1")?;
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
    test("fr", "SimpleSpeak", expr, "la matrice 3 par 1 colonne; \
            rangée 1; x; \
            rangée 2; a; \
            rangée 3; fraction, x sur, x plus 1, fin de fraction")?;
    test("fr", "ClearSpeak",  expr, "la matrice 3 par 1 colonne; \
            rangée 1; x; \
            rangée 2; a; \
            rangée 3; la fraction avec numérateur x; et dénominateur x plus 1")?;
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
    test("fr", "ClearSpeak",  expr, "le déterminant 2 par 2; rangée 1; 2, 1; rangée 2; 7, 5")?;
    test("fr", "SimpleSpeak", expr, "le déterminant 2 par 2; rangée 1; 2, 1; rangée 2; 7, 5")?;
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
    test("fr", "ClearSpeak",  expr, "la matrice 2 par 3; rangée 1; 3, 1, 4; rangée 2; 0, 2, 6")?;
    test("fr", "SimpleSpeak", expr, "la matrice 2 par 3; rangée 1; 3, 1, 4; rangée 2; 0, 2, 6")?;
    return Ok(());

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
    test("fr", "ClearSpeak",  expr,
        "la matrice 2 par 3; rangée 1 avec l'étiquette (3.1); colonne 1; 3, colonne 2; 1, colonne 3; 4; \
                                   rangée 2; colonne 1; 0, colonne 2; 2, colonne 3; 6")?;
    test("fr", "SimpleSpeak", expr,
        "la matrice 2 par 3; rangée 1 avec nom (3.1); colonne 1; 3, colonne 2; 1, colonne 3; 4; \
                                   rangée 2; colonne 1; 0, colonne 2; 2, colonne 3; 6")?;
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
    test("fr", "ClearSpeak",  expr, "la matrice 3 par 1 colonne; 1; 2; 3")?;
    test("fr", "SimpleSpeak", expr, "la matrice 3 par 1 colonne; 1; 2; 3")?;
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
    test("fr", "ClearSpeak",  expr, "la matrice 4 par 1 colonne; rangée 1; 3; rangée 2; 6; rangée 3; 1; rangée 4; 2")?;
    test("fr", "SimpleSpeak", expr, "la matrice 4 par 1 colonne; rangée 1; 3; rangée 2; 6; rangée 3; 1; rangée 4; 2")?;
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
    test("fr", "ClearSpeak",  expr,
        "la matrice 4 par 1 colonne; rangée 1; 3; rangée 2; 6; rangée 3; 1; rangée 4 avec l'étiquette (3.1); 2")?;
    test("fr", "SimpleSpeak", expr,
        "la matrice 4 par 1 colonne; rangée 1; 3; rangée 2; 6; rangée 3; 1; rangée 4 avec nom (3.1); 2")?;
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
    test("fr", "ClearSpeak",  expr, "la matrice 1 par 4 rangée; colonne 1; 3, colonne 2; 6, colonne 3; 1, colonne 4; 2")?;
    test("fr", "SimpleSpeak", expr, "la matrice 1 par 4 rangée; colonne 1; 3, colonne 2; 6, colonne 3; 1, colonne 4; 2")?;
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
    test("fr", "ClearSpeak",  expr, "la matrice 4 par 4; \
          rangée 1; colonne 1; 0, colonne 2; 3, colonne 3; 4, colonne 4; 3; \
          rangée 2; colonne 1; 2, colonne 2; 1, colonne 3; 0, colonne 4; 9; \
          rangée 3; colonne 1; 3, colonne 2; 0, colonne 3; 2, colonne 4; 1; \
          rangée 4; colonne 1; 6, colonne 2; 2, colonne 3; 9, colonne 4; 0")?;
    test("fr", "SimpleSpeak", expr, "la matrice 4 par 4; \
          rangée 1; colonne 1; 0, colonne 2; 3, colonne 3; 4, colonne 4; 3; \
          rangée 2; colonne 1; 2, colonne 2; 1, colonne 3; 0, colonne 4; 9; \
          rangée 3; colonne 1; 3, colonne 2; 0, colonne 3; 2, colonne 4; 1; \
          rangée 4; colonne 1; 6, colonne 2; 2, colonne 3; 9, colonne 4; 0")?;
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
    test("fr", "ClearSpeak",  expr, "la matrice 4 par 2; \
              rangée 1; colonne 1; 1, colonne 2; 3; \
              rangée 2; colonne 1; 4, colonne 2; 2; \
              rangée 3; colonne 1; 2, colonne 2; 1; \
              rangée 4; colonne 1; 0, colonne 2; 5\
    ")?;
    test("fr", "SimpleSpeak", expr, "la matrice 4 par 2; \
              rangée 1; colonne 1; 1, colonne 2; 3; \
              rangée 2; colonne 1; 4, colonne 2; 2; \
              rangée 3; colonne 1; 2, colonne 2; 1; \
              rangée 4; colonne 1; 0, colonne 2; 5\
    ")?;
    return Ok(());
}

// put absolue value test here since it is related to determinate and is small for its own file
#[test]
fn simple_absolute_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>x</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test("fr", "SimpleSpeak", expr, "la valeur absolue de x")?;
  test("fr", "ClearSpeak",  expr, "la valeur absolue de x")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "Auto")], expr, "valeur absolue de x")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "la valeur absolue de x, fin valeur absolue")?;
  return Ok(());
}
  
#[test]
fn absolute_value_plus_1() -> Result<()> {
let expr = "<math>
    <mrow><mrow><mo>|</mo>
      <mrow><mi>x</mi><mo>+</mo><mn>1</mn> </mrow>
    <mo>|</mo></mrow></mrow>
  </math>";
  test("fr", "ClearSpeak", expr, "la valeur absolue de x plus 1")?;
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "valeur absolue de x plus 1, fin valeur absolue")?;
  return Ok(());
}

#[test]
fn simple_cardinality_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>S</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_AbsoluteValue", "Cardinality")], expr,
             "la cardinalité de s majuscule")?;
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
  test_ClearSpeak("fr", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "la matrice 2 par 2; rangée 1; colonne 1; 2, colonne 2; 1; rangée 2; colonne 1; 7, colonne 2; 5")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "la matrice 3 par 1 colonne; rangée 1; 1; rangée 2; 2; rangée 3; 3")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "la matrice 1 par 2 rangée; colonne 1; 1, colonne 2; 2")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "la matrice 2 par 2; rangée 1; colonne 1; b indice 1 1; colonne 2; b indice 1 2; \
                                                rangée 2; colonne 1; b indice 2 1; colonne 2; b indice 2 2")?;
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
  test_ClearSpeak("fr", "ClearSpeak_Matrix", "SilentColNum",
        expr, "la matrice 2 par 2; rangée 1; 2, 1; rangée 2; 7, 5")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "SilentColNum",
        expr, "la matrice 3 par 1 colonne; 1; 2; 3")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "SilentColNum",
        expr, "la matrice 1 par 2 rangée; 1, 2")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "SilentColNum",
        expr, "la matrice 2 par 2; rangée 1; b indice 1 1; b indice 1 2; \
                                                rangée 2; b indice 2 1; b indice 2 2")?;
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
  test_ClearSpeak("fr", "ClearSpeak_Matrix", "EndMatrix",
        expr, "la matrice 2 par 2; rangée 1; 2, 1; rangée 2; 7, 5; fin matrice")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "EndMatrix",
        expr, "la matrice 3 par 1 colonne; 1; 2; 3; fin matrice")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "EndMatrix",
        expr, "la matrice 1 par 2 rangée; 1, 2; fin matrice")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "EndMatrix",
        expr, "la matrice 2 par 2; rangée 1; colonne 1; b indice 1 1; colonne 2; b indice 1 2; \
                                                rangée 2; colonne 1; b indice 2 1; colonne 2; b indice 2 2; fin matrice")?;
    return Ok(());
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
  test_ClearSpeak("fr", "ClearSpeak_Matrix", "Vector",
        expr, "la matrice 2 par 2; rangée 1; 2, 1; rangée 2; 7, 5")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "Vector",
        expr, "le vecteur 3 par 1 colonne; 1; 2; 3")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "Vector",
        expr, "le vecteur 1 par 2 rangée; 1, 2")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "Vector",
        expr, "la matrice 2 par 2; rangée 1; colonne 1; b indice 1 1; colonne 2; b indice 1 2; \
                                                rangée 2; colonne 1; b indice 2 1; colonne 2; b indice 2 2")?;
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
  test_ClearSpeak("fr", "ClearSpeak_Matrix", "EndVector",
        expr, "la matrice 2 par 2; rangée 1; 2, 1; rangée 2; 7, 5; fin matrice")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "EndVector",
        expr, "le vecteur 3 par 1 colonne; 1; 2; 3; fin vecteur")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "EndVector",
        expr, "le vecteur 1 par 2 rangée; 1, 2; fin vecteur")?;
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
test_ClearSpeak("fr", "ClearSpeak_Matrix", "EndVector",
        expr, "la matrice 2 par 2; rangée 1; colonne 1; b indice 1 1; colonne 2; b indice 1 2; \
                                                 rangée 2; colonne 1; b indice 2 1; colonne 2; b indice 2 2; fin matrice")?;
    return Ok(());
  }



#[test]
fn matrix_binomial() -> Result<()> {
  let expr = "<math>
      <mo>(</mo><mrow>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
      </mrow><mo>)</mo>
    </math>";
  test_ClearSpeak("fr", "ClearSpeak_Matrix", "Combinatorics", expr, "3 parmi 2")?;
    return Ok(());
  }

#[test]
fn matrix_times() -> Result<()> {
  let expr = "<math>
    <mfenced><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable></mfenced>
    <mfenced><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable></mfenced>
  </math>";
  test("fr", "SimpleSpeak", expr,
    "la matrice 2 par 2; rangée 1; 1, 2; rangée 2; 3, 4; fois; la matrice 2 par 2; rangée 1; a, b; rangée 2; c, d")?;
    return Ok(());
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
    test("fr", "ClearSpeak",  expr,
         "2 lignes; ligne 1; a est égal à, b plus c moins d; ligne 2; plus e moins f")?;
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
  test("fr", "SimpleSpeak", expr,
    "la matrice zéro 2 par 2")?;
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
  test("fr", "SimpleSpeak", expr,
    "la matrice identité 3 par 3")?;
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
  test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "la matrice diagonale 2 par 2; colonne 1; 1; colonne 2; moins 1")?;
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
  test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "la matrice diagonale 2 par 2; colonne 1; 1")?;
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
  test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "la matrice diagonale 3 par 3; colonne 1; 2; colonne 2; 1; colonne 3; x au carré")?;
  // test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Verbose")],
  //     expr, "the 3 par 3 diagonal matrice; rangée 1, colonne 1, 2; rangée 2, colonne 2, 1; rangée 3, colonne 3, x squared");
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
  test_prefs("fr", "ClearSpeak", vec![("Verbosity", "Terse")],
      expr, "1 ligne, avec étiquette 2; b égale 2")?;
  test_prefs("fr", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "1 équation, avec étiquette 2; b égale 2")?;
    return Ok(());
  }
