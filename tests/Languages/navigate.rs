use libmathcat::navigate::{do_navigate_command_string, NAVIGATION_STATE};
use libmathcat::{
    abs_rules_dir_path, errors_to_string, set_mathml,
    set_preference, set_rules_dir,
};
use sxd_document::dom::Element;

#[cfg(test)]
/// Assert if result_id != '' and it doesn't match the id of the result of the move
/// Returns the speech from the command
pub fn test_command(command: &'static str, mathml: Element, result_id: &str) -> String {
    // debug!("\nCommand: {}", command);
    NAVIGATION_STATE.with(|nav_stack| {
        let (start_id, _) = nav_stack.borrow().get_navigation_mathml_id(mathml);
        match do_navigate_command_string(mathml, command) {
            Err(e) => {
                panic!("\nStarting at '{}', '{} failed.\n{}",
                       start_id, command, &errors_to_string(&e))
            },
            Ok(nav_speech) => {
                let nav_speech = nav_speech.trim_end_matches(&[' ', ',', ';']);
                // debug!("Full speech: {}", nav_speech);
                if !result_id.is_empty() {
                    let (id, _) = nav_stack.borrow().get_navigation_mathml_id(mathml);
                    assert_eq!(result_id, id, "\nStarting at '{}', '{} failed.", start_id, command);
                }
                return nav_speech.to_string();
            }
        };
    })
}

pub fn init_default_prefs(mathml: &str, nav_mode_default: &str) {
    init_prefs(mathml, nav_mode_default, "en");
}

