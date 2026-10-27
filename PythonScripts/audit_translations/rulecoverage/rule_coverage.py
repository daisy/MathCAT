"""Run the full Rust test suite and report which rule YAML files it exercises."""

import json
import re
import shutil
import subprocess
import sys
import webbrowser
from collections import Counter, defaultdict
from collections.abc import Callable
from pathlib import Path, PurePosixPath
from typing import TextIO

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
from .rule_coverage_report import RuleKey, coverage, render_html

ROOT = Path(__file__).resolve().parents[3]
OUTPUT = ROOT / "target" / "rule-coverage"
EVENTS = OUTPUT / "events"
COMMAND = ["cargo", "test", "--features", "rule-coverage"]

# cargo writes `Compiling`/`Running`/`Doc-tests` to stderr; libtest writes listings and results to stdout
COMPILING = re.compile(r"^\s*Compiling (\S+ \S+)")
TEST_BINARY = re.compile(r"^\s*(?:Running (?:unittests )?(\S+)|Doc-tests (\S+))")
LISTED_TEST = re.compile(r"^(.+): test$")
TEST_RESULT = re.compile(r"^test (.+) \.\.\. (ok|FAILED|ignored.*)$")


def stream(command: list[str], log: TextIO, on_line: Callable[[str], None]) -> int:
    """Run a command from the repo root, copying each output line to the log and passing it to `on_line`."""
    log.write(f"$ {' '.join(command)}\n")
    log.flush()
    try:
        process = subprocess.Popen(
            command,
            cwd=ROOT,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            encoding="utf-8",
            errors="replace",
        )
    except OSError as error:
        log.write(f"Could not run cargo: {error}\n")
        return 1
    with process:
        for line in process.stdout:
            log.write(line)
            on_line(line.rstrip("\r\n"))
        return process.wait()


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


def read_events() -> tuple[set[RuleKey], dict[RuleKey, Counter[str]], list[str]]:
    defined_rules: set[RuleKey] = set()
    hits_by_test: dict[RuleKey, Counter[str]] = defaultdict(Counter)
    errors: list[str] = []
    for event_file in sorted(EVENTS.glob("*.jsonl")):
        for number, line in enumerate(event_file.read_text(encoding="utf-8").splitlines(), 1):
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                errors.append(f"Invalid JSON in {event_file.name}:{number}")
                continue
            if not isinstance(event, dict):
                errors.append(f"Invalid event in {event_file.name}:{number}")
                continue
            kind = event.get("kind")
            path = event.get("path")
            if not isinstance(path, str):
                errors.append(f"Invalid event in {event_file.name}:{number}")
                continue
            path = path.replace("\\", "/")
            parts = PurePosixPath(path).parts
            if (
                not parts
                or PurePosixPath(path).is_absolute()
                or ".." in parts
                or PurePosixPath(path).suffix not in (".yaml", ".yml")
            ):
                errors.append(f"Invalid event in {event_file.name}:{number}")
            elif kind in ("defined-rule", "matched-rule") and {"name", "tag"} <= event.keys():
                name, tag = event["name"], event["tag"]
                if not isinstance(name, str) or not isinstance(tag, str) or not name or not tag:
                    errors.append(f"Empty rule identity in {event_file.name}:{number}")
                elif kind == "defined-rule" and event.keys() == {"kind", "path", "name", "tag"}:
                    defined_rules.add((path, name, tag))
                elif (
                    kind == "matched-rule"
                    and event.keys() == {"kind", "path", "name", "tag", "test"}
                    and isinstance(event["test"], str)
                    and event["test"]
                ):
                    key = (path, name, tag)
                    hits_by_test[key][event["test"]] += 1
                else:
                    errors.append(f"Invalid event in {event_file.name}:{number}")
            else:
                errors.append(f"Invalid event in {event_file.name}:{number}")
    return defined_rules, dict(hits_by_test), errors


def run() -> int:
    """Generate the HTML report and open it when the run finishes."""
    OUTPUT.mkdir(parents=True, exist_ok=True)
    (OUTPUT / "report.md").unlink(missing_ok=True)
    if EVENTS.exists():
        shutil.rmtree(EVENTS)
    EVENTS.mkdir()

    log_path = OUTPUT / "test.log"
    print(f"Running {' '.join(COMMAND)}; saving output to {log_path.relative_to(ROOT)}", flush=True)
    with log_path.open("w", encoding="utf-8") as log, CargoProgress() as progress:
        # listing first gives the bar a total; a failure here (e.g. a compile error) skips the real run
        test_status = stream([*COMMAND, "--", "--list"], log, progress.listing_line)
        if not test_status:
            progress.start_tests()
            test_status = stream(COMMAND, log, progress.test_line)

    defined_rules, hits_by_test, errors = read_events()
    matched_rules = set(hits_by_test)
    rule_files = {path for path, _, _ in defined_rules}
    matched_files = {path for path, _, _ in matched_rules}
    if test_status:
        errors.insert(0, f"cargo test failed (exit status {test_status}); see test.log")
    if not defined_rules:
        errors.append("No active rule definitions found")
    if not matched_rules:
        errors.append("No matched rule events found")
    if matched_rules - defined_rules:
        errors.append("Matched rules lack definition events")

    status = "Incomplete" if errors else "Complete"
    html_path = OUTPUT / "index.html"
    html_path.write_text(render_html(defined_rules, hits_by_test, errors), encoding="utf-8")
    print(
        f"{status}: {coverage(len(matched_files), len(rule_files))} rule files matched; "
        f"{coverage(len(matched_rules), len(defined_rules))} rules matched; report: {html_path}"
    )
    try:
        opened = webbrowser.open(html_path.resolve().as_uri())
    except OSError, webbrowser.Error:
        opened = False
    if not opened:
        print(f"Browser unavailable; open {html_path} manually")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(run())
