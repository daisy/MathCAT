//! Reverse UEB braille → print character map, built lazily from
//! `Rules/Braille/UEB/unicode.yaml` and `unicode-full.yaml` the first time
//! UEB→MathML parsing needs symbols.
//!
//! Includes single-character entries and ranged `translate(...)` alphabets.
//! Flag letters in YAML templates are expanded the same way as UEB cleanup
//! (`UEB_INDICATOR_REPLACEMENTS` + prefs for `𝔹` / `S` / `D` / `V`).
//! ASCII a–z/A–Z/0–9 and bare Latin letter cells are omitted so the
//! mode-aware letter/number lexer keeps those. Homophones prefer canonical
//! Greek / plane-0 blackboard / punctuation over lookalike math alphanumerics.

#![allow(clippy::needless_return)]

use crate::errors::Result;
use crate::prefs::{PreferenceManager, NO_PREFERENCE};
use crate::shim_filesystem::read_to_string_shim;
use anyhow::Context;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use yaml_rust::{Yaml, YamlLoader};

type SymbolTableLoad = std::result::Result<Vec<(String, String)>, String>;

thread_local! {
    /// Longest-first (braille cells, print character). `None` until first use.
    static UEB_SYMBOLS: RefCell<Option<SymbolTableLoad>> = const { RefCell::new(None) };
}

/// Ensure the UEB reverse symbol table is loaded (reads YAML on first call).
pub fn ensure_ueb_symbols_loaded() -> Result<()> {
    UEB_SYMBOLS.with(|cell| {
        if cell.borrow().is_none() {
            *cell.borrow_mut() = Some(load_ueb_symbols().map_err(|e| format!("{e:#}")));
        }
        match cell.borrow().as_ref().unwrap() {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("UEB symbol table failed to load: {e}")),
        }
    })
}

/// Longest-match lookup of a UEB symbol at the start of `input`.
/// Returns `(braille_prefix, print_char)`. Loads YAML on first use.
pub fn match_ueb_symbol(input: &str) -> Option<(String, String)> {
    let _ = ensure_ueb_symbols_loaded();
    UEB_SYMBOLS.with(|cell| {
        let table = cell.borrow();
        let Ok(table) = table.as_ref().unwrap() else {
            return None;
        };
        for (braille, print) in table.iter() {
            if input.starts_with(braille.as_str()) {
                return Some((braille.clone(), print.clone()));
            }
        }
        None
    })
}

/// Drop the cached table so the next parse reloads YAML (e.g. after prefs change).
pub fn clear_ueb_symbol_cache() {
    UEB_SYMBOLS.with(|cell| *cell.borrow_mut() = None);
}

/// Number of symbols currently cached (0 if not loaded). Test/debug helper.
#[cfg(test)]
fn cached_symbol_count() -> usize {
    UEB_SYMBOLS.with(|cell| {
        cell.borrow()
            .as_ref()
            .and_then(|r| r.as_ref().ok())
            .map(|v| v.len())
            .unwrap_or(0)
    })
}

