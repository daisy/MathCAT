use crate::common::*;

#[test]
fn matrix_1x1() {
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
    test("ru", "ClearSpeak",  expr, "матрица 1 на 1 с элементом 3");
    test("ru", "SimpleSpeak", expr, "матрица 1 на 1 с элементом 3");
}

#[test]
fn determinant_1x1() {
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
    test("ru", "ClearSpeak",  expr, "определитель 1 на 1 с элементом 3");
    test("ru", "SimpleSpeak", expr, "определитель 1 на 1 с элементом 3");
}


#[test]
fn matrix_1x2() {
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
    test("ru", "ClearSpeak",  expr, "матрица-строка 1 на 2; 3, 5");
    test("ru", "SimpleSpeak", expr, "матрица-строка 1 на 2; 3, 5");
}


#[test]
fn matrix_1x3() {
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
    test("ru", "ClearSpeak", expr, "матрица-строка 1 на 3; минус икс, 5, 12");
    test("ru", "SimpleSpeak", expr, "матрица-строка 1 на 3; минус икс, 5, 12");
}

#[test]
fn matrix_2x1_not_simple() {
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
    test("ru", "ClearSpeak", expr, "матрица-столбец 2 на 1; строка 1; икс плюс 1; строка 2; икс минус 1");
    test("ru", "SimpleSpeak", expr, "матрица-столбец 2 на 1; строка 1; икс плюс 1; строка 2; икс минус 1");
}
#[test]
fn matrix_3x1_not_simple() {
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
    test("ru", "SimpleSpeak", expr, "матрица-столбец 3 на 1; \
            строка 1; икс; \
            строка 2; а; \
            строка 3; дробь, икс делить на, икс плюс 1, конец дроби");
    test("ru", "ClearSpeak",  expr, "матрица-столбец 3 на 1; \
            строка 1; икс; \
            строка 2; а; \
            строка 3; дробь, числитель икс; знаменатель икс плюс 1");
}

#[test]
fn determinant_2x2() {
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
    test("ru", "ClearSpeak",  expr, "определитель 2 на 2; строка 1; 2, 1; строка 2; 7, 5");
    test("ru", "SimpleSpeak", expr, "определитель 2 на 2; строка 1; 2, 1; строка 2; 7, 5");
}

#[test]
fn matrix_2x3() {
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
    test("ru", "ClearSpeak",  expr, "матрица 2 на 3; строка 1; 3, 1, 4; строка 2; 0, 2, 6");
    test("ru", "SimpleSpeak", expr, "матрица 2 на 3; строка 1; 3, 1, 4; строка 2; 0, 2, 6");
}

#[test]
fn matrix_2x3_labeled() {
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
    test("ru", "ClearSpeak",  expr,
        "матрица 2 на 3; строка 1 с меткой (3.1); столбец 2; 3, столбец 3; 1, столбец 4; 4; \
                                   строка 2; столбец 1; 0, столбец 2; 2, столбец 3; 6");
    test("ru", "SimpleSpeak", expr,
        "матрица 2 на 3; строка 1 с меткой (3.1); столбец 2; 3, столбец 3; 1, столбец 4; 4; \
                                   строка 2; столбец 1; 0, столбец 2; 2, столбец 3; 6");
}

#[test]
fn matrix_3x1() {
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
    test("ru", "ClearSpeak",  expr, "матрица-столбец 3 на 1; 1; 2; 3");
    test("ru", "SimpleSpeak", expr, "матрица-столбец 3 на 1; 1; 2; 3");
}

#[test]
fn matrix_4x1() {
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
    test("ru", "ClearSpeak",  expr, "матрица-столбец 4 на 1; строка 1; 3; строка 2; 6; строка 3; 1; строка 4; 2");
    test("ru", "SimpleSpeak", expr, "матрица-столбец 4 на 1; строка 1; 3; строка 2; 6; строка 3; 1; строка 4; 2");
}

#[test]
fn matrix_4x1_labeled() {
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
    test("ru", "ClearSpeak",  expr,
        "матрица-столбец 4 на 1; строка 1; 3; строка 2; 6; строка 3; 1; строка 4 с меткой (3.1); 2");
    test("ru", "SimpleSpeak", expr,
        "матрица-столбец 4 на 1; строка 1; 3; строка 2; 6; строка 3; 1; строка 4 с меткой (3.1); 2");
}

#[test]
fn matrix_1x4() {
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
    test("ru", "ClearSpeak",  expr, "матрица-строка 1 на 4; столбец 1; 3, столбец 2; 6, столбец 3; 1, столбец 4; 2");
    test("ru", "SimpleSpeak", expr, "матрица-строка 1 на 4; столбец 1; 3, столбец 2; 6, столбец 3; 1, столбец 4; 2");
}

