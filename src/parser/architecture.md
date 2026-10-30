# Parser architecture

MathCAT's usual direction is MathML to speech and braille. The parsers go the other way: a notation string in, presentation MathML out. The public entry points are `Braille_to_MathML` and `Braille_to_Expr` in `src/parser/mod.rs`. `set_mathml_from_braille` in `src/interface.rs` parses and then runs the result through `set_mathml`, so canonicalize, speech, and braille all see the same tree as any other MathML.

Four notations are currently registered: UEB, Nemeth, ASCIIMath, and LaTeX. Each one is its own lexer and grammar. They do not share productions. What they share is the `Expr` AST, the YAML symbol loader, and the notation list.

## Shared pieces

All of the parsers share some common parts.

### Registry

`NOTATIONS` is the list of codes. Each entry is a braille code name, a `braille_to_expr` function, and a function that drops that code's cached symbol table. `clear_symbol_caches` walks the list. Speech calls it when the rules directory or the braille code changes, so a stale table is not reused.

Adding a notation is three steps, listed at the top of `mod.rs`: a parser module, a symbol table that reads that code's `unicode.yaml` through `symbols::collect_templates`, and one registry entry.

### `Expr`

`src/parser/expr.rs` is the presentation tree every parser builds. Leaves are numbers, identifiers, operators, and text. Structure covers rows, groups, scripts, fractions, roots, fences, tables, over/under scripts, prescripts, and `menclose`. `Expr::Missing` is the "not typed yet" slot and becomes `<mtext>&#xFFFD;</mtext>` (U+FFFD, the replacement character). An unclosed fence stores `close: None` and does not invent a closer.

`Expr::row` flattens a one-item list, folds a degree sign after a number into a superscript, folds a doubled tilde, and joins juxtaposed letter rows. Those folds exist so canonicalize does not merge `45°` into ℃/℉ or split a letter run that the braille treated as one word. LaTeX builds rows with `Expr::Row` directly when it must not apply those folds (a German degree sign stays an operator).

`to_mathml` wraps the tree in tags. A braille space becomes `<mo>&#xA0;</mo>` unless it is the automatic space the forward rules insert around a comparison operator. Non-ASCII characters are written as numeric character references so tests stay stable.

### Symbol tables

`src/parser/symbols.rs` reads the standard speech/braille unicode YAML files into `(print character, template)` pairs so that the mapping is reversed. Single-character keys contribute every translated string. Ranged alphabets (`translate('.', '…', 'a-z')`) are expanded with a digit cell and a letter cell. UEB uses the literary digit cells (1 is ⠁). Nemeth uses the lower digit cells (1 is ⠂). The letter cell is the same in both (a is ⠁).

Templates contain flag letters that are not part of the braille or the typed name. `𝐖` and `𝐰` are word-boundary markers. UEB and Nemeth also use `L`, `N`, `C`, and similar indicators. Each symbol module expands or strips those flags itself. The shared loader only collects the raw strings.

Tables are cached in a thread-local `RefCell`. A failed load stays cached so a missing file is not reread on every character. Lookup is longest match: the table is ordered by descending byte length, or a trie walks the longest key that is a complete token.

On an include-zip build the rules live in `rules.zip`, with each braille code inside `Rules/Braille/<Code>/<Code>.zip`. `read_to_string_shim` opens that archive on the first read. Parser tests call it before anything else has touched the zip, so the index has to be built there.

### Incomplete input

A parse does not fail because the user stopped typing. A missing argument, an unclosed fence, an unclosed fraction, or an unclosed radical stays in the tree. What "missing" looks like differs by code. UEB and Nemeth use `Expr::Missing` (U+FFFD) when a required item was announced and not typed. LaTeX uses an empty row for a slot that was never started (`\frac` is an empty fraction; `2^` is a superscript with an empty exponent). ASCIIMath uses `Expr::Missing` for a missing script or argument and leaves a fence open when the closer was not typed.

### Round trip

A round trip is notation to MathML to notation again. The audit is opt-in:

```text
cargo test --features from-braille-roundtrip --test from_braille_roundtrip -- --nocapture
```