fn load_ueb_symbols() -> Result<Vec<(String, String)>> {
    let rules_dir = rules_dir_for_ueb()?;
    let unicode_path = rules_dir.join("Braille/UEB/unicode.yaml");
    let full_path = rules_dir.join("Braille/UEB/unicode-full.yaml");

    let typeforms = TypeformPrefs::current();
    let unicode_text = read_to_string_shim(&unicode_path)
        .with_context(|| format!("cannot read '{}'", unicode_path.display()))?;
    let full_text = read_to_string_shim(&full_path)
        .with_context(|| format!("cannot read '{}'", full_path.display()))?;

    let full_pairs = collect_unicode_file(&full_text)?;
    let short_pairs = collect_unicode_file(&unicode_text)?;

    let mut braille_to_print: HashMap<String, String> = HashMap::new();

    // unicode-full first (weaker); unicode.yaml last-wins on braille collisions.
    insert_pairs(&mut braille_to_print, &full_pairs, &typeforms, false);
    insert_pairs(&mut braille_to_print, &short_pairs, &typeforms, true);

    // When G1 is already in effect, templates that bake in ⠰ / ⠰⠰ also match without it.
    let extras: Vec<(String, String)> = braille_to_print
        .iter()
        .filter_map(|(braille, ch)| {
            if braille.starts_with("⠰⠰") && braille.chars().count() > 2 {
                let stripped: String = braille.chars().skip(2).collect();
                Some((stripped, ch.clone()))
            } else if braille.starts_with('⠰') && braille.chars().count() > 1 {
                let stripped: String = braille.chars().skip(1).collect();
                Some((stripped, ch.clone()))
            } else {
                None
            }
        })
        .collect();
    for (braille, ch) in extras {
        // Don't let stripped forms steal bare letter cells or overwrite stronger entries
        if should_skip_mapping(&ch, &braille) {
            continue;
        }
        // Stripped G1 forms of ⠔… / ⠢… collide with grade-2 "in"/"en" (Sin30 vs S³…).
        if matches!(braille.chars().next(), Some('⠔' | '⠢')) {
            continue;
        }
        if let Some(prev) = braille_to_print.get(&braille) {
            if should_replace_print(prev, &ch) {
                braille_to_print.insert(braille, ch);
            }
        } else {
            braille_to_print.insert(braille, ch);
        }
    }

    let mut items: Vec<(String, String)> = braille_to_print.into_iter().collect();
    items.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(&b.0)));
    Ok(items)
}

fn insert_pairs(
    map: &mut HashMap<String, String>,
    pairs: &[(String, String)],
    typeforms: &TypeformPrefs,
    overwrite: bool,
) {
    for (ch, raw) in pairs {
        if should_skip_print_char(ch) {
            continue;
        }
        let braille = expand_flags(raw, typeforms);
        if should_skip_mapping(ch, &braille) {
            continue;
        }
        if overwrite {
            if let Some(prev) = map.get(&braille)
                && !should_replace_print(prev, ch) {
                    continue;
                }
            map.insert(braille, ch.clone());
        } else if let Some(prev) = map.get(&braille) {
            if should_replace_print(prev, ch) {
                map.insert(braille, ch.clone());
            }
        } else {
            map.insert(braille, ch.clone());
        }
    }
}

/// When several print chars share a braille sequence, prefer the form that is
/// most useful for reverse math parsing (canonical Greek, plane-0 blackboard,
/// punctuation/operators over lookalike Mathematical Alphanumeric Symbols).
fn should_replace_print(existing: &str, new: &str) -> bool {
    preference_rank(new) >= preference_rank(existing)
}

fn preference_rank(s: &str) -> i32 {
    let Some(c) = s.chars().next() else {
        return -1;
    };
    let o = c as u32;
    // Unassigned double-struck slots in the math alphanumeric block
    if matches!(
        o,
        0x1D53A | 0x1D53F | 0x1D545 | 0x1D547 | 0x1D548 | 0x1D549
    ) {
        return -1;
    }
    if is_greek_variant_form(c) {
        return 0;
    }
    if c == 'ς' {
        return 1;
    }
    // Mathematical Alphanumeric Symbols — weak; collide with Greek / typeforms
    if (0x1D400..=0x1D7FF).contains(&o) {
        return 2;
    }
    if c.is_alphabetic() && !(0x0370..=0x03FF).contains(&o) {
        return 3;
    }
    // Punctuation / operators / symbols (e.g. §)
    if !c.is_alphabetic() {
        // Prefer BMP chemistry/equilibrium arrows over Supplemental lookalikes
        // (🣑 shares UEB with ⇌).
        if (0x1F800..=0x1F8FF).contains(&o) {
            return 3;
        }
        // Prefer WHITE SQUARE □ over WHITE MEDIUM SQUARE ◻ (same UEB shape).
        if c == '□' {
            return 5;
        }
        if matches!(c, '◻' | '▫' | '◽') {
            return 2;
        }
        // Prefer ASCII colon over ratio ∶ (time 5:30 vs proportion).
        if c == ':' {
            return 5;
        }
        if c == '∶' {
            return 2;
        }
        // Prefer ASCII '.' over dot-above ˙ (same bare ⠲ after G1 strip).
        if c == '.' {
            return 5;
        }
        if c == '˙' {
            return 2;
        }
        // Prefer n-ary ∑ over Greek Σ (same Cap+Greek s braille).
        if c == '∑' {
            return 6;
        }
        if c == 'Σ' {
            return 3;
        }
        // Prefer · over ⋅, ∗ over *, ⊢ over ⊦.
        if matches!(c, '·' | '∗' | '⊢') {
            return 5;
        }
        if matches!(c, '⋅' | '*' | '⊦') {
            return 2;
        }
        return 4;
    }
    // Canonical Greek letters
    if (0x0370..=0x03FF).contains(&o) {
        return 5;
    }
    // Plane-0 blackboard set letters
    if "ℂℍℕℙℚℝℤ𝕆".contains(c) {
        return 6;
    }
    3
}

