#![no_main]

use libfuzzer_sys::fuzz_target;
use std::path::Path;
use std::sync::Once;

static INIT_RULES: Once = Once::new();

fn rules_dir() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../Rules")
        .to_string_lossy()
        .into_owned()
}

/// Must match `NAV_COMMANDS` in `src/navigate.rs` (and the `do_navigate_command` docs in
/// `src/interface.rs`).
const NAV_COMMANDS: &[&str] = &[
    "MovePrevious",
    "MoveNext",
    "MoveStart",
    "MoveEnd",
    "MoveLineStart",
    "MoveLineEnd",
    "MoveCellPrevious",
    "MoveCellNext",
    "MoveCellUp",
    "MoveCellDown",
    "MoveColumnStart",
    "MoveColumnEnd",
    "ZoomIn",
    "ZoomOut",
    "ZoomOutAll",
    "ZoomInAll",
    "MoveLastLocation",
    "ReadPrevious",
    "ReadNext",
    "ReadCurrent",
    "ReadCellCurrent",
    "ReadStart",
    "ReadEnd",
    "ReadLineStart",
    "ReadLineEnd",
    "DescribePrevious",
    "DescribeNext",
    "DescribeCurrent",
    "WhereAmI",
    "WhereAmIAll",
    "ToggleZoomLockUp",
    "ToggleZoomLockDown",
    "ToggleSpeakMode",
    "Exit",
    "MoveTo0",
    "MoveTo1",
    "MoveTo2",
    "MoveTo3",
    "MoveTo4",
    "MoveTo5",
    "MoveTo6",
    "MoveTo7",
    "MoveTo8",
    "MoveTo9",
    "Read0",
    "Read1",
    "Read2",
    "Read3",
    "Read4",
    "Read5",
    "Read6",
    "Read7",
    "Read8",
    "Read9",
    "Describe0",
    "Describe1",
    "Describe2",
    "Describe3",
    "Describe4",
    "Describe5",
    "Describe6",
    "Describe7",
    "Describe8",
    "Describe9",
    "SetPlacemarker0",
    "SetPlacemarker1",
    "SetPlacemarker2",
    "SetPlacemarker3",
    "SetPlacemarker4",
    "SetPlacemarker5",
    "SetPlacemarker6",
    "SetPlacemarker7",
    "SetPlacemarker8",
    "SetPlacemarker9",
];

fuzz_target!(|data: &[u8]| {
    INIT_RULES.call_once(|| {
        libmathcat::set_rules_dir(rules_dir()).expect("set_rules_dir must succeed for fuzzing");
    });

    let s = String::from_utf8_lossy(data);
    if libmathcat::set_mathml(s.as_ref()).is_err() {
        return;
    }

    let _ = libmathcat::get_spoken_text();

    // Highlight id: empty (typical) and a lossy tail slice to stress id parsing.
    let _ = libmathcat::get_braille("");
    if !data.is_empty() {
        let tail_len = data.len().min(32);
        let tail = &data[data.len() - tail_len..];
        let id = String::from_utf8_lossy(tail);
        let _ = libmathcat::get_braille(id.as_ref());
    }

    // Drive navigation with valid commands only (indices from input bytes).
    const MAX_NAV_STEPS: usize = 96;
    for &b in data.iter().take(MAX_NAV_STEPS) {
        let cmd = NAV_COMMANDS[b as usize % NAV_COMMANDS.len()];
        let _ = libmathcat::do_navigate_command(cmd);
    }

    let _ = libmathcat::get_spoken_text();
    let _ = libmathcat::get_braille("");
});
