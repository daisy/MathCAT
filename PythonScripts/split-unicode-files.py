#!/usr/bin/env python3
from __future__ import annotations
import re
import sys
import ast
from typing import Set, List, Tuple
sys.stdout.reconfigure(encoding="utf-8")  # type: ignore[attr-defined]


# ------------------------------------------------------------
# Loading & normalization
# ------------------------------------------------------------

def read_set(path: str) -> Set[str]:
    with open(path, "r", encoding="utf-8") as f:
        data = f.read()
    result = ast.literal_eval(data)
    if not isinstance(result, set):
        raise ValueError(f"{path} does not contain a Python set literal.")
    return result


def load_yaml_file_strip_header(path: str) -> List[str]:
    """Load a YAML file, removing any leading '---'."""
    with open(path, "r", encoding="utf-8") as f:
        lines = f.readlines()

    i = 0
    while i < len(lines) and lines[i].strip() == "---":
        i += 1

    return lines[i:]


def load_and_concat_yaml_files(paths: List[str]) -> List[str]:
    lines: List[str] = []
    for p in paths:
        lines.extend(load_yaml_file_strip_header(p))
    return lines


# ------------------------------------------------------------
# Key parsing & expansion
# ------------------------------------------------------------
# Matches: - "a":, - 'b-z':, - "(":, - "\"":, - A-Z:
KEY_RE = re.compile(r'''
    ^\s*-\s*                      # leading whitespace, dash, whitespace
    (?P<key>
        "(?:[^"\\]|\\.)*"         # double-quoted key
      | '(?:[^'\\]|\\.)*'         # single-quoted key
      | [^:\s][^:]*               # unquoted key (no leading colon)
    )
    \s*:
''', re.VERBOSE)


def extract_key(line: str) -> str | None:
    m = KEY_RE.match(line)
    if not m:
        return None

    key = m.group("key")

    # Strip matching quotes
    if (key.startswith('"') and key.endswith('"')) or \
       (key.startswith("'") and key.endswith("'")):
        key = key[1:-1]

    return key


def expand_key(key: str) -> Set[str]:
    """
    Expand a YAML key like:
      "a"        → {"a"}
      "A-Z"      → {"A","B",...,"Z"}
      "0-9"      → {"0","1",...,"9"}
      "𝛛𝛜𝛝..."  → {"𝛛","𝛜","𝛝",...}
      " "        → {" "}
    """
    raw = key              # preserve exactly what extract_key returned
    s = key.strip()        # only for detecting ranges

    # Simple range like A-Z or α-ω
    if "-" in s and len(s) == 3:
        start, end = s[0], s[2]
        return {chr(c) for c in range(ord(start), ord(end) + 1)}

    # Multi-character literal → use raw, not stripped
    if len(raw) > 1:
        return set(raw)

    # Single character (including space)
    return {raw}


# ------------------------------------------------------------
# Matching logic
# ------------------------------------------------------------

def key_matches_set(key: str, ops: Set[str]) -> bool:
    # Case 1: simple alphabetic range
    if "-" in key and len(key) == 3:
        start, end = key[0], key[2]
        return any(start <= ch <= end for ch in ops)

    # Case 2: multi-character literal
    if len(key) > 1:
        return any(ch in ops for ch in key)

    # Case 3: single character
    return key in ops


# ------------------------------------------------------------
# Block parsing
# ------------------------------------------------------------

def is_block_terminator(line: str, key_indent: int) -> bool:
    """
    A block ends when:
      - a comment is LESS indented than the key line
      - a non-comment line is LESS indented than the key line

    A block does NOT end on:
      - blank lines
      - comments indented >= key indent
    """

    stripped = line.lstrip()
    indent = len(line) - len(stripped)

    # Blank lines NEVER terminate a block
    if stripped == "":
        return False

    # Outdented comment → terminator
    if stripped.startswith("#"):
        return indent < key_indent

    # Outdented non-comment → terminator
    return indent < key_indent