#[test]
fn matrix_4x4() {
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
    test("ru", "ClearSpeak",  expr, "матрица 4 на 4; \
          строка 1; столбец 1; 0, столбец 2; 3, столбец 3; 4, столбец 4; 3; \
          строка 2; столбец 1; 2, столбец 2; 1, столбец 3; 0, столбец 4; 9; \
          строка 3; столбец 1; 3, столбец 2; 0, столбец 3; 2, столбец 4; 1; \
          строка 4; столбец 1; 6, столбец 2; 2, столбец 3; 9, столбец 4; 0");
    test("ru", "SimpleSpeak", expr, "матрица 4 на 4; \
          строка 1; столбец 1; 0, столбец 2; 3, столбец 3; 4, столбец 4; 3; \
          строка 2; столбец 1; 2, столбец 2; 1, столбец 3; 0, столбец 4; 9; \
          строка 3; столбец 1; 3, столбец 2; 0, столбец 3; 2, столбец 4; 1; \
          строка 4; столбец 1; 6, столбец 2; 2, столбец 3; 9, столбец 4; 0");}

#[test]
fn matrix_4x2() {
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
    test("ru", "ClearSpeak",  expr, "матрица 4 на 2; \
              строка 1; столбец 1; 1, столбец 2; 3; \
              строка 2; столбец 1; 4, столбец 2; 2; \
              строка 3; столбец 1; 2, столбец 2; 1; \
              строка 4; столбец 1; 0, столбец 2; 5\
    ");
    test("ru", "SimpleSpeak", expr, "матрица 4 на 2; \
              строка 1; столбец 1; 1, столбец 2; 3; \
              строка 2; столбец 1; 4, столбец 2; 2; \
              строка 3; столбец 1; 2, столбец 2; 1; \
              строка 4; столбец 1; 0, столбец 2; 5\
    ");}

// поместим тест для абсолютной величины сюда, так как он связан с определителем и слишком мал для отдельного файла
#[test]
fn simple_absolute_value() {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>x</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test("ru", "SimpleSpeak", expr, "абсолютная величина икс");
  test("ru", "ClearSpeak",  expr, "абсолютная величина икс");
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "Auto")], expr, "абсолютная величина икс");
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "абсолютная величина икс, конец абсолютной величины");
}
  
#[test]
fn absolute_value_plus_1() {
let expr = "<math>
    <mrow><mrow><mo>|</mo>
      <mrow><mi>x</mi><mo>+</mo><mn>1</mn> </mrow>
    <mo>|</mo></mrow></mrow>
  </math>";
  test("ru", "ClearSpeak", expr, "абсолютная величина икс плюс 1");
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "абсолютная величина икс плюс 1, конец абсолютной величины");
}

#[test]
fn simple_cardinality_value() {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>S</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_AbsoluteValue", "Cardinality")], expr,
             "мощность заглавная эс");
}
  
// Тестирование предпочтений
#[test]
fn simple_matrix_speak_col_num() {
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
  test_ClearSpeak("ru", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "матрица 2 на 2; строка 1; столбец 1; 2, столбец 2; 1; строка 2; столбец 1; 7, столбец 2; 5");
}

#[test]
fn col_matrix_3x1_speak_col_num() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "матрица-столбец 3 на 1; строка 1; 1; строка 2; 2; строка 3; 3");
}

#[test]
fn row_matrix_1x2_speak_col_num() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "матрица-строка 1 на 2; столбец 1; 1, столбец 2; 2");
}

#[test]
fn matrix_2x2_speak_col_num() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "матрица 2 на 2; строка 1; столбец 1; бэ нижний индекс 1 1; столбец 2; бэ нижний индекс 1 2; \
                                                строка 2; столбец 1; бэ нижний индекс 2 1; столбец 2; бэ нижний индекс 2 2");
}


#[test]
fn simple_matrix_silent_col_num() {
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
  test_ClearSpeak("ru", "ClearSpeak_Matrix", "SilentColNum",
        expr, "матрица 2 на 2; строка 1; 2, 1; строка 2; 7, 5");
}

#[test]
fn col_matrix_3x1_silent_col_num() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "SilentColNum",
        expr, "матрица-столбец 3 на 1; 1; 2; 3");
}

#[test]
fn row_matrix_1x2_silent_col_num() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "SilentColNum",
        expr, "матрица-строка 1 на 2; 1, 2");
}

#[test]
fn matrix_2x2_silent_col_num() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "SilentColNum",
        expr, "матрица 2 на 2; строка 1; бэ нижний индекс 1 1; бэ нижний индекс 1 2; \
                                                строка 2; бэ нижний индекс 2 1; бэ нижний индекс 2 2");
}


