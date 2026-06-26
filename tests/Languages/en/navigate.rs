//! Navigation is controlled by a `Navigation_Rules.yaml` file in conjunction with preferences.
//! See preference documentation for more info on navigation preferences.
#![allow(clippy::needless_return)]

use sxd_document::dom::Element;

#[cfg(not(target_family = "wasm"))]
use log::debug;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::assert_eq_with_panic_handler;
    use crate::Languages::navigate::{init_prefs, test_command};
    use libmathcat::errors::Result;
    use libmathcat::navigate::{do_navigate_command_string, NavigationPosition, NAVIGATION_STATE};
    use libmathcat::{errors_to_string, get_element, set_preference, MATHML_INSTANCE};

    #[allow(unused_imports)]
    #[test]
    //Do not copy the Russian tests into language-specific tests
    fn zoom_speech_ru() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        init_prefs(mathml_str, "Enhanced", "ru");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            let speech = test_command("ZoomIn", mathml, "msup");
            assert_eq!("переход внутрь; в числитель; бэ в квадрате", speech);
            let speech = test_command("ZoomIn", mathml, "base");
            assert_eq!("переход внутрь; в основание; бэ", speech);
            let speech = test_command("ZoomIn", mathml, "base");
            assert_eq!("достигнута максимальная детализация; бэ", speech);
            let speech = test_command("ZoomOut", mathml, "msup");
            assert_eq!("переход наружу; из основания; бэ в квадрате", speech);
            let speech = test_command("ZoomInAll", mathml, "base");
            assert_eq!("переход к максимальной детализации; в основание; бэ", speech);
            let speech = test_command("ZoomOutAll", mathml, "mfrac");
            assert_eq!("переход к выражению целиком; из основания; из числителя; дробь, числитель: бэ в квадрате, знаменатель: дэ, конец дроби", speech);
            let speech = test_command("ZoomOutAll", mathml, "mfrac");
            assert_eq!("выражение уже показано целиком; дробь, числитель: бэ в квадрате, знаменатель: дэ, конец дроби", speech);
            return Ok(());
        });
    }


    #[test]
    fn move_msubsup_char() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
        <mrow id='id-1'>
          <mn id='id-2'>1</mn>
          <mo id='id-3'>+</mo>
          <msubsup id='id-4'>
            <mi id='id-5'>x</mi>
            <mn id='id-6'>2</mn>
            <mn id='id-7'>3</mn>
          </msubsup>
          <mo id='id-8'>+</mo>
          <mn id='id-9'>4</mn>
        </mrow>
       </math>";
        init_prefs(mathml_str, "Character", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            assert_eq_with_panic_handler(
                "zoomed in all of the way; 1",
                test_command("ZoomInAll", mathml, "id-2"),
            )?;
            assert_eq_with_panic_handler("move right; plus", test_command("MoveNext", mathml, "id-3"))?;
            assert_eq_with_panic_handler(
                "move right; in base; x",
                test_command("MoveNext", mathml, "id-5"),
            )?;
            assert_eq_with_panic_handler(
                "move right; in subscript; 2",
                test_command("MoveNext", mathml, "id-6"),
            )?;
            assert_eq_with_panic_handler(
                "move right; in superscript; 3",
                test_command("MoveNext", mathml, "id-7"),
            )?;
            assert_eq_with_panic_handler(
                "move right; out of superscript; plus",
                test_command("MoveNext", mathml, "id-8"),
            )?;
            assert_eq_with_panic_handler(
                "move left; in superscript; 3",
                test_command("MovePrevious", mathml, "id-7"),
            )?;
            assert_eq_with_panic_handler(
                "move left; in subscript; 2",
                test_command("MovePrevious", mathml, "id-6"),
            )?;
            assert_eq_with_panic_handler(
                "move left; in base; x",
                test_command("MovePrevious", mathml, "id-5"),
            )?;
            assert_eq_with_panic_handler(
                "move left; out of base; plus",
                test_command("MovePrevious", mathml, "id-3"),
            )?;

            return Ok(());
        });
    }
    #[test]
    fn zoom_logbase() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
            <mrow displaystyle='true' id='id-1'>
                <msub id='id-2'>
                    <mi id='id-3'>log</mi>
                    <mn id='id-4'>2</mn>
                </msub>
                <mo data-changed='added' id='id-5'>&#x2061;</mo>
                <mi id='id-6'>x</mi>a
            </mrow>
            </math>";
        init_prefs(mathml_str, "Enhanced", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            assert_eq_with_panic_handler(
                "zoom in; the log base 2",
                test_command("ZoomIn", mathml, "id-2"),
            )?;
            assert_eq_with_panic_handler(
                "zoom in; in base; 2",
                test_command("ZoomIn", mathml, "id-4"),
            )?;
            assert_eq_with_panic_handler(
                "zoomed in all of the way; 2",
                test_command("ZoomInAll", mathml, "id-4"),
            )?;
            debug!("Now zooming out");
            assert_eq_with_panic_handler(
                "zoom out; out of base; the log base 2",
                test_command("ZoomOut", mathml, "id-2"),
            )?;
            assert_eq_with_panic_handler(
                "zoom out; the log base 2, of x",
                test_command("ZoomOut", mathml, "id-1"),
            )?;
            assert_eq_with_panic_handler(
                "zoomed out all of the way; the log base 2, of x",
                test_command("ZoomOut", mathml, "id-1"),
            )?;
            return Ok(());
        });
    }

    #[test]
    fn zoom_logbase_power() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
            <mrow displaystyle='true' id='id-1'>
                <msubsup id='id-2'>
                    <mi id='id-3'>log</mi>
                    <mn id='id-4'>2</mn>
                    <mn id='id-5'>3</mn>
                </msubsup>
                <mo data-changed='added' id='id-6'>&#x2061;</mo>
                <mi id='id-7'>x</mi>
            </mrow>
            </math>";
        init_prefs(mathml_str, "Enhanced", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            assert_eq_with_panic_handler(
                "zoom in; the log base 2, cubed",
                test_command("ZoomIn", mathml, "id-2"),
            )?;
            assert_eq_with_panic_handler(
                "zoom in; in base; the log base 2",
                test_command("ZoomIn", mathml, "id-2-log-base"),
            )?;
            assert_eq_with_panic_handler(
                "zoom in; in base; 2",
                test_command("ZoomIn", mathml, "id-4"),
            )?;
            assert_eq_with_panic_handler(
                "zoomed in all of the way; 2",
                test_command("ZoomIn", mathml, "id-4"),
            )?;
            debug!("Now zooming out");
            assert_eq_with_panic_handler(
                "zoom out; out of base; the log base 2",
                test_command("ZoomOut", mathml, "id-2-log-base"),
            )?;
            assert_eq_with_panic_handler(
                "zoom out; out of base; the log base 2, cubed",
                test_command("ZoomOut", mathml, "id-2"),
            )?;
            assert_eq_with_panic_handler(
                "zoom out; the log base 2, cubed of x",
                test_command("ZoomOut", mathml, "id-1"),
            )?;
            assert_eq_with_panic_handler(
                "zoomed out all of the way; the log base 2, cubed of x",
                test_command("ZoomOut", mathml, "id-1"),
            )?;
            return Ok(());
        });
    }

    #[test]
    fn zoom_msubsup() -> Result<()> {
        // msubsup is trickier because it creates an intent within an intent, so offsets need to be handled properly
        let mathml_str = "<math id='math'><msubsup id='msubsup'><mi id='base'>𝑥</mi><mn id='sub'>1</mn><mn id='sup'>2</mn></msubsup></math>";
        init_prefs(mathml_str, "Enhanced", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            set_preference("NavMode", "Enhanced").unwrap();
            debug!("Enhanced mode");
            do_commands(mathml)?;
            set_preference("NavMode", "Simple").unwrap();
            debug!("Simple mode");
            do_commands(mathml)?;
            set_preference("NavMode", "Character").unwrap();
            debug!("Character mode");
            assert_eq_with_panic_handler(
                "zoom in; in base; x",
                test_command("ZoomIn", mathml, "base"),
            )?;
            assert_eq_with_panic_handler(
                "zoom out; out of base; x sub 1 super 2 end super",
                test_command("ZoomOut", mathml, "msubsup"),
            )?;
            return Ok(());

            /// Enhanced and Simple mode should behave the same
            fn do_commands(mathml: Element) -> Result<()> {
                assert_eq_with_panic_handler(
                    "zoom in; in base; x sub 1",
                    test_command("ZoomIn", mathml, "msubsup-indexed-by"),
                )?;
                assert_eq_with_panic_handler(
                    "zoom in; in base; x",
                    test_command("ZoomIn", mathml, "base"),
                )?;
                assert_eq_with_panic_handler(
                    "zoomed in all of the way; x",
                    test_command("ZoomIn", mathml, "base"),
                )?;
                debug!("Now zooming out");
                assert_eq_with_panic_handler(
                    "zoom out; out of base; x sub 1",
                    test_command("ZoomOut", mathml, "msubsup-indexed-by"),
                )?;
                assert_eq_with_panic_handler(
                    "zoom out; out of base; x sub 1, squared",
                    test_command("ZoomOut", mathml, "msubsup"),
                )?;
                assert_eq_with_panic_handler(
                    "zoomed out all of the way; x sub 1, squared",
                    test_command("ZoomOut", mathml, "msubsup"),
                )?;
                return Ok(());
            }
        });
    }

    #[test]
    fn move_mmultiscripts_char() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
            <mmultiscripts data-mjx-texclass='ORD' data-chem-formula='5' id='id-1'>
                <mrow data-chem-formula='3' id='id-2'>
                    <mo stretchy='false' id='id-3'>[</mo>
                    <mmultiscripts data-chem-formula='3' id='id-4'>
                        <mi data-chem-element='3' id='id-5'>Co</mi>
                        <mn id='id-6'>6</mn>
                        <none id='id-7'></none>
                    </mmultiscripts>
                    <mo stretchy='false' id='id-8'>]</mo>
                </mrow>
                <none id='id-9'></none>
                <mrow id='id-10'>
                    <mn id='id-11'>3</mn>
                    <mo id='id-12'>+</mo>
                </mrow>
            </mmultiscripts>
            </math>";
        init_prefs(mathml_str, "Character", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            assert_eq_with_panic_handler(
                "zoomed in all of the way; in base; open bracket",
                test_command("ZoomInAll", mathml, "id-3"),
            )?;
            assert_eq_with_panic_handler(
                "move right; in base; cap c o",
                test_command("MoveNext", mathml, "id-5"),
            )?;
            assert_eq_with_panic_handler(
                "move right; in subscript; 6",
                test_command("MoveNext", mathml, "id-6"),
            )?;
            assert_eq_with_panic_handler(
                "move right; out of subscript; close bracket",
                test_command("MoveNext", mathml, "id-8"),
            )?;
            assert_eq_with_panic_handler(
                "move right; in superscript; 3",
                test_command("MoveNext", mathml, "id-11"),
            )?;
            assert_eq_with_panic_handler(
                "move right; plus",
                test_command("MoveNext", mathml, "id-12"),
            )?;
            assert_eq_with_panic_handler(
                "cannot move right, end of math",
                test_command("MoveNext", mathml, "id-12"),
            )?;
            assert_eq_with_panic_handler(
                "move left; 3",
                test_command("MovePrevious", mathml, "id-11"),
            )?;
            assert_eq_with_panic_handler(
                "move left; in base; close bracket",
                test_command("MovePrevious", mathml, "id-8"),
            )?;
            assert_eq_with_panic_handler(
                "move left; in subscript; 6",
                test_command("MovePrevious", mathml, "id-6"),
            )?;
            assert_eq_with_panic_handler(
                "move left; in base; cap c o",
                test_command("MovePrevious", mathml, "id-5"),
            )?;
            assert_eq_with_panic_handler(
                "move left; out of base; open bracket",
                test_command("MovePrevious", mathml, "id-3"),
            )?;

            return Ok(());
        });
    }

    #[test]
    fn char_mode_paren_test() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
            <mrow displaystyle='true' id='id-1'>
                <mrow id='id-2'>
                    <mo id='id-3'>(</mo>
                    <mi id='id-4'>a</mi>
                    <mo id='id-5'>)</mo>
                </mrow>
                <mo id='id-6'>&#x2062;</mo>
                <mrow id='id-7'>
                    <mo id='id-8'>(</mo>
                    <mi id='id-9'>b</mi>
                    <mo id='id-10'>)</mo>
                </mrow>
            </mrow>
        </math>";
        init_prefs(mathml_str, "Character", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            debug!("Character mode");
            do_commands(mathml)?;
            set_preference("NavMode", "Simple").unwrap();
            debug!("Simple mode");
            test_command("ZoomIn", mathml, "id-3");  // zooms to the first parenthesis
            do_commands(mathml)?;
            set_preference("NavMode", "Enhanced").unwrap();
            debug!("Enhanced mode");
            test_command("ZoomIn", mathml, "id-4");
            test_command("MoveNext", mathml, "id-6");
            test_command("MoveNext", mathml, "id-9");
            test_command("MovePrevious", mathml, "id-6");
            test_command("MovePrevious", mathml, "id-4");

            return Ok(());
        });

        /// Simple and Character mode should behave the same
        fn do_commands(mathml: Element) -> Result<()> {
            test_command("ZoomIn", mathml, "id-3");
            test_command("MoveNext", mathml, "id-4");
            test_command("MoveNext", mathml, "id-5");
            test_command("MoveNext", mathml, "id-8");
            test_command("MoveNext", mathml, "id-9");
            test_command("MoveNext", mathml, "id-10");
            test_command("MovePrevious", mathml, "id-9");
            test_command("MovePrevious", mathml, "id-8");
            test_command("MovePrevious", mathml, "id-5");
            test_command("ZoomOutAll", mathml, "id-1");
            return Ok(());
        }
    }

    #[test]
    fn char_mode_trig_test() -> Result<()> {
        let mathml_str = "<math id='id-0'>
            <mrow id='id-1'>
            <mi id='id-2'>sin</mi>
            <mo id='id-3'>&#x2061;</mo>
            <mrow id='id-4'>
                <mo id='id-5'>(</mo>
                <mi id='id-6'>x</mi>
                <mo id='id-7'>)</mo>
            </mrow>
            </mrow>
        </math>";
        init_prefs(mathml_str, "Simple", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            do_commands(mathml)?;
            set_preference("NavMode", "Simple").unwrap();
            do_commands(mathml)?;
            set_preference("NavMode", "Enhanced").unwrap();
            test_command("ZoomIn", mathml, "id-2");
            test_command("MoveNext", mathml, "id-6");
            test_command("MovePrevious", mathml, "id-2");

            return Ok(());
        });


        /// Simple and Character mode should behave the same
        fn do_commands(mathml: Element) -> Result<()> {
            test_command("ZoomIn", mathml, "id-2");
            test_command("MoveNext", mathml, "id-5");
            test_command("MoveNext", mathml, "id-6");
            test_command("MoveNext", mathml, "id-7");
            test_command("MovePrevious", mathml, "id-6");
            test_command("MovePrevious", mathml, "id-5");
            test_command("MovePrevious", mathml, "id-2");
            test_command("ZoomOutAll", mathml, "id-1");
            return Ok(());
        }
    }

    #[test]
    fn move_char_speech() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
                <mrow id='id-1'>
                <mfrac id='id-2'>
                    <mi id='id-3'>x</mi>
                    <mi id='id-4'>y</mi>
                </mfrac>
                <mo id='id-5'>&#x2062;</mo>
                <mi id='id-6'>z</mi>
                </mrow>
            </math>";
        init_prefs(mathml_str, "Character", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomInAll", mathml, "id-3");
            assert_eq_with_panic_handler(
                "move right; in denominator; y",
                test_command("MoveNext", mathml, "id-4"),
            )?;
            assert_eq_with_panic_handler(
                "move right; out of denominator; z",
                test_command("MoveNext", mathml, "id-6"),
            )?;
            assert_eq_with_panic_handler(
                "move left; in denominator; y",
                test_command("MovePrevious", mathml, "id-4"),
            )?;
            assert_eq_with_panic_handler(
                "move left; in numerator; x",
                test_command("MovePrevious", mathml, "id-3"),
            )?;

            return Ok(());
        });
    }

    #[test]
    //Do not copy the Russian tests into language-specific tests
    fn move_char_speech_ru() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
                <mrow id='id-1'>
                <mfrac id='id-2'>
                    <mi id='id-3'>x</mi>
                    <mi id='id-4'>y</mi>
                </mfrac>
                <mo id='id-5'>&#x2062;</mo>
                <msqrt id='id-6'><mi id='id-7'>z</mi></msqrt>
                </mrow>
            </math>";
        init_prefs(mathml_str, "Character", "ru");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomInAll", mathml, "id-3");
            let speech = test_command("MoveNext", mathml, "id-4");
            assert_eq!("перемещение вправо; в знаменатель; игрек", speech);
            let speech = test_command("MoveNext", mathml, "id-7");
            assert_eq!("перемещение вправо; из знаменателя; в подкоренное выражение; зэт", speech);
            let speech = test_command("MovePrevious", mathml, "id-4");
            assert_eq!("перемещение влево; из подкоренного выражения; в знаменатель; игрек", speech);
            let speech = test_command("MovePrevious", mathml, "id-3");
            assert_eq!("перемещение влево; в числитель; икс", speech);
            return Ok(());
        });
    }

    #[test]
    fn move_inside_leaves() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
                <mrow id='id-1'>
                    <mfrac id='id-2'>
                        <mi id='id-3'>top</mi>
                        <mi id='id-4'>αβγ</mi>
                    </mfrac>
                </mrow>
            </math>";
        init_prefs(mathml_str, "Character", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomInAll", mathml, "id-3");
            assert_eq_with_panic_handler(
                "zoomed in to first character; t",
                test_command("ZoomIn", mathml, "id-3"),
            )?;
            assert_eq_with_panic_handler("move right; o", test_command("MoveNext", mathml, "id-3"))?;
            assert_eq_with_panic_handler("move right; p", test_command("MoveNext", mathml, "id-3"))?;
            assert_eq_with_panic_handler(
                "move right; in denominator; αβγ",
                test_command("MoveNext", mathml, "id-4"),
            )?;
            assert_eq_with_panic_handler(
                "zoomed in to first character; alpha",
                test_command("ZoomIn", mathml, "id-4"),
            )?;
            assert_eq_with_panic_handler("move right; beta", test_command("MoveNext", mathml, "id-4"))?;
            assert_eq_with_panic_handler(
                "move right; gamma",
                test_command("MoveNext", mathml, "id-4"),
            )?;
            assert_eq_with_panic_handler(
                "cannot move right, end of math",
                test_command("MoveNext", mathml, "id-4"),
            )?;
            assert_eq_with_panic_handler(
                "move left; beta",
                test_command("MovePrevious", mathml, "id-4"),
            )?;
            assert_eq_with_panic_handler("zoom out; αβγ", test_command("ZoomOut", mathml, "id-4"))?;

            return Ok(());
        });
    }

    #[test]
    fn move_enhanced_times() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
        <mrow displaystyle='true' id='id-1'>
          <mn id='id-2'>2</mn>
          <mo id='id-3'>&#x2062;</mo>
          <mrow id='id-4'>
            <mo id='id-5'>(</mo>
            <mrow id='id-6'>
              <mn id='id-7'>1</mn>
              <mo id='id-8'>-</mo>
              <mi id='id-9'>x</mi>
            </mrow>
            <mo id='id-10'>)</mo>
          </mrow>
        </mrow>
       </math>";
        init_prefs(mathml_str, "Enhanced", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomIn", mathml, "id-2");
            assert_eq_with_panic_handler(
                "move right; times",
                test_command("MoveNext", mathml, "id-3"),
            )?;
            assert_eq_with_panic_handler(
                "move right; 1 minus x",
                test_command("MoveNext", mathml, "id-6"),
            )?;
            assert_eq_with_panic_handler(
                "move left; times",
                test_command("MovePrevious", mathml, "id-3"),
            )?;
            assert_eq_with_panic_handler("move left; 2", test_command("MovePrevious", mathml, "id-2"))?;

            return Ok(());
        });
    }

    #[test]
    fn move_simple_no_times() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
        <mrow displaystyle='true' id='id-1'>
          <mn id='id-2'>2</mn>
          <mo id='id-3'>&#x2062;</mo>
          <mrow id='id-4'>
            <mo id='id-5'>(</mo>
            <mrow id='id-6'>
              <mn id='id-7'>1</mn>
              <mo id='id-8'>-</mo>
              <mi id='id-9'>x</mi>
            </mrow>
            <mo id='id-10'>)</mo>
          </mrow>
        </mrow>
       </math>";
        init_prefs(mathml_str, "Simple", "en");
        set_preference("SpeechStyle", "ClearSpeak")?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomIn", mathml, "id-2");
            assert_eq_with_panic_handler(
                "move right; open paren",
                test_command("MoveNext", mathml, "id-5"),
            )?;
            assert_eq_with_panic_handler("move right; 1", test_command("MoveNext", mathml, "id-7"))?;
            assert_eq_with_panic_handler(
                "move left; open paren",
                test_command("MovePrevious", mathml, "id-5"),
            )?;
            assert_eq_with_panic_handler("move left; 2", test_command("MovePrevious", mathml, "id-2"))?;

            return Ok(());
        });
    }

    #[test]
    fn move_cell() -> Result<()> {
        let mathml_str = "<math id='nav-0'>
        <mtable id='nav-1'>
          <mtr id='nav-2'>
            <mtd id='nav-3'> <mn id='nav-4'>1</mn></mtd>
            <mtd id='nav-5'> <mn id='nav-6'>2</mn></mtd>
            <mtd id='nav-7'><mn id='nav-8'>3</mn> </mtd>
          </mtr>
          <mtr id='nav-9'>
            <mtd id='nav-10'>
              <mrow id='nav-11'>
                <mi id='nav-12'>x</mi>
                <mo id='nav-13'>-</mo>
                <mi id='nav-14'>y</mi>
              </mrow>
            </mtd>
            <mtd id='nav-15'>
              <mfrac id='nav-16'>
                <mn id='nav-17'>1</mn>
                <mn id='nav-18'>2</mn>
              </mfrac>
            </mtd>
            <mtd id='nav-19'>
              <mi id='nav-20'>z</mi>
            </mtd>
          </mtr>
          <mtr id='nav-21'>
            <mtd id='nav-22'><mn id='nav-23'>7</mn> </mtd>
            <mtd id='nav-24'><mn id='nav-25'>8</mn> </mtd>
            <mtd id='nav-26'> <mn id='nav-27'>9</mn></mtd>
          </mtr>
          <mtr id='nav-28'>
            <mtd id='nav-29'>
              <mrow id='nav-30'>
                <mi id='nav-31'>sin</mi>
                <mo id='nav-32'>&#x2061;</mo>
                <mi id='nav-33'>x</mi>
              </mrow>
            </mtd>
            <mtd id='nav-34'>
              <msup id='nav-35'>
                <mi id='nav-36'>e</mi>
                <mi id='nav-37'>x</mi>
              </msup>
            </mtd>
            <mtd id='nav-38'>
              <mrow id='nav-39'>
                <mn id='nav-40'>2</mn>
                <mo id='nav-41'>-</mo>
                <mi id='nav-42'>y</mi>
              </mrow>
            </mtd>
          </mtr>
        </mtable>
       </math>";
        init_prefs(mathml_str, "Enhanced", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomInAll", mathml, "nav-4");
            test_command("MoveCellNext", mathml, "nav-6");
            test_command("MoveCellNext", mathml, "nav-8");
            test_command("MoveCellNext", mathml, "nav-8");
            test_command("MoveCellDown", mathml, "nav-20");
            test_command("MoveCellDown", mathml, "nav-27");
            let speech = test_command("MoveCellDown", mathml, "nav-39");
            assert_eq_with_panic_handler("move down, row 4, column 3; 2 minus y", speech)?;
            let speech = test_command("MoveCellDown", mathml, "nav-39");
            assert_eq_with_panic_handler("no next row", speech)?;
            test_command("MoveCellPrevious", mathml, "nav-35");
            test_command("ZoomIn", mathml, "nav-36");
            test_command("MoveCellUp", mathml, "nav-25");
            test_command("MoveCellUp", mathml, "nav-16");
            test_command("MoveCellUp", mathml, "nav-6");
            test_command("MoveCellUp", mathml, "nav-6");

            return Ok(());
        });
    }

    #[test]
    fn where_am_i_all() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        init_prefs(mathml_str, "Enhanced", "en");
        set_preference("SpeechStyle", "ClearSpeak")?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition {
                    current_node: "exp".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            // WhereAmIAll doesn't change the stack
            let speech = test_command("WhereAmIAll", mathml, "exp");
            // should be 2 "inside" strings corresponding to steps to the root
            assert_eq_with_panic_handler("2; inside; b squared; inside; the fraction with numerator; b squared; and denominator d", speech)?;
            return Ok(());
        });
    }

    #[test]
    fn zoom_in_parens() -> Result<()> {
        // (a+b)(c+d) + 1
        let mathml_str = " <math display='block' id='id-0'>
            <mrow id='id-1'>
                <mrow id='id-2'>
                    <mrow id='id-3'>
                    <mo stretchy='false' id='id-4'>(</mo>
                    <mrow id='id-5'>
                        <mi id='id-6'>a</mi>
                        <mo id='id-7'>+</mo>
                        <mi id='id-8'>b</mi>
                    </mrow>
                    <mo stretchy='false' id='id-9'>)</mo>
                    </mrow>
                    <mo id='id-10'>&#x2062;</mo>
                    <mrow id='id-11'>
                    <mo stretchy='false' id='id-12'>(</mo>
                    <mrow id='id-13'>
                        <mi id='id-14'>c</mi>
                        <mo id='id-15'>+</mo>
                        <mi id='id-16'>d</mi>
                    </mrow>
                    <mo stretchy='false' id='id-17'>)</mo>
                    </mrow>
                </mrow>
                <mo id='id-18'>+</mo>
                <mn id='id-19'>1</mn>
            </mrow>
        </math>";
        init_prefs(mathml_str, "Enhanced", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            set_preference("NavMode", "Enhanced")?;
            debug!("\n------EnhancedMode----------");
            test_command("ZoomIn", mathml, "id-2");
            test_command("ZoomIn", mathml, "id-5");
            test_command("ZoomIn", mathml, "id-6");

            // repeat, but this time with "Simple
            set_preference("NavMode", "Simple")?;
            debug!("\n------SimpleMode----------");
            test_command("ZoomOutAll", mathml, "id-1");
            test_command("ZoomIn", mathml, "id-4");
            test_command("ZoomIn", mathml, "id-4");
            return Ok(());
        });
    }
    #[test]
    fn zoom_root() -> Result<()> {
        let mathml_str = r#"<math display='block' id='id-0'>
        <mrow id='id-1'>
            <mo id='id-9'>±</mo>
            <msqrt id='id-10'>
                <mrow id='id-11'>
                    <msup id='id-12'> <mi id='id-13'>b</mi> <mn id='id-14'>2</mn> </msup>
                    <mo id='id-15'>-</mo>
                    <mn id='id-17'>4</mn>
                </mrow>
            </msqrt>
        </mrow>
        </math>"#;

        test_mode(mathml_str, "Enhanced")?;
        test_mode(mathml_str, "Simple")?;
        test_mode(mathml_str, "Character")?;
        return Ok(());

        fn test_mode(mathml_str: &str, mode: &str) -> Result<()> {
            init_prefs(mathml_str, mode, "en");
            set_preference("AutoZoomOut", "False")?;
            return MATHML_INSTANCE.with(|package_instance| {
                debug!("--- Testing mode {mode} ---");
                let package_instance = package_instance.borrow();
                let mathml = get_element(&package_instance);
                test_command("ZoomIn", mathml, "id-9");
                debug!("\nStart zoom in");
                match mode {
                    "Enhanced" => {
                        test_command("MoveNext", mathml, "id-10");
                        let speech = test_command("ZoomIn", mathml, "id-11");
                        assert_eq_with_panic_handler("zoom in; in root; b squared minus 4", speech)?;  // only one arg, so don't say "in root"
                        let speech = test_command("ZoomIn", mathml, "id-12");
                        assert_eq_with_panic_handler("zoom in; b squared", speech)?;  // only one arg, so don't say "in root"
                        let speech = test_command("ZoomIn", mathml, "id-13");
                        assert_eq_with_panic_handler("zoom in; in base; b", speech)?;
                    },
                    "Simple" => {
                        test_command("MoveNext", mathml, "id-10");
                        let speech = test_command("ZoomIn", mathml, "id-12");
                        assert_eq_with_panic_handler("zoom in; in root; b squared", speech)?;
                        let speech = test_command("ZoomIn", mathml, "id-13");
                        assert_eq_with_panic_handler("zoom in; in base; b", speech)?;
                    },
                    _ => { // "Character"
                        let speech = test_command("MoveNext", mathml, "id-13");
                        assert_eq_with_panic_handler("move right; in root; in base; b", speech)?;
                    }
                }
                let squared_speech = if mode == "Character" { "b super 2 end super" } else { "b squared" };
                let sqrt_speech = if mode == "Character" { "root" } else { "square root" };
                let speech = test_command("ZoomOut", mathml, "id-12");
                assert_eq_with_panic_handler(&format!("zoom out; out of base; {squared_speech}"), speech)?;
                let speech = test_command("ZoomOut", mathml, "id-11");
                assert_eq_with_panic_handler(&format!("zoom out; {squared_speech} minus 4"), speech)?;
                let speech = test_command("ZoomOut", mathml, "id-10");
                assert_eq_with_panic_handler(&format!("zoom out; out of root; the {sqrt_speech} of {squared_speech} minus 4, end root"), speech)?;
                return Ok(());
            });
        }
    }

    #[test]
    fn matrix_speech() -> Result<()> {
        let mathml_str = r#"<math id='math'>
            <mrow id='mrow'>
            <mo id='open'>[</mo>
            <mtable columnspacing='1em' rowspacing='4pt' id='table'>
                <mtr id='row-1'>
                    <mtd id='1-1'><mn id='id-6'>9</mn></mtd>
                    <mtd id='1-2'><mrow id='id-8'><mo id='id-9'>-</mo><mn id='id-10'>13</mn></mrow></mtd>
                </mtr>
                <mtr id='row-2'>
                    <mtd id='2-1'><mn id='id-13'>5</mn></mtd>
                    <mtd id='2-2'><mo id='id-16'>-</mo><mn id='id-17'>6</mn></mtd>
                </mtr>
            </mtable>
            <mo id='close'>]</mo>
            </mrow>
        </math>"#;
        init_prefs(mathml_str, "Enhanced", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomIn", mathml, "row-1");
            let speech = test_command("MoveNext", mathml, "row-2");
            assert_eq_with_panic_handler("move right; row 2; 5, negative 6", speech)?;
            let speech = test_command("ZoomIn", mathml, "id-13");
            assert_eq_with_panic_handler("zoom in; column 1; 5", speech)?;
            let speech = test_command("ZoomOut", mathml, "row-2");
            assert_eq_with_panic_handler("zoom out; row 2; 5, negative 6", speech)?;
            let speech = test_command("ZoomOut", mathml, "table");
            assert_eq_with_panic_handler(
                "zoom out; the 2 by 2 matrix; row 1; 9, negative 13; row 2; 5, negative 6",
                speech,
            )?;
            return Ok(());
        });
    }

    #[test]
    fn chem_speech() -> Result<()> {
        // this comes from bug 218
        let mathml_str = "<math display='block' id='id-0'>
            <mrow data-chem-formula='5' id='id-1'>
                <msub data-chem-formula='1' id='id-2'>
                    <mi data-chem-element='1' id='id-3'>H</mi>
                    <mn id='id-4'>2</mn>
                </msub>
                <mo data-chem-formula-op='0' id='id-5'>&#x2063;</mo>
                <mi data-chem-element='1' id='id-6'>S</mi>
                <mo data-chem-formula-op='0' id='id-7'>&#x2063;</mo>
                <msub data-chem-formula='1' id='id-8'>
                    <mi data-chem-element='1' id='id-9'>O</mi>
                    <mn id='id-10'>4</mn>
                </msub>
            </mrow>
        </math>";
        init_prefs(mathml_str, "Enhanced", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomIn", mathml, "id-2");
            let speech = test_command("MoveNext", mathml, "id-6");
            // tables need to check their parent for proper speech
            assert_eq_with_panic_handler("move right; cap s", speech)?;
            return Ok(());
        });
    }

    #[test]
    fn determinant_speech() -> Result<()> {
        let mathml_str = "<math id='math'>
            <mrow id='mrow'>
            <mo id='open'>|</mo>
            <mtable columnspacing='1em' rowspacing='4pt' id='table'>
                <mtr id='row-1'>
                    <mtd id='1-1'><mn id='id-6'>9</mn></mtd>
                    <mtd id='1-2'><mrow id='id-8'><mo id='id-9'>-</mo><mn id='id-10'>13</mn></mrow></mtd>
                </mtr>
                <mtr id='row-2'>
                    <mtd id='2-1'><mn id='id-13'>5</mn></mtd>
                    <mtd id='2-2'><mrow id='row2-negative'><mo id='id-16'>-</mo><mn id='id-17'>6</mn></mrow></mtd>
                </mtr>
            </mtable>
            <mo id='close'>|</mo>
            </mrow>
        </math>";
        init_prefs(mathml_str, "Enhanced", "en");
        set_preference("SpeechStyle", "ClearSpeak")?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            let speech = test_command("ZoomIn", mathml, "row-1");
            assert_eq_with_panic_handler("zoom in; row 1; 9, negative 13", speech)?;
            let speech = test_command("MoveNext", mathml, "row-2");
            assert_eq_with_panic_handler("move right; row 2; 5, negative 6", speech)?;
            let speech = test_command("MoveNext", mathml, "row-2");
            assert_eq_with_panic_handler("cannot move right, end of math", speech)?;
            let speech = test_command("ZoomIn", mathml, "id-13");
            assert_eq_with_panic_handler("zoom in; column 1; 5", speech)?;
            let speech = test_command("MoveNext", mathml, "row2-negative");
            assert_eq_with_panic_handler("move right; column 2, negative 6", speech)?;
            let speech = test_command("ZoomOutAll", mathml, "table");
            assert_eq_with_panic_handler("zoomed out all of the way; the 2 by 2 determinant; row 1; 9, negative 13; row 2; 5, negative 6", speech)?;
            return Ok(());
        });
    }

    #[test]
    fn cases_speech() -> Result<()> {
        let mathml_str = "<math id='id-0'>
        <mrow id='id-1'>
          <mo id='open'>{</mo>
          <mtable columnalign='left left' columnspacing='1em' displaystyle='false' rowspacing='.2em' id='table'>
            <mtr id='row-1'>
              <mtd id='id-5'><mrow id='id-6'><mrow id='id-7'><mo id='id-8'>-</mo><mi id='id-9'>x</mi></mrow><mo id='id-10'>,</mo></mrow></mtd>
              <mtd id='id-11'><mrow id='id-12'><mrow id='id-13'><mtext id='id-14'>if</mtext><mo id='id-15'>&#x2062;</mo><mi id='id-16'>x</mi></mrow><mo id='id-17'>&lt;</mo><mn id='id-18'>0</mn></mrow></mtd>
            </mtr>
            <mtr id='row-2'>
              <mtd id='id-20'><mrow id='id-21'><mrow id='id-22'><mo id='id-23'>+</mo><mi id='id-24'>x</mi></mrow><mo id='id-25'>,</mo></mrow></mtd>
              <mtd id='id-26'><mrow id='id-27'><mrow id='id-28'><mtext id='id-29'>if</mtext><mo id='id-30'>&#x2062;</mo><mi id='id-31'>x</mi></mrow><mo id='id-32'>≥</mo><mn id='id-33'>0</mn></mrow></mtd>
            </mtr>
          </mtable>
        </mrow>
       </math>";
        init_prefs(mathml_str, "Enhanced", "en");
        set_preference("SpeechStyle", "ClearSpeak")?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomIn", mathml, "row-1");
            let speech = test_command("MovePrevious", mathml, "row-1");
            assert_eq_with_panic_handler("move left; start of math", speech)?;
            let speech = test_command("MoveNext", mathml, "row-2");
            assert_eq_with_panic_handler("move right; case 2; positive x comma; if x, is greater than or equal to 0", speech)?;
            let speech = test_command("ZoomOut", mathml, "table");
            assert_eq_with_panic_handler("zoom out; 2 cases; case 1; negative x comma; if x is less than 0; case 2; positive x comma; if x, is greater than or equal to 0", speech)?;
            let speech = test_command("ZoomIn", mathml, "row-1");
            assert_eq_with_panic_handler("zoom in; case 1; negative x comma; if x is less than 0", speech)?;
            set_preference("NavMode", "Character")?;
            let speech = test_command("MovePrevious", mathml, "open");
            assert_eq_with_panic_handler("move left; open brace", speech)?;
            return Ok(());
        });
    }

    #[test]
    fn base_superscript() -> Result<()> {
        // bug #217 -- zoom into base of parenthesized script
        let mathml_str = "<math display='block' id='id-0'>
            <msup id='id-1'>
                <mrow id='id-2'>
                    <mo stretchy='false' id='id-3'>(</mo>
                    <mrow id='id-4'>
                        <mn id='id-5'>2</mn>
                        <mo id='id-6'>&#x2062;</mo>
                        <mi id='id-7'>x</mi>
                    </mrow>
                    <mo stretchy='false' id='id-8'>)</mo>
                </mrow>
                <mn id='id-9'>2</mn>
            </msup>
        </math>";
        init_prefs(mathml_str, "Enhanced", "en");
        set_preference("SpeechStyle", "ClearSpeak")?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            let speech = test_command("ZoomIn", mathml, "id-4");
            assert_eq_with_panic_handler("zoom in; in base; 2 x", speech)?;
            let speech = test_command("MoveNext", mathml, "id-9");
            assert_eq_with_panic_handler("move right; in exponent; 2", speech)?;
            return Ok(());
        });
    }

    #[test]
    fn binomial_intent() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
                    <mrow intent='binomial($n,$k)' id='id-1'>
                        <mo id='id-2'>(</mo>
                        <mfrac linethickness='0pt' id='id-3'>
                            <mi arg='n' id='id-4'>n</mi>
                            <mi arg='k' id='id-5'>k</mi>
                        </mfrac>
                    <mo id='id-6'>)</mo>
                    </mrow>
                </math>";
        init_prefs(mathml_str, "Character", "en");
        set_preference("SpeechStyle", "ClearSpeak")?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            debug!("Character mode");
            let speech = test_command("MoveStart", mathml, "id-2");
            assert_eq_with_panic_handler("move to start of math; open paren", speech)?;
            let speech = test_command("MoveNext", mathml, "id-4");
            // I'm not keen on the use of numerator/denominator here, but character mode turns off intent
            assert_eq_with_panic_handler("move right; in numerator; n", speech)?;
            let speech = test_command("MoveNext", mathml, "id-5");
            assert_eq_with_panic_handler("move right; in denominator; k", speech)?;
            debug!("before zoom out");
            let speech = test_command("ZoomOut", mathml, "id-3");
            assert_eq_with_panic_handler("zoom out; out of denominator; n over k", speech)?;
            // let speech = test_command("ZoomOut", mathml, "id-1");
            // assert_eq_with_panic_handler("zoom out; open paren n over k, close paren", speech)?;

            set_preference("NavMode", "Simple").unwrap();
            debug!("Simple mode");
            let speech = test_command("ZoomIn", mathml, "id-4");
            assert_eq_with_panic_handler("zoom in; in part 1; n", speech)?;
            let speech = test_command("MoveNext", mathml, "id-5");
            assert_eq_with_panic_handler("move right; in part 2; k", speech)?;
            let speech = test_command("MoveNext", mathml, "id-5");
            assert_eq_with_panic_handler("cannot move right, end of math", speech)?;
            let speech = test_command("ZoomOut", mathml, "id-1-literal-0");
            assert_eq_with_panic_handler("zoom out; out of part 2; n choose k", speech)?;

            set_preference("NavMode", "Enhanced").unwrap();
            debug!("Enhanced mode");
            let speech = test_command("ZoomIn", mathml, "id-4");
            assert_eq_with_panic_handler("zoom in; in part 1; n", speech)?;
            let speech = test_command("MoveNext", mathml, "id-5");
            assert_eq_with_panic_handler("move right; in part 2; k", speech)?;
            let speech = test_command("MoveNext", mathml, "id-5");
            assert_eq_with_panic_handler("cannot move right, end of math", speech)?;
            let speech = test_command("ZoomOut", mathml, "id-1-literal-0");
            assert_eq_with_panic_handler("zoom out; out of part 2; n choose k", speech)?;

            return Ok(());
        });
    }

    #[test]
    fn matrix_literal_intent() -> Result<()> {
        let mathml_str = r#"<math display='block' id='id-0'>
            <mrow intent='$m' id='id-1'>
                <mo id='id-2'>(</mo>
                <mtable arg='m' intent='_diagonal:prefix(1,2,3)' id='id-3'>
                <mtr id='id-4'>
                    <mtd id='id-5'><mn id='id-6'>1</mn></mtd>
                    <mtd id='id-7'><mn id='id-8'>0</mn></mtd>
                    <mtd id='id-9'><mn id='id-10'>0</mn></mtd>
                </mtr>
                <mtr id='id-11'>
                    <mtd id='id-12'><mn id='id-13'>0</mn></mtd>
                    <mtd id='id-14'><mn id='id-15'>2</mn></mtd>
                    <mtd id='id-16'><mn id='id-17'>0</mn></mtd>
                </mtr>
                <mtr id='id-18'>
                    <mtd id='id-19'><mn id='id-20'>0</mn></mtd>
                    <mtd id='id-21'><mn id='id-22'>0</mn></mtd>
                    <mtd id='id-23'><mn id='id-24'>3</mn></mtd>
                </mtr>
                </mtable>
                <mo id='id-25'>)</mo>
            </mrow>
        </math>"#;
        init_prefs(mathml_str, "Simple", "en");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            let speech = test_command("ZoomIn", mathml, "id-3-literal-1");
            assert_eq_with_panic_handler("zoom in; 1", speech)?;
            let speech = test_command("MoveNext", mathml, "id-3-literal-2");
            assert_eq_with_panic_handler("move right; 2", speech)?;
            let speech = test_command("MoveNext", mathml, "id-3-literal-3");
            assert_eq_with_panic_handler("move right; 3", speech)?;
            let speech = test_command("MoveNext", mathml, "id-3-literal-3");
            assert_eq_with_panic_handler("cannot move right, end of math", speech)?;
            let speech = test_command("ZoomOut", mathml, "id-3-literal-0");
            assert_eq_with_panic_handler("zoom out; diagonal 1 2 3", speech)?;

            return Ok(());
        });
    }

    #[test]
    fn absolute_value() -> Result<()> {
        let mathml_str = "<math id='math'>
                <mrow id='expr'>
                    <mn id='2'>2</mn>
                    <mrow id='abs'>
                        <mo id='start'>|</mo>
                        <mi id='x'>x</mi>
                        <mo id='end'>|</mo>
                    </mrow>
                </mrow>
            </math>";
        init_prefs(mathml_str, "Enhanced", "en");
        set_preference("SpeechStyle", "ClearSpeak")?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            let speech = test_command("ZoomIn", mathml, "2");
            assert_eq_with_panic_handler("zoom in; 2", speech)?;
            let speech = test_command("MoveNext", mathml, "abs");
            assert_eq_with_panic_handler("move right; the absolute value of x", speech)?;
            let speech = test_command("ZoomIn", mathml, "x");
            assert_eq_with_panic_handler("zoom in; in absolute value; x", speech)?;
            let speech = test_command("MoveNext", mathml, "x");
            assert_eq_with_panic_handler("cannot move right, end of math", speech)?;
            set_preference("NavMode", "Character")?;
            let speech = test_command("MoveNext", mathml, "end");
            assert_eq_with_panic_handler("move right; vertical line", speech)?;
            let speech = test_command("MoveLineStart", mathml, "2");
            assert_eq_with_panic_handler("move to start of line; 2", speech)?;
            let speech = test_command("MoveNext", mathml, "start");
            assert_eq_with_panic_handler("move right; vertical line", speech)?;
            return Ok(());
        });
    }

    #[test]
    fn read_and_describe_fraction() -> Result<()> {
        let mathml_str = "<math id='math'>
            <mrow id='mrow'>
                <mfrac id='frac'>
                    <mrow id='numerator'><mi>b</mi><mo>+</mo><mn>1</mn></mrow>
                <mn id='denom'>3</mn>
                </mfrac>
                <mo id='minus'>-</mo>
                <mn id='3'>3</mn>
            </mrow>
        </math>";
        init_prefs(mathml_str, "Enhanced", "en");
        set_preference("SpeechStyle", "SimpleSpeak")?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomIn", mathml, "frac");
            let speech = test_command("ReadCurrent", mathml, "frac");
            assert_eq_with_panic_handler(
                "read current; fraction, b plus 1, over 3, end fraction",
                speech,
            )?;
            let speech = test_command("DescribeCurrent", mathml, "frac");
            assert_eq_with_panic_handler("describe current; fraction", speech)?;
            return Ok(());
        });
    }

    #[test]
    //Do not copy the Russian tests into language-specific tests
    fn describe_nested_fraction_ru() -> Result<()> {
        let mathml_str = "<math id='math'>
            <mfrac id='frac'>
                <mrow id='num'>
                    <mi id='x1'>x</mi>
                    <mo id='plus'>+</mo>
                    <msqrt id='sqrt'>
                        <mfrac id='inner-frac'>
                            <mn id='one'>1</mn>
                            <mi id='y1'>y</mi>
                        </mfrac>
                    </msqrt>
                </mrow>
                <mrow id='den'>
                    <mi id='x2'>x</mi>
                    <mo id='minus'>-</mo>
                    <mi id='y2'>y</mi>
                </mrow>
            </mfrac>
        </math>";
        init_prefs(mathml_str, "Enhanced", "ru");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            let speech = test_command("ZoomIn", mathml, "");
            assert_eq!("переход внутрь; в числитель; икс  плюс, квадратный корень из 1 разделить на игрек, конец корня", speech);
            match do_navigate_command_string(mathml, "DescribeCurrent") {
                Ok(speech) => {
                    let speech = speech.trim_end_matches(&[' ', ',', ';']).to_string();
                    assert_eq!("описать текущее; икс  плюс, квадратный корень из 1 разделить на игрек", speech);
                },
                Err(e) => {
                    panic!("DescribeCurrent failed: {}", errors_to_string(&e));
                },
            };
            return Ok(());
        });
    }

    #[test]
    fn read_and_describe_mrow() -> Result<()> {
        let mathml_str = "<math id='math'>
            <mrow id='mrow'>
                <mn>1</mn><mo>+</mo>
                <mn>2</mn><mo>+</mo>
                <mn>3</mn><mo>+</mo>
                <mn>4</mn><mo>+</mo>
                <mn>5</mn><mo>+</mo>
                <mn>6</mn><mo>+</mo>
                <mn>7</mn>
            </mrow>
        </math>";
        init_prefs(mathml_str, "Enhanced", "en");
        set_preference("SpeechStyle", "SimpleSpeak")?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            let speech = test_command("ZoomOutAll", mathml, "mrow");
            assert_eq_with_panic_handler(
                "zoomed out all of the way; 1 plus 2 plus 3 plus 4 plus 5 plus 6 plus 7",
                speech,
            )?;
            let speech = test_command("ReadCurrent", mathml, "mrow");
            assert_eq_with_panic_handler(
                "read current; 1 plus 2 plus 3 plus 4 plus 5 plus 6 plus 7",
                speech,
            )?;
            let speech = test_command("DescribeCurrent", mathml, "mrow");
            assert_eq_with_panic_handler("describe current; 1 plus 2 plus 3 and so on", speech)?;
            return Ok(());
        });
    }

    #[test]
    fn read_next_invisible_char() -> Result<()> {
        let mathml_str = "<math id='id-0'>
            <mrow id='id-1'>
                <mi id='id-2'>x</mi>
                <mo id='id-3'>&#x2062;</mo>
                <mi id='id-4'>y</mi>
            </mrow>
            </math>";
        init_prefs(mathml_str, "Simple", "en");
        set_preference("SpeechStyle", "SimpleSpeak")?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            let speech = test_command("ZoomIn", mathml, "id-2");
            assert_eq_with_panic_handler("zoom in; x", speech)?;
            let speech = test_command("ToggleZoomLockUp", mathml, "id-2");
            assert_eq_with_panic_handler("enhanced mode; x", speech)?;
            let speech = test_command("ReadNext", mathml, "id-2");
            assert_eq_with_panic_handler("read right; y", speech)?;
            return Ok(());
        });
    }
}