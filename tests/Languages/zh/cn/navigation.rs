//! Simplified Chinese navigation speech regressions.

use crate::common::*;
use anyhow::Result;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn init_navigation(mathml: &str) -> Result<()> {
    set_rules_dir(abs_rules_dir_path())?;
    set_preference("Language", "zh")?;
    set_preference("SpeechStyle", "SimpleSpeak")?;
    set_preference("Verbosity", "Medium")?;
    set_preference("NavMode", "Enhanced")?;
    set_preference("NavVerbosity", "Verbose")?;
    set_preference("AutoZoomOut", "False")?;
    set_preference("Overview", "False")?;
    set_mathml(mathml)?;
    Ok(())
}

#[test]
fn navigation_enters_fraction_numerator() -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_navigation(
            "<math><mfrac id='frac'><mn id='num'>1</mn><mn id='den'>2</mn></mfrac></math>",
        )?;
        let speech = do_navigate_command("SetPlacemarker1")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("设置位置标记 1; 2 分之 1", speech);

        let speech = do_navigate_command("DescribeCurrent")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("概述 当前项; 2 分之 1", speech);

        let speech = do_navigate_command("ZoomIn")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("进入下一层; 进入 分子; 1", speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn navigation_uses_chinese_row_and_column_number_order() -> Result<()> {
    // Cell location announcements use "第 n 行，第 n 列" rather than noun-first order.
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_navigation(
            "<math><mrow><mo>(</mo><mtable><mtr><mtd><mn id='r1c1'>1</mn></mtd><mtd><mn id='r1c2'>2</mn></mtd></mtr><mtr><mtd><mn id='r2c1'>3</mn></mtd><mtd><mn id='r2c2'>4</mn></mtd></mtr></mtable><mo>)</mo></mrow></math>",
        )?;
        set_navigation_node("r2c2", 0)?;
        let speech = do_navigate_command("ReadCellCurrent")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("朗读当前单元格; 第 2 行, 第 2 列, 4", speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn navigation_overview_keeps_the_root_possessive_marker() -> Result<()> {
    // Overview speech must keep 的 after a complex radicand, just like full speech.
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_navigation(
            "<math><mroot><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mi>n</mi></mroot></math>",
        )?;
        let speech = do_navigate_command("DescribeCurrent")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("概述 当前项; x 加 y 的 n 次方根", speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn navigation_zooms_out_without_debug_speech_and_steps_into_currency_symbol() -> Result<()> {
    // ZoomOut must cleanly return to the parent fraction without debug output, and zooming
    // into a currency expression must visit the amount and currency symbol leaf in spoken order.
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_navigation(
            "<math><mfrac id='frac'><mn id='num'>1</mn><mn id='den'>2</mn></mfrac></math>",
        )?;
        do_navigate_command("ZoomIn")?;
        let speech = do_navigate_command("ZoomOut")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("返回上一层; 离开 分子; 2 分之 1", speech);

        init_navigation("<math><mrow id='amt'><mo id='sym'>$</mo><mn id='val'>3</mn></mrow></math>")?;
        let speech = do_navigate_command("ZoomIn")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("进入下一层; 美元", speech);
        let speech = do_navigate_command("MoveNext")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("移到 后一项; 3", speech);
        return Ok(());
    }));
    return report_any_panic(result);
}
