"""
Tests for parsers.py.
"""

import pytest
from ruamel.yaml import YAML
from ruamel.yaml.scanner import ScannerError

from ..models import UntranslatedEntry
from ..parsers import (
    build_line_map,
    find_untranslated_text_entries,
    has_audit_ignore,
    parse_rules_file,
    parse_unicode_file,
)


class TestHasAuditIgnore:
    def test_detects_audit_ignore_comment(self):
        """Ensure detects audit ignore comment."""
        assert has_audit_ignore("- name: foo\n  # audit-ignore\n  tag: bar")

    def test_detects_inline_audit_ignore(self):
        """Ensure detects inline audit ignore."""
        assert has_audit_ignore("- name: foo  # audit-ignore")

    def test_returns_false_when_absent(self):
        """Ensure returns false when absent."""
        assert not has_audit_ignore("- name: foo\n  tag: bar")

    def test_returns_false_for_similar_text(self):
        """Ensure returns false for similar text."""
        assert not has_audit_ignore("audit-ignored")
        assert not has_audit_ignore("# audit ignore")  # missing hyphen


class TestFindUntranslatedTextKeys:
    def test_finds_lowercase_t(self):
        """Ensure finds lowercase t."""
        content = {"t": "hello world"}
        assert [e.text for e in find_untranslated_text_entries(content)] == ["hello world"]

    def test_finds_lowercase_ot(self):
        """Ensure finds lowercase ot."""
        content = {"ot": "open paren"}
        assert [e.text for e in find_untranslated_text_entries(content)] == ["open paren"]

    def test_finds_lowercase_ct(self):
        """Ensure finds lowercase ct."""
        content = {"ct": "close paren"}
        assert [e.text for e in find_untranslated_text_entries(content)] == ["close paren"]

    def test_finds_multiple(self):
        """Ensure finds multiple."""
        content = {"t": "one", "ot": "two", "ct": "three"}
        assert {e.text for e in find_untranslated_text_entries(content)} == {"one", "two", "three"}

    def test_ignores_uppercase_T(self):
        """Ensure ignores uppercase T."""
        content = {"T": "translated"}
        assert [e.text for e in find_untranslated_text_entries(content)] == []

    def test_finds_spell_and_pronounce(self):
        """Detects lowercase spell and pronounce markers.

        Extends coverage beyond basic t/ot/ct fields.
        Flags auxiliary translation-bearing keys."""
        content = {"spell": "alpha", "pronounce": "beta"}
        assert {e.text for e in find_untranslated_text_entries(content)} == {"alpha", "beta"}

    def test_ignores_uppercase_variants(self):
        """Ignores uppercase variants of extended markers.

        Honors already-verified spell/pronounce/IfThenElse content.
        Avoids double-reporting translated data."""
        content = {"PRONOUNCE": "gamma", "IFTHENELSE": "delta"}
        assert [e.text for e in find_untranslated_text_entries(content)] == []

    def test_ignores_variable_references(self):
        """Ensure ignores variable references."""
        content = {"t": "$variable"}
        assert [e.text for e in find_untranslated_text_entries(content)] == []

    def test_ignores_xpath_expressions(self):
        """Ensure ignores xpath expressions."""
        content = {"t": "@attr"}
        assert [e.text for e in find_untranslated_text_entries(content)] == []

    def test_ignores_single_punctuation(self):
        """Ensure ignores single punctuation."""
        content = {"t": "."}
        assert [e.text for e in find_untranslated_text_entries(content)] == []

    def test_finds_entries_with_lines(self):
        """Ensure finds entries with line numbers."""
        yaml = YAML()
        content = """- name: line-check
  tag: mo
  replace:
    - t: "not translated"
"""
        data = yaml.load(content)
        entries = find_untranslated_text_entries(data[0])
        assert entries == [UntranslatedEntry("t", "not translated", 4)]


class TestParseRulesFile:
    def test_parses_simple_rule(self):
        """Ensure parses simple rule."""
        content = """- name: my-rule
  tag: mo
  match: "."
  replace:
    - T: "text"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_rules_file(content, data)
        assert len(rules) == 1
        assert rules[0].name == "my-rule"
        assert rules[0].tag == "mo"
        assert rules[0].key == "my-rule|mo"
        assert rules[0].line_number == 1
        assert rules[0].line_map["match"] == [3]

    def test_parses_multiple_rules(self):
        """Ensure parses multiple rules."""
        content = """- name: rule1
  tag: mo
  match: "."

