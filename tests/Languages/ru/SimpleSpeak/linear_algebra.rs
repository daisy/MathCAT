use crate::common::*;

#[test]
fn transpose() {
  let expr = "<math> <msup><mi>M</mi><mi>T</mi></msup> </math>";
  test("ru", "SimpleSpeak", expr, "эм большое транспонированная");
}

#[test]
fn trace() {
  let expr = "<math> <mi>Tr</mi><mi>M</mi> </math>";
  test("ru", "SimpleSpeak", expr, "след от эм большое");
}

#[test]
fn dimension() {
  let expr = "<math> <mi>Dim</mi><mi>M</mi> </math>";
  test("ru", "SimpleSpeak", expr, "размерность эм большое");
}

#[test]
fn homomorphism() {
  let expr = "<math> <mi>Hom</mi><mo>(</mo><mi>M</mi><mo>)</mo> </math>";
  test("ru", "SimpleSpeak", expr, "гомоморфизм от эм большое");
}

#[test]
fn kernel() {
  let expr = "<math> <mi>ker</mi><mrow><mo>(</mo><mi>L</mi><mo>)</mo></mrow> </math>";
  test("ru", "SimpleSpeak", expr, "ядро от эль большое");
}

#[test]
fn norm() {
  let expr = "  <math>
    <mrow>
      <mo>∥</mo>
      <mi>f</mi>
      <mo>∥</mo>
    </mrow>
</math>
";
  test("ru", "SimpleSpeak", expr, "норма от эф");
}

#[test]
fn norm_non_simple() {
  let expr = "  <math>
    <mrow>
      <mo>∥</mo>
      <mi>x</mi>
      <mo>+</mo>
      <mi>y</mi>
      <mo>∥</mo>
    </mrow>
</math>
";
  test("ru", "SimpleSpeak", expr, "норма от икс плюс игрек конец нормы");
}

#[test]
fn norm_subscripted() {
  let expr = "  <math>
    <msub>
      <mrow>
        <mo>∥</mo>
        <mi>f</mi>
        <mo>∥</mo>
      </mrow>
      <mi>p</mi>
    </msub>
</math>
";
  test("ru", "SimpleSpeak", expr, "p-норма от эф");
}

#[test]
fn not_gradient() {
  // the nabla is at the end, so it can't be gradient because it doesn't operate on anything
  let expr = r#"<math>
  <mo>(</mo>
  <mi>b</mi>
  <mo>&#x22C5;</mo>
  <mrow>
    <mo>&#x2207;</mo>
  </mrow>
  <mo>)</mo>
  <mi>a</mi>
</math>
"#;
  test("ru", "SimpleSpeak", expr, "скобка открывается, бэ умножить на набла, скобка закрывается; умножить на а");
}