"""
Shared pytest configuration for audit translation tests.

Rich can emit ANSI styling codes into captured test output when a terminal or
environment variable forces color output. That made string and golden-output
assertions fail on some machines even though the visible CLI output was correct.
These helpers normalize captured renderer/CLI output so tests compare the text
users see, not terminal control bytes.
"""

import re
import sys

import pytest

from audit_translations.renderer import console

# needed for running tests on Windows
sys.stdout.reconfigure(encoding="utf-8")

ANSI_RE = re.compile(r"\x1b\[[0-?]*[ -/]*[@-~]")


def strip_ansi(text: str) -> str:
    """Remove ANSI escape sequences from Rich output captured in tests."""
    return ANSI_RE.sub("", text)


@pytest.fixture(autouse=True)
def deterministic_rich_output():
    """Keep Rich output assertions stable when the shell forces ANSI colors."""
    old_no_color = console.no_color
    old_force_terminal = console._force_terminal
    console.no_color = True
    console._force_terminal = False
    yield
    console.no_color = old_no_color
    console._force_terminal = old_force_terminal
