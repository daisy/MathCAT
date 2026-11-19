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
    test("el", "ClearSpeak",  expr, "ένα επί ένα πίνακας με καταχώρηση 3")?;
    test("el", "SimpleSpeak", expr, "ένα επί ένα πίνακας με καταχώρηση 3")?;
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
    test("el", "ClearSpeak",  expr, "ένα επί ένα ορίζουσα με καταχώρηση 3")?;
    test("el", "SimpleSpeak", expr, "ένα επί ένα ορίζουσα με καταχώρηση 3")?;
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
    test("el", "ClearSpeak",  expr, "ένα επί 2 πίνακας-γραμμή; 3, 5")?;
    test("el", "SimpleSpeak", expr, "ένα επί 2 πίνακας-γραμμή; 3, 5")?;
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
    test("el", "ClearSpeak", expr, "ένα επί 3 πίνακας-γραμμή; μείον x, 5, 12")?;
    test("el", "SimpleSpeak", expr, "ένα επί 3 πίνακας-γραμμή; μείον x, 5, 12")?;
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
    test("el", "ClearSpeak", expr, "2 επί ένα πίνακας-στήλη; γραμμή 1; x συν 1; γραμμή 2; x μείον 1")?;
    test("el", "SimpleSpeak", expr, "2 επί ένα πίνακας-στήλη; γραμμή 1; x συν 1; γραμμή 2; x μείον 1")?;
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
    test("el", "SimpleSpeak", expr, "3 επί ένα πίνακας-στήλη; \
            γραμμή 1; x; \
            γραμμή 2; a; \
            γραμμή 3; κλάσμα, x προς, x συν 1, τέλος κλάσματος")?;
    test("el", "ClearSpeak",  expr, "3 επί ένα πίνακας-στήλη; \
            γραμμή 1; x; \
            γραμμή 2; a; \
            γραμμή 3; το κλάσμα με αριθμητή x; και παρονομαστή x συν 1")?;
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
    test("el", "ClearSpeak",  expr, "2 επί 2 ορίζουσα; γραμμή 1; 2, 1; γραμμή 2; 7, 5")?;
    test("el", "SimpleSpeak", expr, "2 επί 2 ορίζουσα; γραμμή 1; 2, 1; γραμμή 2; 7, 5")?;
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
    test("el", "ClearSpeak",  expr, "2 επί 3 πίνακας; γραμμή 1; 3, 1, 4; γραμμή 2; 0, 2, 6")?;
    test("el", "SimpleSpeak", expr, "2 επί 3 πίνακας; γραμμή 1; 3, 1, 4; γραμμή 2; 0, 2, 6")?;
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
    test("el", "ClearSpeak",  expr,
        "2 επί 3 πίνακας; γραμμή 1 με ένδειξη (3.1); στήλη 1; 3, στήλη 2; 1, στήλη 3; 4; \
                                   γραμμή 2; στήλη 1; 0, στήλη 2; 2, στήλη 3; 6")?;
    test("el", "SimpleSpeak", expr,
        "2 επί 3 πίνακας; γραμμή 1 με ένδειξη (3.1); στήλη 1; 3, στήλη 2; 1, στήλη 3; 4; \
                                   γραμμή 2; στήλη 1; 0, στήλη 2; 2, στήλη 3; 6")?;
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
    test("el", "ClearSpeak",  expr, "3 επί ένα πίνακας-στήλη; 1; 2; 3")?;
    test("el", "SimpleSpeak", expr, "3 επί ένα πίνακας-στήλη; 1; 2; 3")?;
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
    test("el", "ClearSpeak",  expr, "4 επί ένα πίνακας-στήλη; γραμμή 1; 3; γραμμή 2; 6; γραμμή 3; 1; γραμμή 4; 2")?;
    test("el", "SimpleSpeak", expr, "4 επί ένα πίνακας-στήλη; γραμμή 1; 3; γραμμή 2; 6; γραμμή 3; 1; γραμμή 4; 2")?;
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
    test("el", "ClearSpeak",  expr,
        "4 επί ένα πίνακας-στήλη; γραμμή 1; 3; γραμμή 2; 6; γραμμή 3; 1; γραμμή 4 με ένδειξη (3.1); 2")?;
    test("el", "SimpleSpeak", expr,
        "4 επί ένα πίνακας-στήλη; γραμμή 1; 3; γραμμή 2; 6; γραμμή 3; 1; γραμμή 4 με ένδειξη (3.1); 2")?;
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
    test("el", "ClearSpeak",  expr, "ένα επί 4 πίνακας-γραμμή; στήλη 1; 3, στήλη 2; 6, στήλη 3; 1, στήλη 4; 2")?;
    test("el", "SimpleSpeak", expr, "ένα επί 4 πίνακας-γραμμή; στήλη 1; 3, στήλη 2; 6, στήλη 3; 1, στήλη 4; 2")?;
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
    test("el", "ClearSpeak",  expr, "4 επί 4 πίνακας; \
          γραμμή 1; στήλη 1; 0, στήλη 2; 3, στήλη 3; 4, στήλη 4; 3; \
          γραμμή 2; στήλη 1; 2, στήλη 2; 1, στήλη 3; 0, στήλη 4; 9; \
          γραμμή 3; στήλη 1; 3, στήλη 2; 0, στήλη 3; 2, στήλη 4; 1; \
          γραμμή 4; στήλη 1; 6, στήλη 2; 2, στήλη 3; 9, στήλη 4; 0")?;
    test("el", "SimpleSpeak", expr, "4 επί 4 πίνακας; \
          γραμμή 1; στήλη 1; 0, στήλη 2; 3, στήλη 3; 4, στήλη 4; 3; \
          γραμμή 2; στήλη 1; 2, στήλη 2; 1, στήλη 3; 0, στήλη 4; 9; \
          γραμμή 3; στήλη 1; 3, στήλη 2; 0, στήλη 3; 2, στήλη 4; 1; \
          γραμμή 4; στήλη 1; 6, στήλη 2; 2, στήλη 3; 9, στήλη 4; 0")?;
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
    test("el", "ClearSpeak",  expr, "4 επί 2 πίνακας; \
              γραμμή 1; στήλη 1; 1, στήλη 2; 3; \
              γραμμή 2; στήλη 1; 4, στήλη 2; 2; \
              γραμμή 3; στήλη 1; 2, στήλη 2; 1; \
              γραμμή 4; στήλη 1; 0, στήλη 2; 5\
    ")?;
    test("el", "SimpleSpeak", expr, "4 επί 2 πίνακας; \
              γραμμή 1; στήλη 1; 1, στήλη 2; 3; \
              γραμμή 2; στήλη 1; 4, στήλη 2; 2; \
              γραμμή 3; στήλη 1; 2, στήλη 2; 1; \
              γραμμή 4; στήλη 1; 0, στήλη 2; 5\
    ")?;
    return Ok(());
}

