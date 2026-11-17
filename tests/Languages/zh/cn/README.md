# Simplified Chinese tests

`tests/Languages/zh/cn.rs` is only the module registry. Keep each test in the
smallest file that answers one clear audit question:

| Module | Scope |
| --- | --- |
| `core.rs` | Basic fractions, roots, powers, operators, units, and chemistry regressions |
| `textbook/` | Mainland terminology grouped by functions, sets, algebra, geometry, relations, calculus, and notation |
| `formulas/` | Complete expressions grouped by calculus, algebra, probability, geometry, units/chemistry, structure, and mixed domains |
| `navigation.rs` | Enhanced navigation speech and structural location announcements |
| `unicode/` | Symbol-name audits grouped by Unicode, MathType, and MathJax source families |

When adding a case:

1. Keep one behavior or terminology decision per test function.
2. Put a complete expression in `formulas/` and an isolated wording or
   structure decision in `textbook/` or `core.rs`.
3. Use named table cases when several expressions exercise the same rule, and
   include the case name in the error.
4. Verify the Chinese term from a reliable Mainland source before recording a
   speech expectation. Do not copy Traditional Chinese output mechanically.
5. Use `SimpleSpeak` and `ClearSpeak` in the same test when their expected
   wording is shared. Create a style-specific module only when the behavior
   actually differs.
6. Register top-level modules in `cn.rs` and nested modules in their
   `mod.rs`; `cargo test --test languages Languages::zh::cn -- --list` is the
   quick registration check.
7. Run both filesystem and packaged-rule paths:
   `cargo test --test languages Languages::zh::cn` and
   `cargo test --features include-zip --test languages Languages::zh::cn`.
