"""
Auditing and comparison logic.

Contains functions for comparing English and translated files,
and for performing full language audits.
"""

from pathlib import Path

from .differ import diff_rules
from .models import AuditError, AuditSummary, ComparisonResult, RuleInfo
from .parsers import parse_yaml_file
from .renderer import console, print_audit_header, print_audit_summary, print_language_list, print_warnings


def split_language_into_base_and_region(language: str) -> tuple[str, str | None]:
    """Split a language code into base and optional region."""
    normalized = language.lower().replace("_", "-")
    if "-" in normalized:
        base, region = normalized.split("-", 1)
        return base, region or None
    return normalized, None


def get_rules_dir(rules_dir: str | None = None) -> Path:
    """Get the Rules/Languages directory path"""
    if rules_dir:
        return Path(rules_dir).expanduser()
    # Navigate from the package directory to the Rules directory
    package_dir = Path(__file__).parent
    return package_dir.parent.parent / "Rules" / "Languages"


def get_yaml_files(lang_dir: Path, region_dir: Path | None = None) -> list[Path]:
    """Get all YAML files to audit for a language, including region overrides."""
    files: set[Path] = set()

    def collect_from(directory: Path, root: Path) -> None:
        if not directory.exists():
            return
        for f in directory.glob("*.yaml"):
            if f.name != "prefs.yaml":  # Skip prefs.yaml as it's not translated
                files.add(f.relative_to(root))
        shared_dir = directory / "SharedRules"
        if shared_dir.exists():
            for f in shared_dir.glob("*.yaml"):
                files.add(f.relative_to(root))

    collect_from(lang_dir, lang_dir)
    if region_dir:
        collect_from(region_dir, region_dir)

    return sorted(files)


def compare_files(
    english_path: Path,
    translated_path: Path,
    issue_filter: set[str] | None = None,
    translated_region_path: Path | None = None,
    english_region_path: Path | None = None,
) -> ComparisonResult:
    """Compare English and translated YAML files"""

    def load_rules(path: Path | None) -> list[RuleInfo]:
        if path and path.exists():
            rules, _ = parse_yaml_file(path)
            return rules
        return []

    def merge_rules(base_rules: list[RuleInfo], region_rules: list[RuleInfo]) -> list[RuleInfo]:
        if not region_rules:
            return base_rules
        merged = {r.key: r for r in base_rules}
        for rule in region_rules:
            merged[rule.key] = rule
        return list(merged.values())

    english_rules = merge_rules(
        load_rules(english_path),
        load_rules(english_region_path),
    )
    translated_rules = merge_rules(
        load_rules(translated_path),
        load_rules(translated_region_path),
    )

    # Create lookup dictionaries
    english_by_key = {r.key: r for r in english_rules}
    translated_by_key = {r.key: r for r in translated_rules}

    include_all = issue_filter is None
    include_missing = include_all or "missing" in issue_filter
    include_untranslated = include_all or "untranslated" in issue_filter
    include_extra = include_all or "extra" in issue_filter
    include_diffs = include_all or "diffs" in issue_filter

    # Find missing rules (in English but not in translation)
    missing_rules = []
    if include_missing:
        for key, rule in english_by_key.items():
            if key not in translated_by_key:
                missing_rules.append(rule)

    # Find extra rules (in translation but not in English)
    extra_rules = []
    if include_extra:
        for key, rule in translated_by_key.items():
            if key not in english_by_key:
                extra_rules.append(rule)

    # Find untranslated text in translated file (skip if audit-ignore)
    untranslated_text = []
    if include_untranslated:
        for rule in translated_rules:
            if rule.has_untranslated_text and not rule.audit_ignore:
                untranslated_text.append((rule, rule.untranslated_entries))

    # Find fine-grained differences in rules that exist in both files (skip if audit-ignore)
    rule_differences = []
    if include_diffs:
        for key, en_rule in english_by_key.items():
            if key in translated_by_key:
                tr_rule = translated_by_key[key]
                if not tr_rule.audit_ignore:
                    diffs = diff_rules(en_rule, tr_rule)
                    rule_differences.extend(diffs)

    return ComparisonResult(
        missing_rules=missing_rules,
        extra_rules=extra_rules,
        untranslated_text=untranslated_text,
        rule_differences=rule_differences,
        english_rule_count=len(english_rules),
        translated_rule_count=len(translated_rules),
    )


def audit_language(
    language: str,
    specific_file: str | None = None,
    rules_dir: str | None = None,
    issue_filter: set[str] | None = None,
    verbose: bool = False,
) -> int:
    """Audit translations for a specific language. Returns total issue count."""
    rules_dir_path = get_rules_dir(rules_dir)
    english_dir = rules_dir_path / "en"

    base_language, region = split_language_into_base_and_region(language)
    translated_dir = rules_dir_path / base_language
    translated_region_dir = translated_dir / region if region else None
    english_region_dir = english_dir / region if region else None

    if not english_dir.exists():
        raise AuditError(f"English rules directory not found: {english_dir}")

    if not translated_dir.exists():
        raise AuditError(f"Translation directory not found: {translated_dir}")

    if region and not (translated_region_dir and translated_region_dir.exists()):
        raise AuditError(f"Region directory not found: {translated_region_dir}")

    # Get list of files to audit
    files = [specific_file] if specific_file else get_yaml_files(english_dir, english_region_dir)

    print_audit_header(language, len(files))

    total_issues = 0
    total_missing = 0
    total_untranslated = 0
    total_extra = 0
    total_differences = 0
    files_with_issues = 0
    files_ok = 0

    for file_name in files:
        english_path = english_dir / file_name
        translated_path = translated_dir / file_name
        translated_region_path = translated_region_dir / file_name if translated_region_dir else None
        english_region_path = english_region_dir / file_name if english_region_dir else None

        if not english_path.exists():
            console.print(f"\n[yellow]⚠ Warning:[/] English file not found: {english_path}")
            continue

        result = compare_files(
            english_path,
            translated_path,
            issue_filter,
            translated_region_path if translated_region_path and translated_region_path.exists() else None,
            english_region_path if english_region_path and english_region_path.exists() else None,
        )

        if result.has_issues:
            issues = print_warnings(result, file_name, verbose, language)
            if issues > 0:
                files_with_issues += 1
            total_issues += issues
        else:
            files_ok += 1

        total_missing += len(result.missing_rules)
        total_untranslated += sum(len(entries) for _rule, entries in result.untranslated_text)
        total_extra += len(result.extra_rules)
        total_differences += len(result.rule_differences)

    print_audit_summary(
        AuditSummary(
            files_checked=len(files),
            files_with_issues=files_with_issues,
            files_ok=files_ok,
            total_missing=total_missing,
            total_untranslated=total_untranslated,
            total_extra=total_extra,
            total_differences=total_differences,
            total_issues=total_issues,
        )
    )

    return total_issues


def list_languages(rules_dir: str | None = None) -> None:
    """List available languages for auditing"""
    rules_dir_path = get_rules_dir(rules_dir)

    languages: list[tuple[str, int]] = []
    for lang_dir in sorted(rules_dir_path.iterdir()):
        if not lang_dir.is_dir() or lang_dir.name == "en":
            continue
        languages.append((lang_dir.name, len(get_yaml_files(lang_dir))))

        for region_dir in sorted(lang_dir.iterdir()):
            if not region_dir.is_dir() or region_dir.name.lower() == "sharedrules":
                continue
            code = f"{lang_dir.name}-{region_dir.name}"
            languages.append((code, len(get_yaml_files(lang_dir, region_dir))))

    print_language_list(languages)
