"""Format file and rule coverage events as Markdown and an interactive HTML report."""

from collections import Counter, defaultdict
from pathlib import Path

from jinja2 import Environment, FileSystemLoader, select_autoescape

type RuleKey = tuple[str, str, str]

TEMPLATE = Environment(
    loader=FileSystemLoader(Path(__file__).parent),
    autoescape=select_autoescape(("html",)),
).get_template("rule_coverage.html")


def section(title: str, paths: set[str]) -> str:
    """List covered YAML paths in a Markdown section."""
    lines = [f"## {title} ({len(paths)})"]
    lines.extend(f"- `{path}`" for path in sorted(paths))
    return "\n".join(lines) + "\n"


def coverage(count: int, total: int) -> str:
    """Show the matched fraction and its percentage, including empty groups."""
    return f"{count}/{total} ({100 * count / total:.0f}%)" if total else "0/0 (0%)"


def rule_section(
    title: str,
    rules: set[RuleKey],
    hits: Counter[RuleKey] | None = None,
    tests: dict[RuleKey, Counter[str]] | None = None,
) -> str:
    """List active rules with their source path, name, and MathML tag."""
    lines = [f"## {title} ({len(rules)})"]
    for path, name, tag in sorted(rules):
        key = (path, name, tag)
        line = f"- `{path}`: `{name}` (`{tag}`)"
        if hits is not None and tests is not None:
            line += f" — {hits[key]} hits; tests: " + ", ".join(
                f"`{test}` ({count})" for test, count in sorted(tests[key].items())
            )
        lines.append(line)
    return "\n".join(lines) + "\n"


def render_html(
    loaded: set[str],
    matched: set[str],
    defined_rules: set[RuleKey],
    rule_hits: Counter[RuleKey],
    rule_tests: dict[RuleKey, Counter[str]],
    errors: list[str],
) -> str:
    """Show file coverage and searchable rule details in a standalone page."""
    rules_by_path: dict[str, list[dict]] = defaultdict(list)
    for path, name, tag in defined_rules:
        key = (path, name, tag)
        rules_by_path[path].append({
            "name": name, "tag": tag, "matched": rule_hits[key] > 0,
            "hits": rule_hits[key], "tests": sorted(rule_tests.get(key, {}).items()),
            "tooltip": "Tests:\n" + "\n".join(
                f"{test} ({count} {'hit' if count == 1 else 'hits'})"
                for test, count in sorted(rule_tests.get(key, {}).items())
            ) if rule_hits[key] else "No test hits",
        })

    files = []
    for path in sorted(loaded | matched | rules_by_path.keys()):
        rules = sorted(rules_by_path.get(path, []), key=lambda rule: (rule["name"], rule["tag"]))
        files.append(
            {
                "path": path,
                "matched": path in matched,
                "matched_count": sum(rule["matched"] for rule in rules),
                "rule_count": len(rules),
                "coverage": coverage(sum(rule["matched"] for rule in rules), len(rules)),
                "rules": rules,
            }
        )

    return TEMPLATE.render(
        status="Incomplete" if errors else "Complete",
        loaded_count=len(loaded),
        matched_count=len(matched),
        defined_rule_count=len(defined_rules),
        matched_rule_count=len(rule_hits),
        file_coverage=coverage(len(matched), len(loaded)),
        rule_coverage=coverage(len(rule_hits), len(defined_rules)),
        errors=errors,
        files=files,
    )