It does not fail the build. It writes `notes/_from_braille_roundtrip.json` with a category for each mismatch. The FromBraille tests (`tests/FromBraille/`, included from `tests/from_braille.rs`) are stricter: both the original MathML and the parsed MathML go through `set_mathml`, then the trees are compared while ignoring presentation-only attributes (`FROM_BRAILLE_IGNORE_ATTRS` in `tests/common/mod.rs`: `mathvariant`, `stretchy`, `accent`, spacing, and the `data-*` bookkeeping). When the forward MathML cannot be recovered, the expected tree is the parser's tree, recorded in an overrides JSON. Those files are the list of structural mismatches, not a license to edit the forward goldens.

Two kinds of mismatch show up for every code.

- **Indicators and spacing are not in the MathML.** Grade-1, numeric, and capital indicators, and the spaces the forward rules insert around operators, are decided again when braille is generated. The regenerated string can be a correct reading of the same MathML and still differ in those cells.
- **Canonicalize rewrites the tree.** Invisible operators (U+2062 versus U+2063), empty slots, and merged digits are applied on the way back. The parser cannot put the original attribute soup back.

U+FFFD, when the parser does emit it, is brailled as that code's omission sign (UEB `⠬`, Nemeth `⠿`, and so on). Codes with no omission sign (LaTeX, ASCIIMath, and Russian's general blank) contribute an empty cell, or nothing in the text codes. See `tests/braille/omission.rs`.

## UEB

`src/parser/ueb.rs` implements the ICEB Guidelines for Technical Material. The symbol table is `src/ueb_symbols.rs`.

The lexer is mode-aware. Capital mode is one symbol, one word, or a passage (`⠠`, `⠠⠠`, `⠠⠠⠠`). Grade 1 is the same three widths (`⠰`, `⠰⠰`, `⠰⠰⠰`, closed by `⠰⠄`). A typeform indicator (`⠘` bold, and the other GTM 2.7 prefixes) applies to the next letter or number run. Those modes are consumed by the lexer. They are not nodes in the tree, which is why they reappear in different places on the way back out.

Numbers start with `⠼`. Letters are one cell at a time unless capitals or a grade-1 word joins them. Print symbols that are not letters or digits come from the YAML table, longest match. A cell sequence the lexer does not recognize is `BrailleText` and becomes `<mtext>`, so a half-typed prefix such as a lone `⠐` does not fail the parse.

Structure the grammar builds:

- Level changes `⠔` and `⠢` are superscript and subscript. A script with no item yet is `Expr::Missing`.
- Directly-above `⠨⠔` and directly-below `⠨⠢` are `Over` and `Under`. Simple bars, dots, and arrows are folded onto the preceding item.
- Fractions use the general-fraction brackets and the line `⠨⠌`. A mixed number is a whole number beside a fraction.
- Roots use `⠩` … `⠬`.
- Fences are the print brackets plus the enlarged (dot-6) fences used for GTM 15 matrices. A vertical bar `⠸⠳` is one cell for both open and close. An unclosed fence keeps `close: None`.
- Multi-row tables use the row marker `⠸⠀`. An equation-line table that is not the whole expression uses `⠠⠀` as a bound.
- Grouping indicators `⠣` … `⠜` are `Expr::Group`. They are not printed fences. A script or accent binds to the whole group.

Special cases:

- Spaces next to a comparison operator are omitted on output, because the forward `mo` rule would have inserted them.
- A degree sign after a number becomes a superscript (`Expr::row`).
- Blackboard and other math-alphanumeric ranges in the YAML are expanded with UEB digit cells. Homophones prefer plane-0 Greek and punctuation over lookalike math alphanumerics.
- Bare Latin letter cells and ASCII digits are left to the letter and number lexer so the table cannot steal them.

### Where a UEB round trip fails

The saved audit in `notes/_from_braille_roundtrip.json` is a UEB snapshot (the `iceb.rs` cases). The categories the audit assigns are the ones that still matter:

