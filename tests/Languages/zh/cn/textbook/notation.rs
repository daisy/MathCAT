//! Mainland textbook terminology and structural speech regressions.

use crate::common::*;
use anyhow::Result;

#[test]
fn menclose_names_the_mark_instead_of_the_result() -> Result<()> {
    // The notation describes the drawn enclosure: a radical sign or two crossing strike lines.
    test(
        "zh",
        "SimpleSpeak",
        "<math><menclose notation='radical'><mi>x</mi></menclose></math>",
        "根号, 包围 x",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><menclose notation='updiagonalstrike downdiagonalstrike'><mi>x</mi></menclose></math>",
        "交叉, 划掉, 包围 x",
    )
}