// put absolute value test here since it is related to determinate and is small for its own file
#[test]
fn simple_absolute_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>x</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test("el", "SimpleSpeak", expr, "η απόλυτη τιμή του x")?;
  test("el", "ClearSpeak",  expr, "απόλυτη τιμή του x")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "Auto")], expr, "απόλυτη τιμή του x")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "απόλυτη τιμή του x, τέλος απόλυτης τιμής")?;
  return Ok(());
}
  
#[test]
fn absolute_value_plus_1() -> Result<()> {
let expr = "<math>
    <mrow><mrow><mo>|</mo>
      <mrow><mi>x</mi><mo>+</mo><mn>1</mn> </mrow>
    <mo>|</mo></mrow></mrow>
  </math>";
  test("el", "ClearSpeak", expr, "απόλυτη τιμή του x συν 1")?;
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "απόλυτη τιμή του x συν 1, τέλος απόλυτης τιμής")?;
  return Ok(());
}

#[test]
fn simple_cardinality_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>S</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_AbsoluteValue", "Cardinality")], expr,
             "πληθικός αριθμός του κεφαλαίο s")?;
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
  test_ClearSpeak("el", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "2 επί 2 πίνακας; γραμμή 1; στήλη 1; 2, στήλη 2; 1; γραμμή 2; στήλη 1; 7, στήλη 2; 5")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "3 επί ένα πίνακας-στήλη; γραμμή 1; 1; γραμμή 2; 2; γραμμή 3; 3")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "ένα επί 2 πίνακας-γραμμή; στήλη 1; 1, στήλη 2; 2")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "2 επί 2 πίνακας; γραμμή 1; στήλη 1; b δείκτης 1 1; στήλη 2; b δείκτης 1 2; \
                                                γραμμή 2; στήλη 1; b δείκτης 2 1; στήλη 2; b δείκτης 2 2")?;
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
  test_ClearSpeak("el", "ClearSpeak_Matrix", "SilentColNum",
        expr, "2 επί 2 πίνακας; γραμμή 1; 2, 1; γραμμή 2; 7, 5")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "SilentColNum",
        expr, "3 επί ένα πίνακας-στήλη; 1; 2; 3")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "SilentColNum",
        expr, "ένα επί 2 πίνακας-γραμμή; 1, 2")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "SilentColNum",
        expr, "2 επί 2 πίνακας; γραμμή 1; b δείκτης 1 1; b δείκτης 1 2; \
                                                γραμμή 2; b δείκτης 2 1; b δείκτης 2 2")?;
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
  test_ClearSpeak("el", "ClearSpeak_Matrix", "EndMatrix",
        expr, "2 επί 2 πίνακας; γραμμή 1; 2, 1; γραμμή 2; 7, 5; τέλος πίνακα")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "EndMatrix",
        expr, "3 επί ένα πίνακας-στήλη; 1; 2; 3; τέλος πίνακα")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "EndMatrix",
        expr, "ένα επί 2 πίνακας-γραμμή; 1, 2; τέλος πίνακα")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "EndMatrix",
        expr, "2 επί 2 πίνακας; γραμμή 1; στήλη 1; b δείκτης 1 1; στήλη 2; b δείκτης 1 2; \
                                                γραμμή 2; στήλη 1; b δείκτης 2 1; στήλη 2; b δείκτης 2 2; τέλος πίνακα")?;
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
  test_ClearSpeak("el", "ClearSpeak_Matrix", "Vector",
        expr, "2 επί 2 πίνακας; γραμμή 1; 2, 1; γραμμή 2; 7, 5")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "Vector",
        expr, "3 επί ένα διάνυσμα-στήλη; 1; 2; 3")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "Vector",
        expr, "ένα επί 2 διάνυσμα-γραμμή; 1, 2")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "Vector",
        expr, "2 επί 2 πίνακας; γραμμή 1; στήλη 1; b δείκτης 1 1; στήλη 2; b δείκτης 1 2; \
                                                γραμμή 2; στήλη 1; b δείκτης 2 1; στήλη 2; b δείκτης 2 2")?;
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
  test_ClearSpeak("el", "ClearSpeak_Matrix", "EndVector",
        expr, "2 επί 2 πίνακας; γραμμή 1; 2, 1; γραμμή 2; 7, 5; τέλος πίνακα")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "EndVector",
        expr, "3 επί ένα διάνυσμα-στήλη; 1; 2; 3; τέλος διανύσματος")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "EndVector",
        expr, "ένα επί 2 διάνυσμα-γραμμή; 1, 2; τέλος διανύσματος")?;
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
test_ClearSpeak("el", "ClearSpeak_Matrix", "EndVector",
        expr, "2 επί 2 πίνακας; γραμμή 1; στήλη 1; b δείκτης 1 1; στήλη 2; b δείκτης 1 2; \
                                                  γραμμή 2; στήλη 1; b δείκτης 2 1; στήλη 2; b δείκτης 2 2; τέλος πίνακα")?;
    return Ok(());
  }



