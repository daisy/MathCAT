//! Compare forward vs FromBraille expected MathML after `set_mathml` canonicalization.
//! Run: `cargo test --test nemeth_mathml_canon_compare report -- --nocapture`

#![allow(clippy::needless_return)]

mod common;

use common::{abs_rules_dir_path, FROM_BRAILLE_IGNORE_ATTRS};
use libmathcat::interface::{
    get_element, init_panic_handler, is_same_element, set_mathml, set_preference, set_rules_dir,
    trim_element,
};
use regex::Regex;
use sxd_document_no_unsafe::parser;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

fn normalize_canon(mathml: &str) -> String {
    mathml
        .replace("<mtext", "<mi")
        .replace("</mtext>", "</mi>")
        .replace("<mprescripts></mprescripts>", "<mprescripts/>")
        .replace('\u{2062}', "\u{2063}")
        .replace("&#x2062;", "&#x2063;")
        .replace("&#x2062", "&#x2063")
}

fn compact_ws(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn extract_exprs(path: &Path) -> HashMap<String, String> {
    let text = fs::read_to_string(path).expect("read test file");
    let fn_re = Regex::new(r"#\[test\]\s*(?:#\[ignore[^\]]*\]\s*)*fn\s+(\w+)\s*\(").unwrap();
    let expr_re =
        Regex::new(r##"let expr = (?:r#"(.*?)"#|"(.*?)")\s*;"##).unwrap();
    let mut out = HashMap::new();
    let matches: Vec<_> = fn_re.find_iter(&text).collect();
    for (i, m) in matches.iter().enumerate() {
        let name = fn_re.captures(m.as_str()).unwrap()[1].to_string();
        let start = m.start();
        let end = matches.get(i + 1).map(|n| n.start()).unwrap_or(text.len());
        let body = &text[start..end];
        let exprs: Vec<String> = expr_re
            .captures_iter(body)
            .map(|c| {
                c.get(1)
                    .or_else(|| c.get(2))
                    .unwrap()
                    .as_str()
                    .to_string()
            })
            .collect();
        for (j, expr) in exprs.into_iter().enumerate() {
            let key = if j == 0 {
                name.clone()
            } else {
                format!("{}_{}", name, j + 1)
            };
            out.insert(key, compact_ws(&expr));
        }
    }
    out
}

fn canon(mathml: &str) -> Result<String, String> {
    set_mathml(mathml)
        .map(|s| normalize_canon(&s))
        .map_err(|e| format!("{e}"))
}

fn compare_trees(fwd: &str, rev: &str) -> Result<(String, String), String> {
    let fwd_canon = canon(fwd).map_err(|e| format!("fwd: {e}"))?;
    let rev_canon = canon(rev).map_err(|e| format!("rev: {e}"))?;
    let pkg_fwd = parser::parse(&fwd_canon).map_err(|e| format!("parse fwd: {e}"))?;
    let pkg_rev = parser::parse(&rev_canon).map_err(|e| format!("parse rev: {e}"))?;
    let el_fwd = get_element(&pkg_fwd);
    let el_rev = get_element(&pkg_rev);
    trim_element(el_fwd, false);
    trim_element(el_rev, false);
    is_same_element(el_fwd, el_rev, FROM_BRAILLE_IGNORE_ATTRS)
        .map(|_| (fwd_canon, rev_canon))
        .map_err(|e| e.to_string())
}

fn classify(err: &str, fwd: &str, rev: &str) -> &'static str {
    let e = err.to_lowercase();
    if e.contains("names not the same") {
        if fwd.contains("menclose") || rev.contains("menclose") {
            return "menclose";
        }
        if fwd.contains("mtext") || rev.contains("mtext") {
            return "mtext-vs-mi";
        }
        return "element-name";
    }
    if e.contains("children") {
        if fwd.contains("mmultiscripts")
            || rev.contains("mmultiscripts")
            || fwd.contains("msup")
            || rev.contains("msup")
            || fwd.contains("msub")
            || rev.contains("msub")
        {
            return "script-structure";
        }
        return "child-count";
    }
    if e.contains("text differs") {
        return "text-content";
    }
    if fwd.contains("intent=") && !rev.contains("intent=") {
        return "intent-dropped";
    }
    if fwd.contains("mathvariant") && !rev.contains("mathvariant") {
        return "typeform-encoding";
    }
    "other"
}

#[test]
fn report() {
    init_panic_handler();
    set_rules_dir(abs_rules_dir_path()).unwrap();
    set_preference("DecimalSeparator", "Auto").unwrap();
    set_preference("BrailleNavHighlight", "Off").unwrap();
    set_preference("BrailleCode", "Nemeth").unwrap();
    set_preference("LaTeX_UseShortName", "false").unwrap();
    set_preference("Language", "en").unwrap();
    set_preference("UseSpacesAroundAllOperators", "false").unwrap();

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let modules = ["rules", "other", "chemistry"];
    let mut total = 0usize;
    let mut same = 0usize;
    let mut differ = 0usize;
    let mut canon_fail = 0usize;
    let mut canon_fail_examples: Vec<(String, String, String, String)> = Vec::new();
    let mut by_cat: HashMap<&'static str, Vec<String>> = HashMap::new();
    let mut examples: HashMap<&'static str, Vec<(String, String, String, String)>> = HashMap::new();

    for mod_name in modules {
        let fwd_path = root.join(format!("tests/braille/Nemeth/{mod_name}.rs"));
        let rev_path = root.join(format!("tests/FromBraille/Nemeth/{mod_name}.rs"));
        let fwd = extract_exprs(&fwd_path);
        let rev = extract_exprs(&rev_path);
        let mut names: Vec<_> = fwd.keys().filter(|k| rev.contains_key(*k)).cloned().collect();
        names.sort();
        for name in names {
            total += 1;
            let f = &fwd[&name];
            let r = &rev[&name];
            let label = format!("{mod_name}::{name}");
            match compare_trees(f, r) {
                Ok(_) => same += 1,
                Err(e) => {
                    if e.contains("parse") || e.contains("Invalid") {
                        canon_fail += 1;
                        by_cat.entry("canon-failed").or_default().push(label.clone());
                        canon_fail_examples.push((
                            label,
                            f.chars().take(200).collect(),
                            r.chars().take(200).collect(),
                            e.clone(),
                        ));
                    } else {
                        differ += 1;
                        let cat = classify(&e, f, r);
                        by_cat.entry(cat).or_default().push(label.clone());
                        examples.entry(cat).or_default().push((
                            label,
                            f.chars().take(160).collect(),
                            r.chars().take(160).collect(),
                            e.chars().take(120).collect(),
                        ));
                    }
                }
            }
        }
    }

    println!("Nemeth MathML compare (set_mathml + attr strip)");
    println!("common pairs: {total}");
    println!("canonically equal: {same}");
    println!("differ: {differ}");
    println!("canon/parse failed: {canon_fail}");
    println!();
    println!("By category:");
    let mut cats: Vec<_> = by_cat.iter().collect();
    cats.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(b.0)));
    for (cat, names) in &cats {
        println!("  {cat}: {}", names.len());
        if let Some(ex) = examples.get(*cat) {
            for (label, f, r, err) in ex.iter().take(3) {
                println!("    - {label}");
                println!("      err: {err}");
                println!("      fwd: {f}...");
                println!("      rev: {r}...");
            }
        }
    }
    println!();
    println!("Canon/parse failures:");
    for (label, f, r, err) in &canon_fail_examples {
        println!("  - {label}");
        println!("    err: {err}");
        println!("    fwd: {f}");
        println!("    rev: {r}");
    }
    println!();
    println!("All differing names:");
    for (cat, names) in &cats {
        if **cat == "canon-failed" || names.is_empty() {
            continue;
        }
        println!("  [{cat}] ({})", names.len());
        for n in names.iter() {
            println!("    {n}");
        }
    }

    let out_path = root.join("notes/_nemeth_mathml_canon_compare.txt");
    let summary = format!(
        "total={total}\nsame={same}\ndiffer={differ}\ncanon_fail={canon_fail}\n"
    );
    fs::write(&out_path, summary).ok();

    assert!(total > 0, "expected Nemeth test pairs");
}
