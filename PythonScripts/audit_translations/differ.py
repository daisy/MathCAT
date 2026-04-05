"""
Rule diffing logic.

Compares English and translated rules to find fine-grained structural differences.
"""

from .extractors import (
    extract_conditions,
    extract_match_pattern,
    extract_structure_elements,
    extract_variables,
    normalize_xpath,
)
from .models import DiffType, RuleDifference, RuleInfo


def dedup_list(values: list[str]) -> list[str]:
    """
    Return a list without duplicates while preserving first-seen order.
    Originally, rule differences were stored as sets, losing their original order,
    which is not helpful and why it changed with the help of this function.

    Example:
        >>> dedup_list(["if:a", "if:b", "if:a"])
        ['if:a', 'if:b']
    """
    return list(dict.fromkeys(values))  # dict preserves insertion order (guaranteed in Python 3.7+)


def diff_rules(english_rule: RuleInfo, translated_rule: RuleInfo) -> list[RuleDifference]:
    """
    Compare two rules and return fine-grained differences.
    Ignores text content differences (T/t values) but catches structural changes.
    """
    differences: list[RuleDifference] = []

    def add_difference(diff_type: DiffType, description: str, english_snippet: str, translated_snippet: str) -> None:
        differences.append(
            RuleDifference(
                english_rule,
                translated_rule,
                diff_type,
                description,
                english_snippet,
                translated_snippet,
            )
        )

    # Check match pattern differences
    en_match_raw = extract_match_pattern(english_rule.data)
    tr_match_raw = extract_match_pattern(translated_rule.data)
    en_match = normalize_xpath(en_match_raw)
    tr_match = normalize_xpath(tr_match_raw)
    if en_match != tr_match and en_match and tr_match:
        add_difference(DiffType.MATCH, "Match pattern differs", en_match, tr_match)

    # Check condition differences
    en_conditions_raw = extract_conditions(english_rule.data)
    tr_conditions_raw = extract_conditions(translated_rule.data)
    en_conditions = [normalize_xpath(c) for c in en_conditions_raw]
    tr_conditions = [normalize_xpath(c) for c in tr_conditions_raw]
    if en_conditions != tr_conditions:
        # Find specific differences
        en_set, tr_set = set(en_conditions), set(tr_conditions)
        if en_set != tr_set:
            add_difference(
                DiffType.CONDITION,
                "Conditions differ",
                ", ".join(dedup_list(en_conditions)) or "(none)",
                ", ".join(dedup_list(tr_conditions)) or "(none)",
            )

    # Check variable differences
    en_vars = extract_variables(english_rule.data)
    tr_vars = extract_variables(translated_rule.data)
    if en_vars != tr_vars:
        en_var_names = {v[0] for v in en_vars}
        tr_var_names = {v[0] for v in tr_vars}
        if en_var_names != tr_var_names:
            add_difference(
                DiffType.VARIABLES,
                "Variable definitions differ",
                ", ".join(sorted(en_var_names)) or "(none)",
                ", ".join(sorted(tr_var_names)) or "(none)",
            )

    # Check structural differences (test/if/then/else blocks)
    en_structure = extract_structure_elements(english_rule.data)
    tr_structure = extract_structure_elements(translated_rule.data)
    if en_structure != tr_structure:
        add_difference(
            DiffType.STRUCTURE,
            "Rule structure differs (test/if/then/else blocks)",
            " ".join(en_structure),
            " ".join(tr_structure),
        )

    return differences
