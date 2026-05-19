"""
Tests for extractors.py.
"""

from ..extractors import (
    extract_conditions,
    extract_match_pattern,
    extract_structure_elements,
    extract_variables,
)


class TestExtractMatchPattern:
    def test_extracts_inline_match(self):
        """Ensure extracts inline match."""
        data = {"match": "self::m:mo"}
        assert extract_match_pattern(data) == "self::m:mo"

    def test_extracts_array_match(self):
        """Ensure extracts array match."""
        data = {"match": ["self::m:mo", "@intent"]}
        assert extract_match_pattern(data) == "self::m:mo @intent"

    def test_returns_empty_for_no_match(self):
        """Ensure returns empty for no match."""
        data = {"replace": [{"T": "text"}]}
        assert extract_match_pattern(data) == ""


class TestExtractConditions:
    def test_extracts_single_condition(self):
        """Ensure extracts single condition."""
        data = {"if": "$Verbosity"}
        assert extract_conditions(data) == ["$Verbosity"]

    def test_extracts_multiple_conditions(self):
        """Ensure extracts multiple conditions."""
        data = {"if": "condition1", "then": "something", "else_test": {"if": "condition2"}}
        conditions = extract_conditions(data)
        assert "condition1" in conditions
        assert "condition2" in conditions


class TestExtractVariables:
    def test_extracts_variables(self):
        """Ensure extracts variables."""
        data = {"variables": [{"name": "value"}, {"other": "val2"}]}
        variables = extract_variables(data)
        assert ("name", "value") in variables
        assert ("other", "val2") in variables

    def test_returns_empty_for_no_variables(self):
        """Ensure returns empty for no variables."""
        data = {"match": "."}
        assert extract_variables(data) == []


class TestExtractStructureElements:
    def test_extracts_test_structure(self):
        """Ensure extracts test structure."""
        data = {"test": {"if": "condition", "then": [{"T": "yes"}], "else": [{"T": "no"}]}}
        elements = extract_structure_elements(data)
        assert "test:" in elements
        assert "if:" in elements
        assert "then:" in elements
        assert "else:" in elements
