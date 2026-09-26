"""Checks the rule coverage command without rerunning the Rust test suite."""

import json
import subprocess
import sys
from pathlib import Path

import pytest

from .. import cli
from ..rulecoverage import rule_coverage


def test_coverage_command_generates_reports_and_opens_browser(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    """The CLI uses test events to write both reports and opens the HTML page."""
    output = tmp_path / "target" / "rule-coverage"
    monkeypatch.setattr(rule_coverage, "ROOT", tmp_path)
    monkeypatch.setattr(rule_coverage, "OUTPUT", output)
    monkeypatch.setattr(rule_coverage, "EVENTS", output / "events")

    def fake_cargo(command: list[str], **kwargs: object) -> subprocess.CompletedProcess[str]:
        assert command == ["cargo", "test", "--features", "rule-coverage"]
        assert kwargs["cwd"] == tmp_path
        events = [
            {"kind": "loaded", "path": "Languages/en/SimpleSpeak_Rules.yaml"},
            {"kind": "loaded", "path": "Languages/en/SharedRules/empty.yaml"},
            {"kind": "loaded", "path": "Languages/en/definitions.yaml"},
            {"kind": "defined-rule", "path": "Languages/en/SimpleSpeak_Rules.yaml", "name": "simple", "tag": "mi"},
            {"kind": "defined-rule", "path": "Languages/en/SimpleSpeak_Rules.yaml", "name": "default", "tag": "mi"},
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
        return subprocess.CompletedProcess(command, 0)

    opened: list[str] = []
    monkeypatch.setattr(rule_coverage.subprocess, "run", fake_cargo)
    monkeypatch.setattr(rule_coverage.webbrowser, "open", lambda url: opened.append(url) or True)
    monkeypatch.setattr(sys, "argv", ["audit-translations", "--rule-coverage"])

    with pytest.raises(SystemExit) as result:
        cli.main()

    assert result.value.code == 0
    assert opened == [(output / "index.html").as_uri()]
    assert "Status: **Complete**" in (output / "report.md").read_text(encoding="utf-8")
    assert "## Matched rules (1)" in (output / "report.md").read_text(encoding="utf-8")
    assert "## Active rules with no match (1)" in (output / "report.md").read_text(encoding="utf-8")
    assert "3 hits; tests: `test_alpha` (2), `test_beta` (1)" in (output / "report.md").read_text(encoding="utf-8")
    html = (output / "index.html").read_text(encoding="utf-8")
    assert 'data-rule-search="simple mi" data-status="matched"' in html
    assert 'data-rule-search="default mi" data-status="unmatched"' in html
    assert "1/2 (50%) active rules matched" in html
    assert "1/2 (50%) rules" in html
    assert 'role="tooltip">Tests:\ntest_alpha (2 hits)\ntest_beta (1 hit)</span>' in html
    assert ".rules li:hover .rule-tooltip, .rules li:focus .rule-tooltip { display: block; }" in html
    assert 'aria-describedby="rule-tooltip-1-1"' in html
    assert "3 hits</span>" in html
    assert "0 hits</span>" in html
    assert "No active pattern rules" in html
    assert (output / "test.log").is_file()


def test_failed_coverage_run_opens_incomplete_report(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    """A failed test run remains unsuccessful but still produces and opens a useful report."""
    output = tmp_path / "target" / "rule-coverage"
    monkeypatch.setattr(rule_coverage, "ROOT", tmp_path)
    monkeypatch.setattr(rule_coverage, "OUTPUT", output)
    monkeypatch.setattr(rule_coverage, "EVENTS", output / "events")
    monkeypatch.setattr(
        rule_coverage.subprocess,
        "run",
        lambda command, **kwargs: subprocess.CompletedProcess(command, 1),
    )
    opened: list[str] = []
    monkeypatch.setattr(rule_coverage.webbrowser, "open", lambda url: opened.append(url) or True)

    assert rule_coverage.run() == 1
    assert opened == [(output / "index.html").as_uri()]
    assert "Status: **Incomplete**" in (output / "report.md").read_text(encoding="utf-8")
    assert "No loaded YAML events found" in (output / "index.html").read_text(encoding="utf-8")


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

    loaded, matched, defined, rule_hits, rule_tests, errors = rule_coverage.read_events()

    assert loaded == matched == set()
    assert rule_hits == rule_tests == {}
    assert defined == {(path, name, "mfrac")}
    assert errors == []