#[test]
fn simple_matrix_end_matrix() {
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
  test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndMatrix",
        expr, "матрица 2 на 2; строка 1; 2, 1; строка 2; 7, 5; конец матрицы");
}

#[test]
fn col_matrix_3x1_end_matrix() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndMatrix",
        expr, "матрица-столбец 3 на 1; 1; 2; 3; конец матрицы");
}

#[test]
fn row_matrix_1x2_end_matrix() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndMatrix",
        expr, "матрица-строка 1 на 2; 1, 2; конец матрицы");
}

#[test]
fn matrix_2x2_end_matrix() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndMatrix",
        expr, "матрица 2 на 2; строка 1; столбец 1; бэ нижний индекс 1 1; столбец 2; бэ нижний индекс 1 2; \
                                                строка 2; столбец 1; бэ нижний индекс 2 1; столбец 2; бэ нижний индекс 2 2; конец матрицы");
}


#[test]
fn simple_matrix_vector() {
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
  test_ClearSpeak("ru", "ClearSpeak_Matrix", "Vector",
        expr, "матрица 2 на 2; строка 1; 2, 1; строка 2; 7, 5");
}

#[test]
fn col_matrix_3x1_vector() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "Vector",
        expr, "вектор-столбец 3 на 1; 1; 2; 3");
}

#[test]
fn row_matrix_1x2_vector() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "Vector",
        expr, "вектор-строка 1 на 2; 1, 2");
}

#[test]
fn matrix_2x2_vector() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "Vector",
        expr, "матрица 2 на 2; строка 1; столбец 1; бэ нижний индекс 1 1; столбец 2; бэ нижний индекс 1 2; \
                                                строка 2; столбец 1; бэ нижний индекс 2 1; столбец 2; бэ нижний индекс 2 2");
}


#[test]
fn simple_matrix_end_vector() {
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
  test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndVector",
        expr, "матрица 2 на 2; строка 1; 2, 1; строка 2; 7, 5; конец матрицы");
}

#[test]
fn col_matrix_3x1_end_vector() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndVector",
        expr, "вектор-столбец 3 на 1; 1; 2; 3; конец вектора");
}

#[test]
fn row_matrix_1x2_end_vector() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndVector",
        expr, "вектор-строка 1 на 2; 1, 2; конец вектора");
}

#[test]
fn matrix_2x2_end_vector() {
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
test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndVector",
        expr, "матрица 2 на 2; строка 1; столбец 1; бэ нижний индекс 1 1; столбец 2; бэ нижний индекс 1 2; \
                                                 строка 2; столбец 1; бэ нижний индекс 2 1; столбец 2; бэ нижний индекс 2 2; конец матрицы");
}



#[test]
fn matrix_binomial() {
  let expr = "<math>
      <mo>(</mo><mrow>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
      </mrow><mo>)</mo>
    </math>";
  test_ClearSpeak("ru", "ClearSpeak_Matrix", "Combinatorics", expr, "число сочетаний из 3 по 2");
}

#[test]
fn matrix_times() {
  let expr = "<math>
    <mfenced><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable></mfenced>
    <mfenced><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable></mfenced>
  </math>";
  test("ru", "SimpleSpeak", expr,
    "матрица 2 на 2; строка 1; 1, 2; строка 2; 3, 4; умножить на, матрица 2 на 2; строка 1; а, б; строка 2; цэ, дэ");
}

#[test]
fn unknown_mtable_property() {
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
    test("ru", "ClearSpeak",  expr,
         "2 строки; строка 1; а равно b плюс цэ минус дэ; строка 2; плюс e минус f");
}


#[test]
fn zero_matrix() {
  let expr = "<math>
      <mo>[</mo>
      <mtable>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
      </mtable>
      <mo>]</mo>
  </math>";
  test("ru", "SimpleSpeak", expr,
    "нулевая матрица 2 на 2");
}

#[test]
fn identity_matrix() {
  let expr = "<math>
      <mo>(</mo>
      <mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr>
      </mtable>
      <mo>)</mo>
  </math>";
  test("ru", "SimpleSpeak", expr,
    "единичная матрица 3 на 3");
}

#[test]
fn diagonal_matrix() {
  let expr = "<math>
      <mo>(</mo>
      <mtable>
        <mtr><mtd><mn>2</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd><mtd><msup><mi>x</mi><mn>2</mn></msup></mtd></mtr>
      </mtable>
      <mo>)</mo>
  </math>";
  test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "диагональная матрица 3 на 3; столбец 1; 2; столбец 2; 1; столбец 3; икс в квадрате");
  // test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")],
  //     expr, "диагональная матрица 3 на 3; строка 1, столбец 1, 2; строка 2, столбец 2, 1; строка 3, столбец 3, икс в квадрате");
}
