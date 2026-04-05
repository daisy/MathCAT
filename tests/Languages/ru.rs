#![allow(non_snake_case)]

use crate::common::*;
use anyhow::Result;

mod ClearSpeak {
    use crate::common::*;
    use anyhow::Result;

    mod functions;
    mod large_ops;
    mod menclose;
    mod mfrac;
    mod mroot;
    mod msup;
    mod sets;
    mod symbols_and_adornments;
    mod multiline;
}

mod SimpleSpeak {
    use crate::common::*;
    use anyhow::Result;

    mod functions;
    mod large_ops;
    // mod menclose;
    mod mfrac;
    // mod mroot;
    mod msup;
    mod sets;
    mod geometry;
    mod linear_algebra;
    mod multiline;
    mod subscripts;
}

mod shared;
mod units;
mod chemistry;
mod alphabets;
mod intent;
mod mtable;