fn is_greek_variant_form(c: char) -> bool {
    matches!(
        c,
        'ϑ' | 'ϵ' | 'ϰ' | 'ϕ' | 'ϱ' | 'ϖ' | 'Ϝ' | 'µ' | 'Ω' | '∆'
    )
}

fn should_skip_print_char(ch: &str) -> bool {
    let mut chars = ch.chars();
    let Some(c) = chars.next() else {
        return true;
    };
    if chars.next().is_some() {
        return true; // only single-codepoint print chars
    }
    // ASCII letters/digits are owned by the mode-aware letter/number lexer.
    if c.is_ascii_alphabetic() || c.is_ascii_digit() {
        return true;
    }
    // Prefer U+2212 (−) for ⠐⠤ over ASCII hyphen-minus.
    if c == '-' {
        return true;
    }
    false
}

fn should_skip_mapping(ch: &str, braille: &str) -> bool {
    if braille.is_empty() || !braille.chars().any(is_braille) {
        return true;
    }
    // Bare Latin letter cells (no typeform / numeric / Greek indicators) would steal
    // a–z from the letter lexer (e.g. ⅆ → ⠙).
    if is_only_latin_letter_cells(braille) {
        return true;
    }
    let _ = ch;
    false
}

fn is_only_latin_letter_cells(braille: &str) -> bool {
    let mut any = false;
    for c in braille.chars() {
        if !is_braille(c) {
            return false;
        }
        if !is_latin_letter_cell(c) {
            return false;
        }
        any = true;
    }
    any
}

fn is_latin_letter_cell(c: char) -> bool {
    matches!(
        c,
        '⠁' | '⠃' | '⠉' | '⠙' | '⠑' | '⠋' | '⠛' | '⠓' | '⠊' | '⠚' | '⠅' | '⠇'
            | '⠍' | '⠝' | '⠕' | '⠏' | '⠟' | '⠗' | '⠎' | '⠞' | '⠥' | '⠧' | '⠺' | '⠭'
            | '⠽' | '⠵'
    )
}

fn rules_dir_for_ueb() -> Result<PathBuf> {
    let pm = PreferenceManager::get();
    let pm = pm.borrow();
    let dir = pm.get_rules_dir();
    if !dir.as_os_str().is_empty() {
        return Ok(dir);
    }
    Ok(PathBuf::from(crate::abs_rules_dir_path()))
}

struct TypeformPrefs {
    double_struck: String,
    sans_serif: String,
    fraktur: String,
    greek_variant: String,
}

impl TypeformPrefs {
    fn current() -> Self {
        let defaults = Self {
            double_struck: "⠈".to_string(),
            sans_serif: "⠈⠼".to_string(),
            fraktur: "⠈".to_string(),
            greek_variant: "⠨".to_string(),
        };
        let pm = PreferenceManager::get();
        let pm = pm.borrow();
        if pm.get_rules_dir().as_os_str().is_empty() {
            return defaults;
        }
        let pick = |key: &str, fallback: &str| {
            let v = pm.pref_to_string(key);
            if v.is_empty() || v == NO_PREFERENCE {
                fallback.to_string()
            } else {
                v
            }
        };
        Self {
            double_struck: pick("UEB_DoubleStruck", &defaults.double_struck),
            sans_serif: pick("UEB_SansSerif", &defaults.sans_serif),
            fraktur: pick("UEB_Fraktur", &defaults.fraktur),
            greek_variant: pick("UEB_GreekVariant", &defaults.greek_variant),
        }
    }
}

