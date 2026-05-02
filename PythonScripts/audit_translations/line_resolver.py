"""
Line number resolution for rule differences.

Maps rule diff types and structure tokens to precise YAML source line numbers.
"""

from .extractors import extract_structure_elements
from .models import DiffType, RuleDifference, RuleInfo


def _get_line_map_lines(rule: RuleInfo, kind: DiffType, token: str | None = None) -> list[int]:
    """Return the line-number list for a given element kind from the rule's line map."""
    if kind in (DiffType.MATCH, DiffType.CONDITION, DiffType.VARIABLES):
        return rule.line_map.get(kind, [])
    if kind == DiffType.STRUCTURE and token:
        return rule.line_map.get(f"structure:{token.rstrip(':')}", [])
    return []


def first_structure_mismatch(
    english_tokens: list[str],
    translated_tokens: list[str],
) -> tuple[str | None, str | None, int]:
    """
    Find the first structural mismatch between two token lists.

    Returns (en_token, tr_token, mismatch_position).
    Position is the index in the token list where they first differ.
    """
    min_len = min(len(english_tokens), len(translated_tokens))
    for idx in range(min_len):
        if english_tokens[idx] != translated_tokens[idx]:
            return english_tokens[idx], translated_tokens[idx], idx
    if len(english_tokens) > min_len:
        return english_tokens[min_len], None, min_len
    if len(translated_tokens) > min_len:
        return None, translated_tokens[min_len], min_len
    return None, None, -1


def resolve_issue_line_at_position(
    rule: RuleInfo,
    kind: DiffType,
    token: str | None = None,
    position: int = 0,
) -> int | None:
    """
    Resolve the line number for a specific occurrence of an element within a rule.

    Args:
        rule: The rule to search in
        kind: The kind of element ('match', 'condition', 'variables', 'structure')
        token: For structure kind, the specific token to find
        position: The occurrence index (0 for first, 1 for second, etc.)

    Returns:
        The line number if found, None if the element doesn't exist at that position.
    """
    lines = _get_line_map_lines(rule, kind, token)
    return lines[position] if position < len(lines) else None


def resolve_issue_line(rule: RuleInfo, kind: DiffType, token: str | None = None) -> int | None:
    """
    Resolve the line number for an issue within a rule.

    Returns the line number if found, None if the element doesn't exist in the rule.
    For 'structure' kind with a missing token, returns None instead of falling back
    to rule.line_number to avoid misleading line numbers when elements are missing.
    """
    lines = _get_line_map_lines(rule, kind, token)
    if kind == DiffType.STRUCTURE and token:
        return lines[0] if lines else None
    return lines[0] if lines else rule.line_number


def structure_token_occurrence_index(tokens: list[str], position: int) -> int | None:
    """
    Return which occurrence of a token appears at a given absolute token position.

    Example: for ["test:", "if:", "test:"], position 2 returns 1.
    """
    if position < 0 or position >= len(tokens):
        return None
    token = tokens[position]
    return sum(1 for current in tokens[:position] if current == token)


def resolve_structure_issue_lines(diff: RuleDifference) -> tuple[int, int] | None:
    """
    Resolve stable line anchors for a structural rule difference.

    Strategy:
    - Use position-aware token occurrence matching when possible.
    - For insert/delete cases (one side missing token), anchor to the previous
      shared structural token; if unavailable, anchor to `replace:`.
    """
    en_tokens = extract_structure_elements(diff.english_rule.data)
    tr_tokens = extract_structure_elements(diff.translated_rule.data)
    en_token, tr_token, mismatch_pos = first_structure_mismatch(en_tokens, tr_tokens)

    if mismatch_pos < 0:
        return None

    # Insertion/deletion: anchor to the previous shared token if possible.
    if en_token is None or tr_token is None:
        anchor_pos = mismatch_pos - 1
        if (
            anchor_pos >= 0
            and anchor_pos < len(en_tokens)
            and anchor_pos < len(tr_tokens)
            and en_tokens[anchor_pos] == tr_tokens[anchor_pos]
        ):
            anchor_token = en_tokens[anchor_pos]
            en_occ = structure_token_occurrence_index(en_tokens, anchor_pos)
            tr_occ = structure_token_occurrence_index(tr_tokens, anchor_pos)
            if en_occ is not None and tr_occ is not None:
                line_en = resolve_issue_line_at_position(diff.english_rule, DiffType.STRUCTURE, anchor_token, en_occ)
                line_tr = resolve_issue_line_at_position(diff.translated_rule, DiffType.STRUCTURE, anchor_token, tr_occ)
                if line_en is not None and line_tr is not None:
                    return line_en, line_tr

        # Fallback: anchor both sides to replace, which is the rule body entrypoint.
        line_en = resolve_issue_line(diff.english_rule, DiffType.STRUCTURE, "replace:") or diff.english_rule.line_number
        line_tr = resolve_issue_line(diff.translated_rule, DiffType.STRUCTURE, "replace:") or diff.translated_rule.line_number
        return line_en, line_tr

    # Exact token available on both sides: resolve by occurrence index at mismatch.
    en_occ = structure_token_occurrence_index(en_tokens, mismatch_pos)
    tr_occ = structure_token_occurrence_index(tr_tokens, mismatch_pos)
    if en_occ is not None and tr_occ is not None:
        line_en = resolve_issue_line_at_position(diff.english_rule, DiffType.STRUCTURE, en_token, en_occ)
        line_tr = resolve_issue_line_at_position(diff.translated_rule, DiffType.STRUCTURE, tr_token, tr_occ)
        if line_en is not None and line_tr is not None:
            return line_en, line_tr

    line_en = resolve_issue_line(diff.english_rule, DiffType.STRUCTURE, en_token)
    line_tr = resolve_issue_line(diff.translated_rule, DiffType.STRUCTURE, tr_token)
    if line_en is None or line_tr is None:
        return None
    return line_en, line_tr


def resolve_diff_lines(diff: RuleDifference) -> tuple[int | None, int | None] | None:
    """
    Resolve issue line numbers for a rule difference.

    Returns (line_en, line_tr), or None only for unresolvable structure diffs.
    This is the single entry point used by the renderer to avoid duplicating
    the structure vs non-structure branching logic.
    """
    if diff.diff_type == DiffType.STRUCTURE:
        return resolve_structure_issue_lines(diff)
    return (
        resolve_issue_line(diff.english_rule, diff.diff_type),
        resolve_issue_line(diff.translated_rule, diff.diff_type),
    )
