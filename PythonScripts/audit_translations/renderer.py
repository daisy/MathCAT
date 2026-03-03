"""
Rich console rendering and issue serialization.

Handles all output concerns: rich terminal display, JSONL issue normalization,
and the IssueWriter interface.
"""

import json
from pathlib import Path
from typing import Any

from rich.console import Console
from rich.markup import escape

from .dataclasses import ComparisonResult, RuleDifference, RuleInfo
from .line_resolver import resolve_diff_lines

console = Console()


def rule_label(rule: RuleInfo) -> str:
    if rule.name is None:
        return f"[yellow]\"{escape(rule.key)}\"[/]"
    tag = rule.tag or "unknown"
    return f"[cyan]{escape(rule.name)}[/] [dim]({escape(tag)})[/]"


def issue_type_sort_key(issue_type: str) -> tuple[int, str]:
    """
    Stable ordering for per-rule issue groups.

    The first tuple element defines user-facing priority (missing/untranslated/
    match/condition/variables/structure/extra). The second element keeps sorting
    deterministic for unknown keys.
    """
    order = {
        "missing_rule": 0,
        "untranslated_text": 1,
        "rule_difference:match": 2,
        "rule_difference:condition": 3,
        "rule_difference:variables": 4,
        "rule_difference:structure": 5,
        "extra_rule": 6,
    }
    return order.get(issue_type, 99), issue_type


def issue_type_label(issue_type: str) -> str:
    """
    Return the display label used in rich grouped output.

    Unknown issue types fall back to their raw key so renderer behavior remains
    robust when new categories are introduced.
    """
    labels = {
        "missing_rule": "Missing in Translation",
        "untranslated_text": "Untranslated Text",
        "rule_difference:match": "Match Pattern Differences",
        "rule_difference:condition": "Condition Differences",
        "rule_difference:variables": "Variable Differences",
        "rule_difference:structure": "Structure Differences",
        "extra_rule": "Extra in Translation",
    }
    return labels.get(issue_type, issue_type)


def issue_base(rule: RuleInfo, file_name: str, language: str) -> dict:
    return {
        "language": language,
        "file": Path(file_name).as_posix(),
        "rule_name": rule.name or "",
        "rule_tag": rule.tag or "",
        "rule_key": rule.key,
        "issue_line_en": None,
        "issue_line_tr": None,
        "rule_line_en": None,
        "rule_line_tr": None,
    }


def collect_issues(
    result: ComparisonResult,
    file_name: str,
    language: str,
) -> list[dict]:
    """
    Flatten a ComparisonResult into one normalized dictionary per issue.

    This is the canonical bridge from parser/diff objects to serializable
    records consumed by JSONL output, snapshot tests, and line-level assertions.
    """
    issues = []

    for rule in result.missing_rules:
        issue = issue_base(rule, file_name, language)
        issue.update(
            issue_type="missing_rule",
            diff_type="",
            issue_line_en=rule.line_number,
            rule_line_en=rule.line_number,
            description="Rule present in English but missing in translation",
            english_snippet="",
            translated_snippet="",
            untranslated_texts=[],
        )
        issues.append(issue)

    for rule in result.extra_rules:
        issue = issue_base(rule, file_name, language)
        issue.update(
            issue_type="extra_rule",
            diff_type="",
            issue_line_tr=rule.line_number,
            rule_line_tr=rule.line_number,
            description="Rule present in translation but missing in English",
            english_snippet="",
            translated_snippet="",
            untranslated_texts=[],
        )
        issues.append(issue)

    for rule, entries in result.untranslated_text:
        for key, text, line in entries:
            issue = issue_base(rule, file_name, language)
            issue.update(
                issue_type="untranslated_text",
                diff_type="",
                issue_line_tr=line or rule.line_number,
                rule_line_tr=rule.line_number,
                description="Lowercase t/ot/ct keys indicate untranslated text",
                english_snippet="",
                translated_snippet="",
                untranslated_texts=[text],
            )
            issues.append(issue)

    for diff in result.rule_differences:
        lines = resolve_diff_lines(diff)
        if lines is None:
            continue
        issue_line_en, issue_line_tr = lines
        issue = issue_base(diff.english_rule, file_name, language)
        issue.update(
            issue_type="rule_difference",
            diff_type=diff.diff_type,
            issue_line_en=issue_line_en,
            issue_line_tr=issue_line_tr,
            rule_line_en=diff.english_rule.line_number,
            rule_line_tr=diff.translated_rule.line_number,
            description=diff.description,
            english_snippet=diff.english_snippet,
            translated_snippet=diff.translated_snippet,
            untranslated_texts=[],
        )
        issues.append(issue)

    return issues


