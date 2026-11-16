"""Checks the rule coverage command without rerunning the Rust test suite."""

import json
import sys
from collections.abc import Callable
from pathlib import Path

import pytest

from .. import cli
from ..rulecoverage import rule_coverage
from ..rulecoverage.rule_coverage_report import render_html

LIST_COMMAND = [*rule_coverage.COMMAND, "--", "--list"]


def fake_popen(run_cargo: Callable[..., tuple[int, list[str]]]) -> type:
    """Stand in for `subprocess.Popen`, replaying `(exit status, output lines)` from `run_cargo(command, **kwargs)`."""

    class FakeProcess:
        def __init__(self, command: list[str], **kwargs: object) -> None:
            self.returncode, lines = run_cargo(command, **kwargs)
            self.stdout = iter(line + "\n" for line in lines)

        def __enter__(self) -> FakeProcess:
            return self

        def __exit__(self, *exc_info: object) -> None:
            pass

        def wait(self) -> int:
            return self.returncode

    return FakeProcess


def test_coverage_command_generates_html_and_opens_browser(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    """The CLI writes only the HTML report, removes an old Markdown report, and opens the page."""
    output = tmp_path / "target" / "rule-coverage"
    output.mkdir(parents=True)
    (output / "report.md").write_text("old report", encoding="utf-8")
    monkeypatch.setattr(rule_coverage, "ROOT", tmp_path)
    monkeypatch.setattr(rule_coverage, "OUTPUT", output)
    monkeypatch.setattr(rule_coverage, "EVENTS", output / "events")

    def fake_cargo(command: list[str], **kwargs: object) -> tuple[int, list[str]]:
        assert kwargs["cwd"] == tmp_path
        if command == LIST_COMMAND:
            return 0, ["   Compiling mathcat v0.7.8", "     Running unittests src/lib.rs", "test_alpha: test", "test_beta: test"]
        assert command == ["cargo", "test", "--features", "rule-coverage"]
        events = [
            {"kind": "defined-rule", "path": "Languages/en/SimpleSpeak_Rules.yaml", "name": "simple", "tag": "mi"},
            {"kind": "defined-rule", "path": "Languages/en/SimpleSpeak_Rules.yaml", "name": "default", "tag": "mi"},
            {"kind": "defined-rule", "path": "Languages/en/Z_Rules.yaml", "name": "unused", "tag": "mn"},
            {
                "kind": "matched-rule",
                "path": "Languages/en/SimpleSpeak_Rules.yaml",
                "name": "simple",
                "tag": "mi",
                "test": "test_alpha",
            },
            {
                "kind": "matched-rule",
                "path": "Languages/en/SimpleSpeak_Rules.yaml",
                "name": "simple",
                "tag": "mi",
                "test": "test_alpha",
            },
            {
                "kind": "matched-rule",
                "path": "Languages/en/SimpleSpeak_Rules.yaml",
                "name": "simple",
                "tag": "mi",
                "test": "test_beta",
            },
        ]
        (output / "events" / "pid-123.jsonl").write_text(
            "\n".join(json.dumps(event) for event in events) + "\n",
            encoding="utf-8",
        )
        return 0, ["running 2 tests", "test test_alpha ... ok", "test test_beta ... ok"]

    opened: list[str] = []
    monkeypatch.setattr(rule_coverage.subprocess, "Popen", fake_popen(fake_cargo))
    monkeypatch.setattr(rule_coverage.webbrowser, "open", lambda url: opened.append(url) or True)
    monkeypatch.setattr(sys, "argv", ["audit-translations", "--rule-coverage"])

    with pytest.raises(SystemExit) as result:
        cli.main()

    assert result.value.code == 0
    assert opened == [(output / "index.html").as_uri()]
    assert not (output / "report.md").exists()
    html = (output / "index.html").read_text(encoding="utf-8")
    assert "Status: <strong>Complete</strong>" in html
    assert "1/2 (50%) rule files matched" in html
    assert "Languages/en/Z_Rules.yaml" in html
    assert 'data-rule-search="simple mi" data-status="matched"' in html
    assert 'data-rule-search="default mi" data-status="unmatched"' in html
    assert "1/3 (33%) active rules matched" in html
    assert "1/2 (50%) rules" in html
    assert 'role="tooltip">Tests:\ntest_alpha (2 hits)\ntest_beta (1 hit)</span>' in html
    assert ".rules li:hover .rule-tooltip, .rules li:focus .rule-tooltip { display: block; }" in html
    assert 'aria-describedby="rule-tooltip-0-1"' in html
    assert "3 hits</span>" in html
    assert "0 hits</span>" in html
    assert "2 files with active rules" in html
    assert (output / "test.log").is_file()


def test_failed_coverage_run_opens_incomplete_report(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    """A failed test run remains unsuccessful but still produces and opens a useful report."""
    output = tmp_path / "target" / "rule-coverage"
    monkeypatch.setattr(rule_coverage, "ROOT", tmp_path)
    monkeypatch.setattr(rule_coverage, "OUTPUT", output)
    monkeypatch.setattr(rule_coverage, "EVENTS", output / "events")
    monkeypatch.setattr(
        rule_coverage.subprocess,
        "Popen",
        fake_popen(lambda command, **kwargs: (0, []) if command == LIST_COMMAND else (1, ["test test_alpha ... FAILED"])),
    )
    opened: list[str] = []
    monkeypatch.setattr(rule_coverage.webbrowser, "open", lambda url: opened.append(url) or True)

    assert rule_coverage.run() == 1
    assert opened == [(output / "index.html").as_uri()]
    html = (output / "index.html").read_text(encoding="utf-8")
    assert "Status: <strong>Incomplete</strong>" in html
    assert "No active rule definitions found" in html
    assert not (output / "report.md").exists()


def test_jsonl_rule_identity_preserves_separators_and_unicode(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    """JSONL rule names survive tabs, newlines, and Unicode without splitting events."""
    events = tmp_path / "events"
    events.mkdir()
    monkeypatch.setattr(rule_coverage, "EVENTS", events)
    name = "fraction\tname\nüber"
    path = "Languages/en/SimpleSpeak_Rules.yaml"
    (events / "pid-123.jsonl").write_text(
        json.dumps({"kind": "defined-rule", "path": path, "name": name, "tag": "mfrac"}) + "\n",
        encoding="utf-8",
    )

    defined, hits_by_test, errors = rule_coverage.read_events()

    assert hits_by_test == {}
    assert defined == {(path, name, "mfrac")}
    assert errors == []


def test_invalid_and_unknown_rule_events_make_report_incomplete(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    """Malformed events and hits without definitions must keep the report incomplete."""
    output = tmp_path / "target" / "rule-coverage"
    monkeypatch.setattr(rule_coverage, "ROOT", tmp_path)
    monkeypatch.setattr(rule_coverage, "OUTPUT", output)
    monkeypatch.setattr(rule_coverage, "EVENTS", output / "events")
    path = "Languages/en/SimpleSpeak_Rules.yaml"

    def fake_cargo(command: list[str], **kwargs: object) -> tuple[int, list[str]]:
        if command == LIST_COMMAND:
            return 0, []
        events = [
            {"kind": "loaded", "path": path},
            {"kind": "defined-rule", "path": path, "name": "known", "tag": "mi"},
            {"kind": "matched-rule", "path": path, "name": "unknown", "tag": "mi", "test": "test_alpha"},
        ]
        (output / "events" / "pid-123.jsonl").write_text(
            "\n".join(json.dumps(event) for event in events) + "\n{bad json\n",
            encoding="utf-8",
        )
        return 0, []

    monkeypatch.setattr(rule_coverage.subprocess, "Popen", fake_popen(fake_cargo))
    monkeypatch.setattr(rule_coverage.webbrowser, "open", lambda url: True)

    assert rule_coverage.run() == 1
    html = (output / "index.html").read_text(encoding="utf-8")
    assert "Status: <strong>Incomplete</strong>" in html
    assert "Invalid event in pid-123.jsonl:1" in html
    assert "Invalid JSON in pid-123.jsonl:4" in html
    assert "Matched rules lack definition events" in html
    assert not (output / "report.md").exists()


def test_report_files_are_tagged_with_language_or_braille_code() -> None:
    """Each file gets a rule-set tag (language, `lang-region`, braille code, or other group) for the filter dropdown,
    so `de` selects only German files even though `default.yaml` and rule names like `default` also contain "de"."""
    paths = [
        "Languages/de/SharedRules/default.yaml",
        "Languages/en/SharedRules/default.yaml",
        "Languages/zh/tw/navigate.yaml",
        "Braille/Nemeth/Nemeth_Rules.yaml",
        "Intent/general.yaml",
    ]
    html = render_html({(path, "default", "mi") for path in paths}, {}, [])

    assert 'data-file-search="Languages/de/SharedRules/default.yaml" data-rule-set="de"' in html
    assert 'data-file-search="Languages/en/SharedRules/default.yaml" data-rule-set="en"' in html
    assert 'data-file-search="Languages/zh/tw/navigate.yaml" data-rule-set="zh-tw"' in html
    assert 'data-file-search="Braille/Nemeth/Nemeth_Rules.yaml" data-rule-set="Nemeth"' in html
    assert 'data-file-search="Intent/general.yaml" data-rule-set="Intent"' in html
    assert '<select id="rule-set" aria-label="Filter by language or braille code">' in html
    assert (
        '<optgroup label="Speech languages">\n'
        '<option value="de">de</option><option value="en">en</option><option value="zh-tw">zh-tw</option>\n'
        "</optgroup>"
    ) in html
    assert '<optgroup label="Braille codes">\n<option value="Nemeth">Nemeth</option>' in html
    assert html.index('label="Speech languages"') < html.index('label="Braille codes"') < html.index('label="Other"')