- **Grade-1, numeric, and capital indicators.** Stripping `⠰` / `⠼` / `⠠` from both strings often makes them equal. The MathML has no place to store "this letter needed a grade-1 indicator." The forward rules place indicators from standing-alone and word context, which the parse does not reconstruct. Contractions and passage indicators (`⠰⠰⠰` … `⠰⠄`) move for the same reason.
- **Spacing.** The same non-space cells with `⠀` in different places. Forward rules insert spaces around operators and in mixed text; the parser keeps only the spaces that were actually in the input, and drops the ones that match the automatic-space rule.
- **Script order and nesting.** UEB writes superscript then subscript in some constructions (`x^2_k` versus `x_k^2`). The tree records one `msubsup`. Regenerating follows the MathML child order, not the original cell order.
- **Chemistry and prescripts.** A prescript plus a postscript becomes `mmultiscripts`. The forward rules sometimes use a different nesting (a subscript of a multiscript, or a grade-1 wrapper around the prescript). The cells differ even though the print is the same.
- **Typeform on digits.** Bold or script digit runs can be split in the source MathML (`67` and `845` as two `mn`s with a typeform indicator between them). The parser joins a digit run. The indicator cells are gone on the way back.
- **Shapes, accents, and omission.** A shape or an accent that was a single modified item may be regenerated with grouping indicators. A wide space that the forward rules treated as omission `⠬` can come back as an ordinary space, because the parse saw a space and the MathML no longer says "this space was an omission."
- **Fractions.** A general-fraction bracket sequence that the parser did not recognize as a fraction comes back as a row, and the fraction markers are gone.

## Nemeth

`src/parser/nemeth.rs` implements BANA Nemeth. The symbol table is `src/nemeth_symbols.rs`.

The lexer has to know which mode a cell is in, because the same cells are numbers, fraction closers, or English-letter indicators.

- `⠼` opens a number. Inside a number, `⠨` is the decimal point and `⠠` is a comma. Outside a number, `⠼` can close a simple fraction.
- Unmarked digits are allowed after an operator, a fence, a fraction line, or a letter (the implicit numeric subscript). They are not allowed at the start of an expression.
- The English-letter indicator versus a subscript is decided from what follows and from whether the pattern is a prescript (`⠰` item `⠐` base).
- Typeform prefixes on a numeric indicator (`⠸⠼` bold, `⠈⠼` script, `⠠⠸⠼` double-struck, `⠠⠨⠼` sans-serif) style the following digits. Script digits have no Unicode math-alphanumeric form, so they keep `mathvariant="script"`.
- Print symbols come from the YAML table, longest match. Flag letters are expanded the same way as Nemeth cleanup.

Structure:

- Simple, mixed, complex, and hypercomplex fractions. The kind is the number of `⠠` prefixes on `⠹` / `⠌` / `⠼`. A beveled line is `⠸⠌`.
- Radicals: index `⠣`, opener, closer `⠻`. `⠻` is a modifier terminator when a radical is not open.
- Scripts are level indicators relative to the baseline. `⠐` is either baseline or the multipurpose indicator; the parser decides from what follows. Directly-over `⠣` and directly-under `⠩` build `Over`, `Under`, and binomials.
- A letter, function name, or chemical element followed by unmarked digits is a subscript (`Cl₂`, `H₂`), not digits glued onto the letters.
- Linearized matrices use `⣍` between rows.
- Spaces that are not the automatic spaces around comparisons become `&#xA0;`.

Special cases:

- **`⠨⠅` is `=` or κ.** The YAML table keeps `=` for those two cells (it outranks Greek). The parser rewrites the token. `=` only when a blank sits on both sides. A blank is the braille space `⠀` or any Unicode whitespace; U+2800 is not Unicode whitespace, so both have to be checked. End of string is not a blank, with one exception: Nemeth cleanup trims a trailing `⠀`, so a comparison at the end of the generated braille (`…2 =`) has a blank before `⠨⠅` and nothing after. That stays `=`, unless the cells are `⠀⠨⠅⠀⠨⠅`, which is `= κ` (the second sign follows an equals that already had blanks on both sides). If indicators are glued on (`⠐` before, `⠣` or `⠩` after, a punctuation `⠠` after a blank), the reading is unclear and stays `=`. Longer symbols that are not a pure run of `⠨⠅` (`⠌⠨⠅` for ≠, `⠠⠨⠅` for Κ) win by longest match before this rule runs. `⠨⠅⠨⠅` and `⠨⠅⠨⠅⠨⠅` are split into single `⠨⠅` cells first, so `⠭⠀⠨⠅⠀⠨⠅⠨⠅` is x = κκ rather than x = ⩵.
- `should_leave_for_lexer` keeps `⠐` plus a digit, decimal, or plus for the multipurpose indicator instead of letting the symbol table swallow them. `⠠⠠` plus one letter with more letters after is a capital word, not a Hebrew letter.

### Where a Nemeth round trip fails

`notes/_nemeth_frombraille_overrides.json` lists forward MathML the parser does not rebuild. The test then expects the parser's tree. The reasons cluster:

