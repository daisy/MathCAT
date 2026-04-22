"""
Tests for differ.py.
"""

from ..differ import diff_rules
from ..models import RuleDifference, RuleInfo


def make_rule(name: str, tag: str, data) -> RuleInfo:
    """Helper to create RuleInfo for testing"""
    return RuleInfo(
        name=name,
        tag=tag,
        key=f"{name}|{tag}",
        line_number=1,
        raw_content="",
        data=data,
    )


class TestDiffRules:
    def test_identical_rules_no_diff(self):
        """Ensure identical rules no diff."""
        data = {"name": "test", "tag": "mo", "match": "self::m:mo", "replace": [{"T": "text"}]}
        en = make_rule("test", "mo", data)
        tr = make_rule("test", "mo", data)
        assert diff_rules(en, tr) == []

    def test_detects_match_pattern_difference(self):
        """Ensure detects match pattern difference."""
        en = make_rule("test", "mo", {"match": "self::m:mo"})
        tr = make_rule("test", "mo", {"match": "self::m:mi"})
        diffs = diff_rules(en, tr)
        assert len(diffs) == 1
        assert diffs[0].diff_type == "match"
        assert "self::m:mo" in diffs[0].english_snippet
        assert "self::m:mi" in diffs[0].translated_snippet

    def test_detects_condition_difference(self):
        """Ensure detects condition difference."""
        en = make_rule("test", "mo", {"if": "condition1"})
        tr = make_rule("test", "mo", {"if": "condition2"})
        diffs = diff_rules(en, tr)
        assert any(d.diff_type == "condition" for d in diffs)

    def test_condition_snippet_preserves_rule_order(self):
        """
        Condition snippets should preserve the order seen in each rule.
        Originally, alphabetical order was used, which is not very helpful.
        """
        en = make_rule(
            "test",
            "mo",
            {
                "test": {
                    "if": "condition_b",
                    "then": [
                        {
                            "test": {
                                "if": "condition_a",
                                "then": [{"T": "x"}],
                            }
                        }
                    ],
                }
            },
        )
        tr = make_rule("test", "mo", {"if": "condition_c"})
        diffs: list[RuleDifference] = diff_rules(en, tr)
        cond_diff: RuleDifference = next(d for d in diffs if d.diff_type == "condition")
        assert cond_diff.english_snippet == "condition_b, condition_a"
        assert cond_diff.translated_snippet == "condition_c"

    def test_condition_snippet_deduplicates_repeated_conditions(self):
        """
        Repeated conditions should be shown once, in first-seen order.
        """
        en = make_rule(
            "test",
            "mo",
            {
                "test": {
                    "if": "condition_a",
                    "then": [
                        {
                            "test": {
                                "if": "condition_a",
                                "then": [{"T": "x"}],
                            }
                        },
                        {
                            "test": {
                                "if": "condition_b",
                                "then": [{"T": "y"}],
                            }
                        },
                    ],
                }
            },
        )
        tr = make_rule("test", "mo", {"if": "condition_c"})
        diffs: list[RuleDifference] = diff_rules(en, tr)
        cond_diff: RuleDifference = next(d for d in diffs if d.diff_type == "condition")

        # without deduplication, we'd have "condition_a" repeated.
        assert cond_diff.english_snippet == "condition_a, condition_b"
        assert cond_diff.translated_snippet == "condition_c"

    def test_detects_missing_condition(self):
        """Ensure detects missing condition."""
        en = make_rule("test", "mo", {"if": "condition1"})
        tr = make_rule("test", "mo", {"replace": [{"T": "text"}]})
        diffs = diff_rules(en, tr)
        assert any(d.diff_type == "condition" for d in diffs)

    def test_detects_variable_difference(self):
        """Ensure detects variable difference."""
        en = make_rule("test", "mo", {"variables": [{"foo": "bar"}]})
        tr = make_rule("test", "mo", {"variables": [{"baz": "qux"}]})
        diffs = diff_rules(en, tr)
        assert any(d.diff_type == "variables" for d in diffs)

    def test_detects_structure_difference(self):
        """Ensure detects structure difference."""
        en = make_rule("test", "mo", {"test": {"if": "cond", "then": [{"T": "yes"}], "else": [{"T": "no"}]}})
        tr = make_rule("test", "mo", {"test": {"if": "cond", "then": [{"T": "ja"}]}})
        diffs = diff_rules(en, tr)
        assert any(d.diff_type == "structure" for d in diffs)

    def test_multiple_differences(self):
        """Ensure multiple differences."""
        en = make_rule("test", "mo", {"match": "self::m:mo", "if": "cond1"})
        tr = make_rule("test", "mo", {"match": "self::m:mi", "if": "cond2"})
        diffs = diff_rules(en, tr)
        assert len(diffs) == 2
        types = {d.diff_type for d in diffs}
        assert "match" in types
        assert "condition" in types

    def test_ignores_text_content_differences(self):
        """Ensure ignores text content differences."""
        en = make_rule("test", "mo", {"replace": [{"T": "hello"}]})
        tr = make_rule("test", "mo", {"replace": [{"T": "hallo"}]})
        diffs = diff_rules(en, tr)
        assert diffs == []  # text differences are intentional translations