pub fn init_prefs(mathml: &str, nav_mode_default: &str, language: &str) {
    set_rules_dir(abs_rules_dir_path()).unwrap();
    set_preference("NavMode", nav_mode_default).unwrap();
    set_preference("NavVerbosity", "Verbose").unwrap();
    set_preference("AutoZoomOut", "True").unwrap();
    set_preference("Language", language).unwrap();
    set_preference("SpeechStyle", "SimpleSpeak").unwrap();
    set_preference("Verbosity", "Medium").unwrap();
    set_preference("Overview", "False").unwrap();
    set_mathml(mathml).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use libmathcat::errors::Result;
    use libmathcat::navigate::{
        do_navigate_command_and_param, NavigationCommand, NavigationParam, NavigationPosition,
        NAVIGATION_STATE,
    };
    use libmathcat::{get_element, get_supported_languages, set_preference, MATHML_INSTANCE};
    use log::debug;
    use std::cell::RefCell;
    use sxd_document::Package;
    #[allow(unused_imports)]
    #[test]
    fn zoom_in() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomIn", mathml, "msup");
            test_command("ZoomIn", mathml, "base");
            test_command("ZoomIn", mathml, "base");
            return Ok( () );
        });
    }

    #[test]
    fn test_init_navigate_move_right() -> Result<()> {
        // this is how navigation typically starts up
        let mathml_str = " <math display='block' id='id-0'>
            <mrow id='id-1'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mo id='id-3'>=</mo>
                <mrow id='id-4'>
                    <mi id='id-5'>a</mi>
                    <mo id='id-6'>-</mo>
                    <mn id='id-7'>2</mn>
                </mrow>
            </mrow>
        </math>";
        init_default_prefs(mathml_str, "Enhanced");
        debug!("--- Enhanced ---");
        MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomIn", mathml, "msup");
            test_command("MoveNext", mathml, "id-3");
        });

        init_default_prefs(mathml_str, "Simple");
        debug!("--- Simple ---");
        MATHML_INSTANCE.with(|package_instance: &RefCell<Package>| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomIn", mathml, "msup");
            test_command("MoveNext", mathml, "id-3");
        });

        init_default_prefs(mathml_str, "Character");
        debug!("--- Character ---");
        MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomIn", mathml, "base");
            test_command("MoveNext", mathml, "exp");
        });
        return Ok( () );
    }
    #[test]
    fn zoom_in_all() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomInAll", mathml, "base");
            return Ok( () );
        });
    }

    #[test]
    fn zoom_out() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "base".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            test_command("ZoomOut", mathml, "msup");

            let _nav_speech = do_navigate_command_and_param(mathml, NavigationCommand::Zoom, NavigationParam::Previous)?;
            NAVIGATION_STATE.with(|nav_stack| {
                let (id, _) = nav_stack.borrow().get_navigation_mathml_id(mathml);
                assert_eq!(id, "mfrac");
            });
            return Ok( () );
        });
    }

    #[test]
    fn zoom_out_all() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "base".to_string(),
                    current_node_offset: 0
                }, "None")
            });

            test_command("ZoomOutAll", mathml, "mfrac");
            return Ok( () );
        });
    }

    #[test]
    fn move_start_end() -> Result<()> {
        let mathml_str = " <math display='block' id='id-0'>
        <mrow id='id-1'>
          <mi id='id-2'>x</mi>
          <mo id='id-3'>=</mo>
          <mrow id='id-4'>
            <mi id='id-5'>a</mi>
            <mo id='id-6'>-</mo>
            <mn id='id-7'>2</mn>
          </mrow>
        </mrow>
       </math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "id-4".to_string(),
                    current_node_offset: 0
                }, "None")
            });

            set_preference("NavMode", "Character")?;
            test_command("MoveStart", mathml, "id-2");
            test_command("MoveEnd", mathml, "id-7");
            set_preference("NavMode", "Simple")?;
            test_command("MoveStart", mathml, "id-2");
            test_command("MoveEnd", mathml, "id-7");
            set_preference("NavMode", "Enhanced")?;
            test_command("MoveStart", mathml, "id-2");
            test_command("MovePrevious", mathml, "id-2");
            test_command("MoveEnd", mathml, "id-4");
            test_command("MoveNext", mathml, "id-4");
            return Ok( () );
        });
    }

    #[test]
    fn move_line_start_end() -> Result<()> {
        let mathml_str = " <math display='block' id='id-0'>
        <mfrac displaystyle='true' id='id-1'>
          <mi id='id-2'>x</mi>
          <mrow id='id-3'>
            <msup id='id-4'>
              <mi id='id-5'>y</mi>
              <mn id='id-6'>2</mn>
            </msup>
            <mo id='id-7'>+</mo>
            <mn id='id-8'>1</mn>
          </mrow>
        </mfrac>
       </math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "id-7".to_string(),
                    current_node_offset: 0
                }, "None")
            });

            set_preference("NavMode", "Character")?;
            test_command("MoveLineStart", mathml, "id-5");
            test_command("MoveLineEnd", mathml, "id-8");
            set_preference("NavMode", "Simple")?;
            test_command("MoveLineStart", mathml, "id-4");
            test_command("MoveLineEnd", mathml, "id-8");
            set_preference("NavMode", "Enhanced")?;
            test_command("MoveLineStart", mathml, "id-4");
            test_command("MoveLineEnd", mathml, "id-8");
            test_command("MoveEnd", mathml, "id-3");
            return Ok( () );
        });
    }

    #[test]
    fn text_extremes_and_move_last_location() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "base".to_string(),
                    current_node_offset: 0
                }, "None")
            });

            test_command("ZoomOutAll", mathml, "mfrac");
            test_command("ZoomOut", mathml, "mfrac");
            test_command("MoveLastLocation", mathml, "base");       // second zoom out should do nothing

            test_command("ZoomOut", mathml, "msup");
            test_command("ZoomInAll", mathml, "base");
            test_command("ZoomIn", mathml, "base");
            test_command("MoveLastLocation", mathml, "msup");       // second zoom in should do nothing

            return Ok( () );
        });
    }

    #[test]
    fn move_to_start() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <mrow id='num'><msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup><mo id='factorial'>!</mo></mrow>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "denom".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            test_command("MoveLineStart", mathml, "denom");

            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "factorial".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            test_command("MoveLineStart", mathml, "msup");

            let _nav_speech = do_navigate_command_and_param(mathml, NavigationCommand::Move, NavigationParam::Start)?;
            NAVIGATION_STATE.with(|nav_stack| {
                let (id, _) = nav_stack.borrow().get_navigation_mathml_id(mathml);
                assert_eq!(id, "num");
            });
            return Ok( () );
        });
    }

    #[test]
    fn move_right_sup() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
        <mrow id='id-1'>
          <msup id='id-2'>
            <mn id='id-3'>2</mn>
            <mi id='id-4'>q</mi>
          </msup>
          <mo id='id-5'>-</mo>
          <mi id='id-6'>x</mi>
        </mrow>
        </math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "id-2".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            set_preference("NavMode", "Enhanced")?;
            test_command("MoveNext", mathml, "id-5");

            // reset start and test Simple
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "id-2".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            set_preference("NavMode", "Simple")?;
            test_command("MoveNext", mathml, "id-5");

            // reset start and test Character
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "id-3".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            set_preference("NavMode", "Character")?;
            test_command("MoveNext", mathml, "id-4");
            test_command("MoveNext", mathml, "id-5");
            return Ok( () );
        });
    }

    #[test]
    fn move_right_char() -> Result<()> {
        let mathml_str = "<math id='id-0'>
        <mrow displaystyle='true' id='id-1'>
          <mi id='id-2'>x</mi>
          <mo id='id-3'>=</mo>
          <mrow id='id-4'>
            <mfrac id='id-5'>
              <mn id='id-6'>1</mn>
              <mrow id='id-7'>
                <mi id='id-8'>a</mi>
                <mo id='id-9'>+</mo>
                <mn id='id-10'>2</mn>
              </mrow>
            </mfrac>
            <mo id='id-11'>+</mo>
            <mrow id='id-12'>
              <mn id='id-13'>3</mn>
              <mo id='id-14'>&#x2062;</mo>
              <mi id='id-15'>b</mi>
            </mrow>
          </mrow>
        </mrow>
        </math>";
        init_default_prefs(mathml_str, "Character");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomInAll", mathml, "id-2");
            test_command("MoveNext", mathml, "id-3");
            test_command("MoveNext", mathml, "id-6");
            test_command("MoveNext", mathml, "id-8");
            test_command("MoveNext", mathml, "id-9");
            test_command("MoveNext", mathml, "id-10");
            test_command("MoveNext", mathml, "id-11");
            test_command("MoveNext", mathml, "id-13");
            test_command("MoveNext", mathml, "id-15");
            test_command("MoveNext", mathml, "id-15");

            return Ok( () );
        });
    }

    #[test]
    fn move_cell_char_mode() -> Result<()> {
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
        init_default_prefs(mathml_str, "Character");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "nav-8".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            test_command("MoveNext", mathml, "nav-12");
            test_command("MoveNext", mathml, "nav-13");
            test_command("MoveNext", mathml, "nav-14");
            test_command("MoveNext", mathml, "nav-17");
            test_command("MovePrevious", mathml, "nav-14");
            test_command("MoveCellNext", mathml, "nav-17");
            test_command("MoveCellPrevious", mathml, "nav-14");
            test_command("MovePrevious", mathml, "nav-13");
            test_command("MovePrevious", mathml, "nav-12");
            test_command("MoveCellPrevious", mathml, "nav-12");
            test_command("MovePrevious", mathml, "nav-8");
            test_command("MoveCellDown", mathml, "nav-20");
            test_command("MoveCellDown", mathml, "nav-27");
            test_command("MoveCellDown", mathml, "nav-40");
            test_command("MoveCellDown", mathml, "nav-40");
            test_command("MoveCellPrevious", mathml, "nav-37");
            test_command("MoveCellUp", mathml, "nav-25");

            return Ok( () );
        });
    }

    #[test]
    fn placemarker() -> Result<()> {
        let mathml_str = "<math display='block' id='math'>
        <mrow displaystyle='true' id='mrow'>
          <mi id='a'>a</mi>
          <mo id='plus-1'>+</mo>
          <mi id='b'>b</mi>
          <mo id='plus-2'>+</mo>
          <mi id='c'>c</mi>
        </mrow>
        </math>";
        init_default_prefs(mathml_str, "Character");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("MoveStart", mathml, "a");
            test_command("SetPlacemarker0", mathml, "a");
            test_command("MoveEnd", mathml, "c");
            test_command("Read0", mathml, "c");
            test_command("Describe0", mathml, "c");
            test_command("SetPlacemarker1", mathml, "c");
            test_command("MoveTo0", mathml, "a");
            test_command("MoveTo1", mathml, "c");
            test_command("MoveLastLocation", mathml, "a");

            return Ok( () );
        });
    }

    #[test]
    fn auto_zoom_out_mrow() -> Result<()> {
        let mathml_str = "<math id='math'>
        <mrow id='id-1'>
          <mrow id='id-2'>
            <mrow id='2ax'>
              <mn id='2'>2</mn>
              <mo id='id-5'>&#x2062;</mo>
              <mi id='a'>a</mi>
              <mo id='id-7'>&#x2062;</mo>
              <mi id='x'>x</mi>
            </mrow>
            <mo id='plus'>+</mo>
            <mi id='b'>b</mi>
          </mrow>
          <mo id='equal'>=</mo>
          <mn id='10'>10</mn>
        </mrow>
       </math>";
        init_default_prefs(mathml_str, "Enhanced");
        set_preference("AutoZoomOut", "False")?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomInAll", mathml, "2");
            test_command("MoveNext", mathml, "a");
            test_command("MoveNext", mathml, "x");
            test_command("MoveNext", mathml, "plus");
            test_command("MovePrevious", mathml, "2ax");
            return Ok( () );
        });
    }

    #[test]
    fn auto_zoom_out_fraction() -> Result<()> {
        let mathml_str = "<math id='math'>
            <mrow id='mrow'>
                <mfrac id='frac'>
                    <mrow id='num'><mi id='a'>a</mi><mo id='plus'>+</mo><mn id='1'>1</mn></mrow>
                    <mrow id='denom'><mn id='2'>2</mn><mo id='invisible-times'>&#x2062;</mo><mi id='b'>b</mi></mrow>
                </mfrac>
                <mo id='minus'>-</mo>
                <mn id='3'>3</mn>
            </mrow>
        </math>";
        init_default_prefs(mathml_str, "Enhanced");
        set_preference("AutoZoomOut", "False")?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&package_instance);
            test_command("ZoomIn", mathml, "frac");
            test_command("ZoomIn", mathml, "num");
            test_command("MoveNext", mathml, "denom");
            test_command("MoveNext", mathml, "denom");
            test_command("MovePrevious", mathml, "num");
            test_command("MovePrevious", mathml, "num");
            test_command("ZoomOut", mathml, "frac");
            test_command("MoveNext", mathml, "minus");
            return Ok( () );
        });
    }

    #[test]
    fn basic_language_test() -> Result<()> {
        // this is basically a sanity check that all the language's navigation.yaml files are at least syntactically correct
        // FIX: should look through the Languages dir and figure this is out
        let mathml_str = "<math id='math'>
                <mrow id='contents'>
                    <mrow id='lhs'>
                        <mrow id='term'>
                            <mn id='2'>2</mn>
                            <mo id='invisible-times'>&#x2062;</mo>
                            <msup id='msup'>
                                <mi id='x'>x</mi>
                                <mn id='3'>3</mn>
                            </msup>
                        </mrow>
                        <mo id='plus'>+</mo>
                        <mn id='1'>1</mn>
                    </mrow>
                <mo id='id-11'>=</mo>
                <mi id='id-12'>y</mi>
                </mrow>
            </math>";

        set_rules_dir(abs_rules_dir_path()).unwrap();
        for lang in get_supported_languages().unwrap_or_default() {
            test_language(&lang, mathml_str);
        }
        return Ok( () );

        fn test_language(lang: &str, mathml_str: &str) {
            init_default_prefs(mathml_str, "Enhanced");
            set_preference("Language", lang).unwrap();

            set_preference("NavMode", "Enhanced").unwrap();
            MATHML_INSTANCE.with(|package_instance| {
                let package_instance = package_instance.borrow();
                let mathml = get_element(&package_instance);
                test_command("ZoomInAll", mathml, "2");
                test_command("MoveNext", mathml, "msup");
                test_command("MoveNext", mathml, "plus");
                test_command("MovePrevious", mathml, "term");
                test_command("MovePrevious", mathml, "term");
                test_command("ZoomOutAll", mathml, "contents");
            });

            set_preference("NavMode", "Simple").unwrap();
            MATHML_INSTANCE.with(|package_instance: &RefCell<Package>| {
                let package_instance = package_instance.borrow();
                let mathml = get_element(&package_instance);
                test_command("ZoomInAll", mathml, "2");
                test_command("MoveNext", mathml, "msup");
                test_command("MoveNext", mathml, "plus");
                test_command("MovePrevious", mathml, "msup");
                test_command("MovePrevious", mathml, "2");
                test_command("MovePrevious", mathml, "2");
                test_command("ZoomOutAll", mathml, "contents");
            });

            set_preference("NavMode", "Character").unwrap();
            MATHML_INSTANCE.with(|package_instance| {
                let package_instance = package_instance.borrow();
                let mathml = get_element(&package_instance);
                test_command("ZoomIn", mathml, "2");
                test_command("MoveNext", mathml, "x");
                test_command("MoveNext", mathml, "3");
                test_command("MoveNext", mathml, "plus");
                test_command("MovePrevious", mathml, "3");
                test_command("MovePrevious", mathml, "x");
                test_command("MovePrevious", mathml, "2");
                test_command("MovePrevious", mathml, "2");
            });

            // simple sanity check that "overview.yaml" doesn't have a syntax error
            set_preference("Overview", "True").unwrap();
            set_preference("NavMode", "Character").unwrap();
            MATHML_INSTANCE.with(|package_instance| {
                let package_instance = package_instance.borrow();
                let mathml = get_element(&package_instance);
                test_command("ZoomIn", mathml, "2");
            });
        }
    }
}