- name: rule2
  tag: mi
  match: "x"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_rules_file(content, data)
        assert len(rules) == 2
        assert rules[0].name == "rule1"
        assert rules[1].name == "rule2"

    def test_detects_untranslated_text(self):
        """Ensure detects untranslated text."""
        content = """- name: untranslated
  tag: mo
  replace:
    - t: "not translated"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_rules_file(content, data)
        assert rules[0].has_untranslated_text
        assert "not translated" in rules[0].untranslated_keys
        assert rules[0].untranslated_entries == [UntranslatedEntry("t", "not translated", 4)]

    def test_detects_audit_ignore(self):
        """Ensure detects audit ignore."""
        content = """- name: ignored-rule
  tag: mo  # audit-ignore
  match: "."
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_rules_file(content, data)
        assert rules[0].audit_ignore

    def test_handles_array_tag(self):
        """Ensure handles array tag."""
        content = """- name: multi-tag
  tag: [mo, mtext]
  match: "."
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_rules_file(content, data)
        assert rules[0].tag == "[mo, mtext]"

    def test_sorts_tag_lists(self):
        """Normalizes unordered tag lists for stable comparison.

        Confirms sorting prevents false positives in diffs.
        Keeps tag-based keys consistent across translations."""
        content = """- name: multi-tag
  tag: [mtext, mo]
  match: "."
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_rules_file(content, data)
        assert rules[0].tag == "[mo, mtext]"

    def test_returns_empty_for_non_list_data(self):
        """Non-list YAML data returns no rules."""
        rules = parse_rules_file("key: value", {"key": "value"})
        assert rules == []

    def test_skips_items_without_name(self):
        """Items like '- include: file' that lack a 'name' key are skipped."""
        content = """- include: shared.yaml
- name: real-rule
  tag: mo
  match: "."
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_rules_file(content, data)
        assert len(rules) == 1
        assert rules[0].name == "real-rule"

    def test_mixed_valid_and_skipped_items(self):
        """Valid rules interspersed with non-rule items keep correct line numbers."""
        content = """- name: first
  tag: mo
  match: "."

- include: other.yaml

- name: second
  tag: mi
  match: "x"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_rules_file(content, data)
        assert len(rules) == 2
        assert rules[0].name == "first"
        assert rules[0].line_number == 1
        assert rules[1].name == "second"
        assert rules[1].line_number == 7

    def test_parse_yaml_file_handles_tabs(self, tmp_path):
        """Ensure parse yaml file handles tabs."""
        content = """- name: tabbed
  tag: mo
  match: "."
  replace:
    - t: "x"\t# tab before comment
"""
        file_path = tmp_path / "tabbed.yaml"
        file_path.write_text(content, encoding="utf-8")
        from ..parsers import parse_yaml_file

        rules, _ = parse_yaml_file(file_path)
        assert len(rules) == 1
        assert rules[0].name == "tabbed"

    def test_parse_yaml_file_strict_rejects_tabs(self, tmp_path):
        """Ensure parse yaml file strict rejects tabs."""
        content = """- name: tabbed
  tag: mo
  match: "."
  replace:
    - t: "x"\t# tab before comment
"""
        file_path = tmp_path / "tabbed.yaml"
        file_path.write_text(content, encoding="utf-8")
        from ..parsers import parse_yaml_file

        with pytest.raises(ScannerError):
            parse_yaml_file(file_path, strict=True)


class TestParseUnicodeFile:
    def test_parses_single_char_entry(self):
        """Ensure parses single char entry."""
        content = """- "a":
    - t: "a"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_unicode_file(content, data)
        assert len(rules) == 1
        assert rules[0].key == "a"
        assert rules[0].name is None
        assert rules[0].tag is None

    def test_parses_range_entry(self):
        """Ensure parses range entry."""
        content = """- "0-9":
    - t: "digit"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_unicode_file(content, data)
        assert rules[0].key == "0-9"

    def test_parses_multiple_entries(self):
        """Ensure parses multiple entries."""
        content = """- "a":
    - t: "a"
- "b":
    - t: "b"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_unicode_file(content, data)
        assert len(rules) == 2

    def test_returns_empty_for_non_list_data(self):
        """Non-list YAML data returns no rules."""
        rules = parse_unicode_file("key: value", {"key": "value"})
        assert rules == []

    def test_skips_multi_key_dicts(self):
        """Dicts with more than one key are not valid unicode entries and are skipped."""
        content = """- "a":
    - t: "a"
- "b":
    - t: "b"
  "c":
    - t: "c"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_unicode_file(content, data)
        assert len(rules) == 1
        assert rules[0].key == "a"

    def test_mixed_valid_and_skipped_items(self):
        """Valid entries interspersed with non-entry items keep correct line numbers."""
        content = """- "a":
    - t: "alpha"

- not: a unicode entry
  extra: key

- "b":
    - t: "bravo"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_unicode_file(content, data)
        assert len(rules) == 2
        assert rules[0].key == "a"
        assert rules[0].line_number == 1
        assert rules[1].key == "b"
        assert rules[1].line_number == 7


class TestBuildLineMap:
    def test_builds_line_map_for_rule_elements(self):
        """Ensure line map captures nested element lines."""
        content = """- name: line-map
  tag: mo
  match: "."
  if: cond
  variables:
    - foo: bar
  test:
    if: cond2
    then:
      - t: "x"
"""
        yaml = YAML()
        data = yaml.load(content)
        line_map = build_line_map(data[0])
        assert line_map["match"] == [3]
        assert line_map["condition"] == [4, 8]
        assert line_map["variables"] == [5]
        assert line_map["structure:test"] == [7]
        assert line_map["structure:if"] == [4, 8]
