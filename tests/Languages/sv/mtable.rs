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
    test("sv", "ClearSpeak",  expr, "ett-gånger-ett matris med element 3")?;
    test("sv", "SimpleSpeak", expr, "ett-gånger-ett matris med element 3")?;
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
    test("sv", "ClearSpeak",  expr, "ett-gånger-ett determinant med element 3")?;
    test("sv", "SimpleSpeak", expr, "ett-gånger-ett determinant med element 3")?;
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
    test("sv", "ClearSpeak",  expr, "ett-gånger 2 rad-matris; 3, 5")?;
    test("sv", "SimpleSpeak", expr, "ett-gånger 2 rad-matris; 3, 5")?;
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
    test("sv", "ClearSpeak", expr, "ett-gånger 3 rad-matris; minus x, 5, 12")?;
    test("sv", "SimpleSpeak", expr, "ett-gånger 3 rad-matris; minus x, 5, 12")?;
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
    test("sv", "ClearSpeak", expr, "2 gånger-ett kolumn-matris; rad 1; x plus 1; rad 2; x minus 1")?;
    test("sv", "SimpleSpeak", expr, "2 gånger-ett kolumn-matris; rad 1; x plus 1; rad 2; x minus 1")?;
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
    test("sv", "SimpleSpeak", expr, "3 gånger-ett kolumn-matris; \
            rad 1; x; \
            rad 2; a; \
            rad 3; division, x genom, x plus 1, slut division")?;
    test("sv", "ClearSpeak",  expr, "3 gånger-ett kolumn-matris; \
            rad 1; x; \
            rad 2; a; \
            rad 3; division med täljaren x; och nämnaren x plus 1")?;
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
    test("sv", "ClearSpeak",  expr, "2 gånger 2 determinant; rad 1; 2, 1; rad 2; 7, 5")?;
    test("sv", "SimpleSpeak", expr, "2 gånger 2 determinant; rad 1; 2, 1; rad 2; 7, 5")?;
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
    test("sv", "ClearSpeak",  expr, "2 gånger 3 matris; rad 1; 3, 1, 4; rad 2; 0, 2, 6")?;
    test("sv", "SimpleSpeak", expr, "2 gånger 3 matris; rad 1; 3, 1, 4; rad 2; 0, 2, 6")?;
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
    test("sv", "ClearSpeak",  expr,
        "2 gånger 3 matris; rad 1 med etiketten (3 punkt 1); kolumn 1; 3, kolumn 2; 1, kolumn 3; 4; rad 2; kolumn 1; 0, kolumn 2; 2, kolumn 3; 6")?;
    test("sv", "SimpleSpeak", expr,
        "2 gånger 3 matris; rad 1 med etiketten (3 punkt 1); kolumn 1; 3, kolumn 2; 1, kolumn 3; 4; rad 2; kolumn 1; 0, kolumn 2; 2, kolumn 3; 6")?;
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
    test("sv", "ClearSpeak",  expr, "3 gånger-ett kolumn-matris; 1; 2; 3")?;
    test("sv", "SimpleSpeak", expr, "3 gånger-ett kolumn-matris; 1; 2; 3")?;
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
    test("sv", "ClearSpeak",  expr, "4 gånger-ett kolumn-matris; rad 1; 3; rad 2; 6; rad 3; 1; rad 4; 2")?;
    test("sv", "SimpleSpeak", expr, "4 gånger-ett kolumn-matris; rad 1; 3; rad 2; 6; rad 3; 1; rad 4; 2")?;
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
    test("sv", "ClearSpeak",  expr,
        "4 gånger-ett kolumn-matris; rad 1; 3; rad 2; 6; rad 3; 1; rad 4 med etiketten (3 punkt 1); 2")?;
    test("sv", "SimpleSpeak", expr,
        "4 gånger-ett kolumn-matris; rad 1; 3; rad 2; 6; rad 3; 1; rad 4 med etiketten (3 punkt 1); 2")?;
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
    test("sv", "ClearSpeak",  expr, "ett-gånger 4 rad-matris; kolumn 1; 3, kolumn 2; 6, kolumn 3; 1, kolumn 4; 2")?;
    test("sv", "SimpleSpeak", expr, "ett-gånger 4 rad-matris; kolumn 1; 3, kolumn 2; 6, kolumn 3; 1, kolumn 4; 2")?;
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
    test("sv", "ClearSpeak",  expr, "4 gånger 4 matris; \
            rad 1; kolumn 1; 0, kolumn 2; 3, kolumn 3; 4, kolumn 4; 3; \
            rad 2; kolumn 1; 2, kolumn 2; 1, kolumn 3; 0, kolumn 4; 9; \
            rad 3; kolumn 1; 3, kolumn 2; 0, kolumn 3; 2, kolumn 4; 1; \
            rad 4; kolumn 1; 6, kolumn 2; 2, kolumn 3; 9, kolumn 4; 0")?;
    test("sv", "SimpleSpeak", expr, "4 gånger 4 matris; \
            rad 1; kolumn 1; 0, kolumn 2; 3, kolumn 3; 4, kolumn 4; 3; \
            rad 2; kolumn 1; 2, kolumn 2; 1, kolumn 3; 0, kolumn 4; 9; \
            rad 3; kolumn 1; 3, kolumn 2; 0, kolumn 3; 2, kolumn 4; 1; \
            rad 4; kolumn 1; 6, kolumn 2; 2, kolumn 3; 9, kolumn 4; 0")?;
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
    test("sv", "ClearSpeak",  expr, "4 gånger 2 matris; \
            rad 1; kolumn 1; 1, kolumn 2; 3; \
            rad 2; kolumn 1; 4, kolumn 2; 2; \
            rad 3; kolumn 1; 2, kolumn 2; 1; \
            rad 4; kolumn 1; 0, kolumn 2; 5")?;
    test("sv", "SimpleSpeak", expr, "4 gånger 2 matris; \
            rad 1; kolumn 1; 1, kolumn 2; 3; \
            rad 2; kolumn 1; 4, kolumn 2; 2; \
            rad 3; kolumn 1; 2, kolumn 2; 1; \
            rad 4; kolumn 1; 0, kolumn 2; 5")?;
    return Ok(());
}

