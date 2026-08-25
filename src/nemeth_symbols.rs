//! Reverse Nemeth braille → print character map, built lazily from
//! `Rules/Braille/Nemeth/unicode.yaml` and `unicode-full.yaml`.
//!
//! Flag letters in YAML templates are expanded the same way as Nemeth cleanup
//! (`NEMETH_INDICATOR_REPLACEMENTS`). ASCII a–z/A–Z/0–9 and bare Latin letter
//! cells are omitted so the mode-aware letter/number lexer keeps those.

#![allow(clippy::needless_return)]

use crate::errors::Result;
use crate::shim_filesystem::read_to_string_shim;
use anyhow::Context;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use yaml_rust::{Yaml, YamlLoader};

thread_local! {
    static NEMETH_SYMBOLS: RefCell<Option<std::result::Result<Vec<(String, String)>, String>>> =
        const { RefCell::new(None) };
}

pub fn ensure_nemeth_symbols_loaded() -> Result<()> {
    NEMETH_SYMBOLS.with(|cell| {
        if cell.borrow().is_none() {
            *cell.borrow_mut() = Some(load_nemeth_symbols().map_err(|e| format!("{e:#}")));
        }
        match cell.borrow().as_ref().unwrap() {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("Nemeth symbol table failed to load: {e}")),
        }
    })
}