def print_warnings(
    result: ComparisonResult,
    file_name: str,
    verbose: bool = False,
    target_language: str = "tr",
) -> int:
    """Print warnings to console. Returns count of issues found."""
    issues = 0
    display_name = Path(file_name).as_posix()
    target_label = target_language.lower().replace("_", "-")

    if not result.has_issues:
        return issues

    style, icon = ("green", "✓") if result.translated_rule_count == result.english_rule_count else \
                  ("red", "✗") if result.translated_rule_count == 0 else ("yellow", "⚠")
    console.print()
    console.rule(style="cyan")
    console.print(f"[{style}]{icon}[/] [bold]{escape(display_name)}[/]")
    console.print(f"  [dim]English: {result.english_rule_count} rules  →  Translated: {result.translated_rule_count} rules[/]")
    console.rule(style="cyan")

    grouped_issues: dict[str, dict[str, Any]] = {}

    def add_issue(rule: RuleInfo, issue_type: str, payload: dict[str, Any]) -> None:
        if rule.key not in grouped_issues:
            grouped_issues[rule.key] = {"rule": rule, "by_type": {}}
        grouped_issues[rule.key]["by_type"].setdefault(issue_type, []).append(payload)

    for rule in result.missing_rules:
        add_issue(rule, "missing_rule", {"line_en": rule.line_number})

    for rule, entries in result.untranslated_text:
        for _, text, line in entries:
            add_issue(rule, "untranslated_text", {"line_tr": line or rule.line_number, "text": text})

    for diff in result.rule_differences:
        lines = resolve_diff_lines(diff)
        if lines is None:
            continue
        line_en, line_tr = lines
        add_issue(
            diff.english_rule,
            f"rule_difference:{diff.diff_type}",
            {"line_en": line_en, "line_tr": line_tr, "diff": diff},
        )

    for rule in result.extra_rules:
        add_issue(rule, "extra_rule", {"line_tr": rule.line_number})

    if grouped_issues:
        total_grouped_issues = sum(
            len(entries)
            for group in grouped_issues.values()
            for entries in group["by_type"].values()
        )
        console.print(
            f"\n  [magenta]≠[/] [bold]Rule Issues[/] "
            f"[[magenta]{total_grouped_issues}[/]] [dim](grouped by rule and issue type)[/]"
        )
        for group in grouped_issues.values():
            rule = group["rule"]
            by_type: dict[str, list[dict[str, Any]]] = group["by_type"]
            console.print(f"      [dim]•[/] {rule_label(rule)}")
            for issue_type in sorted(by_type.keys(), key=issue_type_sort_key):
                entries = by_type[issue_type]
                console.print(
                    f"          [dim]{issue_type_label(issue_type)} "
                    f"[{len(entries)}][/]"
                )
                for entry in entries:
                    if issue_type == "missing_rule":
                        console.print(
                            f"              [dim]•[/] [dim](line {entry['line_en']} in English)[/]"
                        )
                        issues += 1
                    elif issue_type == "extra_rule":
                        console.print(
                            f"              [dim]•[/] [dim](line {entry['line_tr']} in {target_label})[/]"
                        )
                        issues += 1
                    elif issue_type == "untranslated_text":
                        console.print(
                            f"              [dim]•[/] [dim](line {entry['line_tr']} {target_label})[/] "
                            f"[yellow]\"{escape(entry['text'])}\"[/]"
                        )
                        issues += 1
                    else:
                        diff: RuleDifference = entry["diff"]
                        console.print(
                            f"              [dim]•[/] [dim](line {entry['line_en']} en, {entry['line_tr']} {target_label})[/]"
                        )
                        console.print(f"                  [dim]{diff.description}[/]")
                        if verbose:
                            console.print(f"                  [green]en:[/] {escape(diff.english_snippet)}")
                            console.print(f"                  [red]{target_label}:[/] {escape(diff.translated_snippet)}")
                        issues += 1

    return issues