- **`⠨⠅` and trimmed blanks.** A generated equals at the end of the string loses its trailing blank. The heuristic above usually restores `=`, but `= κ` at the end of the string is the same cells as `κ =` with the trailing blank trimmed (`⠨⠅⠀⠨⠅`). The rule keeps `κ` when the first `⠨⠅` is itself a spaced equals, and `=` when the first `⠨⠅` is unspaced. Other spacings of the same two cells can regenerate with a different last sign.
- **Chemistry.** Bonds (`:`), reaction arrows, state symbols `(g)` / `(aq)`, charges, and staggered scripts are linearized in the braille. The parser returns a flat row or a different script nesting than the mhchem source. The override is the flat reading.
- **Implicit subscripts and function spaces.** `sin 30°` is an identifier, a space, and a superscript. The space is real in the input and becomes `&#xA0;`, while the forward rules may have omitted it. The regenerated braille gains or loses that cell.
- **Baseline and nested scripts.** A multipurpose indicator that returns to the baseline is not stored. A tower of superscripts can be parsed as nested `msup` or as one script whose inner `⠘` was a level indicator. The forward rules emit the level indicators again from the tree they receive, which is not always the original tower.
- **Incomplete indicators become U+FFFD.** A radical or script opener with no item (`dots_2_37_a`, `integral_77_4_26`, `superscript_80_a_2`) leaves `Expr::Missing`. Brailling U+FFFD is the omission sign `⠿`, not the original dots. That is a different string even when the surrounding structure matches.
- **Typeform and double-struck digits.** Styled digits are one `mn`. The source sometimes splits them, or uses a mathvariant the forward rules encode with a different indicator sequence.
- **Numbers and spaces.** A space inside a number (`1 3 5`, `123 456`) is kept as `&#xA0;` between `mn`s. The forward numeric-indicator rules may treat that as one number or as an omission. The cells differ.

## ASCIIMath

`src/parser/asciimath.rs` follows the grammar at <https://asciimath.org/>, the same recursive descent as ASCIIMathML.js. The symbol table is `src/asciimath_symbols.rs`.

Built-in names (`sin`, `frac`, `sum`, `hat`, …) are fixed at a rank the YAML cannot override, so a unicode file cannot change a function into a constant. New names loaded from YAML are constants. Load order is LaTeX `unicode-full.yaml`, LaTeX `unicode.yaml`, ASCIIMath `unicode-full.yaml`, ASCIIMath `unicode.yaml`. The short file wins. `𝐖` / `𝐰` are stripped. A template with an internal space or with braces (other than `\{` and `\}`) is not a single name. Apostrophe-only names are skipped so `f'` stays a superscript.

Lookup is longest match. `succeq` beats `succ`, and `\partialmeetcontraction` beats `\partial`. A leading backslash is optional: `\sin` matches the builtin `sin` by stripping the slash, and the direct hit is kept when it consumes more of the input than the stripped one (`\cancel` beats a short LaTeX `\c`).

The grammar is `parse_expr` of infix operators over `parse_i` (scripts and functions) over `parse_s` (atoms). Atoms include constants, numbers, fences, `\left` / `\right`, absolute value, `text`, unary and binary operators, fractions, roots, and accents.

Special cases:

- **Decimals.** The separator comes from `DecimalSeparators`. A comma is the decimal mark only when the preference contains `,` and not `.`. A comma followed by a space is a list separator even then.
- **Matrices.** Comma-separated fenced rows that all use `()` or all use `[]`, with more than one row or more than one cell, become `Expr::Table`. One parenthesized group is not a matrix.
- **Fonts.** `bb`, `bbb`, and the other font commands map letters into the Mathematical Alphanumeric block, with the same BMP holes as `shift_char` in `canonicalize.rs`.
- **Prescripts.** A leading `_` or `^` with no base is `Expr::Prescripts`.
- **Minus after an infix.** If `-` follows an infix operator, it is unary and binds the next atom (`/-b`).
- **Definitions.** A `def` symbol is expanded and the tail is reparsed, with a depth cap of 12, so a definition cannot loop.
- **Functions.** `sin^2 x` takes `x` as the argument. `f` and `g` take an argument only when it is parenthesized.
- **Large operators.** `sum`, `lim`, `prod`, and the under/over accents attach scripts as `munder` / `mover` / `munderover`.
- Incomplete input stays in the tree: an unclosed `(` is a fence with `close: None`, and a missing script item is `Expr::Missing`.

