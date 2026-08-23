#![no_main]

use libfuzzer_sys::fuzz_target;
use std::path::Path;
use std::sync::OnceLock;
use yaml_rust::{Yaml, YamlLoader};

struct FuzzState {
    braille_codes: Vec<String>,
    /// Languages from `get_supported_languages` other than `"en"`.
    non_en_languages: Vec<String>,
    /// Flattened prefs from `Rules/prefs.yaml` (same key shape as `PreferenceManager` / `set_preference`),
    /// each with a small list of candidate string values for fuzzing.
    pref_choices: Vec<(String, Vec<String>)>,
}

static FUZZ_STATE: OnceLock<FuzzState> = OnceLock::new();

fn rules_dir() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../Rules")
        .to_string_lossy()
        .into_owned()
}

fn yaml_scalar_to_string(y: &Yaml) -> Option<String> {
    match y {
        Yaml::String(s) => Some(s.clone()),
        Yaml::Boolean(b) => Some(b.to_string()),
        Yaml::Integer(i) => Some(i.to_string()),
        Yaml::Real(r) => Some(r.clone()),
        _ => None,
    }
}

/// Walk one top-level prefs section (`Speech`, `Navigation`, …) the same way `prefs.rs` `add_prefs` does.
fn walk_prefs_section(section: &Yaml, prefix: &str, out: &mut Vec<(String, String)>) {
    let Some(h) = section.as_hash() else {
        return;
    };
    for (k, v) in h {
        let Some(name) = k.as_str().map(str::trim) else {
            continue;
        };
        if name.is_empty() {
            continue;
        }
        let key_so_far = format!("{prefix}{name}");
        match v {
            Yaml::Hash(_) => walk_prefs_section(v, &format!("{key_so_far}_"), out),
            Yaml::String(_) | Yaml::Boolean(_) | Yaml::Integer(_) | Yaml::Real(_) => {
                if let Some(val) = yaml_scalar_to_string(v) {
                    out.push((key_so_far, val));
                }
            }
            _ => {}
        }
    }
}

fn load_prefs_yaml_flat(rules_dir: &str) -> Vec<(String, String)> {
    let path = Path::new(rules_dir).join("prefs.yaml");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let docs = YamlLoader::load_from_str(&text).expect("prefs.yaml must parse");
    assert_eq!(docs.len(), 1, "prefs.yaml must contain a single YAML document");
    let doc = &docs[0];
    let mut out = Vec::new();
    for section in ["Speech", "Navigation", "Braille", "Other"] {
        let node = &doc[section];
        if node.is_badvalue() {
            panic!("prefs.yaml: missing top-level key '{section}'");
        }
        walk_prefs_section(node, "", &mut out);
    }
    out
}

fn dedupe_sorted_strings(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v.dedup();
    v
}

/// Legal-value hints from `Rules/prefs.yaml` comments; keys not listed keep the YAML default only.
fn value_choices_for_key(key: &str, default: &str) -> Vec<String> {
    let mut v = match key {
        "Impairment" => vec!["Blindness", "LearningDisability", "LowVision"],
        "Verbosity" => vec!["Terse", "Medium", "Verbose"],
        "SpeechStyle" => vec!["ClearSpeak", "SimpleSpeak"],
        "IgnoreBold" => vec!["true", "false"],
        "MathRate" | "PauseFactor" => vec!["50", "100", "150"],
        "SpeechSound" => vec!["None", "Beep"],
        "SubjectArea" => vec!["General"],
        "Chemistry" => vec!["SpellOut", "AsCompound", "Off"],
        "MathSpeak" => vec!["Verbose", "Brief", "SuperBrief"],
        "NavMode" => vec!["Enhanced", "Simple", "Character"],
        "ResetNavMode" | "Overview" | "ResetOverview" | "AutoZoomOut" | "UseSpacesAroundAllOperators" => {
            vec!["true", "false"]
        }
        "NavVerbosity" => vec!["Terse", "Medium", "Full"],
        "CopyAs" => vec!["MathML", "LaTeX", "ASCIIMath"],
        "BrailleNavHighlight" => vec!["Off", "FirstChar", "EndPoints", "All"],
        "UEB_StartMode" => vec!["Grade1", "Grade2"],
        "LaTeX_UseShortName" | "Vietnam_UseDropNumbers" => vec!["true", "false"],
        "DecimalSeparator" => vec!["Auto", ".", ","],
        _ if key.starts_with("ClearSpeak_") => vec![
            "Auto", "Verbose", "Ordinal", "Over", "None", "Speak", "TrigInverse", "AbsEnd",
        ],
        _ => vec![default],
    }
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    if !v.iter().any(|s| s == default) {
        v.push(default.to_string());
    }
    dedupe_sorted_strings(v)
}

fn expand_pref_choices(
    flat: Vec<(String, String)>,
    braille_codes: &[String],
    non_en_languages: &[String],
) -> Vec<(String, Vec<String>)> {
    let mut out = Vec::with_capacity(flat.len());
    for (key, default) in flat {
        let choices = if key == "Language" {
            let mut v = vec!["Auto".to_string(), "en".to_string(), default.clone()];
            v.extend(non_en_languages.iter().cloned());
            dedupe_sorted_strings(v)
        } else if key == "BrailleCode" {
            braille_codes.to_vec()
        } else {
            value_choices_for_key(&key, &default)
        };
        out.push((key, choices));
    }
    out
}

