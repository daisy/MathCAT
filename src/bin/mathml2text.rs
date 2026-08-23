// *** MathCAT doesn't normally want to build a binary ***
// *** This file is here because it is useful for trying out things ***
#![allow(clippy::needless_return)]

use libmathcat::{errors::*, interface::*};
use log::*;
use std::path::PathBuf;
use clap::{Parser, ValueEnum};

// Maybe also have this speak to test the TTS generation.
// There is a rust winapi crate that mirrors the WinPAI and has "Speak(...)" in it

// env RUST_LOG=DEBUG cargo run --features "include-zip"
fn get_rules_dir() -> String {
    // for testing with zipped rules dir
    // let rules_path = std::env::current_exe().unwrap().parent().unwrap().join("../../../MathCATForPython/addon/globalPlugins/MathCAT/Rules");
    let rules_path = std::env::current_exe().unwrap().parent().unwrap().join("../../Rules");
    return rules_path.as_os_str().to_str().unwrap().to_string();
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputType {
    Text,
    Braille,
    #[cfg(feature="tts")]
    Speech,
}

#[derive(Parser)]
#[command(version, about)]
struct Options {
    #[arg(short, long)]
    rules_dir: Option<PathBuf>,

    input_file: Option<PathBuf>,

    #[arg(short, long, default_value="en")]
    language: String,

    #[arg(value_enum, long, default_value="text")]
    output: OutputType,
}


fn main() -> Result<()> {
    env_logger::builder()
      .format_timestamp(None)
      .format_module_path(false)
      .format_indent(Some(2))
      .format_level(false)
      .init();

    let cli = Options::parse();

    let expr = if let Some(f) = cli.input_file {
	std::fs::read_to_string(&f).with_context(|| format!("unable to open {}", f.to_str().unwrap_or_default()))?
    } else {
        r#"
            <math xmlns="http://www.w3.org/1998/Math/MathML"><mo>(</mo><mn>1</mn><mo>)</mo></math>
		"#.to_string()
    };

    if let Err(e) = set_rules_dir(get_rules_dir()) {
	panic!("Error: exiting -- {}", errors_to_string(&e));
    }
    debug!("Languages: {}", libmathcat::interface::get_supported_languages()?.join(", "));

    #[cfg(feature = "include-zip")]
    info!("***********include-zip is present**********");
    info!("Version = '{}' using Rules dir {}", get_version(), get_rules_dir());
    set_preference("Language", cli.language)?;

    set_preference("DecimalSeparator", "Auto").unwrap();
    set_preference("BrailleCode", "Nemeth").unwrap();
    set_preference("TTS", "None").unwrap();
    set_preference("Verbosity", "Verbose").unwrap();
    set_preference("NavVerbosity", "Verbose").unwrap();
    set_preference("NavMode", "Enhanced").unwrap();
    set_preference("Impairment", "Blindness").unwrap();
    set_preference("SpeechOverrides_CapitalLetters", "").unwrap();
    set_preference("MathRate", "80").unwrap();
    set_preference("CapitalLetters_Beep", "true").unwrap();
    set_preference("IntentErrorRecovery", "Error").unwrap();

    set_preference("Bookmark", "false").unwrap();
    set_preference("SpeechStyle", "ClearSpeak").unwrap();
    info!("Languages: {}", libmathcat::interface::get_supported_languages()?.join(", "));
    info!("Speech styles: {}", libmathcat::interface::get_supported_speech_styles("ClearSpeak")?.join(", "));
    info!("BrailleCodes: {}", libmathcat::interface::get_supported_braille_codes()?.join(", "));

    debug!("Speech language is {}", get_preference("Language").unwrap());
    debug!("DecimalSeparator: {:?}", get_preference("DecimalSeparator").unwrap());
    debug!("DecimalSeparators: {:?}, BlockSeparators: {:?}", get_preference("DecimalSeparators").unwrap(), get_preference("BlockSeparators").unwrap());
    debug!("SpeechStyle: {:?}", get_preference("SpeechStyle").unwrap());
    debug!("Verbosity: {:?}", get_preference("Verbosity").unwrap());

    match set_mathml(&expr) {
	Err(e) => {
	    panic!("Error: exiting -- {}", errors_to_string(&e));
	},
	Ok(fmt) => {
	    info!("formatted input mathml into {fmt}");
	}
    }

    match cli.output {
	OutputType::Text => {
	    match get_spoken_text() {
		Ok(speech) => println!("{speech}"),
		Err(e) => panic!("{}", errors_to_string(&e)),
	    }
	},
	OutputType::Braille => {
	    debug!("...using BrailleCode: {:?}", get_preference("BrailleCode").unwrap());
	    match get_braille("") {
		Ok(braille) => println!("{braille}"),
		Err(e) => panic!("{}", errors_to_string(&e)),
	    }
	},
	#[cfg(feature="tts")]
	OutputType::Speech => {
	    // Create the NaturalTts struct using the builder pattern.
	    let mut natural = natural_tts::NaturalTtsBuilder::default()
		.gtts_model(natural_tts::models::gtts::GttsModel::default())
		.default_model(natural_tts::Model::Gtts)
		.build().expect("failed to generate natural tts gtts model");


	    // Start producing an output using the default_model.
	    let _ = natural.start(get_spoken_text().unwrap(), &PathBuf::from("output.wav"));

	    // Play the audio until it finishes
	    natural.sleep_until_end();
	}
    }

    Ok(())
}
