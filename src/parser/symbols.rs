//! Shared loader for reverse symbol tables.
//!
//! Each notation keeps its own grammar and its own flag expansion. This module
//! is the part that is the same for every code: find the rules directory, read
//! a unicode YAML file into `(print, template)` pairs, and look up the longest
//! matching key in a cached table.
//!
//! UEB literary digits and Nemeth lower digits are both here because alphabet
//! ranges in `unicode-full.yaml` are expanded with one of those two cells.

#![allow(clippy::needless_return)]

use crate::errors::Result;
use crate::prefs::PreferenceManager;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use yaml_rust::{Yaml, YamlLoader};

pub(crate) type Cached<T> = RefCell<Option<std::result::Result<T, String>>>;

pub(crate) fn rules_dir() -> Result<PathBuf> {
    let pm = PreferenceManager::get();
    let pm = pm.borrow();
    let dir = pm.get_rules_dir();
    if !dir.as_os_str().is_empty() {
        return Ok(dir);
    }
    Ok(PathBuf::from(crate::abs_rules_dir_path()))
}

pub(crate) fn is_braille(ch: char) -> bool {
    ('\u{2800}'..='\u{28FF}').contains(&ch)
}

pub(crate) fn is_latin_letter_cell(c: char) -> bool {
    matches!(
        c,
        '⠁' | '⠃' | '⠉' | '⠙' | '⠑' | '⠋' | '⠛' | '⠓' | '⠊' | '⠚' | '⠅' | '⠇'
            | '⠍' | '⠝' | '⠕' | '⠏' | '⠟' | '⠗' | '⠎' | '⠞' | '⠥' | '⠧' | '⠺' | '⠭'
            | '⠽' | '⠵'
    )
}

pub(crate) fn is_only_latin_letter_cells(braille: &str) -> bool {
    let mut any = false;
    for c in braille.chars() {
        if !is_braille(c) || !is_latin_letter_cell(c) {
            return false;
        }
        any = true;
    }
    any
}

/// Braille cell for a Latin letter, the same in UEB and Nemeth (`a` → ⠁).
pub(crate) fn letter_cell(ch: char) -> &'static str {
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

/// UEB literary digits (`1` → ⠁, `0` → ⠚).
pub(crate) fn ueb_digit_cell(ch: char) -> &'static str {
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

/// Nemeth lower digits (`1` → ⠂, `0` → ⠴).
pub(crate) fn nemeth_digit_cell(ch: char) -> &'static str {
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

/// Load `cell` once. A failed load stays cached so later calls report the same error.
pub(crate) fn ensure<T>(
    cell: &Cached<T>,
    load: impl FnOnce() -> Result<T>,
    what: &str,
) -> Result<()> {
    if cell.borrow().is_none() {
        *cell.borrow_mut() = Some(load().map_err(|e| format!("{e:#}")));
    }
    match cell.borrow().as_ref().unwrap() {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!("{what} failed to load: {e}")),
    }
}

pub(crate) fn clear<T>(cell: &Cached<T>) {
    *cell.borrow_mut() = None;
}

pub(crate) fn with_ok<T, R>(cell: &Cached<T>, f: impl FnOnce(&T) -> R) -> Option<R> {
    let borrowed = cell.borrow();
    let ok = borrowed.as_ref().and_then(|r| r.as_ref().ok())?;
    Some(f(ok))
}

#[cfg(test)]
pub(crate) fn cached_len<T>(cell: &Cached<T>, len: impl Fn(&T) -> usize) -> usize {
    cell.borrow()
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .map(len)
        .unwrap_or(0)
}

/// Longest-prefix match. `table` must be sorted by descending key length.
pub(crate) fn match_longest(table: &[(String, String)], input: &str) -> Option<(String, String)> {
    for (braille, print) in table {
        if input.starts_with(braille.as_str()) {
            return Some((braille.clone(), print.clone()));
        }
    }
    None
}

pub(crate) fn into_longest_first(map: HashMap<String, String>) -> Vec<(String, String)> {
    let mut items: Vec<(String, String)> = map.into_iter().collect();
    items.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(&b.0)));
    items
}

/// Keep `print` when it outranks the entry already stored for `braille`.
///
/// `overwrite` is the short `unicode.yaml` policy: a new spelling replaces the
/// full-file entry unless the full-file print ranks higher.
pub(crate) fn insert_ranked(
    map: &mut HashMap<String, String>,
    braille: String,
    print: String,
    overwrite: bool,
    rank: impl Fn(&str) -> i32,
) {
    let outranks = |new: &str, existing: &str| rank(new) >= rank(existing);
    if overwrite {
        if let Some(prev) = map.get(&braille)
            && !outranks(&print, prev)
        {
            return;
        }
        map.insert(braille, print);
    } else if let Some(prev) = map.get(&braille) {
        if outranks(&print, prev) {
            map.insert(braille, print);
        }
    } else {
        map.insert(braille, print);
    }
}

/// Read `(print character, raw template)` pairs from one unicode YAML file.
///
/// Single-character keys contribute every `t:` / `tc:` / `ot:` string.
/// Multi-character keys (alphabet ranges) are expanded only when `range_letters`
/// is the second `translate()` capture, for example `"[a-zA-Z0-9]+"`.
/// `range_template(glyph, letter, yaml_prefix)` builds the template those pairs use.
pub(crate) fn collect_templates(
    text: &str,
    range_letters: Option<&str>,
    mut range_template: impl FnMut(char, char, &str) -> String,
) -> Result<Vec<(String, String)>> {
    let docs = YamlLoader::load_from_str(text).map_err(|e| anyhow::anyhow!("YAML parse: {e}"))?;
    let doc = docs
        .first()
        .ok_or_else(|| anyhow::anyhow!("empty unicode YAML"))?;
    let list = doc
        .as_vec()
        .ok_or_else(|| anyhow::anyhow!("unicode YAML root is not an array"))?;

    let translate_re = range_letters.map(|letters| {
        regex::Regex::new(&format!(
            r"translate\('\.',\s*'([^']+)',\s*'({letters})'\)"
        ))
        .expect("translate() regex")
    });

    let mut pairs = Vec::new();
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
        let Some(translate_re) = translate_re.as_ref() else {
            continue;
        };
        let prefix = first_text_prefix(value).unwrap_or_default();
        let value_str = yaml_to_flat_string(value);
        for caps in translate_re.captures_iter(&value_str) {
            let glyphs = caps.get(1).unwrap().as_str();
            let letters = caps.get(2).unwrap().as_str();
            for (g, letter) in glyphs.chars().zip(letters.chars()) {
                let raw = range_template(g, letter, &prefix);
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
                if let Some(key) = k.as_str()
                    && matches!(key, "t" | "T" | "tc" | "TC" | "ct" | "CT")
                    && let Some(s) = v.as_str()
                    && !s.trim().is_empty()
                {
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
        Yaml::Array(arr) => arr
            .iter()
            .map(yaml_to_flat_string)
            .collect::<Vec<_>>()
            .join("\n"),
        Yaml::Hash(h) => h
            .iter()
            .map(|(k, v)| format!("{}: {}", yaml_to_flat_string(k), yaml_to_flat_string(v)))
            .collect::<Vec<_>>()
            .join("\n"),
        _ => String::new(),
    }
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
                            if let Some(s) = v.as_str()
                                && !s.trim().is_empty()
                            {
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
