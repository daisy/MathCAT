"""Object-level golden tests for audit findings."""

from pathlib import Path

from ..auditor import compare_files
from ..line_resolver import resolve_diff_lines


def collect_issue_tuples(language: str = "de", issue_filter: set[str] | None = None) -> list[tuple]:
    base_dir = Path(__file__).parent
    fixtures_dir = base_dir / "fixtures"
    english_dir = fixtures_dir / "en"
    translated_dir = fixtures_dir / language
    rows: list[tuple] = []

    for english_path in sorted(english_dir.glob("*.yaml")):
        file_name = english_path.name
        result = compare_files(
            english_path,
            translated_dir / file_name,
            issue_filter,
        )

        for rule in result.missing_rules:
            rows.append((file_name, "missing_rule", rule.key, "", rule.line_number, None, ""))

        for rule in result.extra_rules:
            rows.append((file_name, "extra_rule", rule.key, "", None, rule.line_number, ""))

        for rule, entries in result.untranslated_text:
            for entry in entries:
                rows.append((file_name, "untranslated_text", rule.key, "", None, entry.line or rule.line_number, entry.text))

        for diff in result.rule_differences:
            lines = resolve_diff_lines(diff)
            if lines is None:
                continue
            line_en, line_tr = lines
            rows.append((file_name, "rule_difference", diff.english_rule.key, diff.diff_type.value, line_en, line_tr, ""))

    return rows


def test_object_findings_match_golden_fixture() -> None:
    expected = [
        ("basic.yaml", "missing_rule", "rule-2|mn", "", 7, None, ""),
        ("basic.yaml", "extra_rule", "rule-3|mo", "", None, 7, ""),
        ("basic.yaml", "untranslated_text", "rule-1|mo", "", None, 5, "equals"),
        ("condition_none.yaml", "rule_difference", "condition-none|mi", "condition", 6, 6, ""),
        ("match.yaml", "rule_difference", "match-rule|mo", "match", 3, 3, ""),
        ("structure.yaml", "rule_difference", "cond-rule|mi", "condition", 6, 6, ""),
        ("structure_diff.yaml", "rule_difference", "struct-rule|mi", "structure", 7, 7, ""),
        ("structure_empty.yaml", "rule_difference", "struct-empty|mi", "structure", 4, 1, ""),
        ("structure_misaligned.yaml", "rule_difference", "misaligned-structure|root", "condition", 6, 6, ""),
        ("structure_misaligned.yaml", "rule_difference", "misaligned-structure|root", "structure", 11, 11, ""),
        ("structure_missing_else.yaml", "rule_difference", "missing-else-block|root", "structure", 7, 7, ""),
        ("unicode.yaml", "missing_rule", "b", "", 3, None, ""),
        ("unicode.yaml", "extra_rule", "c", "", None, 5, ""),
        ("unicode.yaml", "untranslated_text", "a", "", None, 2, "a"),
        ("variables.yaml", "rule_difference", "vars-rule|mo", "variables", 4, 4, ""),
        ("variables_none.yaml", "rule_difference", "vars-none|mo", "variables", 4, 1, ""),
    ]

    assert collect_issue_tuples("de") == expected


def test_object_findings_filter_missing_extra_only() -> None:
    expected = [
        ("basic.yaml", "missing_rule", "rule-2|mn", "", 7, None, ""),
        ("basic.yaml", "extra_rule", "rule-3|mo", "", None, 7, ""),
        ("unicode.yaml", "missing_rule", "b", "", 3, None, ""),
        ("unicode.yaml", "extra_rule", "c", "", None, 5, ""),
    ]
    assert collect_issue_tuples("de", {"missing", "extra"}) == expected