fn is_braille(ch: char) -> bool {
    ('\u{2800}'..='\u{28FF}').contains(&ch)
}

/// Expand YAML flag letters to braille cells (UEB defaults + typeform prefs).
fn expand_flags(raw: &str, tf: &TypeformPrefs) -> String {
    let mut out = String::new();
    for ch in raw.chars() {
        let piece: &str = match ch {
            'S' => tf.sans_serif.as_str(),
            'B' => "⠘",
            '𝔹' => tf.double_struck.as_str(),
            'T' => "⠈",
            'I' => "⠨",
            'R' => "",
            '1' => "⠰",
            '𝟙' => "⠰⠰",
            'L' | 'A' | 'o' | 'c' | 'b' | '#' => "",
            'D' => tf.fraktur.as_str(),
            'G' => "⠨",
            'V' => tf.greek_variant.as_str(),
            'C' | '𝐶' | '𝑐' => "⠠",
            'N' => "⠼",
            't' => "⠱",
            'W' | '𝐖' => "⠀",
            's' => "⠆",
            'w' => "⠂",
            'e' => "⠄",
            ',' => "⠂",
            '.' => "⠲",
            '-' => "⠤",
            '—' => "⠠⠤",
            '―' => "⠐⠠⠤",
            c if is_braille(c) => {
                out.push(c);
                continue;
            }
            c if (c as u32) < 128 => continue, // other ASCII meta
            _ => continue,
        };
        out.push_str(piece);
    }
    out
}

/// Collect (print_char, raw_template) from a UEB unicode YAML file.
/// Includes single-character entries and ranged `translate(...)` alphabets.
fn collect_unicode_file(text: &str) -> Result<Vec<(String, String)>> {
    let docs = YamlLoader::load_from_str(text).map_err(|e| anyhow::anyhow!("YAML parse: {e}"))?;
    let doc = docs
        .first()
        .ok_or_else(|| anyhow::anyhow!("empty unicode YAML"))?;
    let list = doc
        .as_vec()
        .ok_or_else(|| anyhow::anyhow!("unicode YAML root is not an array"))?;

    let mut pairs = Vec::new();
    let translate_re =
        Regex::new(r"translate\('\.',\s*'([^']+)',\s*'([a-zA-Z0-9]+)'\)").unwrap();

    for entry in list {
        let Some(hash) = entry.as_hash() else {
            continue;
        };
        if hash.len() != 1 {
            continue;
        }
        let (key, value) = hash.iter().next().unwrap();
        let Some(key_str) = key.as_str() else {
            continue;
        };

        let mut key_chars = key_str.chars();
        let Some(first) = key_chars.next() else {
            continue;
        };

        // Single character definition
        if key_chars.next().is_none() {
            collect_text_templates(value, &first.to_string(), &mut pairs);
            continue;
        }

        // Range / multi-char key — expand via translate() + t/tc prefix when present
        let prefix = first_text_prefix(value).unwrap_or_default();
        let value_str = yaml_to_flat_string(value);
        for caps in translate_re.captures_iter(&value_str) {
            let glyphs = caps.get(1).unwrap().as_str();
            let letters = caps.get(2).unwrap().as_str();
            for (g, letter) in glyphs.chars().zip(letters.chars()) {
                let raw = raw_template_for_translated_glyph(g, letter, &prefix);
                if !raw.is_empty() {
                    pairs.push((g.to_string(), raw));
                }
            }
        }
    }
    Ok(pairs)
}

/// First `t:` / `tc:` string under a node (used as alphabet prefix for ranges).
fn first_text_prefix(yaml: &Yaml) -> Option<String> {
    match yaml {
        Yaml::Array(arr) => arr.iter().find_map(first_text_prefix),
        Yaml::Hash(h) => {
            for (k, v) in h {
                if let Some(key) = k.as_str()
                    && matches!(key, "t" | "T" | "tc" | "TC" | "ct" | "CT")
                        && let Some(s) = v.as_str()
                            && !s.trim().is_empty() {
                                return Some(s.to_string());
                            }
            }
            h.values().find_map(first_text_prefix)
        }
        _ => None,
    }
}

