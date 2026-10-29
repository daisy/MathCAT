"""Live progress display for the rule coverage `cargo test` run."""

import re

from rich.console import Group
from rich.live import Live
from rich.progress import (
    BarColumn,
    MofNCompleteColumn,
    Progress,
    SpinnerColumn,
    TaskProgressColumn,
    TextColumn,
    TimeRemainingColumn,
)
from rich.text import Text

from ..renderer import console

# cargo writes `Compiling`/`Running`/`Doc-tests` to stderr; libtest writes listings and results to stdout
COMPILING = re.compile(r"^\s*Compiling (\S+ \S+)")
TEST_BINARY = re.compile(r"^\s*(?:Running (?:unittests )?(\S+)|Doc-tests (\S+))")
LISTED_TEST = re.compile(r"^(.+): test$")
TEST_RESULT = re.compile(r"^test (.+) \.\.\. (ok|FAILED|ignored.*)$")


class CargoProgress:
    """One in-place status line (bar, ETA, current test) on a terminal; a few plain lines otherwise."""

    def __init__(self) -> None:
        self.interactive = console.is_terminal
        self.progress = Progress(
            SpinnerColumn(),
            TextColumn("{task.description}"),
            BarColumn(),
            MofNCompleteColumn(),
            TaskProgressColumn(),
            TimeRemainingColumn(),
            TextColumn("{task.fields[failures]}", style="red"),
            console=console,
        )
        self.task = self.progress.add_task("Compiling", total=None, failures="")
        self.current = Text("", style="dim", no_wrap=True, overflow="ellipsis")
        self.live = Live(Group(self.progress, self.current), console=console, transient=True)
        self.binary = ""
        self.listed = 0
        self.failed = 0

    def __enter__(self) -> CargoProgress:
        if self.interactive:
            self.live.start()
        else:
            console.print("Compiling and listing tests…")
        return self

    def __exit__(self, *exc_info: object) -> None:
        if self.interactive:
            self.live.stop()

    def listing_line(self, line: str) -> None:
        """Count tests from `cargo test -- --list`, which also compiles everything first."""
        if compiling := COMPILING.match(line):
            self.current.plain = f"Compiling {compiling[1]}"
        elif TEST_BINARY.match(line):
            self.progress.update(self.task, description="Listing tests")
            self.current.plain = ""
        elif LISTED_TEST.match(line):
            self.listed += 1

    def start_tests(self) -> None:
        """Switch to a bar over the listed tests; resetting restarts the ETA clock after compiling."""
        self.progress.reset(self.task, total=self.listed, description="Running tests")
        if not self.interactive:
            console.print(f"Running {self.listed} tests…")

    def test_line(self, line: str) -> None:
        """Advance on each libtest result and show the binary and test that just finished."""
        if binary := TEST_BINARY.match(line):
            self.binary = binary[1] or binary[2]
        elif result := TEST_RESULT.match(line):
            name, outcome = result.groups()
            self.current.plain = f"{self.binary} · {name}"
            if outcome == "FAILED":
                self.failed += 1
                console.print(Text(f"✗ {self.binary} · {name}", style="red"))
            self.progress.update(self.task, advance=1, failures=f"{self.failed} failed" if self.failed else "")
