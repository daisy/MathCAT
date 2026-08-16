"""Models for aggregate audit output."""

from dataclasses import dataclass


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
    total_missing_definitions: int
    total_extra_definitions: int
    total_definition_type_mismatches: int
    total_issues: int