fn yaml_to_flat_string(yaml: &Yaml) -> String {
    match yaml {
        Yaml::String(s) => s.clone(),
        Yaml::Array(arr) => arr.iter().map(yaml_to_flat_string).collect::<Vec<_>>().join("\n"),
        Yaml::Hash(h) => h
            .iter()
            .map(|(k, v)| format!("{}: {}", yaml_to_flat_string(k), yaml_to_flat_string(v)))
            .collect::<Vec<_>>()
            .join("\n"),
        _ => String::new(),
    }
}

/// Build a YAML-style template for one glyph from a `translate` alphabet range.
fn raw_template_for_translated_glyph(glyph: char, letter: char, yaml_prefix: &str) -> String {
    let o = glyph as u32;
    // Mathematical Double-Struck ranges are labeled `D`/`DC` in unicode-full.yaml (same as
    // Fraktur) but must use the blackboard flag so they remain distinct when prefs map
    // Fraktur and DoubleStruck to the same cells.
    let use_blackboard = (0x1D538..=0x1D56B).contains(&o) || (0x1D7D8..=0x1D7E1).contains(&o);

    if letter.is_ascii_digit() {
        let cell = digit_cell(letter);
        if use_blackboard {
            return format!("𝔹N{cell}");
        }
        return format!("{yaml_prefix}N{cell}");
    }

    let cell = letter_cell(letter.to_ascii_lowercase());
    if cell.is_empty() {
        return String::new();
    }
    let is_cap = letter.is_ascii_uppercase()
        || (0x1D400..=0x1D419).contains(&o) // bold A-Z
        || (0x1D434..=0x1D44D).contains(&o) // italic A-Z
        || (0x1D468..=0x1D481).contains(&o) // bold italic A-Z
        || (0x1D49C..=0x1D4B5).contains(&o) // script A-Z
        || (0x1D4D0..=0x1D4E9).contains(&o) // bold script A-Z
        || (0x1D504..=0x1D51D).contains(&o) // fraktur A-Z
        || (0x1D538..=0x1D551).contains(&o) // double-struck A-Z
        || (0x1D56C..=0x1D585).contains(&o) // bold fraktur A-Z
        || (0x1D5D4..=0x1D5ED).contains(&o) // sans bold A-Z
        || (0x1D608..=0x1D621).contains(&o) // sans italic A-Z
        || (0x1D63C..=0x1D655).contains(&o) // sans bold italic A-Z
        || (0x1D670..=0x1D689).contains(&o); // monospace A-Z

    if use_blackboard {
        if is_cap {
            return format!("𝔹CL{cell}");
        }
        return format!("𝔹L{cell}");
    }

    // Prefix from YAML already includes typeforms + optional C; append L + cell.
    // If the prefix omitted C for an uppercase glyph, add it.
    let mut prefix = yaml_prefix.to_string();
    if is_cap && !prefix.contains('C') && !prefix.contains('𝐶') {
        prefix.push('C');
    }
    format!("{prefix}L{cell}")
}

/// Gather every `t:` / `tc:` / `ct:` / `ot:` string under a unicode replacement node.
fn collect_text_templates(yaml: &Yaml, ch: &str, out: &mut Vec<(String, String)>) {
    match yaml {
        Yaml::Array(arr) => {
            for item in arr {
                collect_text_templates(item, ch, out);
            }
        }
        Yaml::Hash(h) => {
            for (k, v) in h {
                if let Some(key) = k.as_str() {
                    match key {
                        "t" | "T" | "tc" | "TC" | "ct" | "CT" | "ot" | "OT" => {
                            if let Some(s) = v.as_str()
                                && !s.trim().is_empty() {
                                    out.push((ch.to_string(), s.to_string()));
                                }
                        }
                        _ => collect_text_templates(v, ch, out),
                    }
                } else {
                    collect_text_templates(v, ch, out);
                }
            }
        }
        _ => {}
    }
}

