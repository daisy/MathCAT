"""Models for definitions.yaml parsing and comparison."""

from dataclasses import dataclass
from enum import StrEnum
from typing import Any


class DefinitionKind(StrEnum):
    """Collection shapes supported by MathCAT definitions files."""

    VECTOR = "vector"
    SET = "set"
    MAP = "map"


@dataclass
class DefinitionInfo:
    """Information about one literal entry in a definitions file."""

    name: str
    kind: DefinitionKind
    line_number: int
    raw_content: str
    data: Any


@dataclass
class DefinitionTypeMismatch:
    """A definition whose collection kind differs between source and target."""

    source_definition: DefinitionInfo
    target_definition: DefinitionInfo


@dataclass
class DefinitionComparisonResult:
    """Results from comparing two literal definitions files."""

    missing_definitions: list[DefinitionInfo]
    extra_definitions: list[DefinitionInfo]
    type_mismatches: list[DefinitionTypeMismatch]
    source_definition_count: int
    target_definition_count: int

    @property
    def issue_count(self) -> int:
        """Count actionable findings; target-only definitions are informational."""
        return len(self.missing_definitions) + len(self.type_mismatches)

    @property
    def has_findings(self) -> bool:
        return bool(self.missing_definitions or self.extra_definitions or self.type_mismatches)
