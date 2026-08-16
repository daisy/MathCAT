use crate::common::*;
use anyhow::Result;

fn test_both(expr: &str, speech: &str) -> Result<()> {
    test("ur", "ClearSpeak", expr, speech)?;
    test("ur", "SimpleSpeak", expr, speech)
}

// Verifies that numbers remain usable before Urdu number-to-word rules are added.
#[test]
fn numbers() -> Result<()> {
    test_both("<math><mn>123</mn></math>", "123")?;
    test_both("<math><mn>3.14</mn></math>", "3.14")
}

// Verifies every Latin identifier currently defined in the Urdu Unicode table.
#[test]
fn latin_identifiers() -> Result<()> {
    test_both("<math><mi>a</mi></math>", "اے")?;
    test_both("<math><mi>b</mi></math>", "بی")?;
    test_both("<math><mi>n</mi></math>", "این")?;
    test_both("<math><mi>x</mi></math>", "ایکس")?;
    test_both("<math><mi>y</mi></math>", "وائے")
}

// Verifies every Urdu identifier currently defined in the Unicode table.
#[test]
fn urdu_identifiers() -> Result<()> {
    test_both("<math><mi>س</mi></math>", "س")?;
    test_both("<math><mi>ص</mi></math>", "ص")
}

// Verifies binary addition and equality in both speech styles.
#[test]
fn addition_and_equality() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>+</mo><mn>2</mn><mo>=</mo><mi>y</mi></math>";
    test_both(expr, "ایکس جمع 2, برابر وائے")
}

// Verifies both commonly encountered subtraction characters.
#[test]
fn subtraction_characters() -> Result<()> {
    let hyphen = "<math><mi>x</mi><mo>-</mo><mn>1</mn></math>";
    let minus = "<math><mi>x</mi><mo>−</mo><mn>1</mn></math>";
    test_both(hyphen, "ایکس منفی 1")?;
    test_both(minus, "ایکس منفی 1")
}

// Verifies that unary signs select the positive and negative rules.
#[test]
fn unary_signs() -> Result<()> {
    let positive = "<math><mo>+</mo><mi>x</mi></math>";
    let negative = "<math><mo>−</mo><mi>x</mi></math>";
    test_both(positive, "مثبت ایکس")?;
    test_both(negative, "منفی ایکس")
}

// Verifies explicit multiplication and division signs.
#[test]
fn multiplication_and_division() -> Result<()> {
    let multiplication = "<math><mn>6</mn><mo>×</mo><mi>x</mi></math>";
    let division = "<math><mn>6</mn><mo>÷</mo><mn>2</mn></math>";
    test_both(multiplication, "6 ضرب ایکس")?;
    test_both(division, "6 تقسیم 2")
}

// Verifies both ordering relations defined by the Urdu Unicode table.
#[test]
fn comparisons() -> Result<()> {
    let less_than = "<math><mn>1</mn><mo>&lt;</mo><mi>x</mi></math>";
    let greater_than = "<math><mi>x</mi><mo>&gt;</mo><mn>0</mn></math>";
    test_both(less_than, "1 سے کم ایکس")?;
    test_both(greater_than, "ایکس سے زیادہ 0")
}

// Verifies the Urdu reading of a fraction with leaf children.
#[test]
fn simple_fraction() -> Result<()> {
    let expr = "<math><mfrac><mi>x</mi><mn>5</mn></mfrac></math>";
    test_both(expr, "ایکس بٹا 5")
}

// Verifies that operators inside a fraction remain localized.
#[test]
fn compound_fraction() -> Result<()> {
    let expr = "<math><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>y</mi><mo>−</mo><mn>1</mn></mrow></mfrac></math>";
    test_both(expr, "ایکس جمع 1 بٹا وائے منفی 1")
}

// Verifies square roots in both speech styles.
#[test]
fn square_root() -> Result<()> {
    let expr = "<math><msqrt><mi>x</mi></msqrt></math>";
    test_both(expr, "ایکس کا جذر المربع")
}

// Verifies that a compound radicand retains localized operators.
#[test]
fn compound_square_root() -> Result<()> {
    let expr = "<math><msqrt><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>1</mn></mrow></msqrt></math>";
    test_both(expr, "ایکس کا مربع, جمع 1 کا جذر المربع")
}

// Verifies the dedicated cube-root phrase in both speech styles.
#[test]
fn cube_root() -> Result<()> {
    let expr = "<math><mroot><mn>8</mn><mn>3</mn></mroot></math>";
    test_both(expr, "8 کا جذر المکعب")
}

// Verifies the generic indexed-root rule.
#[test]
fn generic_root() -> Result<()> {
    let expr = "<math><mroot><mi>x</mi><mn>4</mn></mroot></math>";
    test_both(expr, "ایکس کا 4 درجے کا جذر")
}

// Verifies the dedicated square and cube power rules.
#[test]
fn square_and_cube() -> Result<()> {
    let square = "<math><msup><mi>x</mi><mn>2</mn></msup></math>";
    let cube = "<math><msup><mi>y</mi><mn>3</mn></msup></math>";
    test_both(square, "ایکس کا مربع")?;
    test_both(cube, "وائے کا مکعب")
}

// Verifies the generic power rule with an identifier exponent.
#[test]
fn generic_power() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mi>n</mi></msup></math>";
    test_both(expr, "ایکس کی قوت این")
}

// Verifies that a compound exponent retains localized operators.
#[test]
fn compound_power() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msup></math>";
    test_both(expr, "ایکس کی قوت این جمع 1")
}

// Verifies numeric and compound subscripts.
#[test]
fn subscripts() -> Result<()> {
    let numeric = "<math><msub><mi>x</mi><mn>1</mn></msub></math>";
    let compound = "<math><msub><mi>x</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msub></math>";
    test_both(numeric, "ایکس زیر نوشت 1")?;
    test_both(compound, "ایکس زیر نوشت این جمع 1")
}

// Verifies the localized rules working together in a small equation.
#[test]
fn quadratic_expression() -> Result<()> {
    let expr = "<math><mfrac><mrow><mo>−</mo><mi>b</mi><mo>+</mo><msqrt><msup><mi>x</mi><mn>2</mn></msup></msqrt></mrow><mn>2</mn></mfrac><mo>=</mo><mi>y</mi></math>";
    test_both(expr, "منفی بی جمع, ایکس کا مربع کا جذر المربع, بٹا 2; برابر وائے")
}