#[test]
fn matrix_binomial() -> Result<()> {
  let expr = "<math>
      <mo>(</mo><mrow>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
      </mrow><mo>)</mo>
    </math>";
  test_ClearSpeak("el", "ClearSpeak_Matrix", "Combinatorics", expr, "3 ανά 2")?;
    return Ok(());
  }

#[test]
fn matrix_times() -> Result<()> {
  let expr = "<math>
    <mfenced><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable></mfenced>
    <mfenced><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable></mfenced>
  </math>";
  test("el", "SimpleSpeak", expr,
    "2 επί 2 πίνακας; γραμμή 1; 1, 2; γραμμή 2; 3, 4; 2 επί 2 πίνακας; γραμμή 1; a, b; γραμμή 2; c, d")?;
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
    test("el", "ClearSpeak",  expr,
         "2 σειρές; σειρά 1; a ισούται με, b συν c μείον d; σειρά 2; συν e μείον f")?;
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
  test("el", "SimpleSpeak", expr,
    "ο 2 επί 2 μηδενικός πίνακας")?;
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
  test("el", "SimpleSpeak", expr,
    "ο 3 επί 3 μοναδιαίος πίνακας")?;
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
  test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; 1; στήλη 2; μείον 1")?;
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
  test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; 1")?;
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
  test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "ο 3 επί 3 διαγώνιος πίνακας; στήλη 1; 2; στήλη 2; 1; στήλη 3; x στο τετράγωνο")?;
//  test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Verbose")],
//       expr, "the 3 by 3 diagonal matrix; row 1, column 1, 2; row 2, column 2, 1; row 3, column 3, x squared")?;
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
  test_prefs("el", "ClearSpeak", vec![("Verbosity", "Terse")],
      expr, "1 σειρά, με ένδειξη 2; b ισούται με 2")?;
  test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "1 εξίσωση, με ένδειξη 2; b ισούται με 2")?;
    return Ok(());
  }
