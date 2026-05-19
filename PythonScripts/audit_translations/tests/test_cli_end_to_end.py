"""
CLI coverage tests for audit_translations.
"""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

import pytest

from .. import cli as audit_cli
from ..renderer import console


def fixture_rules_dir() -> Path:
    return Path(__file__).resolve().parent / "fixtures" / "Rules" / "Languages"


def test_cli_main_rich_only_filters_issue_groups(capsys, monkeypatch) -> None:
    """
    Ensure --only also filters visible rich subgroup sections.

    We expect missing/extra groups to remain while untranslated and all diff
    subgroup labels are omitted from the rendered output.
    """
    args = ["es", "--rules-dir", str(fixture_rules_dir()), "--only", "missing,extra"]

    old_width = console.width
    console.width = 80
    try:
        monkeypatch.setattr(sys, "argv", ["audit_translations", *args])
        audit_cli.main()
        output = capsys.readouterr().out
    finally:
        console.width = old_width

    assert "Missing in Translation" in output
    assert "Extra in Translation" in output
    assert "Untranslated Text" not in output
    assert "Match Pattern Differences" not in output
    assert "Condition Differences" not in output
    assert "Variable Differences" not in output
    assert "Structure Differences" not in output


def test_cli_main_rich_output_groups_by_rule_and_type(capsys, monkeypatch) -> None:
    """
    Ensure rich CLI output is grouped by rule and subgrouped by issue type.

    This is a behavioral assertion test (not snapshot-based): it checks that
    core grouping markers and subgroup ordering are visible in user-facing CLI
    output for a representative fixture file.
    """
    args = ["es", "--rules-dir", str(fixture_rules_dir()), "--file", "SharedRules/calculus.yaml", "--verbose"]

    old_width = console.width
    console.width = 80
    try:
        monkeypatch.setattr(sys, "argv", ["audit_translations", *args])
        audit_cli.main()
        output = capsys.readouterr().out
    finally:
        console.width = old_width

    assert "≠ Rule Issues [13] (grouped by rule and issue type)" in output
    assert "• divergence (divergence)" in output
    assert "Untranslated Text [3]" in output
    assert "Match Pattern Differences [1]" in output
    assert "Condition Differences [1]" in output

    untranslated_idx = output.index("Untranslated Text [3]")
    match_idx = output.index("Match Pattern Differences [1]")
    condition_idx = output.index("Condition Differences [1]")
    assert untranslated_idx < match_idx < condition_idx


def test_cli_main_rich_output_matches_grouped_golden(capsys, monkeypatch) -> None:
    """
    Ensure rich CLI grouped rendering stays stable for a multi-rule fixture.

    The golden file captures overall visual layout so formatting regressions in
    grouped sections are caught even when functional issue counts stay the same.
    """
    args = ["es", "--rules-dir", str(fixture_rules_dir()), "--file", "SharedRules/calculus.yaml", "--verbose"]
    golden_path = Path(__file__).resolve().parent / "golden" / "rich" / "cli_calculus_verbose.golden"

    old_width = console.width
    console.width = 80

    try:
        monkeypatch.setattr(sys, "argv", ["audit_translations", *args])
        audit_cli.main()
        output = capsys.readouterr().out
    finally:
        console.width = old_width

    assert output == golden_path.read_text(encoding="utf-8")


def test_cli_main_requires_language_or_list(capsys, monkeypatch) -> None:
    """
    Ensure CLI exits with a clear error when neither language nor --list is set.

    This protects the expected help/error UX for accidental empty invocations.
    """
    monkeypatch.setattr(sys, "argv", ["audit_translations"])

    with pytest.raises(SystemExit) as exc:
        audit_cli.main()
    output = capsys.readouterr().out

    assert exc.value.code == 1
    assert "Please specify a language code or use --list" in output


def test_cli_main_rejects_unknown_only_token(capsys, monkeypatch) -> None:
    """
    Ensure unsupported --only tokens are rejected before audit execution.

    This keeps filter behavior explicit and prevents silently ignored typos.
    """
    args = ["es", "--rules-dir", str(fixture_rules_dir()), "--only", "missing,bogus"]
    monkeypatch.setattr(sys, "argv", ["audit_translations", *args])

    with pytest.raises(SystemExit) as exc:
        audit_cli.main()
    output = capsys.readouterr().out

    assert exc.value.code == 1
    assert "Unknown issue types: bogus" in output


def test_cli_main_reports_missing_region_directory(capsys, monkeypatch) -> None:
    """
    Ensure region variants fail fast when the requested subdirectory is absent.

    This validates the error path for languages like es-mx when only es exists.
    """
    args = ["es-mx", "--rules-dir", str(fixture_rules_dir())]
    monkeypatch.setattr(sys, "argv", ["audit_translations", *args])

    with pytest.raises(SystemExit) as exc:
        audit_cli.main()
    output = capsys.readouterr().out

    assert exc.value.code == 1
    assert "Region directory not found" in output


def test_cli_module_rich_output_groups_by_rule_and_type() -> None:
    """
    Ensure `python -m audit_translations` rich output also shows grouped sections.

    This complements the in-process CLI test by validating module execution in
    a subprocess with environment wiring and terminal width constraints.
    """
    args = ["es", "--rules-dir", str(fixture_rules_dir()), "--file", "SharedRules/calculus.yaml", "--verbose"]

    python_scripts_dir = Path(__file__).resolve().parents[2]
    env = os.environ.copy()
    env["PYTHONPATH"] = os.pathsep.join([str(python_scripts_dir), env.get("PYTHONPATH", "")]).strip(os.pathsep)
    env["COLUMNS"] = "80"

    result = subprocess.run(
        [sys.executable, "-m", "audit_translations", *args],
        capture_output=True,
        text=True,
        cwd=str(python_scripts_dir),
        env=env,
        check=True,
    )

    output = result.stdout
    assert "≠ Rule Issues [13] (grouped by rule and issue type)" in output
    assert "• laplacian (laplacian)" in output
    assert "• divergence (divergence)" in output
    assert "Structure Differences [1]" in output
