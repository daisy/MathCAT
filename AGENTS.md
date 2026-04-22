# Agent Notes (MathCAT)

Purpose: fast orientation for coding agents. try to keep low overlap with README.md etc., 
but add common mistakes of AI agents here instead.

## Project Scope
- MathCAT converts MathML to speech, braille, and navigation output.
- Core flow: `set_mathml()` -> `canonicalize.rs` -> optional `infer_intent.rs` -> `speech.rs` / `braille.rs`.

## Rules System (`Rules/`)
- YAML rules are loaded at runtime by domain:
- Common per-language files:
  - `ClearSpeak_Rules.yaml`, `SimpleSpeak_Rules.yaml`
  - `SharedRules/`, `unicode.yaml`, `unicode-full.yaml`, `definitions.yaml`, `navigate.yaml`
- `build.rs` can bundle rules into `rules.zip` when `include-zip` is enabled.

## Translation Conventions
- `t:` means untranslated or unverified.
- `T:` means translated and verified.
- don't demote from `T:` to `t:` unless instructed to.
- tool for comparing rules across languages: `uv run --project PythonScripts audit-translations <language-code>`

## Python Tooling (`uv`)
- `uv` is the Python dependency and project manager for repo tooling. Use `uv run <command-name>`
- discuss new packages before adding them. use `uv add <package-name>` and `uv sync` on confirmation
- In sandboxed runs, if needed:
  - set `UV_CACHE_DIR=/tmp/uv-cache`
  - rerun with escalated permissions if macOS `system-configuration` panics occur.
- *always* self-validate: `ùv run pytest`

## Agent Instructions
- Do not mirror README content here; keep guidance agent-specific.
- Avoid broad formatting sweeps; do not run `cargo fmt` in this repo.
- Keep code/rule changes focused and validate with targeted tests first: `cargo test <relevant-tests>`
- do not do any git commands unless explicitly asked for
- Rust coverage is in `target/coverage/`.