### Where an ASCIIMath round trip fails

`notes/_asciimath_from_tests_overrides.json` is the list of forward MathML the parser will not reproduce. The test expects the parser's tree instead.

- **`menclose`.** ASCIIMath has `cancel` and bar characters, not MathML's notation list. A strike becomes `cancel` or is dropped. A box, a circle, or a stack of arrows is a bar, an underline, or the bare body. The `notation` attribute cannot be rebuilt, so the regenerated ASCIIMath is whatever those replacement operators emit.
- **Binomials.** `((n),(k))` is a column of two parenthesized cells. The forward golden is a parenthesized fraction with `linethickness="0"`. The parser keeps the table. Regenerating prints the table, not `binom`.
- **Comma and the decimal preference.** `0,1` with a dot decimal preference is `0`, a comma, and `1`, not one number. The forward MathML had a single `mn`. The regenerated text has a comma and a space.
- **Mixed numbers.** A mixed fraction in the source is a number beside a fraction. The ASCIIMath encoding is juxtaposition. Canonicalize may insert an invisible plus or join them differently on the way back.
- **Invisible operators and fonts.** Juxtaposition becomes U+2062 or U+2063. The comparison normalizes 2062 to 2063, so the FromBraille test passes, but the regenerated ASCIIMath has no character for that operator. Fonts that went through the alphanumeric block can come back as a font command plus a letter, or as the literal Unicode letter, depending on which YAML entry is the forward spelling.

## LaTeX

`src/parser/latex.rs` accepts the LaTeX that `Rules/Braille/LaTeX` emits, not a full TeX grammar. The symbol table is `src/latex_symbols.rs`.

The table is every template that is one token after `𝐖` / `𝐰` are turned into spaces and trimmed. Grouped composites (`\acute{\eta}`) are skipped; the pieces are parsed on their own. A backslash command that ends in a letter does not match inside a longer command (`\in` does not consume `\infty`). Both the long form and the short form are loaded, so `\frac` and `\f`, `\pi` and `~p`, `\le` and `<=`, `\mathbb Z` and `\Z` are all accepted regardless of `LaTeX_UseShortName`. `\mathbb Z` includes the space and the letter in one key. `\not=` is one key for ≠.

Function names from `definitions.yaml` (`\sin`, `\log`, `\lim`, …) are identifiers, except the limit names (`lim`, `max`, `min`, `sup`, `inf`, and the `liminf` / `limsup` pair), which are operators so that scripts on them become under/over. A bare word such as `cos` or `log` is the same function when the next character is not a letter, which is how the short-name output (`cos^2`) is read. Any other letter is one identifier.

Numbers are digits with an optional `.` or `,` glued to more digits. A comma with a space after it is a separator (`1, 2`), which matches LaTeX cleanup: list commas gain a space and commas inside a number do not.

The structural commands, recognized when `{` or `[` follows, or when the command is one of these even with no argument yet:

- `\frac` / `\f`, `\sqrt` / `\s` with an optional `[index]`, `\binom`
- Accents: `\overline` / `\ol`, `\underline`, `\vec`, `\hat`, `\dot`, `\ddot`, `\tilde`, `\overbrace`, `\underbrace`, `\overset`, `\underset`
- `\boxed`, `\circle`, `\overwrite{body}{symbol}`
- `\begin{env}` … `\end{env}` for `pmatrix`, `bmatrix`, `vmatrix`, `Vmatrix`, `Bmatrix`, `matrix`, `array`, and the short names `bpm`, `bbm`, `bvm`, `bVm`, `bBm`
- `\left` / `\right`, `\text`

`\mkern`, `\mskip`, and `\hspace{length}` are a sign, a decimal, and `mu` / `em` / `ex` / `pt` (1 em = 18 mu, 1 ex = 9 mu, 1 pt = 1.8 mu), then the same spaces as `\,` and `\quad`. At least 14 mu is an em space, 9–13 an en, 4.5–8 a three-per-em, 3.5–4.5 a medium space, 2–3.5 a thin space, and anything smaller, zero, or negative a zero-width space. Past 18 mu, one em space is emitted per whole em. `\rm`, `\bf`, `\it`, `\cal`, `\frak`, `\sf`, and `\tt` apply to later letters until the group ends. An argument font such as `\mathbf` ignores an outer declaration, so `{\it \mathbf{x}}` is bold.

