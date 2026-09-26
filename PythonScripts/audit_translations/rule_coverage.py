"""Run the full Rust test suite and report which rule YAML files it exercises."""

import json
import shutil
import subprocess
import sys
import webbrowser
from collections import Counter, defaultdict
from pathlib import Path, PurePosixPath

from .rule_coverage_report import RuleKey, coverage, render_html, rule_section, section

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "target" / "rule-coverage"
EVENTS = OUTPUT / "events"
EXCLUDED_FILE_NAMES = {"definitions.yaml", "unicode.yaml", "unicode-full.yaml"}


def read_events() -> tuple[set[str], set[str], set[RuleKey], Counter[RuleKey], dict[RuleKey, Counter[str]], list[str]]:
    loaded: set[str] = set()
    matched: set[str] = set()
    defined_rules: set[RuleKey] = set()
    rule_hits: Counter[RuleKey] = Counter()
    rule_tests: dict[RuleKey, Counter[str]] = defaultdict(Counter)
    errors: list[str] = []
    for event_file in sorted(EVENTS.glob("*.jsonl")):
        for number, line in enumerate(event_file.read_text(encoding="utf-8").splitlines(), 1):
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                errors.append(f"Invalid JSON in {event_file.name}:{number}")
                continue
            if not isinstance(event, dict):
                errors.append(f"Invalid event in {event_file.name}:{number}")
                continue
            kind = event.get("kind")
            path = event.get("path")
            if not isinstance(path, str):
                errors.append(f"Invalid event in {event_file.name}:{number}")
                continue
            path = path.replace("\\", "/")
            parts = PurePosixPath(path).parts
            if (not parts
                    or PurePosixPath(path).is_absolute() or ".." in parts
                    or PurePosixPath(path).suffix not in (".yaml", ".yml")):
                errors.append(f"Invalid event in {event_file.name}:{number}")
            elif PurePosixPath(path).name.lower() in EXCLUDED_FILE_NAMES:
                continue
            elif kind == "loaded" and event.keys() == {"kind", "path"}:
                loaded.add(path)
            elif kind == "matched" and event.keys() == {"kind", "path"}:
                matched.add(path)
            elif kind in ("defined-rule", "matched-rule") and {"name", "tag"} <= event.keys():
                name, tag = event["name"], event["tag"]
                if not isinstance(name, str) or not isinstance(tag, str) or not name or not tag:
                    errors.append(f"Empty rule identity in {event_file.name}:{number}")
                elif kind == "defined-rule" and event.keys() == {"kind", "path", "name", "tag"}:
                    defined_rules.add((path, name, tag))
                elif kind == "matched-rule" and event.keys() == {"kind", "path", "name", "tag", "test"} and isinstance(event["test"], str) and event["test"]:
                    key = (path, name, tag)
                    rule_hits[key] += 1
                    rule_tests[key][event["test"]] += 1
                else:
                    errors.append(f"Invalid event in {event_file.name}:{number}")
            else:
                errors.append(f"Invalid event in {event_file.name}:{number}")
    return loaded, matched, defined_rules, rule_hits, dict(rule_tests), errors


def run() -> int:
    """Generate all reports and open the HTML page when the run finishes."""
    OUTPUT.mkdir(parents=True, exist_ok=True)
    if EVENTS.exists():
        shutil.rmtree(EVENTS)
    EVENTS.mkdir()

    command = ["cargo", "test", "--features", "rule-coverage"]
    log_path = OUTPUT / "test.log"
    print(f"Running {' '.join(command)}; saving output to {log_path.relative_to(ROOT)}", flush=True)
    with log_path.open("w", encoding="utf-8") as log:
        try:
            result = subprocess.run(command, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT, check=False)
            test_status = result.returncode
        except OSError as error:
            log.write(f"Could not run cargo: {error}\n")
            test_status = 1

    loaded, matched, defined_rules, rule_hits, rule_tests, errors = read_events()
    matched_rules = set(rule_hits)
    if test_status:
        errors.insert(0, f"cargo test failed (exit status {test_status}); see test.log")
    if not loaded:
        errors.append("No loaded YAML events found")
    if not matched:
        errors.append("No matched pattern events found")
    if not defined_rules:
        errors.append("No active rule definitions found")
    if not matched_rules:
        errors.append("No matched rule events found")
    if matched - loaded:
        errors.append("Matched pattern files lack loaded events: " + ", ".join(sorted(matched - loaded)))
    if matched_rules - defined_rules:
        errors.append("Matched rules lack definition events")
    if {path for path, _, _ in defined_rules} - loaded:
        errors.append("Defined rules have no loaded YAML event")
    if matched != {path for path, _, _ in matched_rules}:
        errors.append("File and rule match events disagree")

    status = "Incomplete" if errors else "Complete"
    report = [
        "# Rule Coverage\n",
        f"Status: **{status}**\n",
        f"Pattern files matched: **{coverage(len(matched), len(loaded))}**\n",
        f"Active rules matched: **{coverage(len(matched_rules), len(defined_rules))}**\n",
        "Paths are relative to `Rules/`. A pattern file is matched when a rule from it completes its replacement successfully.\n",
        "Unicode mapping and definition files are omitted because this report measures pattern-rule coverage.\n",
    ]
    if errors:
        report.append("## Problems\n\n" + "\n".join(f"- {error}" for error in errors) + "\n")
    report.extend((
        section("Loaded YAML files", loaded),
        section("Matched pattern files", matched),
        section("Loaded files with no pattern match", loaded - matched),
        rule_section("Matched rules", matched_rules, rule_hits, rule_tests),
        rule_section("Active rules with no match", defined_rules - matched_rules),
    ))
    report_path = OUTPUT / "report.md"
    report_path.write_text("\n".join(report), encoding="utf-8")
    html_path = OUTPUT / "index.html"
    html_path.write_text(render_html(loaded, matched, defined_rules, rule_hits, rule_tests, errors), encoding="utf-8")
    print(
        f"{status}: {len(loaded)} files loaded, {coverage(len(matched), len(loaded))} files matched; "
        f"{coverage(len(matched_rules), len(defined_rules))} rules matched; report: {html_path}"
    )
    try:
        opened = webbrowser.open(html_path.resolve().as_uri())
    except (OSError, webbrowser.Error):
        opened = False
    if not opened:
        print(f"Browser unavailable; open {html_path} manually")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(run())
