"""
Data models for the audit tool.

Contains dataclasses for representing rules and comparison results.
"""

from dataclasses import dataclass, field
from enum import StrEnum
from typing import Any


class AuditError(Exception):
    """Raised when the audit encounters a configuration or validation error."""


class IssueType(StrEnum):
    """Top-level issue categories used by the audit renderer."""

    MISSING_RULE = "missing_rule"
    UNTRANSLATED_TEXT = "untranslated_text"
    RULE_DIFFERENCE = "rule_difference"
    EXTRA_RULE = "extra_rule"


class DiffType(StrEnum):
    """Rule-difference subcategories used for fine-grained diagnostics."""

    MATCH = "match"  # `match` XPath differs between English and translation.
    CONDITION = "condition"  # `if` / `test` condition expressions differ.
    VARIABLES = "variables"  # Variable names defined in `variables` differ.
    STRUCTURE = "structure"  # Control-flow block shape/order differs (if/then/else/with/replace).


@dataclass
class UntranslatedEntry:
    """A single untranslated text fragment found in a rule."""

    key: str
    text: str
    line: int | None


@dataclass
class RuleInfo:
    """
    Information about a single rule parsed from a YAML file.

    Attributes
    ----------
    name : str | None
        Rule name for standard rule files; None for unicode entries.
    tag : str | None
        Rule tag (normalized string); None for unicode entries.
    key : str
        Stable identifier used for matching; for unicode entries this is the character or range key.
    line_number : int
        1-based line number where the rule starts in the source file.
    raw_content : str
        Raw YAML block for this rule (used for reporting/snippets).
    data : Any | None
        Parsed YAML node for the rule; used for structural diffs.
    untranslated_entries : list[UntranslatedEntry]
        Entries extracted from lowercase translation keys.
        Preserves exact text fragments and YAML line numbers for diagnostics.
    line_map : dict[str, list[int]]
        Mapping of element type to line numbers for rule components like match,
        conditions, variables, and structural tokens. This is used to point
        structural diffs at a precise line rather than the top of the rule.
    audit_ignore : bool
        True if the raw content contains an audit-ignore marker.
    """

    name: str | None  # None for unicode entries
    tag: str | None  # None for unicode entries
    key: str  # For unicode entries, this is the character/range
    line_number: int
    raw_content: str
    data: Any | None = None
    untranslated_entries: list[UntranslatedEntry] = field(default_factory=list)
    line_map: dict[str, list[int]] = field(default_factory=dict)
    audit_ignore: bool = False

    @property
    def has_untranslated_text(self) -> bool:
        return bool(self.untranslated_entries)

    @property
    def untranslated_keys(self) -> list[str]:
        return [entry.text for entry in self.untranslated_entries]


@dataclass
class RuleDifference:
    """Fine-grained difference between English and translated rule"""

    english_rule: RuleInfo
    translated_rule: RuleInfo
    diff_type: DiffType
    description: str
    english_snippet: str
    translated_snippet: str

    def __post_init__(self) -> None:
        if isinstance(self.diff_type, str):
            self.diff_type = DiffType(self.diff_type)


@dataclass
class ComparisonResult:
    """Results from comparing English and translated files"""

    missing_rules: list[RuleInfo]  # Rules in English but not in translation
    extra_rules: list[RuleInfo]  # Rules in translation but not in English
    untranslated_text: list[tuple[RuleInfo, list[UntranslatedEntry]]]
    english_rule_count: int
    translated_rule_count: int
    rule_differences: list[RuleDifference] = field(default_factory=list)  # Fine-grained diffs

    @property
    def has_issues(self) -> bool:
        return bool(self.missing_rules or self.untranslated_text or self.extra_rules or self.rule_differences)


@dataclass
class AuditSummary:
    """Accumulated totals from a full language audit."""

    files_checked: int
    files_with_issues: int
    files_ok: int
    total_missing: int
    total_untranslated: int
    total_extra: int
    total_differences: int
    total_issues: int
