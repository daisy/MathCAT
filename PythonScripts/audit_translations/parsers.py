"""
YAML file parsing functions.

Handles parsing of rule files and unicode files to extract rule information.
"""

import re
from pathlib import Path
from typing import Any

from ruamel.yaml import YAML
from ruamel.yaml.scanner import ScannerError

from .extractors import iter_field_matches, mapping_key_line
from .models import RuleInfo, UntranslatedEntry

_yaml = YAML()
_yaml.preserve_quotes = True


def is_unicode_file(file_path: Path) -> bool:
    """Check if this is a unicode.yaml or unicode-full.yaml file"""
    return file_path.name in ("unicode.yaml", "unicode-full.yaml")


def parse_yaml_file(file_path: Path, strict: bool = False) -> tuple[list[RuleInfo], str]:
    """
    Parse a YAML file and extract rules.
    Returns list of RuleInfo and the raw file content.

    For standard rule files: extracts rules with name/tag
    For unicode files: extracts entries with character/range keys
    """
    with open(file_path, encoding="utf-8") as f:
        content = f.read()

    try:
        data = _yaml.load(content)
    except ScannerError as exc:
        if strict:
            raise exc
        if "\t" in content:
            sanitized = content.replace("\t", "    ")
            data = _yaml.load(sanitized)
        else:
            raise exc

    rules = parse_unicode_file(content, data) if is_unicode_file(file_path) else parse_rules_file(content, data)

    return rules, content


def format_tag(tag_value: Any) -> str | None:
    if tag_value is None:
        return None
    if isinstance(tag_value, list):
        normalized = sorted(str(item).strip() for item in tag_value)
        return "[" + ", ".join(normalized) + "]"
    return str(tag_value)


def build_raw_blocks(lines: list[str], starts: list[int]) -> list[str]:
    blocks = []
    if not starts:
        return blocks
    for idx, start in enumerate(starts):
        end = starts[idx + 1] if idx + 1 < len(starts) else len(lines)
        blocks.append("\n".join(lines[start:end]))
    return blocks


def _extract_item_fields(item: Any, is_unicode: bool) -> tuple[str, str | None, str | None, Any] | None:
    if is_unicode:
        if isinstance(item, dict) and len(item) == 1:
            char_key = str(next(iter(item.keys())))
            return char_key, None, None, item[char_key]
    else:
        if isinstance(item, dict) and "name" in item:
            rule_name = str(item.get("name"))
            tag = format_tag(item.get("tag"))
            return f"{rule_name}|{tag or 'unknown'}", rule_name, tag, item
    return None


def _build_rule_items(content: str, data: Any, is_unicode_file: bool) -> list[RuleInfo]:
    if not isinstance(data, list):
        return []

    lines = content.splitlines()
    start_lines: list[int] = []
    extracted: list[tuple[str, str | None, str | None, Any]] = []

    for idx, item in enumerate(data):
        fields = _extract_item_fields(item, is_unicode_file)
        if fields is not None:
            line = data.lc.item(idx)[0] if hasattr(data, "lc") else 0
            start_lines.append(line)
            extracted.append(fields)
    raw_blocks = build_raw_blocks(lines, start_lines)

    rules: list[RuleInfo] = []
    for (key, name, tag, item_data), raw_content, line_idx in zip(extracted, raw_blocks, start_lines, strict=True):
        rules.append(
            RuleInfo(
                name=name,
                tag=tag,
                key=key,
                line_number=line_idx + 1,
                raw_content=raw_content,
                data=item_data,
                untranslated_entries=find_untranslated_text_entries(item_data),
                line_map=build_line_map(item_data),
                audit_ignore=has_audit_ignore(raw_content),
            )
        )
    return rules


def parse_rules_file(content: str, data: Any) -> list[RuleInfo]:
    """Parse a standard rules file with name/tag entries."""
    return _build_rule_items(content, data, is_unicode_file=False)


def parse_unicode_file(content: str, data: Any) -> list[RuleInfo]:
    """Parse a unicode file with character/range keys."""
    return _build_rule_items(content, data, is_unicode_file=True)


def has_audit_ignore(content: str) -> bool:
    """Check if the rule content contains an audit-ignore comment"""
    return re.search(r"#\s*audit-ignore\b", content) is not None


def find_untranslated_text_entries(node: Any) -> list[UntranslatedEntry]:
    """
    Find lowercase text keys (t, ot, ct, spell, pronounce, ifthenelse) and their line numbers.
    Returns list of UntranslatedEntry. Line number is 1-based when available.
    """
    entries: list[UntranslatedEntry] = []
    translation_keys = {"t", "ot", "ct", "spell", "pronounce", "ifthenelse"}

    def should_add(text: str) -> bool:
        if not text.strip():
            return False
        if len(text) == 1 and not text.isalpha():
            return False
        return not (text.startswith("$") or text.startswith("@"))

    for key, child, parent in iter_field_matches(node):
        if key.lower() in translation_keys and not key.isupper() and isinstance(child, str) and should_add(child):
            entries.append(UntranslatedEntry(key, child, mapping_key_line(parent, key)))
    return entries


def build_line_map(node: Any) -> dict[str, list[int]]:
    """
    Build a mapping of rule element types to line numbers.
    Line numbers are 1-based. Missing elements are omitted.
    """
    line_map: dict[str, list[int]] = {}
    structure_tokens = {
        "test",
        "if",
        "else_if",
        "then",
        "else",
        "then_test",
        "else_test",
        "with",
        "replace",
        "intent",
    }

    def add_line(kind: str, line: int | None) -> None:
        if line is None:
            return
        line_map.setdefault(kind, []).append(line)

    for key, _, parent in iter_field_matches(node):
        if key == "match":
            add_line("match", mapping_key_line(parent, key))
        if key in ("if", "else_if"):
            add_line("condition", mapping_key_line(parent, key))
        if key == "variables":
            add_line("variables", mapping_key_line(parent, key))
        if key in structure_tokens:
            add_line(f"structure:{key}", mapping_key_line(parent, key))
    return line_map
