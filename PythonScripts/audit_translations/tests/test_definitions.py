"""Focused tests for definitions.yaml parsing and comparison."""

from pathlib import Path

import pytest
from ruamel.yaml import YAML

from ..auditor import compare_definition_files
from ..errors import AuditError
from ..models.definitions import DefinitionKind
from ..parsers import parse_definitions


def parse(content: str):
    """Parse an in-memory definitions document with a stable diagnostic path."""
    return parse_definitions(content, YAML().load(content), Path("definitions.yaml"))


def assert_invalid(content: str, message: str) -> None:
    """Assert that malformed definition YAML produces a contextual audit error."""
    with pytest.raises(AuditError) as exc:
        parse(content)
    diagnostic = str(exc.value)
    assert "definitions.yaml" in diagnostic
    assert "line 1" in diagnostic
    assert message in diagnostic


def compare(tmp_path, source: str, target: str, issue_filter: set[str] | None = None):
    """Compare two in-memory definition documents through temporary files."""
    source_path = tmp_path / "source-definitions.yaml"
    target_path = tmp_path / "target-definitions.yaml"
    source_path.write_text(source, encoding="utf-8")
    target_path.write_text(target, encoding="utf-8")
    return compare_definition_files(source_path, target_path, issue_filter)


def test_parse_vector_definition() -> None:
    """A non-empty YAML sequence of strings is classified as a vector."""
    definitions = parse('- NumbersTens:\n  - ""\n  - ten\n  - twenty\n')
    assert definitions["NumbersTens"].kind is DefinitionKind.VECTOR
    assert definitions["NumbersTens"].line_number == 1


def test_parse_map_definition() -> None:
    """A non-empty string-to-string YAML mapping is classified as a map."""
    definitions = parse('- NavigationParts:\n    mfrac: "numerator; denominator"\n')
    assert definitions["NavigationParts"].kind is DefinitionKind.MAP
    assert definitions["NavigationParts"].line_number == 1


def test_parse_set_definition() -> None:
    """A non-empty YAML mapping with all-null values is classified as a set."""
    definitions = parse("- TerseFunctionNames:\n    divergence:\n    curl:\n")
    assert definitions["TerseFunctionNames"].kind is DefinitionKind.SET
    assert definitions["TerseFunctionNames"].line_number == 1


def test_parse_definitions_ignores_include() -> None:
    """An include entry is omitted from the parsed definition mapping."""
    assert parse('- include: "../../definitions.yaml"\n') == {}


def test_parse_definitions_rejects_empty_vector() -> None:
    """An empty sequence is rejected because its definition kind is ambiguous."""
    assert_invalid("- Foo: []\n", "empty sequences")


def test_parse_definitions_rejects_empty_mapping() -> None:
    """An empty mapping is rejected because it could be either a set or a map."""
    assert_invalid("- Foo: {}\n", "empty mappings")


def test_parse_definitions_rejects_string_scalar() -> None:
    """A string scalar cannot be used as a definition value."""
    assert_invalid("- Foo: bar\n", "non-empty sequence or mapping")


def test_parse_definitions_rejects_numeric_scalar() -> None:
    """A numeric scalar cannot be used as a definition value."""
    assert_invalid("- Foo: 123\n", "non-empty sequence or mapping")


def test_parse_definitions_rejects_nested_mapping_value() -> None:
    """A nested object is rejected because map values must be strings."""
    assert_invalid("- Foo:\n    nested:\n      object: value\n", "all strings")


def test_parse_definitions_rejects_mixed_set_and_map_values() -> None:
    """A mapping containing both null and string values is rejected."""
    assert_invalid('- Foo:\n    a:\n    b: "value"\n', "mixed set/map")


def test_parse_definitions_rejects_non_string_vector_entry() -> None:
    """Every entry in a vector definition must be a string."""
    assert_invalid("- Foo:\n  - valid\n  - 2\n", "vector entries")


def test_parse_definitions_rejects_non_string_mapping_key() -> None:
    """Every key in a set or map definition must be a string."""
    assert_invalid('- Foo:\n    1: "value"\n', "mapping keys")


def test_parse_definitions_rejects_non_string_definition_name() -> None:
    """A definition name must be a string."""
    assert_invalid('- 42:\n  - "value"\n', "definition name")


def test_parse_definitions_last_duplicate_wins() -> None:
    """The final occurrence of a duplicate definition name replaces earlier occurrences."""
    definitions = parse('- Foo:\n  - first\n- Foo:\n    key: "value"\n')
    assert len(definitions) == 1
    assert definitions["Foo"].kind is DefinitionKind.MAP
    assert definitions["Foo"].line_number == 3


def test_compare_same_definitions_and_kinds_is_clean(tmp_path) -> None:
    """Definitions with matching names and kinds produce no findings."""
    result = compare(tmp_path, "- Foo: [one]\n- Bar: {key: value}\n", "- Foo: [eins]\n- Bar: {taste: wert}\n")
    assert not result.has_findings
    assert result.issue_count == 0


def test_compare_ignores_definition_order(tmp_path) -> None:
    """Reordering definitions does not affect name-based comparison."""
    result = compare(tmp_path, "- Foo: [one]\n- Bar: {key: value}\n", "- Bar: {taste: wert}\n- Foo: [eins]\n")
    assert not result.has_findings