def parse_preamble(lines: List[str], start: int) -> Tuple[List[str], int]:
    """Return (preamble_lines, new_index)."""
    preamble: List[str] = []
    i = start
    n = len(lines)

    while i < n and extract_key(lines[i]) is None:
        preamble.append(lines[i])
        i += 1

    return preamble, i


def parse_blocks(lines: List[str], start: int, ops: Set[str]) -> Tuple[List[List[str]], List[List[str]], Set[str]]:
    matched: List[List[str]] = []
    unmatched: List[List[str]] = []
    yaml_char_universe: Set[str] = set()

    i = start
    n = len(lines)

    while i < n:
        line = lines[i]
        key = extract_key(line)

        if line.find('":"') != -1 or line.find('" "') != -1:
            print(f"key: '{key}', expanded: '{expand_key(key)}'")
        if key is None:
            i += 1
            continue

        # Track characters used in YAML keys
        yaml_char_universe.update(expand_key(key))

        block: List[str] = [line]
        key_indent = len(line) - len(line.lstrip())

        is_match = key_matches_set(key, ops)

        i += 1
        while i < n:
            next_line = lines[i]

            # 1) New key at same or less indent → end of this block
            next_key = extract_key(next_line)
            if next_key is not None:
                next_indent = len(next_line) - len(next_line.lstrip())
                if next_indent <= key_indent:
                    break  # do NOT consume; outer loop will handle it

            # 2) Normal terminators (blank or outdented comment)
            if is_block_terminator(next_line, key_indent):
                break

            block.append(next_line)
            i += 1

        # Include terminator comment (but not blank)
        if i < n and lines[i].strip().startswith("#"):
            block.append(lines[i])
            i += 1

        if is_match:
            matched.append(block)
        else:
            unmatched.append(block)

    return matched, unmatched, yaml_char_universe


# ------------------------------------------------------------
# Output & sanity checks
# ------------------------------------------------------------

def sanity_check(ops: Set[str], yaml_chars: Set[str]) -> None:
    unknown = sorted(ch for ch in ops if ch not in yaml_chars)
    if unknown:
        print("WARNING: The following characters from the set do not appear in any YAML key:")
        for ch in unknown:
            display = ch if ch.strip() else repr(ch)
            print(f"  {display}")
        print()


def write_output_files(preamble: List[str], matched: List[List[str]], unmatched: List[List[str]]) -> None:
    with open("matched.yaml", "w", encoding="utf-8") as f:
        f.write("---\n")
        f.writelines(preamble)
        for block in matched:
            f.writelines(block)
            f.write("\n")

    with open("not-matched.yaml", "w", encoding="utf-8") as f:
        f.write("---\n")
        f.writelines(preamble)
        for block in unmatched:
            f.writelines(block)
            f.write("\n")


# ------------------------------------------------------------
# Orchestration
# ------------------------------------------------------------

def split_yaml_by_set(set_path: str, yaml_paths: List[str]) -> None:
    ops = read_set(set_path)
    lines = load_and_concat_yaml_files(yaml_paths)

    preamble, idx = parse_preamble(lines, 0)
    matched, unmatched, yaml_chars = parse_blocks(lines, idx, ops)

    sanity_check(ops, yaml_chars)
    write_output_files(preamble, matched, unmatched)

    print(f"Done. Wrote {len(matched)} matched blocks and {len(unmatched)} unmatched blocks.")


# ------------------------------------------------------------
# CLI
# ------------------------------------------------------------
def main() -> None:
    if len(sys.argv) < 3:
        print("Usage: python split_yaml_by_set.py <setfile> <yaml1> [yaml2 ...]")
        sys.exit(1)

    set_path = sys.argv[1]
    yaml_paths = sys.argv[2:]

    split_yaml_by_set(set_path, yaml_paths)


if __name__ == "__main__":
    main()
