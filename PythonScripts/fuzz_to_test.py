"""Generate Rust regression tests from fuzz crash artifacts.

Usage:
    python PythonScripts/fuzz_to_test.py [ARTIFACT_PATH ...]

If no paths are given, defaults to all files under fuzz/artifacts/fuzz_target_1/.

Each crash file is converted to a #[test] function in tests/fuzz_regressions.rs.
Existing tests in that file are preserved; duplicates (by hash) are skipped.
"""

from __future__ import annotations

import hashlib
import os
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
ARTIFACT_DIR = REPO_ROOT / "fuzz" / "artifacts" / "fuzz_target_1"
TEST_FILE = REPO_ROOT / "tests" / "fuzz_regressions.rs"

HEADER = '''\
//! Regression tests auto-generated from fuzz crash artifacts.
//! Run: `cargo test --test fuzz_regressions`
//! Generate: `python PythonScripts/fuzz_to_test.py`
#![allow(unused_imports, dead_code)]

mod common;
use common::abs_rules_dir_path;
'''


def artifact_hash(data: bytes) -> str:
    """Short hex hash used as the test function suffix."""
    return hashlib.sha256(data).hexdigest()[:16]


def rust_byte_literal(data: bytes) -> str:
    """Format bytes as a Rust byte-string literal (b\"...\"), escaping as needed."""
    parts: list[str] = []
    for b in data:
        if b == 0x5C:  # backslash
            parts.append("\\\\")
        elif b == 0x22:  # double quote
            parts.append('\\"')
        elif b == 0x0A:  # newline
            parts.append("\\n")
        elif b == 0x0D:  # carriage return
            parts.append("\\r")
        elif b == 0x09:  # tab
            parts.append("\\t")
        elif b == 0x00:
            parts.append("\\0")
        elif 0x20 <= b <= 0x7E:
            parts.append(chr(b))
        else:
            parts.append(f"\\x{b:02x}")
    return 'b"' + "".join(parts) + '"'


def rust_string_literal(text: str) -> str:
    """Format a Python string as a Rust raw string literal r#"..."#.

    Falls back to regular string with escapes if the content contains
    both `"#` sequences at every nesting level (extremely unlikely for MathML).
    """
    # Find a raw-string delimiter that doesn't clash with the content.
    for hashes in range(0, 10):
        delim = "#" * hashes
        closing = '"' + delim
        if closing not in text:
            return f'r{delim}"{text}"{delim}'
    # Pathological fallback: use an escaped regular string.
    escaped = text.replace("\\", "\\\\").replace('"', '\\"')
    return f'"{escaped}"'


def make_test(data: bytes, hash_id: str) -> str:
    """Return a single #[test] function as a string."""
    try:
        text = data.decode("utf-8")
        lit = rust_string_literal(text)
        return f'''\

#[test]
fn fuzz_crash_{hash_id}() {{
    libmathcat::set_rules_dir(abs_rules_dir_path()).unwrap();
    let mathml = {lit};
    if libmathcat::set_mathml(mathml).is_ok() {{
        let _ = libmathcat::get_spoken_text();
        let _ = libmathcat::get_braille("");
    }}
}}
'''
    except UnicodeDecodeError:
        # Non-UTF-8 input: use byte literal and String::from_utf8_lossy.
        lit = rust_byte_literal(data)
        lossy_preview = data.decode("utf-8", errors="replace")[:200]
        comment = f"    // Lossy preview: {lossy_preview!r}"
        return f'''\

#[test]
fn fuzz_crash_{hash_id}() {{
{comment}
    libmathcat::set_rules_dir(abs_rules_dir_path()).unwrap();
    let data: &[u8] = {lit};
    let s = String::from_utf8_lossy(data);
    if libmathcat::set_mathml(s.as_ref()).is_ok() {{
        let _ = libmathcat::get_spoken_text();
        let _ = libmathcat::get_braille("");
    }}
}}
'''


def existing_hashes(path: Path) -> set[str]:
    """Return set of hash ids already present in the test file."""
    if not path.exists():
        return set()
    text = path.read_text(encoding="utf-8")
    return set(re.findall(r"fn fuzz_crash_([0-9a-f]{16})\b", text))


def collect_artifacts(paths: list[str]) -> list[Path]:
    """Resolve explicit paths or default to the standard artifact directory."""
    if paths:
        result = []
        for p in paths:
            pp = Path(p)
            if pp.is_dir():
                result.extend(sorted(pp.iterdir()))
            elif pp.is_file():
                result.append(pp)
        return result

    if not ARTIFACT_DIR.is_dir():
        return []
    return sorted(ARTIFACT_DIR.iterdir())


def main() -> None:
    artifacts = collect_artifacts(sys.argv[1:])
    if not artifacts:
        print("No crash artifacts found.", file=sys.stderr)
        if not sys.argv[1:]:
            print(f"  Looked in: {ARTIFACT_DIR}", file=sys.stderr)
        sys.exit(0)

    known = existing_hashes(TEST_FILE)
    new_tests: list[str] = []
    for artifact in artifacts:
        if not artifact.is_file():
            continue
        data = artifact.read_bytes()
        h = artifact_hash(data)
        if h in known:
            print(f"  skip (duplicate): {artifact.name}  [{h}]")
            continue
        known.add(h)
        new_tests.append(make_test(data, h))
        print(f"  added: {artifact.name}  ->  fuzz_crash_{h}")

    if not new_tests:
        print("No new tests to add.")
        return

    if TEST_FILE.exists():
        existing = TEST_FILE.read_text(encoding="utf-8")
    else:
        existing = HEADER

    TEST_FILE.write_text(existing + "\n".join(new_tests) + "\n", encoding="utf-8")
    print(f"\nWrote {len(new_tests)} test(s) to {TEST_FILE.relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
