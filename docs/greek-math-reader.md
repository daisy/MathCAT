# Greek Math Reader for MathCAT

This contribution adds `GreekMathReader`, a native Greek speech style, and integrates the complete 101-concept terminology export from [NVDA Greek Math / Greek Math Reader](https://github.com/ChristosBouronikos/NVDA-Greek-Math). It is a draft language contribution for mathematical and listening review. The implementation uses MathCAT's YAML rules, intent processing, navigation, preferences, and braille engine.

Select Greek (`el`) and the `GreekMathReader` speech style in a host that includes these rules. Library callers set `Language` to `el` and `SpeechStyle` to `GreekMathReader`. The style is discoverable through `get_supported_speech_styles("el")`. NVDA's [speech-style discovery](https://github.com/nvaccess/nvda/blob/master/source/mathPres/MathCAT/localization.py) enumerates the rule filenames, so a version containing this rules package can offer the style. This has been checked in source; Windows/NVDA interaction and listening remain to be tested.

## Implementation

| Capability | Native implementation |
| --- | --- |
| Greek pronunciation | Phonetic Latin letter names, selected Greek letter spellings, and the existing capital-word, pitch, beep, and typeface preferences. The established styles keep their letter conventions. |
| Arithmetic and algebra | Greek equality and scalar multiplication; decimal comma speech with configurable input separators; concise simple fractions; explicit compound-fraction, root, exponent, and subscript boundaries. |
| Calculus | First ordinary and partial derivative notation, differential tails in bounded integrals, and native MathCAT limits, sums, vector calculus, and general structural fallbacks. Higher-order or domain-specific concepts can use explicit intent. |
| Probability and statistics | Conventional `E`, `P`, `Var`, and `Cov` function notation, including binary conditional expectation/probability. An author can request `:literal` to retain structural notation. |
| Quantum notation | Paired angle brackets and bars for bra–ket products and matrix elements. Explicit absolute-value intent is preserved. A bare dagger retains its symbol reading; an author can specify `quantum-adjoint` explicitly. |
| Advanced terminology | All 101 exported semantic names: 92 new mappings and nine existing Greek mappings. Greek argument grammar covers coordinates, derivatives, transforms, evaluation, normal distributions, modular congruence, quotient groups, and other multi-argument concepts. |
| Native structures | Matrices, cases, multiscripts, units, chemistry, and other structures inherit the Greek SimpleSpeak/shared rules. Currency singulars/plurals and a missing matrix-dimension word are localized. The fullwidth plus sign is corrected to addition instead of equality. |
| Navigation | Greek command prefixes, directions and part names; next/previous character movement; corrected child selection when zooming into a structure. Regression tests check spoken output and cursor positions in fractions and matrices. |
| Braille | Uses MathCAT's existing braille codes. A regression checks that changing among the three Greek speech styles preserves UEB output. No Greek mathematical-braille code is introduced. |

Custom semantic grammar checks arity and fixity. Unsupported argument counts retain the generic operand-preserving fallback. Explicit literal notation and silent concept names remain available. Statistical recognition is a convention of this speech style, not a claim that an isolated `E`, `P`, or other letter identifies a subject area.

The style supplies the standard Greek Math Reader conventions within MathCAT. The add-on's separate school/university terminology profiles, domain selector, personal overrides, conversion engines, diagnostics UI, provider hooks, keyboard/clipboard commands, and voice manager are host/add-on features. They are not implemented by this MathCAT language PR. MathCAT's existing verbosity and presentation preferences remain available; they are not equivalent to those add-on profiles.

## Inputs and document corpus

MathCAT consumes Presentation MathML. Content MathML and input in LaTeX, UnicodeMath, images or document formats require conversion before calling it. A `semantics` presentation branch or `annotation-xml` with `MathML-Presentation` is supported; a TeX annotation alone is not a TeX parser.

`tests/Languages/el/reader_documents.rs` includes all nine contributed document examples, plus nine notation/fallback regressions. Two source inputs are adapted: the Content MathML interval is expressed as Presentation MathML with `open-closed-interval` intent, and the Word-style `mml:` prefix receives its required namespace declaration. These cases test MathML payloads, not live Word, PDF, browser, or UI Automation extraction.

The expected strings are native MathCAT expectations. They are not a claim of exact add-on output parity: MathCAT supplies its own pauses and boundaries, announces bold type when requested, and orders multiscript descriptions using its native convention. Latin `E` is pronounced `ί`, consistently with the letter table, rather than the older corpus's `έψιλον`. A complex denominator receives fraction boundaries. Both the integral bounds and differential variable remain audible.

For ambiguous advanced notation, use explicit intent and named arguments:

```xml
<math xmlns="http://www.w3.org/1998/Math/MathML">
  <mrow intent="cross-product($u,$v)">
    <mi arg="u">u</mi><mo>×</mo><mi arg="v">v</mi>
  </mrow>
</math>
```

A [listening page](greek-math-reader-examples.html) provides the nine document examples with the expected native speech for expert and NVDA review.

## Provenance and review status

Greek Math Reader is by **Bouronikos Christos (cbouronikos@uth.gr)**. The source snapshot is [`af0641133e4865a82d3296d58703489816dc2102`](https://github.com/ChristosBouronikos/NVDA-Greek-Math/tree/af0641133e4865a82d3296d58703489816dc2102), registry `2026.08.2`. Its [`LICENSE.md`](https://github.com/ChristosBouronikos/NVDA-Greek-Math/blob/af0641133e4865a82d3296d58703489816dc2102/LICENSE.md) explicitly offers `mathcat-el/generated/` for inclusion under MathCAT's terms. The rules here are native MathCAT implementations; the add-on's Python engine is not embedded. Existing Greek contributors Theodora Antonakopoulou and Paraskevi Riga remain credited in [the project documentation](index.md).

The registry contains **16 `reviewed` and 85 `source-checked-pending-expert-review`** entries. The table below preserves those original labels and source descriptions, including entries with citations pending. These are source-registry labels, not evidence of independent expert approval, MathCAT acceptance, or blind-user validation. New text remains lowercase `t:`; no existing verified `T:` entry is demoted.

The translation audit now loads the Greek pack. It previously encountered malformed Unicode quote keys, a duplicate YAML condition, duplicate plural keys, and an empty `KnownWords` set. The duplicate plural keys are consolidated using the last value already used by MathCAT. The audit parser now accepts an empty mapping as a set, matching the Rust loader. Other language rules and the Rust runtime are unchanged.

## Validation and remaining review

The implementation is based on MathCAT `3bbbd4426725b1b20e6f501d2312e7d92f2ed6bf` (`0.7.6-rc.4`). Use a matching MathCAT build when testing the rules package.

The active Greek suite contains **164 tests** across speech, the 101 exported concepts, real document payloads, symbol pronunciation, preferences, braille preservation, and navigation. The language is now registered in `tests/languages.rs`. The 1,449 pre-existing Greek test annotations remain in their previously disabled modules: they include untranslated/stale expectations and are not counted as passing coverage. Existing speech, braille and navigation goldens are unchanged.

Run the targeted checks and upstream configurations:

```sh
cargo test --locked --test languages Languages::el
uv run --project PythonScripts pytest PythonScripts/audit_translations/tests
uv run --project PythonScripts audit-translations el
RUSTFLAGS=-Dwarnings cargo test --locked
RUSTFLAGS=-Dwarnings cargo test --locked --features no-unsafe
RUSTFLAGS=-Dwarnings cargo clippy --locked
RUSTFLAGS=-Dwarnings cargo clippy --locked --features no-unsafe
cargo run --locked --bin package-rules -- Rules Rules.zip
cargo run --locked --bin package-rules -- Rules Rules-minimized.zip --minimize
```

The audit compares the 12 English-reference files, so additional style/shared files are covered by the Rust regressions and require direct linguistic review. The two reported missing Unicode keys are already covered by Greek's separate `Ⓐ-Ⓩ`/`ⓐ-ⓩ` ranges and separate `⟂`/`⊥` entries. There are no missing definitions or definition type mismatches. The audit reports 3,852 lowercase text entries and 277 rule differences across the reference files. Lowercase text and intentional structural differences remain reported; successful parsing is not translation approval.

Before marking the contribution ready for merge, follow the [MathCAT translator guide](https://daisy.github.io/MathCAT/helpers.html): review terminology and grammar with a native-speaking mathematics expert, listen with Greek NVDA voices, obtain blind-user feedback, and review the pre-existing Greek translation issues. Exercise fractions, matrices, cases, statistics, quantum expressions, document extraction, navigation and braille on Windows. Record the NVDA and MathCAT versions, synthesizer, preferences, findings and reviewer approval. These human checks are pending.

## Release path

This is one MathCAT pull request. Maintainers decide its style and terminology acceptance, then release the rules. NVDA must subsequently include a MathCAT version/package containing the change. A MathCAT PR or merge does not guarantee inclusion in a particular NVDA release. See the [developer guide](https://daisy.github.io/MathCAT/developers.html) and [upstream CI workflow](https://github.com/daisy/MathCAT/blob/main/.github/workflows/rust.yml).

## Generated terminology registry snapshot

| Concept ID | Standard head (`forms.standard`) | Registry source | Registry review status |
| --- | --- | --- | --- |
| absolute_value | απόλυτη τιμή του | Greek school books | reviewed |
| acceleration | επιτάχυνση | Greek school books / Kallipos | source-checked-pending-expert-review |
| adjoint | συζυγής ανάστροφος του | Kallipos | source-checked-pending-expert-review |
| angular_momentum | στροφορμή | Kallipos | source-checked-pending-expert-review |
| anticommutator | αντιμεταθέτης των | Kallipos | source-checked-pending-expert-review |
| asymptotic_equivalence | είναι ασυμπτωτικά ισοδύναμο με | Kallipos | source-checked-pending-expert-review |
| augmented_matrix | επαυξημένος πίνακας | Kallipos | source-checked-pending-expert-review |
| block_matrix | πίνακας κατά μπλοκ | Greek academic usage; citation pending | source-checked-pending-expert-review |
| boundary_condition | συνοριακή συνθήκη | Kallipos | source-checked-pending-expert-review |
| bounded_operator | φραγμένος τελεστής | Greek academic usage; citation pending | source-checked-pending-expert-review |
| bra | μπρα | Kallipos | source-checked-pending-expert-review |
| braket | εσωτερικό γινόμενο | Kallipos | source-checked-pending-expert-review |
| cardinality | πληθάριθμος του | Greek school books / Kallipos | reviewed |
| classical_hamiltonian | χαμιλτονιανή συνάρτηση των | Maintainer wording approval 2026-09-08; expert review pending | source-checked-pending-expert-review |
| closed_interval | κλειστό διάστημα | Greek school books | reviewed |
| closed_open_interval | διάστημα κλειστό αριστερά ανοιχτό δεξιά | Greek school books | reviewed |
| commutator | μεταθέτης των | Kallipos | source-checked-pending-expert-review |
| conditional_expectation | δεσμευμένη αναμενόμενη τιμή του | Kallipos | source-checked-pending-expert-review |
| confidence_interval | διάστημα εμπιστοσύνης | Kallipos | source-checked-pending-expert-review |
| contravariant_index | ανταλλοίωτος δείκτης του | Kallipos | source-checked-pending-expert-review |
| coordinate | σημείο με συντεταγμένες | Greek school books | reviewed |
| covariance | συνδιακύμανση των | Kallipos | source-checked-pending-expert-review |
| covariant_index | συναλλοίωτος δείκτης του | Kallipos | source-checked-pending-expert-review |
| cross_product | διανυσματικό γινόμενο | Kallipos | source-checked-pending-expert-review |
| curl | στροβιλισμός του | Kallipos | source-checked-pending-expert-review |
| diagonal_matrix | διαγώνιος πίνακας του | Kallipos | source-checked-pending-expert-review |
| differential_form | διαφορική μορφή | Greek academic usage; citation pending | source-checked-pending-expert-review |
| dirac_delta | δέλτα του Ντιράκ στο | Maintainer wording approval 2026-09-08; expert review pending | source-checked-pending-expert-review |
| directional_derivative | παράγωγος κατά την κατεύθυνση | Kallipos | source-checked-pending-expert-review |
| divergence | απόκλιση του | Kallipos | source-checked-pending-expert-review |
| dot_product | εσωτερικό γινόμενο | Greek school books / Kallipos | reviewed |
| eigenvalue | ιδιοτιμή του | Kallipos | source-checked-pending-expert-review |
| eigenvector | ιδιοδιάνυσμα του | Kallipos | source-checked-pending-expert-review |
| electric_field | ηλεκτρικό πεδίο | Greek school books / Kallipos | source-checked-pending-expert-review |
| entropy | εντροπία | Kallipos | source-checked-pending-expert-review |
| estimator | εκτιμητής του | Kallipos | source-checked-pending-expert-review |
| evaluation | υπολογισμένο | Kallipos | source-checked-pending-expert-review |
| expectation | αναμενόμενη τιμή του | Greek school books / Kallipos | source-checked-pending-expert-review |
| exterior_product | εξωτερικό γινόμενο | Kallipos | source-checked-pending-expert-review |
| field_structure | σώμα | Greek academic usage; citation pending | source-checked-pending-expert-review |
| four_momentum | τετραορμή | Kallipos | source-checked-pending-expert-review |
| four_vector | τετραδιάνυσμα | Kallipos | source-checked-pending-expert-review |
| fourier_transform | μετασχηματισμός Φουριέ της | Kallipos | source-checked-pending-expert-review |
| gamma_function | συνάρτηση γάμα του | Maintainer wording approval 2026-09-08; expert review pending | source-checked-pending-expert-review |
| generalized_function | γενικευμένη συνάρτηση | Greek academic usage; citation pending | source-checked-pending-expert-review |
| gradient | ανάδελτα του | Kallipos | source-checked-pending-expert-review |
| group | ομάδα | Greek academic usage; citation pending | source-checked-pending-expert-review |
| hamiltonian | χαμιλτονιανός τελεστής | Kallipos | source-checked-pending-expert-review |
| hessian | εσσιανός πίνακας του | Kallipos | source-checked-pending-expert-review |
| hypothesis_test | έλεγχος υπόθεσης | Kallipos | source-checked-pending-expert-review |
| ideal | ιδεώδες | Greek academic usage; citation pending | source-checked-pending-expert-review |
| identity_matrix | μοναδιαίος πίνακας τάξης | Kallipos | source-checked-pending-expert-review |
| independence | ανεξάρτητο από | Kallipos | source-checked-pending-expert-review |
| initial_condition | αρχική συνθήκη | Kallipos | source-checked-pending-expert-review |
| interval | διάστημα | Greek school books | reviewed |
| jacobian | ιακωβιανός πίνακας του | Kallipos | source-checked-pending-expert-review |
| ket | κετ | Kallipos | source-checked-pending-expert-review |
| lagrangian | λαγκρανζιανή συνάρτηση | Kallipos | source-checked-pending-expert-review |
| laplace_transform | μετασχηματισμός Λαπλάς του | Kallipos | source-checked-pending-expert-review |
| laplacian | λαπλασιανή του | Kallipos | source-checked-pending-expert-review |
| line_integral | επικαμπύλιο ολοκλήρωμα του | Kallipos | source-checked-pending-expert-review |
| magnetic_field | μαγνητικό πεδίο | Greek school books / Kallipos | source-checked-pending-expert-review |
| manifold | πολλαπλότητα | Greek academic usage; citation pending | source-checked-pending-expert-review |
| material_derivative | υλική παράγωγος του | Kallipos | source-checked-pending-expert-review |
| matrix_element | στοιχείο πίνακα | Kallipos | source-checked-pending-expert-review |
| measure | μέτρο του | Greek academic usage; citation pending | source-checked-pending-expert-review |
| metric_tensor | μετρικός τανυστής | Kallipos | source-checked-pending-expert-review |
| modular_congruence | ισότιμο με | Maintainer wording approval 2026-09-08; expert review pending | source-checked-pending-expert-review |
| momentum | ορμή | Greek school books / Kallipos | source-checked-pending-expert-review |
| morphism | μορφισμός | Greek academic usage; citation pending | source-checked-pending-expert-review |
| multiple_integral | πολλαπλό ολοκλήρωμα του | Kallipos | source-checked-pending-expert-review |
| norm | νόρμα του | Kallipos | reviewed |
| normal_distribution | ακολουθεί κανονική κατανομή | Maintainer wording approval 2026-09-08; expert review pending | source-checked-pending-expert-review |
| open_closed_interval | διάστημα ανοιχτό αριστερά κλειστό δεξιά | Greek school books | reviewed |
| open_interval | ανοιχτό διάστημα | Greek school books | reviewed |
| open_set | ανοιχτό σύνολο | Greek academic usage; citation pending | source-checked-pending-expert-review |
| ordinary_derivative | παράγωγος του | Greek school books / Kallipos | source-checked-pending-expert-review |
| partial_derivative | μερική παράγωγος του | Kallipos | source-checked-pending-expert-review |
| partition_function | συνάρτηση επιμερισμού | Greek academic usage; citation pending | source-checked-pending-expert-review |
| point_definition | έχει συντεταγμένες | Maintainer wording approval 2026-09-08; expert review pending | source-checked-pending-expert-review |
| position_vector | διάνυσμα θέσης | Kallipos | source-checked-pending-expert-review |
| power | δύναμη | Greek school books | reviewed |
| probability | πιθανότητα του | Greek school books | reviewed |
| proper_time | ιδιοχρόνος | Kallipos | source-checked-pending-expert-review |
| quadratic_form | τετραγωνική μορφή του | Kallipos | source-checked-pending-expert-review |
| quantum_adjoint | ερμιτιανός συζυγής του | Kallipos | source-checked-pending-expert-review |
| quantum_operator | κβαντικός τελεστής | Kallipos | source-checked-pending-expert-review |
| quotient_group | ομάδα πηλίκο της | Maintainer wording approval; explicit quotient-group intent only | source-checked-pending-expert-review |
| ring | δακτύλιος | Greek academic usage; citation pending | source-checked-pending-expert-review |
| semantic_entailment | συνεπάγεται σημασιολογικά | Greek academic usage; citation pending | source-checked-pending-expert-review |
| standard_deviation | τυπική απόκλιση του | Greek school books / Kallipos | reviewed |
| stochastic_integral | στοχαστικό ολοκλήρωμα του | Greek academic usage; citation pending | source-checked-pending-expert-review |
| stochastic_process | στοχαστική διαδικασία | Kallipos | source-checked-pending-expert-review |
| surface_integral | επιφανειακό ολοκλήρωμα του | Kallipos | source-checked-pending-expert-review |
| tensor_product | τανυστικό γινόμενο | Kallipos | reviewed |
| topological_space | τοπολογικός χώρος | Greek academic usage; citation pending | source-checked-pending-expert-review |
| torque | ροπή | Greek school books / Kallipos | source-checked-pending-expert-review |
| transpose | ανάστροφος του | Greek school books / Kallipos | reviewed |
| variance | διακύμανση του | Greek school books / Kallipos | reviewed |
| velocity | ταχύτητα | Greek school books / Kallipos | source-checked-pending-expert-review |
| wavefunction | κυματοσυνάρτηση | Kallipos | source-checked-pending-expert-review |