/// Longest-match lookup of a Nemeth symbol at the start of `input`.
pub fn match_nemeth_symbol(input: &str) -> Option<(String, String)> {
    let _ = ensure_nemeth_symbols_loaded();
    NEMETH_SYMBOLS.with(|cell| {
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

pub fn clear_nemeth_symbol_cache() {
    NEMETH_SYMBOLS.with(|cell| *cell.borrow_mut() = None);
}

#[cfg(test)]
fn cached_symbol_count() -> usize {
    NEMETH_SYMBOLS.with(|cell| {
        cell.borrow()
            .as_ref()
            .and_then(|r| r.as_ref().ok())
            .map(|v| v.len())
            .unwrap_or(0)
    })
}

fn load_nemeth_symbols() -> Result<Vec<(String, String)>> {
    let rules_dir = rules_dir()?;
    let unicode_path = rules_dir.join("Braille/Nemeth/unicode.yaml");
    let full_path = rules_dir.join("Braille/Nemeth/unicode-full.yaml");

    let unicode_text = read_to_string_shim(&unicode_path)
        .with_context(|| format!("cannot read '{}'", unicode_path.display()))?;
    let full_text = read_to_string_shim(&full_path)
        .with_context(|| format!("cannot read '{}'", full_path.display()))?;

    let full_pairs = collect_unicode_file(&full_text)?;
    let short_pairs = collect_unicode_file(&unicode_text)?;

    let mut braille_to_print: HashMap<String, String> = HashMap::new();
    insert_pairs(&mut braille_to_print, &full_pairs, false);
    insert_pairs(&mut braille_to_print, &short_pairs, true);

    let mut items: Vec<(String, String)> = braille_to_print.into_iter().collect();
    items.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(&b.0)));
    Ok(items)
}

fn insert_pairs(map: &mut HashMap<String, String>, pairs: &[(String, String)], overwrite: bool) {
    for (ch, raw) in pairs {
        if should_skip_print_char(ch) {
            continue;
        }
        let braille = expand_flags(raw);
        if should_skip_mapping(&braille) {
            continue;
        }
        if overwrite {
            if let Some(prev) = map.get(&braille) {
                if !should_replace_print(prev, ch) {
                    continue;
                }
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

fn should_replace_print(existing: &str, new: &str) -> bool {
    preference_rank(new) >= preference_rank(existing)
}

fn preference_rank(s: &str) -> i32 {
    let Some(c) = s.chars().next() else {
        return -1;
    };
    let o = c as u32;
    // Equals shares ⠨⠅ with Greek kappa — prefer the comparison operator.
    if c == '=' {
        return 8;
    }
    if (0x1D400..=0x1D7FF).contains(&o) {
        return 2;
    }
    if c.is_alphabetic() && !(0x0370..=0x03FF).contains(&o) {
        return 3;
    }
    if !c.is_alphabetic() {
        if c == '+' || c == '×' || c == '−' || c == '±' {
            return 6;
        }
        return 4;
    }
    if (0x0370..=0x03FF).contains(&o) {
        return 5;
    }
    3
}

fn should_skip_print_char(ch: &str) -> bool {
    let mut chars = ch.chars();
    let Some(c) = chars.next() else {
        return true;
    };
    if chars.next().is_some() {
        return true;
    }
    if c.is_ascii_alphabetic() || c.is_ascii_digit() {
        return true;
    }
    if c == '-' {
        return true;
    }
    // Fullwidth Latin/digits (FF01–FF5E) steal ⠠+letter / digit cells from the lexer.
    let o = c as u32;
    if (0xFF01..=0xFF5E).contains(&o) {
        return true;
    }
    false
}

fn should_skip_mapping(braille: &str) -> bool {
    if braille.is_empty() || !braille.chars().any(is_braille) {
        return true;
    }
    if is_only_latin_letter_cells(braille) {
        return true;
    }
    // Dropped digit cells are owned by the numeric lexer (and numeric subscripts).
    let mut chars = braille.chars();
    if let (Some(c), None) = (chars.next(), chars.next()) {
        if matches!(
            c,
            '⠴' | '⠂' | '⠆' | '⠒' | '⠲' | '⠢' | '⠖' | '⠶' | '⠦' | '⠔'
                | '⠠' | '⠰' | '⠘' | '⠐' | '⠼' | '⠹' | '⠌' | '⠜' | '⠻' | '⠣'
                | '⠩' | '⠷' | '⠾'
        ) {
            return true;
        }
    }
    // Capital Latin is ⠠ + letter cell; typeforms always add another prefix.
    let mut chars = braille.chars();
    if chars.next() == Some('⠠') {
        if let (Some(c), None) = (chars.next(), chars.next()) {
            if is_latin_letter_cell(c) {
                return true;
            }
        }
    }
    // ELI + Latin letter is owned by the letter lexer (not math-italic YAML).
    let mut chars = braille.chars();
    if chars.next() == Some('⠰') {
        let mut n = chars.next();
        if n == Some('⠠') {
            n = chars.next();
        }
        if n.is_some_and(is_latin_letter_cell) && chars.next().is_none() {
            return true;
        }
    }
    false
}

fn is_only_latin_letter_cells(braille: &str) -> bool {
    let mut any = false;
    for c in braille.chars() {
        if !is_braille(c) || !is_latin_letter_cell(c) {
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

fn rules_dir() -> Result<PathBuf> {
    let pm = crate::prefs::PreferenceManager::get();
    let pm = pm.borrow();
    let dir = pm.get_rules_dir();
    if !dir.as_os_str().is_empty() {
        return Ok(dir);
    }
    Ok(PathBuf::from(crate::abs_rules_dir_path()))
}

fn is_braille(ch: char) -> bool {
    ('\u{2800}'..='\u{28FF}').contains(&ch)
}

/// Expand Nemeth YAML flag letters to the cells `nemeth_cleanup` emits.
fn expand_flags(raw: &str) -> String {
    let mut out = String::new();
    for ch in raw.chars() {
        let piece: &str = match ch {
            'S' => "⠠⠨",
            'B' => "⠸",
            '𝔹' => "⠠⠸",
            'T' => "⠈",
            'I' => "⠨",
            'R' => "",
            'E' => "⠰",
            'D' => "⠸",
            'G' => "⠨",
            'V' => "⠨⠈",
            'H' => "⠠⠠",
            'U' => "⠈⠈",
            'C' => "⠠",
            'P' | '𝐏' => "⠸",
            'L' | 'l' | 'M' | 'N' | '𝑁' => "",
            'm' | 'b' => "⠐",
            'n' => "⠼",
            'W' | 'w' => "⠀",
            ',' => "⠠⠀",
            '↑' => "⠘",
            '↓' => "⠰",
            c if is_braille(c) => {
                out.push(c);
                continue;
            }
            c if (c as u32) < 128 => continue,
            _ => continue,
        };
        out.push_str(piece);
    }
    out
}

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
        Regex::new(r"translate\('\.',\s*'([^']+)',\s*'([^']+)'\)").unwrap();

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

        if key_chars.next().is_none() {
            collect_text_templates(value, &first.to_string(), &mut pairs);
            continue;
        }

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

fn first_text_prefix(yaml: &Yaml) -> Option<String> {
    match yaml {
        Yaml::Array(arr) => arr.iter().find_map(first_text_prefix),
        Yaml::Hash(h) => {
            for (k, v) in h {
                if let Some(key) = k.as_str() {
                    if matches!(key, "t" | "T" | "tc" | "TC" | "ct" | "CT") {
                        if let Some(s) = v.as_str() {
                            if !s.trim().is_empty() {
                                return Some(s.to_string());
                            }
                        }
                    }
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

fn raw_template_for_translated_glyph(_glyph: char, letter: char, yaml_prefix: &str) -> String {
    if letter.is_ascii_digit() {
        let cell = digit_cell(letter);
        return format!("{yaml_prefix}N{cell}");
    }
    let cell = letter_cell(letter.to_ascii_lowercase());
    if cell.is_empty() {
        return String::new();
    }
    let is_cap = letter.is_ascii_uppercase();
    let mut prefix = yaml_prefix.to_string();
    if is_cap && !prefix.contains('C') {
        prefix.push('C');
    }
    format!("{prefix}L{cell}")
}

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
                            if let Some(s) = v.as_str() {
                                if !s.trim().is_empty() {
                                    out.push((ch.to_string(), s.to_string()));
                                }
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
        '0' => "⠴",
        '1' => "⠂",
        '2' => "⠆",
        '3' => "⠒",
        '4' => "⠲",
        '5' => "⠢",
        '6' => "⠖",
        '7' => "⠶",
        '8' => "⠦",
        '9' => "⠔",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_equals() {
        clear_nemeth_symbol_cache();
        ensure_nemeth_symbols_loaded().unwrap();
        let m = match_nemeth_symbol("⠨⠅");
        assert_eq!(m.as_ref().map(|x| x.1.as_str()), Some("="));
    }

    #[test]
    fn loads_plus() {
        clear_nemeth_symbol_cache();
        ensure_nemeth_symbols_loaded().unwrap();
        let m = match_nemeth_symbol("⠬");
        assert_eq!(m.as_ref().map(|x| x.1.as_str()), Some("+"));
    }

    #[test]
    fn loads_both_unicode_files() {
        clear_nemeth_symbol_cache();
        ensure_nemeth_symbols_loaded().unwrap();
        let n = cached_symbol_count();
        assert!(n > 200, "expected Nemeth unicode.yaml coverage, got {n}");
    }

    #[test]
    fn greek_alpha() {
        clear_nemeth_symbol_cache();
        ensure_nemeth_symbols_loaded().unwrap();
        let m = match_nemeth_symbol("⠨⠁");
        assert!(
            m.as_ref().is_some_and(|(_, p)| p == "α"),
            "expected α, got {m:?}"
        );
    }

    #[test]
    fn does_not_steal_bare_letter() {
        clear_nemeth_symbol_cache();
        ensure_nemeth_symbols_loaded().unwrap();
        let m = match_nemeth_symbol("⠭");
        assert!(
            m.is_none() || m.as_ref().is_some_and(|(b, _)| b != "⠭"),
            "bare ⠭ must remain for the letter lexer, got {m:?}"
        );
    }

    #[test]
    fn does_not_steal_capital_latin() {
        clear_nemeth_symbol_cache();
        ensure_nemeth_symbols_loaded().unwrap();
        let m = match_nemeth_symbol("⠠⠓");
        assert!(
            m.is_none() || m.as_ref().is_some_and(|(b, _)| b != "⠠⠓"),
            "⠠⠓ must remain for the capital-letter lexer, got {m:?}"
        );
    }
}
