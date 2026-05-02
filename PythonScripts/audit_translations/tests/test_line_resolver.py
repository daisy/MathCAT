"""
Unit tests for line_resolver.py.
"""

from ..line_resolver import first_structure_mismatch, resolve_diff_lines
from ..models import RuleDifference, RuleInfo


def _make_rule(name: str, line_map: dict, line_number: int = 1) -> RuleInfo:
    return RuleInfo(
        name=name,
        tag="mo",
        key=f"{name}|mo",
        line_number=line_number,
        raw_content="",
        line_map=line_map,
    )


def test_first_structure_mismatch_marks_missing_token_as_none():
    """
    When English has a token that the translation omits, the mismatch position
    points to the extra English token and tr_token is None.
    """
    en = ["test:", "if:", "then:", "else:"]
    tr = ["test:", "if:", "then:"]

    en_token, tr_token, pos = first_structure_mismatch(en, tr)

    assert pos == 3
    assert en_token == "else:"
    assert tr_token is None


def test_resolve_diff_lines_uses_line_map_and_falls_back_to_rule_line():
    """
    For non-structure diffs, resolve_diff_lines reads from the line_map when
    the element is present, and falls back to rule.line_number when it is not.
    """
    en_with_map = _make_rule("r", {"match": [10]}, line_number=5)
    tr_with_map = _make_rule("r", {"match": [20]}, line_number=15)
    en_no_map = _make_rule("r", {}, line_number=5)
    tr_no_map = _make_rule("r", {}, line_number=15)

    def make_match_diff(en, tr):
        return RuleDifference(
            english_rule=en,
            translated_rule=tr,
            diff_type="match",
            description="Match differs",
            english_snippet="a",
            translated_snippet="b",
        )

    assert resolve_diff_lines(make_match_diff(en_with_map, tr_with_map)) == (10, 20)
    assert resolve_diff_lines(make_match_diff(en_no_map, tr_no_map)) == (5, 15)
