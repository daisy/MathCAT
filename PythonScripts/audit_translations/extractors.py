"""
Rule data extraction functions.

Extracts structural elements, match patterns, conditions, and variables
from parsed YAML rule data.
"""

from collections.abc import Iterator
from typing import Any

from jsonpath_ng.ext import parse
from jsonpath_ng.jsonpath import Fields

_ALL_FIELDS_EXPR = parse("$..*")  # '..' is recursive descent
_MATCH_EXPR = parse("$.match")


def mapping_key_line(mapping: Any, key: str) -> int | None:
    """
    - 'lc' is line and column in YAML file: https://yaml.dev/doc/ruamel.yaml/detail/
    """
    if hasattr(mapping, "lc") and hasattr(mapping.lc, "data"):
        line_info = mapping.lc.data.get(key)
        return line_info[0] + 1
    return None


def iter_field_matches(node: Any) -> Iterator[tuple[str, Any, Any]]:
    """
    Iterate nested mapping fields using jsonpath.

    Returns tuples of (key, child_value, parent_mapping) in traversal order.
    """
    for match in _ALL_FIELDS_EXPR.find(node):
        path = match.path
        if isinstance(path, Fields) and len(path.fields) == 1:
            key = path.fields[0]
            parent = match.context.value if match.context is not None else None
            yield key, match.value, parent


def normalize_match(value: Any) -> str:
    if isinstance(value, list):
        return " ".join(str(item) for item in value)
    if isinstance(value, str):
        return value
    return ""


def normalize_xpath(value: str) -> str:
    return " ".join(value.split())


def extract_match_pattern(rule_data: Any) -> str:
    if isinstance(rule_data, dict):
        matches = _MATCH_EXPR.find(rule_data)
        if matches:
            return normalize_match(matches[0].value)
    return ""


def extract_conditions(rule_data: Any) -> list[str]:
    """Extract all if/else conditions from a rule"""
    conditions: list[str] = []
    for key, child, _ in iter_field_matches(rule_data):
        if key in ("if", "else_if") and isinstance(child, str):
            conditions.append(child)
    return conditions


def extract_variables(rule_data: Any) -> list[tuple[str, str]]:
    """Extract variable definitions from a rule"""
    variables: list[tuple[str, str]] = []

    def add_from_value(value: Any) -> None:
        if isinstance(value, dict):
            for name, expr in value.items():
                variables.append((str(name), str(expr)))
        elif isinstance(value, list):
            for item in value:
                if isinstance(item, dict):
                    for name, expr in item.items():
                        variables.append((str(name), str(expr)))

    for key, child, _ in iter_field_matches(rule_data):
        if key == "variables":
            add_from_value(child)
    return variables


def extract_structure_elements(rule_data: Any) -> list[str]:
    """Extract structural elements (test, with, replace blocks) ignoring text content"""
    elements: list[str] = []
    tokens = {"test", "if", "else_if", "then", "else", "then_test", "else_test", "with", "replace", "intent"}
    for key, _, _ in iter_field_matches(rule_data):
        if key in tokens:
            elements.append(f"{key}:")
    return elements
