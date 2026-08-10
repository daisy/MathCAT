//! Round-trip audit: FromBraille braille → MathML → braille.
//! Run: `cargo test --test from_braille_roundtrip -- --nocapture`
#![allow(non_snake_case)]
#![allow(clippy::needless_return)]

mod common;

use anyhow::Result;
use common::*;
use libmathcat::interface::*;
use libmathcat::parser::Braille_to_MathML;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Case {
    file: String,
    name: String,
    original: String,
}

#[derive(Debug)]
struct Mismatch {
    file: String,
    name: String,
    original: String,
    parsed_mathml: String,
    regenerated: String,
    category: String,
    detail: String,
}

fn from_braille_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/FromBraille/UEB")
}

fn extract_cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for entry in fs::read_dir(from_braille_dir()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let file = path.file_name().unwrap().to_string_lossy().to_string();
        let text = fs::read_to_string(&path).unwrap();
        let mut name = String::new();
        for line in text.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("fn ") {
                if let Some(n) = rest.split('(').next() {
                    name = n.trim().to_string();
                }
            }
            if let Some(idx) = t.find("test_from_braille(\"UEB\", expr, \"") {
                let after = &t[idx + "test_from_braille(\"UEB\", expr, \"".len()..];
                if let Some(end) = after.find('"') {
                    cases.push(Case {
                        file: file.clone(),
                        name: name.clone(),
                        original: after[..end].to_string(),
                    });
                }
            }
        }
    }
    cases.sort_by(|a, b| (&a.file, &a.name).cmp(&(&b.file, &b.name)));
    cases
}

fn categorize(original: &str, regenerated: &str, mathml: &str) -> (String, String) {
    if original == regenerated {
        return ("match".into(), String::new());
    }

    let orig_cells: Vec<char> = original.chars().collect();
    let regen_cells: Vec<char> = regenerated.chars().collect();

    let strip_g1 = |s: &str| {
        s.replace("⠰⠰⠰", "")
            .replace("⠰⠰", "")
            .replace("⠰⠄", "")
            .replace('⠰', "")
    };
    if strip_g1(original) == strip_g1(regenerated) {
        return (
            "grade1-indicators".into(),
            "Same cells aside from Grade-1 indicators (⠰ / ⠰⠰ / ⠰⠰⠰ / ⠰⠄).".into(),
        );
    }

    let strip_num = |s: &str| s.replace('⠼', "");
    if strip_g1(&strip_num(original)) == strip_g1(&strip_num(regenerated)) {
        return (
            "numeric-indicators".into(),
            "Same content aside from numeric indicators (⠼) and/or Grade-1.".into(),
        );
    }

    let strip_cap = |s: &str| s.replace("⠠⠠⠠", "").replace("⠠⠠", "").replace('⠠', "");
    if strip_g1(&strip_cap(original)) == strip_g1(&strip_cap(regenerated)) {
        return (
            "capital-indicators".into(),
            "Same cells aside from capital indicators (⠠…) and/or Grade-1.".into(),
        );
    }

    if original.replace('⠀', "") == regenerated.replace('⠀', "") {
        return (
            "spacing".into(),
            "Same non-space cells; braille space (⠀) placement differs.".into(),
        );
    }

    if regenerated.starts_with(original) || original.starts_with(regenerated) {
        return (
            "length-prefix".into(),
            format!(
                "One is a prefix of the other (orig {} cells, regen {} cells).",
                orig_cells.len(),
                regen_cells.len()
            ),
        );
    }

    if mathml.contains("<mrow>") && original.contains('⠷') && !mathml.contains("<mfrac>") {
        return (
            "structure-loss".into(),
            "Original used general-fraction markers; parsed MathML likely lost fraction structure."
                .into(),
        );
    }

    let mut i = 0;
    while i < orig_cells.len() && i < regen_cells.len() && orig_cells[i] == regen_cells[i] {
        i += 1;
    }
    let detail = format!(
        "First difference at cell index {i}: orig {:?} vs regen {:?}.",
        orig_cells.get(i),
        regen_cells.get(i)
    );
    ("other".into(), detail)
}