`\bra` and `\ket` are the fences ⟨ ∣ and ∣ ⟩. `[\![`, `]\!]`, and `\lbrack\!\lbrack` / `\rbrack\!\rbrack` (including after `\left` / `\right`) are ⟦ and ⟧, read before `\!` becomes a zero-width space. `\cr` is a row break. `\begin{cases}` puts `{` before the table and `\begin{rcases}` puts `}` after it. `smallmatrix`, `split`, `gathered`, `align`, `align*`, `aligned`, `gather`, `eqnarray`, and `multline` are bare tables. `alignat` and `alignedat` drop the `{n}` column spec the way `array` drops `{cc}`. `equation` and `equation*` are that one cell, or a bare table when the body has more than one cell. `\eqalign{...}` is the same table inside one group. `\xrightarrow[below]{above}` and `\xleftarrow` are under, over, or underover on → or ←. `\genfrac{left}{right}{thick}{style}{num}{den}` is a fraction (linethickness 0 when the thickness is 0) inside those fences; the style is ignored. `\root n \of expr` is an `mroot`. `\color` and `\textcolor` parse and drop the color.

`\sum`, `\int`, `\prod`, and the limit names take `_` and `^` as under and over scripts. Other bases take them as sub and sup. A leading `_` or `^` with no base is an empty-base `msubsup` followed by the next atom, which is how `_{95}^{238}U` is emitted. Primes are a superscript of `'`.

Special cases:

- **Empty slots, not U+FFFD.** A `{` that was never typed is an empty row. `\frac` is an empty fraction. `2^` and `2_{` are a script with an empty script. `\sqrt` and `\sqrt{` are an empty square root. `\sqrt[3]{` is a cube root whose radicand is empty. `\binom` with a missing argument leaves that part empty. An opened group keeps what was typed (`2^{x` is `x` in the exponent).
- **Arrays.** Cells are split on `&`, rows on `\\` or `\cr`. A trailing `&` or `\\` starts an empty cell or an empty row. `\begin{array}{cc}` and `\begin{alignat}{2}` treat the following group as a column spec and do not put it in a cell. The environment name supplies the fences even when `\end` was not typed.
- **`\left` without `\right`** is a fence with `close: None`. No closer is invented.
- **`\binom`** is emitted with parentheses around the linethickness-0 fraction, because the forward rule only writes `\binom` for a parenthesized binomial.
- **Large-operator scripts** on `∑` and `lim` are `munder` / `mover` / `munderover`, matching the movable-limits rule.
- A stray `}`, `&`, `]`, or `\\` outside the construct that owns it is kept as an operator or dropped, so the parse cannot loop on a closer it did not expect.

### Where a LaTeX round trip fails

`notes/_latex_from_tests_overrides.json` holds the four `menclose` cases the parser cannot rebuild. Everything else in the LaTeX FromBraille tests matches the forward MathML after canonicalize.

- **`menclose` notation is not in the LaTeX.** `\boxed{x}` is what the rules emit both for `notation="box"` and for all four sides. The parser reports `box`. Nested `\overwrite` stores the strike and arrow names in the order the braces nest, which is not the order of the `notation` attribute (`uparrow downarrow leftarrow rightarrow` comes back as `leftarrow rightarrow uparrow downarrow`). A trailing space in the attribute is not in the LaTeX at all. `|x|` plus `\overline{\underline{x}}` is fences and accents, not `notation="left right"` and `notation="top bottom"`.
- **Short names and spacing.** The parser accepts both spellings. Regenerating uses the current `LaTeX_UseShortName` and the current cleanup, which inserts `𝐖` as spaces and then deletes the space before `^`, `_`, and commas. The text can differ by those spaces even when the MathML matches.
- **Empty slots versus the original tree.** An incomplete command becomes an empty `<mrow>`, which canonicalize turns into an empty-content `mtext`. The forward rules then emit nothing (LaTeX has no omission sign). A tree that originally used U+FFFD in that slot brailles as nothing, not as the omission dots some other code would use.
- **Binomials and matrices.** `\binom` always gains parentheses. A `pmatrix` is parentheses around an `mtable`. The source may have been `mfenced` or a pair of `mo`s with `minsize`. Canonicalize usually erases that difference; when it does not, the regenerated LaTeX follows the parser's fences.
