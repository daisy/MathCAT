"""
Rich console rendering.

Handles rich terminal display for grouped translation issues.
"""

from pathlib import Path
from typing import Any

from rich.console import Console
from rich.markup import escape
from rich.panel import Panel
from rich.table import Table

from .line_resolver import resolve_diff_lines
from .models import AuditSummary, ComparisonResult, DiffType, IssueType, RuleDifference, RuleInfo

console = Console()


type IssueGroupKey = tuple[IssueType, DiffType | None]


def issue_group_key(issue_type: IssueType, diff_type: DiffType | None = None) -> IssueGroupKey:
    return issue_type, diff_type


ISSUE_GROUP_SPECS: list[tuple[IssueGroupKey, str]] = [
    (issue_group_key(IssueType.MISSING_RULE), "Missing in Translation"),
    (issue_group_key(IssueType.UNTRANSLATED_TEXT), "Untranslated Text"),
    (issue_group_key(IssueType.RULE_DIFFERENCE, DiffType.MATCH), "Match Pattern Differences"),
    (issue_group_key(IssueType.RULE_DIFFERENCE, DiffType.CONDITION), "Condition Differences"),
    (issue_group_key(IssueType.RULE_DIFFERENCE, DiffType.VARIABLES), "Variable Differences"),
    (issue_group_key(IssueType.RULE_DIFFERENCE, DiffType.STRUCTURE), "Structure Differences"),
    (issue_group_key(IssueType.EXTRA_RULE), "Extra in Translation"),
]
ISSUE_GROUP_LABELS = {key: label for key, label in ISSUE_GROUP_SPECS}


def rule_label(rule: RuleInfo) -> str:
    if rule.name is None:
        return f'[yellow]"{escape(rule.key)}"[/]'
    tag = rule.tag or "unknown"
    return f"[cyan]{escape(rule.name)}[/] [dim]({escape(tag)})[/]"


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

    style, icon = (
        ("green", "✓")
        if result.translated_rule_count == result.english_rule_count
        else ("red", "✗")
        if result.translated_rule_count == 0
        else ("yellow", "⚠")
    )
    console.print()
    console.rule(style="cyan")
    console.print(f"[{style}]{icon}[/] [bold]{escape(display_name)}[/]")
    console.print(f"  [dim]English: {result.english_rule_count} rules  →  Translated: {result.translated_rule_count} rules[/]")
    console.rule(style="cyan")

    grouped_issues: dict[str, dict[str, Any]] = {}

    def add_issue(rule: RuleInfo, group_key: IssueGroupKey, payload: dict[str, Any]) -> None:
        if rule.key not in grouped_issues:
            grouped_issues[rule.key] = {"rule": rule, "by_type": {}}
        grouped_issues[rule.key]["by_type"].setdefault(group_key, []).append(payload)

    for rule in result.missing_rules:
        add_issue(rule, issue_group_key(IssueType.MISSING_RULE), {"line_en": rule.line_number})

    for rule, entries in result.untranslated_text:
        for entry in entries:
            add_issue(
                rule,
                issue_group_key(IssueType.UNTRANSLATED_TEXT),
                {"line_tr": entry.line or rule.line_number, "text": entry.text},
            )

    for diff in result.rule_differences:
        lines = resolve_diff_lines(diff)
        if lines is None:
            continue
        line_en, line_tr = lines
        add_issue(
            diff.english_rule,
            issue_group_key(IssueType.RULE_DIFFERENCE, diff.diff_type),
            {"line_en": line_en, "line_tr": line_tr, "diff": diff},
        )

    for rule in result.extra_rules:
        add_issue(rule, issue_group_key(IssueType.EXTRA_RULE), {"line_tr": rule.line_number})

    if grouped_issues:
        total_grouped_issues = sum(len(entries) for group in grouped_issues.values() for entries in group["by_type"].values())
        console.print(
            f"\n  [magenta]≠[/] [bold]Rule Issues[/] "
            f"[[magenta]{total_grouped_issues}[/]] [dim](grouped by rule and issue type)[/]"
        )
        for group in grouped_issues.values():
            rule = group["rule"]
            by_type: dict[IssueGroupKey, list[dict[str, Any]]] = group["by_type"]
            console.print(f"      [dim]•[/] {rule_label(rule)}")
            ordered_group_keys = [group_key for group_key, _ in ISSUE_GROUP_SPECS if group_key in by_type]
            for group_key in ordered_group_keys:
                entries = by_type[group_key]
                issue_type, _diff_type = group_key
                console.print(f"          [dim]{ISSUE_GROUP_LABELS[group_key]} [{len(entries)}][/]")
                for entry in entries:
                    if issue_type is IssueType.MISSING_RULE:
                        console.print(f"              [dim]•[/] [dim](line {entry['line_en']} in English)[/]")
                    elif issue_type is IssueType.EXTRA_RULE:
                        console.print(f"              [dim]•[/] [dim](line {entry['line_tr']} in {target_label})[/]")
                    elif issue_type is IssueType.UNTRANSLATED_TEXT:
                        console.print(
                            f"              [dim]•[/] [dim](line {entry['line_tr']} {target_label})[/] "
                            f'[yellow]"{escape(entry["text"])}"[/]'
                        )
                    else:
                        diff: RuleDifference = entry["diff"]
                        console.print(
                            f"              [dim]•[/] [dim](line {entry['line_en']} en, {entry['line_tr']} {target_label})[/]"
                        )
                        console.print(f"                  [dim]{diff.description}[/]")
                        if verbose:
                            console.print(f"                  [green]en:[/] {escape(diff.english_snippet)}")
                            console.print(f"                  [red]{target_label}:[/] {escape(diff.translated_snippet)}")
                issues += len(entries)

    return issues