def test_compare_reports_missing_definition_as_issue(tmp_path) -> None:
    """A source-only definition is reported as an audit issue."""
    result = compare(tmp_path, "- Foo: [one]\n- Bar: [two]\n", "- Foo: [eins]\n")
    assert [definition.name for definition in result.missing_definitions] == ["Bar"]
    assert result.issue_count == 1


def test_compare_reports_extra_definition_as_information(tmp_path) -> None:
    """A target-only definition is informational and does not increase the issue count."""
    result = compare(tmp_path, "- Foo: [one]\n", "- Foo: [eins]\n- TargetSpecific: {key: value}\n")
    assert [definition.name for definition in result.extra_definitions] == ["TargetSpecific"]
    assert result.issue_count == 0


def test_compare_reports_type_mismatch_as_issue(tmp_path) -> None:
    """Different kinds for the same definition name produce a type-mismatch issue."""
    result = compare(tmp_path, "- Foo: [one]\n", "- Foo: {key: value}\n")
    assert len(result.type_mismatches) == 1
    assert result.type_mismatches[0].source_definition.kind is DefinitionKind.VECTOR
    assert result.type_mismatches[0].target_definition.kind is DefinitionKind.MAP
    assert result.issue_count == 1


def test_compare_ignores_different_vector_contents(tmp_path) -> None:
    """Translated vector strings are not compared when both definitions are vectors."""
    result = compare(tmp_path, "- Foo: [one, two, three]\n", "- Foo: [eins, zwei, drei]\n")
    assert not result.has_findings


def test_compare_ignores_different_vector_lengths(tmp_path) -> None:
    """Vector length differences are outside the scope of definition auditing."""
    result = compare(tmp_path, "- Foo: [one, two, three]\n", "- Foo: [eins, zwei]\n")
    assert not result.has_findings


def test_compare_ignores_different_map_contents(tmp_path) -> None:
    """Map keys and values are not compared when both definitions are maps."""
    result = compare(tmp_path, "- Foo: {one: first}\n", "- Foo: {two: second, three: third}\n")
    assert not result.has_findings


def test_compare_ignores_different_set_contents(tmp_path) -> None:
    """Set members are not compared when both definitions are sets."""
    result = compare(tmp_path, "- Foo: {one, two}\n", "- Foo: {three}\n")
    assert not result.has_findings


def test_compare_ignores_different_includes(tmp_path) -> None:
    """Different include paths do not produce definition findings."""
    result = compare(
        tmp_path,
        '- include: "../../definitions.yaml"\n- Foo: [one]\n',
        '- include: "target-specific.yaml"\n- Foo: [eins]\n',
    )
    assert not result.has_findings


def test_compare_missing_filter_returns_only_missing_definitions(tmp_path) -> None:
    """The missing filter includes missing definitions and suppresses other findings."""
    result = compare(
        tmp_path,
        "- Missing: [one]\n- Different: [one]\n",
        "- Extra: [eins]\n- Different: {key: value}\n",
        {"missing"},
    )
    assert len(result.missing_definitions) == 1
    assert result.extra_definitions == []
    assert result.type_mismatches == []


def test_compare_extra_filter_returns_only_extra_definitions(tmp_path) -> None:
    """The extra filter includes target-only definitions and suppresses other findings."""
    result = compare(
        tmp_path,
        "- Missing: [one]\n- Different: [one]\n",
        "- Extra: [eins]\n- Different: {key: value}\n",
        {"extra"},
    )
    assert result.missing_definitions == []
    assert len(result.extra_definitions) == 1
    assert result.type_mismatches == []


def test_compare_diffs_filter_returns_only_type_mismatches(tmp_path) -> None:
    """The diffs filter includes definition type mismatches and suppresses coverage findings."""
    result = compare(
        tmp_path,
        "- Missing: [one]\n- Different: [one]\n",
        "- Extra: [eins]\n- Different: {key: value}\n",
        {"diffs"},
    )
    assert result.missing_definitions == []
    assert result.extra_definitions == []
    assert len(result.type_mismatches) == 1


def test_compare_untranslated_filter_returns_no_definition_findings(tmp_path) -> None:
    """The untranslated filter never fabricates translation-state findings for definitions."""
    result = compare(
        tmp_path,
        "- Missing: [one]\n- Different: [one]\n",
        "- Extra: [eins]\n- Different: {key: value}\n",
        {"untranslated"},
    )
    assert result.missing_definitions == []
    assert result.extra_definitions == []
    assert result.type_mismatches == []


def test_compare_without_filter_returns_all_definition_findings(tmp_path) -> None:
    """An unfiltered comparison returns missing, extra, and type-mismatch findings."""
    result = compare(
        tmp_path,
        "- Missing: [one]\n- Different: [one]\n",
        "- Extra: [eins]\n- Different: {key: value}\n",
    )
    assert len(result.missing_definitions) == 1
    assert len(result.extra_definitions) == 1
    assert len(result.type_mismatches) == 1


def test_compare_merges_region_definitions_by_name(tmp_path) -> None:
    """A regional definition replaces the base definition with the same name."""
    source_path = tmp_path / "source.yaml"
    target_path = tmp_path / "target.yaml"
    target_region_path = tmp_path / "target-region.yaml"
    source_path.write_text("- Base: [one]\n- Override: {key: value}\n", encoding="utf-8")
    target_path.write_text("- Base: [eins]\n- Override: [wrong-kind]\n", encoding="utf-8")
    target_region_path.write_text("- Override: {translated: value}\n", encoding="utf-8")

    result = compare_definition_files(source_path, target_path, target_region_path=target_region_path)
    assert not result.has_findings