fn letter_cell(ch: char) -> &'static str {
    match ch {
        'a' => "⠁",
        'b' => "⠃",
        'c' => "⠉",
        'd' => "⠙",
        'e' => "⠑",
        'f' => "⠋",
        'g' => "⠛",
        'h' => "⠓",
        'i' => "⠊",
        'j' => "⠚",
        'k' => "⠅",
        'l' => "⠇",
        'm' => "⠍",
        'n' => "⠝",
        'o' => "⠕",
        'p' => "⠏",
        'q' => "⠟",
        'r' => "⠗",
        's' => "⠎",
        't' => "⠞",
        'u' => "⠥",
        'v' => "⠧",
        'w' => "⠺",
        'x' => "⠭",
        'y' => "⠽",
        'z' => "⠵",
        _ => "",
    }
}

fn digit_cell(ch: char) -> &'static str {
    match ch {
        '1' => "⠁",
        '2' => "⠃",
        '3' => "⠉",
        '4' => "⠙",
        '5' => "⠑",
        '6' => "⠋",
        '7' => "⠛",
        '8' => "⠓",
        '9' => "⠊",
        '0' => "⠚",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_questioned_equals() {
        clear_ueb_symbol_cache();
        ensure_ueb_symbols_loaded().unwrap();
        let m = match_ueb_symbol("⠦⠻⠐⠶⠀⠼⠁");
        assert!(
            m.is_some(),
            "expected ⠦⠻⠐⠶ in symbol table (from ≟ / 𝟙⠦⠻⠐⠶)"
        );
        let (b, p) = m.unwrap();
        assert_eq!(b, "⠦⠻⠐⠶");
        assert_eq!(p, "≟");
    }

    #[test]
    fn loads_equals() {
        clear_ueb_symbol_cache();
        ensure_ueb_symbols_loaded().unwrap();
        let m = match_ueb_symbol("⠐⠶");
        assert_eq!(m.as_ref().map(|x| x.1.as_str()), Some("="));
    }

    #[test]
    fn loads_both_unicode_files_with_many_symbols() {
        clear_ueb_symbol_cache();
        ensure_ueb_symbols_loaded().unwrap();
        let n = cached_symbol_count();
        // Short file alone is ~370 entries; full alphabets plus G1-stripped aliases
        // dedupe to ~788 unique braille keys (collision resolution drops some YAML rows).
        assert!(
            n > 750,
            "expected full unicode.yaml + unicode-full.yaml coverage, got {n}"
        );
    }

    #[test]
    fn loads_vulgar_fraction_and_section() {
        clear_ueb_symbol_cache();
        ensure_ueb_symbols_loaded().unwrap();
        // ½ → #N⠁N⠌N⠃ → ⠼⠁⠼⠌⠼⠃ (after flag expand)
        let half = match_ueb_symbol("⠼⠁⠼⠌⠼⠃");
        assert!(
            half.as_ref().is_some_and(|(_, p)| p == "½"),
            "expected ½ mapping, got {half:?}"
        );
        let section = match_ueb_symbol("⠘⠎");
        assert!(
            section.as_ref().is_some_and(|(_, p)| p == "§"),
            "expected § mapping, got {section:?}"
        );
    }

    #[test]
    fn loads_blackboard_n() {
        clear_ueb_symbol_cache();
        ensure_ueb_symbols_loaded().unwrap();
        // 𝔹CL⠝ with DoubleStruck=⠈ → ⠈⠠⠝
        let m = match_ueb_symbol("⠈⠠⠝");
        assert!(
            m.as_ref().is_some_and(|(_, p)| p == "ℕ"),
            "expected ℕ, got {m:?}"
        );
    }

    #[test]
    fn does_not_steal_bare_letter_d() {
        clear_ueb_symbol_cache();
        ensure_ueb_symbols_loaded().unwrap();
        // ⅆ is tc: "⠙" in unicode-full — must not claim bare ⠙
        let m = match_ueb_symbol("⠙");
        assert!(
            m.is_none() || m.as_ref().is_some_and(|(b, _)| b != "⠙"),
            "bare ⠙ must remain for the letter lexer, got {m:?}"
        );
    }
}
