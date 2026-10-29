//! Reverse Nemeth braille → print character map, built lazily from
//! `Rules/Braille/Nemeth/unicode.yaml` and `unicode-full.yaml`.
//!
//! Flag letters in YAML templates are expanded the same way as Nemeth cleanup
//! (`NEMETH_INDICATOR_REPLACEMENTS`). ASCII a–z/A–Z/0–9 and bare Latin letter
//! cells are omitted so the mode-aware letter/number lexer keeps those.

#![allow(clippy::needless_return)]

use crate::errors::Result;
use crate::parser::symbols;
use crate::shim_filesystem::read_to_string_shim;
use anyhow::Context;
use std::cell::RefCell;
use std::collections::HashMap;

type SymbolTableLoad = std::result::Result<Vec<(String, String)>, String>;

thread_local! {
    static NEMETH_SYMBOLS: RefCell<Option<SymbolTableLoad>> = const { RefCell::new(None) };
}

pub fn ensure_nemeth_symbols_loaded() -> Result<()> {
    NEMETH_SYMBOLS.with(|cell| symbols::ensure(cell, load_nemeth_symbols, "Nemeth symbol table"))
}

/// Longest-match lookup of a Nemeth symbol at the start of `input`.
pub fn match_nemeth_symbol(input: &str) -> Option<(String, String)> {
    let _ = ensure_nemeth_symbols_loaded();
    NEMETH_SYMBOLS.with(|cell| {
        symbols::with_ok(cell, |table| symbols::match_longest(table, input)).flatten()
    })
}

pub fn clear_nemeth_symbol_cache() {
    NEMETH_SYMBOLS.with(symbols::clear);
}

#[cfg(test)]
fn cached_symbol_count() -> usize {
    NEMETH_SYMBOLS.with(|cell| symbols::cached_len(cell, |table| table.len()))
}

fn load_nemeth_symbols() -> Result<Vec<(String, String)>> {
    let rules_dir = symbols::rules_dir()?;
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

    Ok(symbols::into_longest_first(braille_to_print))
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
        symbols::insert_ranked(map, braille, ch.clone(), overwrite, preference_rank);
    }
}

fn preference_rank(s: &str) -> i32 {
    let Some(c) = s.chars().next() else {
        return -1;
    };
    let o = c as u32;
    // Equals shares ⠨⠅ with Greek kappa. The table keeps "="; the parser
    // rewrites an unspaced ⠨⠅ to κ (spaces on both sides stay "=").
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
    if braille.is_empty() || !braille.chars().any(symbols::is_braille) {
        return true;
    }
    if symbols::is_only_latin_letter_cells(braille) {
        return true;
    }
    // Dropped digit cells are owned by the numeric lexer (and numeric subscripts).
    let mut chars = braille.chars();
    if let (Some(c), None) = (chars.next(), chars.next())
        && matches!(
            c,
            '⠴' | '⠂' | '⠆' | '⠒' | '⠲' | '⠢' | '⠖' | '⠶' | '⠦' | '⠔'
                | '⠠' | '⠰' | '⠘' | '⠐' | '⠼' | '⠹' | '⠌' | '⠜' | '⠻' | '⠣'
                | '⠩' | '⠷' | '⠾'
        ) {
            return true;
        }
    // Capital Latin is ⠠ + letter cell; typeforms always add another prefix.
    let mut chars = braille.chars();
    if chars.next() == Some('⠠')
        && let (Some(c), None) = (chars.next(), chars.next())
            && symbols::is_latin_letter_cell(c) {
                return true;
            }
    // ELI + Latin letter is owned by the letter lexer (not math-italic YAML).
    let mut chars = braille.chars();
    if chars.next() == Some('⠰') {
        let mut n = chars.next();
        if n == Some('⠠') {
            n = chars.next();
        }
        if n.is_some_and(symbols::is_latin_letter_cell) && chars.next().is_none() {
            return true;
        }
    }
    false
}

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
            c if symbols::is_braille(c) => {
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
    symbols::collect_templates(text, Some("[^']+"), raw_template_for_translated_glyph)
}

fn raw_template_for_translated_glyph(_glyph: char, letter: char, yaml_prefix: &str) -> String {
    if letter.is_ascii_digit() {
        let cell = symbols::nemeth_digit_cell(letter);
        return format!("{yaml_prefix}N{cell}");
    }
    let cell = symbols::letter_cell(letter.to_ascii_lowercase());
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