fn roundtrip_one(original: &str) -> Result<(String, String), String> {
    let parsed = Braille_to_MathML(original, "UEB").map_err(|e| format!("parse: {e}"))?;
    set_rules_dir(abs_rules_dir_path()).map_err(|e| format!("rules: {e}"))?;
    set_preference("DecimalSeparator", "Auto").ok();
    set_preference("BrailleNavHighlight", "Off").ok();
    set_preference("BrailleCode", "UEB").map_err(|e| format!("pref: {e}"))?;
    set_preference("LaTeX_UseShortName", "false").ok();
    set_preference("Language", "en").ok();
    set_mathml(&parsed).map_err(|e| format!("set_mathml: {e}"))?;
    let regen = get_braille("").map_err(|e| format!("get_braille: {e}"))?;
    Ok((parsed, regen))
}

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "")
}

#[test]
fn report_from_braille_roundtrips() {
    let cases = extract_cases();
    assert!(!cases.is_empty(), "no FromBraille cases found");

    let mut mismatches = Vec::new();
    let mut parse_failures = Vec::new();
    let mut matches = 0usize;

    for case in &cases {
        match roundtrip_one(&case.original) {
            Ok((mathml, regen)) => {
                if regen == case.original {
                    matches += 1;
                } else {
                    let (category, detail) = categorize(&case.original, &regen, &mathml);
                    mismatches.push(Mismatch {
                        file: case.file.clone(),
                        name: case.name.clone(),
                        original: case.original.clone(),
                        parsed_mathml: mathml,
                        regenerated: regen,
                        category,
                        detail,
                    });
                }
            }
            Err(e) => {
                parse_failures.push((
                    case.file.clone(),
                    case.name.clone(),
                    case.original.clone(),
                    e,
                ));
            }
        }
    }

    let out_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("notes/_from_braille_roundtrip.json");
    let mut json = String::from("{\n");
    json.push_str(&format!(
        "  \"total\": {},\n  \"matches\": {},\n  \"mismatches\": {},\n  \"parse_failures\": {},\n",
        cases.len(),
        matches,
        mismatches.len(),
        parse_failures.len()
    ));
    json.push_str("  \"mismatch_list\": [\n");
    let mut items = Vec::new();
    for m in &mismatches {
        items.push(format!(
            "    {{\"file\":\"{}\",\"name\":\"{}\",\"category\":\"{}\",\"detail\":\"{}\",\"original\":\"{}\",\"regenerated\":\"{}\",\"mathml\":\"{}\"}}",
            json_escape(&m.file),
            json_escape(&m.name),
            json_escape(&m.category),
            json_escape(&m.detail),
            json_escape(&m.original),
            json_escape(&m.regenerated),
            json_escape(&m.parsed_mathml),
        ));
    }
    for (file, name, original, err) in &parse_failures {
        items.push(format!(
            "    {{\"file\":\"{}\",\"name\":\"{}\",\"category\":\"parse-failure\",\"detail\":\"{}\",\"original\":\"{}\",\"regenerated\":\"\",\"mathml\":\"\"}}",
            json_escape(file),
            json_escape(name),
            json_escape(err),
            json_escape(original),
        ));
    }
    json.push_str(&items.join(",\n"));
    json.push_str("\n  ]\n}\n");
    fs::write(&out_path, &json).unwrap();

    println!(
        "Round-trip: {} total, {} exact matches, {} mismatches, {} parse failures",
        cases.len(),
        matches,
        mismatches.len(),
        parse_failures.len()
    );
    println!("Report written to {}", out_path.display());

    use std::collections::BTreeMap;
    let mut by_cat: BTreeMap<&str, usize> = BTreeMap::new();
    for m in &mismatches {
        *by_cat.entry(m.category.as_str()).or_default() += 1;
    }
    for (cat, n) in &by_cat {
        println!("  category {cat}: {n}");
    }
    for m in &mismatches {
        println!(
            "MISMATCH {}::{} [{}]\n  orig:  {}\n  regen: {}\n  why:  {}\n  mathml (trunc): {}\n",
            m.file,
            m.name,
            m.category,
            m.original,
            m.regenerated,
            m.detail,
            m.parsed_mathml.chars().take(220).collect::<String>()
        );
    }
    for (file, name, original, err) in &parse_failures {
        println!("PARSE-FAIL {file}::{name}\n  braille: {original}\n  error: {err}\n");
    }

    assert_eq!(
        cases.len(),
        matches + mismatches.len() + parse_failures.len()
    );
}
