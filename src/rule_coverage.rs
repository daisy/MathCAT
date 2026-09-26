//! Per-process YAML rule coverage events, enabled only by `rule-coverage`.

use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum EventKind {
    Loaded,
    Matched,
    DefinedRule,
    MatchedRule,
}

static RECORDED: OnceLock<Mutex<HashSet<(EventKind, PathBuf, String, String)>>> = OnceLock::new();

fn rule_relative_path(path: &Path) -> Option<PathBuf> {
    let rules_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Rules");
    let relative = path.strip_prefix(&rules_dir)
        .or_else(|_| path.strip_prefix("Rules"))
        .ok()?;
    if relative.as_os_str().is_empty() || !matches!(relative.extension().and_then(|s| s.to_str()), Some("yaml" | "yml")) {
        return None;
    }
    Some(relative.to_path_buf())
}

fn record(kind: EventKind, path: &Path, name: &str, tag: &str) {
    let Some(relative) = rule_relative_path(path) else { return };
    let mut recorded = RECORDED.get_or_init(|| Mutex::new(HashSet::new()))
        .lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let key = (kind, relative.clone(), name.to_string(), tag.to_string());
    if kind != EventKind::MatchedRule && recorded.contains(&key) {
        return;
    }

    let event_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/rule-coverage/events");
    fs::create_dir_all(&event_dir).expect("cannot create rule coverage event directory");
    let event_file = event_dir.join(format!("pid-{}.jsonl", std::process::id()));
    let mut output = OpenOptions::new().create(true).append(true).open(event_file)
        .expect("cannot open rule coverage event file");
    let path = relative.to_string_lossy();
    let event = match kind {
        EventKind::Loaded => serde_json::json!({"kind": "loaded", "path": path}),
        EventKind::Matched => serde_json::json!({"kind": "matched", "path": path}),
        EventKind::DefinedRule => serde_json::json!({"kind": "defined-rule", "path": path, "name": name, "tag": tag}),
        EventKind::MatchedRule => serde_json::json!({
            "kind": "matched-rule", "path": path, "name": name, "tag": tag,
            "test": std::thread::current().name().unwrap_or("(unnamed thread)"),
        }),
    };
    writeln!(output, "{event}").expect("cannot write rule coverage event");
    if kind != EventKind::MatchedRule {
        recorded.insert(key);
    }
}

pub(crate) fn loaded(path: &Path) {
    record(EventKind::Loaded, path, "", "");
}

pub(crate) fn defined_rule(path: &Path, name: &str, tag: &str) {
    record(EventKind::DefinedRule, path, name, tag);
}

pub(crate) fn matched_rule(path: &Path, name: &str, tag: &str) {
    record(EventKind::Matched, path, "", "");
    record(EventKind::MatchedRule, path, name, tag);
}
