//! Mainland textbook terminology and structural speech regressions.

use crate::common::*;
use anyhow::Result;

#[test]
fn definite_integral_announces_limits() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><msubsup><mo>&#x222b;</mo><mn>0</mn><mn>1</mn></msubsup><mi>f</mi><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mi>d</mi><mi>x</mi></math>",
        "积分 从 0 到 1, f x d x",
    )
}