GREEN_FILE_COUNT_THRESHOLD = 7
YELLOW_FILE_COUNT_THRESHOLD = 4


def file_count_color(file_count: int) -> str:
    """Map number of translated YAML files to a display color."""
    if file_count >= GREEN_FILE_COUNT_THRESHOLD:
        return "green"
    if file_count >= YELLOW_FILE_COUNT_THRESHOLD:
        return "yellow"
    return "red"


def print_audit_header(language: str, file_count: int) -> None:
    """Print the audit header panel."""
    console.print(Panel(f"MathCAT Translation Audit: {language.upper()}", style="bold cyan"))
    console.print("\n  [dim]Comparing against English (en) reference files[/]")
    console.print(f"  [dim]Files to check: {file_count}[/]")


def print_audit_summary(summary: AuditSummary) -> None:
    """Print the audit summary table."""
    table = Table(title="SUMMARY", title_style="bold", box=None, show_header=False, padding=(0, 2))
    table.add_column(width=30)
    table.add_column()
    for label, value, color in [
        ("Files checked", summary.files_checked, None),
        ("Files with issues", summary.files_with_issues, "yellow" if summary.files_with_issues else "green"),
        ("Files OK", summary.files_ok, "green" if summary.files_ok else None),
        ("Missing rules", summary.total_missing, "red" if summary.total_missing else "green"),
        ("Untranslated text", summary.total_untranslated, "yellow" if summary.total_untranslated else "green"),
        ("Rule differences", summary.total_differences, "magenta" if summary.total_differences else "green"),
        ("Extra rules", summary.total_extra, "blue" if summary.total_extra else None),
    ]:
        table.add_row(label, f"[{color}]{value}[/]" if color else str(value))
    console.print(Panel(table, style="cyan"))


def print_language_list(languages: list[tuple[str, int]]) -> None:
    """Print the available languages table.

    Args:
        languages: List of (language_code, yaml_file_count) tuples.
    """
    console.print(Panel("Available Languages", style="bold cyan"))

    table = Table(show_header=True, header_style="dim")
    table.add_column("Language", justify="center", style="cyan")
    table.add_column("YAML files", justify="right")

    for code, count in languages:
        color = file_count_color(count)
        table.add_row(code, f"[{color}]{count}[/] files")

    console.print(table)
    console.print("\n  [dim]Reference: en (English) - base translation[/]\n")