/// Apply a small deterministic subset of preferences so each corpus input explores different combinations.
fn apply_random_prefs(data: &[u8], state: &FuzzState, other_lang: &str) {
    let choices = &state.pref_choices;
    if choices.is_empty() {
        return;
    }
    // 2–13 prefs touched per input (keeps runs fast).
    let count = 2 + (data.first().copied().unwrap_or(0) as usize % 11);
    let count = count.min(choices.len()).max(1);
    let mut i = 1usize;
    for step in 0..count {
        let idx = (data.get(i).copied().unwrap_or(0) as usize)
            .wrapping_add(step.wrapping_mul(17))
            % choices.len();
        i = i.wrapping_add(1);
        let (key, vals) = &choices[idx];
        if vals.is_empty() {
            continue;
        }
        let v_idx = data.get(i).copied().unwrap_or(0) as usize % vals.len();
        i = i.wrapping_add(1);
        let value = if key == "Language" {
            let mode = data.get(i).copied().unwrap_or(0) % 4;
            i = i.wrapping_add(1);
            match mode {
                0 => other_lang.to_string(),
                1 => "en".to_string(),
                2 => "Auto".to_string(),
                _ => vals[v_idx].clone(),
            }
        } else if key == "BrailleCode" {
            let mode = data.get(i).copied().unwrap_or(0) % 2;
            i = i.wrapping_add(1);
            if mode == 0 && !state.braille_codes.is_empty() {
                let j = data.get(i).copied().unwrap_or(0) as usize % state.braille_codes.len();
                i = i.wrapping_add(1);
                state.braille_codes[j].clone()
            } else {
                vals[v_idx].clone()
            }
        } else {
            vals[v_idx].clone()
        };
        let _ = libmathcat::set_preference(key, &value);
    }
}

fn fuzz_state() -> &'static FuzzState {
    FUZZ_STATE.get_or_init(|| {
        let rd = rules_dir();
        libmathcat::set_rules_dir(&rd).expect("set_rules_dir must succeed for fuzzing");
        let braille_codes = libmathcat::get_supported_braille_codes()
            .expect("get_supported_braille_codes must succeed after set_rules_dir");
        assert!(
            !braille_codes.is_empty(),
            "no braille tables under Rules/Braille — nothing to fuzz for braille"
        );

        let all_langs = libmathcat::get_supported_languages()
            .expect("get_supported_languages must succeed after set_rules_dir");
        let non_en_languages: Vec<String> = all_langs.into_iter().filter(|l| l != "en").collect();
        assert!(
            !non_en_languages.is_empty(),
            "need at least one non-\"en\" language under Rules/Languages for two-language fuzzing"
        );

        let flat = load_prefs_yaml_flat(&rd);
        let pref_choices = expand_pref_choices(flat, &braille_codes, &non_en_languages);

        FuzzState {
            braille_codes,
            non_en_languages,
            pref_choices,
        }
    })
}

fn exercise_speech(lang: &str) {
    if libmathcat::set_preference("Language", lang).is_err() {
        return;
    }
    let _ = libmathcat::get_spoken_text();
}

/// Braille tables are exercised independently of speech language; use a fixed language.
/// Two corpus-selected codes per input (distinct when more than one table exists).
fn exercise_two_braille_codes(braille_codes: &[String], data: &[u8], tail_id: &Option<String>) {
    let _ = libmathcat::set_preference("Language", "en");
    let n = braille_codes.len();
    if n == 0 {
        return;
    }
    let i1 = data.get(1).copied().unwrap_or(0) as usize % n;
    let i2 = if n == 1 {
        0
    } else {
        let mut j = data.get(2).copied().unwrap_or(0) as usize % n;
        if j == i1 {
            j = (i1 + 1) % n;
        }
        j
    };
    for idx in [i1, i2] {
        let code = &braille_codes[idx];
        if libmathcat::set_preference("BrailleCode", code).is_err() {
            continue;
        }
        let _ = libmathcat::get_braille("");
        if let Some(id) = tail_id {
            let _ = libmathcat::get_braille(id.as_str());
        }
    }
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

const MAX_NAV_STEPS: usize = 20;

/// Map `step` and `data` to a nav command index (deterministic; spreads reads across the buffer).
fn nav_command_index(data: &[u8], step: usize) -> usize {
    let n = NAV_COMMANDS.len();
    if data.is_empty() {
        return step % n;
    }
    let len = data.len();
    let p = (step.wrapping_mul(37).wrapping_add(11)) % len;
    let q = (step.wrapping_mul(19).wrapping_add(len / 2)) % len;
    let mix = (data[p] as usize) ^ (data[q] as usize) ^ step.wrapping_mul(17);
    mix % n
}

fuzz_target!(|data: &[u8]| {
    let state = fuzz_state();

    // Deterministic “random” non-English language from the corpus byte stream.
    let other_lang = {
        let i = data.first().copied().unwrap_or(0) as usize;
        state.non_en_languages[i % state.non_en_languages.len()].as_str()
    };

    apply_random_prefs(data, state, other_lang);

    let s = String::from_utf8_lossy(data);
    if libmathcat::set_mathml(s.as_ref()).is_err() {
        return;
    }

    let tail_id = if !data.is_empty() {
        let tail_len = data.len().min(32);
        let tail = &data[data.len() - tail_len..];
        Some(String::from_utf8_lossy(tail).into_owned())
    } else {
        None
    };

    exercise_speech("en");
    exercise_speech(other_lang);
    exercise_two_braille_codes(state.braille_codes.as_slice(), data, &tail_id);

    // Navigation depends only on the MathML and the command sequence, not on BrailleCode or speech prefs.
    for step in 0..MAX_NAV_STEPS {
        let cmd = NAV_COMMANDS[nav_command_index(data, step)];
        let _ = libmathcat::do_navigate_command(cmd);
    }
});