// put absolute value test here since it is related to determinate and is small for its own file
#[test]
fn simple_absolute_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>x</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test("sv", "SimpleSpeak", expr, "absolutbeloppet av x")?;
  test("sv", "ClearSpeak",  expr, "absolutbeloppet av x")?;
  test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "Auto")], expr, "absolutbeloppet av x")?;
  test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "absolutbeloppet av x, slut absolutbelopp")?;
  return Ok(());
}
  
#[test]
fn absolute_value_plus_1() -> Result<()> {
let expr = "<math>
    <mrow><mrow><mo>|</mo>
      <mrow><mi>x</mi><mo>+</mo><mn>1</mn> </mrow>
    <mo>|</mo></mrow></mrow>
  </math>";
  test("sv", "ClearSpeak", expr, "absolutbeloppet av x plus 1")?;
  test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "absolutbeloppet av x plus 1, slut absolutbelopp")?;
  return Ok(());
}

#[test]
fn simple_cardinality_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>S</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_AbsoluteValue", "Cardinality")], expr,
             "kardinaliteten av versal s")?;
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
  test_ClearSpeak("sv", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "2 gånger 2 matris; rad 1; kolumn 1; 2, kolumn 2; 1; rad 2; kolumn 1; 7, kolumn 2; 5")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "3 gånger-ett kolumn-matris; rad 1; 1; rad 2; 2; rad 3; 3")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "ett-gånger 2 rad-matris; kolumn 1; 1, kolumn 2; 2")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "2 gånger 2 matris; rad 1; kolumn 1; b nedsänkt 1 1, kolumn 2; b nedsänkt 1 2; \
                    rad 2; kolumn 1; b nedsänkt 2 1, kolumn 2; b nedsänkt 2 2")?;
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
  test_ClearSpeak("sv", "ClearSpeak_Matrix", "SilentColNum",
        expr, "2 gånger 2 matris; rad 1; 2, 1; rad 2; 7, 5")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "SilentColNum",
        expr, "3 gånger-ett kolumn-matris; 1; 2; 3")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "SilentColNum",
        expr, "ett-gånger 2 rad-matris; 1, 2")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "SilentColNum",
        expr, "2 gånger 2 matris; rad 1; b nedsänkt 1 1, b nedsänkt 1 2; \
                            rad 2; b nedsänkt 2 1, b nedsänkt 2 2")?;
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
  test_ClearSpeak("sv", "ClearSpeak_Matrix", "EndMatrix",
        expr, "2 gånger 2 matris; rad 1; 2, 1; rad 2; 7, 5; slut matris")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "EndMatrix",
        expr, "3 gånger-ett kolumn-matris; 1; 2; 3; slut matris")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "EndMatrix",
        expr, "ett-gånger 2 rad-matris; 1, 2; slut matris")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "EndMatrix",
        expr, "2 gånger 2 matris; rad 1; kolumn 1; b nedsänkt 1 1, kolumn 2; b nedsänkt 1 2; \
                            rad 2; kolumn 1; b nedsänkt 2 1, kolumn 2; b nedsänkt 2 2; slut matris")?;
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
  test_ClearSpeak("sv", "ClearSpeak_Matrix", "Vector",
        expr, "2 gånger 2 matris; rad 1; 2, 1; rad 2; 7, 5")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "Vector",
        expr, "3 gånger-ett kolumn-vektor; 1; 2; 3")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "Vector",
        expr, "ett-gånger 2 rad-vektor; 1, 2")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "Vector",
        expr, "2 gånger 2 matris; rad 1; kolumn 1; b nedsänkt 1 1, kolumn 2; b nedsänkt 1 2; \
                                rad 2; kolumn 1; b nedsänkt 2 1, kolumn 2; b nedsänkt 2 2")?;
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
  test_ClearSpeak("sv", "ClearSpeak_Matrix", "EndVector",
        expr, "2 gånger 2 matris; rad 1; 2, 1; rad 2; 7, 5; slut matris")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "EndVector",
        expr, "3 gånger-ett kolumn-vektor; 1; 2; 3; slut vektor")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "EndVector",
        expr, "ett-gånger 2 rad-vektor; 1, 2; slut vektor")?;
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
	test_ClearSpeak("sv", "ClearSpeak_Matrix", "EndVector",
        expr, "2 gånger 2 matris; rad 1; kolumn 1; b nedsänkt 1 1, kolumn 2; b nedsänkt 1 2; \
                                rad 2; kolumn 1; b nedsänkt 2 1, kolumn 2; b nedsänkt 2 2; slut matris")?;
  return Ok(());
}



#[test]
fn matrix_binomial() -> Result<()> {
  let expr = "<math>
      <mo>(</mo><mrow>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
      </mrow><mo>)</mo>
    </math>";
  test_ClearSpeak("sv", "ClearSpeak_Matrix", "Combinatorics", expr, "3 över 2")?;
  return Ok(());
}
