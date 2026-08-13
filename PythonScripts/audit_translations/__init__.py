"""
MathCAT Translation Audit Tool

Compares source YAML rule files with translated versions to identify translation
gaps and issues. This tool helps translators ensure their translations are complete
and properly formatted.

Read README.md for more details.
"""

from .cli import main

__all__ = [
    "main",
]
