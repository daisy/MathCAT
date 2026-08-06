# A translator's guide to MathCAT

This guide explains how to create or update a MathCAT translation, using Portuguese as example target language.

Most of this guide focuses on **speech (TTS) translation**. If you are working on Braille, you should still read the main workflow first, because much of the setup and testing process is similar. Then go to the section on [Braille translation](#braille-translation) to see what is different.

You do **not** need to be an experienced programmer to do this work well. Several successful translators started with only basic programming knowledge and no previous experience with YAML or Rust. What matters most is that you know your target language well, understand how mathematics is normally spoken or written in that language, and are willing to test carefully.

A background in mathematics is very helpful. You do not need to be a professional mathematician, but you do need to recognize when something sounds awkward, unclear, or mathematically misleading. If you are a native speaker and are comfortable with mathematics at about university level, that is a strong starting point.

## Time estimate

A full translation takes time, especially if you want it to be reliable and pleasant to use.

Based on real projects, a thorough TTS translation may take around **300–450 hours**. A Braille translation may take around **160–240 hours**. That can vary a lot depending on how good your starting point is, whether a similar language already exists, and how much previous programming experience you have.

## Need help getting started?

This guide was written by Marthe Gjelstad, Tim Arborealis Lötberg and Anders Eklund, who made the Norwegian and Swedish translations of MathCAT. If you need help getting started, we're happy to help. Get in touch with us at:

**[mathcat-wg@daisylists.org](mailto:mathcat-wg@daisylists.org)**

---

## Recommended workflow

A good workflow for translating MathCAT looks like this:

1. Set up your working copy
2. Update and audit the translation
3. Listen to the speech early
4. Add and run tests
5. Learn about speech modes and verbosity settings
6. Work on the rule files
7. Review with a mathematician
8. Do user testing
9. Submit the translation
10. Keep the translation up-to-date

---

## Step 1: Set up your working copy

### Get the repository

MathCAT is hosted on GitHub.

If you have never used GitHub before, do not let that put you off. For translation work, you only need a small part of it. In practice, you just need to get a copy of the project, edit files, and save your changes.

The easiest way to get started is often with **GitHub Desktop**:
[https://desktop.github.com/](https://desktop.github.com/)

Basic workflow:

1. Fork the MathCAT repository.
2. Clone it to your computer.
3. Work in your own branch, for example `pt`.

That is enough to get started.

### Install the tools you need

You will need a few tools. It sounds like a lot at first, but each one has a clear purpose.

* Git: [https://git-scm.com/](https://git-scm.com/)
* GitHub Desktop: [https://desktop.github.com/](https://desktop.github.com/)
* A code editor, for example VS Code: [https://code.visualstudio.com/](https://code.visualstudio.com/)
* Rust: [https://rust-lang.org/tools/install/](https://rust-lang.org/tools/install/)
* NVDA: [https://www.nvaccess.org/download/](https://www.nvaccess.org/download/)
* MathCAT (installed from the NVDA Add-on Store)

Rust is only needed to run the automated tests. You do not need to learn Rust as a programming language in order to translate MathCAT.

---

## Step 2: Update and audit the translation

Before you start translating, make sure your language files match the current English structure. If you skip this step, you may spend time fixing problems that are really caused by outdated files.

### Use the audit tool

Documentation:
[https://github.com/daisy/MathCAT/blob/main/PythonScripts/audit_translations/README.md](https://github.com/daisy/MathCAT/blob/main/PythonScripts/audit_translations/README.md)

Use the audit tool to find missing rules in the “pt” files. Add these rules to the “pt” files. The documentation contains detailed instructions on how to use the tool, so start there.

### Start from a similar language if possible

If a language that is grammatically very similar to yours already exists, it is often better to start from that instead of from English or from a machine-generated translation.

For example, the Norwegian translation started from Swedish. That can save a lot of time, especially in places where grammar and word order differ from English in systematic ways. But, remember to use the audit tool to look for missing rules before you start translating to your own language.

---

## Step 3: Listen to the speech early

Listening to the output as you go along will help you identify errors early. A translation can look perfectly fine in a YAML file and still sound awkward, repetitive, or confusing when spoken. A good procedure is to listen to some expressions through a screen reader after each file has been translated.

### Set up NVDA with your local rules

1. Download NVDA: [https://www.nvaccess.org/download/](https://www.nvaccess.org/download/)
2. In NVDA, open **Tools → Add-on Store** and install MathCAT.
3. Make a copy of your language folder, for example `pt`, and add it to:

```text
%AppData%\nvda\addons\MathCAT\globalPlugins\MathCAT\Rules\Languages
```

This makes it possible to listen to your local translation in NVDA.

### Try real expressions

A practical way to test speech is to take MathML expressions from the Rust test files and put them into a simple HTML document. Then open that file and listen with NVDA.

Make one change at a time, listen again, and keep notes. This is an efficient way to learn how the rules behave in practice.

---

## Step 4: Add and run tests

Automated tests are a core part of the workflow. They help you catch syntax errors, document the speech you expect, and make sure future MathCAT changes do not silently break your translation. It is helpful to translate the tests as you work on the translation and not save them for last.

In `MathCAT/tests`, open `languages.rs` and add (in the case of Portuguese):

```rust
mod pt;
```

Then in `MathCAT/tests/Languages`:

1. Copy `en.rs` to `pt.rs`.
2. Copy the `en` folder to `pt`.

If you only choose one speech style at first, you can remove or comment out the other one.

In each test, replace `"en"` with `"pt"`.

Once that is done, run:

```bash
cargo test Languages::pt
```

The first time, many tests will probably fail. That is normal.

### Understanding test failures

A typical failure report will show something like this:

```rust
left: "1 half"
right: "1 halv"
```

The important part is the comparison:

* `left` is the expected text written in the test
* `right` is the text MathCAT actually produced

If the generated output is correct, copy it into the test. If it is wrong, fix the rule instead.

It can actually be faster to run the tests before translating all the expected output. Then you can use the failure messages to see what MathCAT already produces in your language, and copy the correct ones into place.

### Example of a simple test

```rust
#[test]
fn common_fraction_half() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("pt", "ClearSpeak", expr, "um meio")?;
    return Ok(());
}
```

Here:

* `common_fraction_half` is the name of the test
* `expr` contains the MathML expression
* `"pt"` is the language code
* `"ClearSpeak"` is the speech mode
* `"um meio"` is the expected spoken output

### Example with preferences

Some tests use `test_prefs` so that you can check a rule under specific settings.

```rust
#[test]
fn common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "17 tenths")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "17 tenths")?;
    return Ok(());
}
```

In this kind of test:

* `test_prefs` runs the expression with specific preferences
* `vec![]` contains those preferences
* `("Verbosity", "Medium")` sets the verbosity level
* `("ClearSpeak_Fractions", "Auto")` sets a ClearSpeak preference

MathCAT also adds pauses in some places. In the test strings, these show up as punctuation such as commas and semicolons. So if a test almost matches, always check whether the real difference is pausing rather than wording.

---

## Step 5: Learn about speech modes and verbosity settings

There are two main speech modes: ClearSpeak and SimpleSpeak. The paper [A Comparison of Different Styles of Speech for Mathematics](https://scholarworks.calstate.edu/downloads/5t34sv64c), written by Neil, includes a good explanation of the difference between the two speech styles. In short:

* ClearSpeak is supposed to be similar to how a teacher would say a mathematical expression in a classroom.
* SimpleSpeak speaks simple expressions compactly. For example $\frac{x}{y} + 1$ is spoken as "x over y plus one". There are no bracketing words to indicate where the fraction begins and ends. This is because both the numerator and denominator are simple. The expression $\frac{x}{y+1}$ is spoken as "fraction, x over y plus one, end fraction". Here, bracketing words are used because the denominator is not simple.

It is up to you if you want to translate both speech styles or just focus on one. The translated speech styles may be more or less different from one another than the English ones depending on what variety of spoken math makes sense in the target language.

MathCAT also supports the verbosity levels **Terse**, **Medium**, and **Verbose**. It is up to you as a translator to what degree these levels should differ in your language. In some cases, the natural translation may collapse two English variants into one (such as when the word "the" does not exist in the target language). In others, there may be several ways of speaking an expression where there is only one option in English. That is all fine. Clarity matters more than forcing artificial differences.

For ClearSpeak there are many different preferences. These are used in the rules and in the tests. You can read about the preferences here: [ClearSpeak Preferences](https://github.com/daisy/MathCAT/blob/main/docs/ClearSpeakRulesAndPreferences.docx). At the moment, the preferences are not visible for users in the MathCAT settings in NVDA.

---

## Step 6: Work on the rule files

### Which files need translating?

Most of your work will be in YAML rule files. If you're translating to Portuguese, your language files will be inside the folder `MathCAT\Rules\Languages\pt`. It contains the following files to translate:

* `ClearSpeak_Rules.yaml`
* `definitions.yaml`
* `navigate.yaml`
* `overview.yaml` (optional)
* `SimpleSpeak_Rules.yaml`
* `unicode.yaml`
* `unicode-full.yaml` (semi-optional)

The subfolder `MathCAT\Rules\Languages\pt\SharedRules` contains shorter rules that are the building blocks that the rules in both `ClearSpeak_Rules.yaml` and `SimpleSpeak_Rules.yaml` are made from. All of these need translating as well.

### Where to start

It can be helpful to start out with for example `ClearSpeak_Rules.yaml` and a few of the files in the `SharedRules`, to get a feel of how the rules work.

Many of the changes which need to be made will be discovered through the tests, as stated above. However, the files `unicode.yaml`, `unicode-full.yaml`, `navigate.yaml` and `definitions.yaml` are not sufficiently covered by the tests. These especially need a lot of manual attention.

Do not spend a lot of time going through `unicode-full.yaml` at first. It is several thousand lines long, and most of the symbols are very rarely used in real life (some won't even render in VS Code). This file can be saved for last, and it is enough to look through it and check the most recognisable symbols.

Likewise, the file `overview.yaml` is less critical than the others, because it does not affect the reading of mathematical expressions directly. However, it provides screen readers with access to short structural summaries of expressions, and so it is worth going over eventually.

### What to do

Your main task will be to locate all yaml key names `t:`, which correspond to text strings to be spoken. Lowercase `t:` means the text has not yet been translated. Verify (if the auto translation happens to be correct) or change the translation until you are happy with it. Once this is done, change it into a capital `T:` to indicate you have gone over it.

The same procedure of capitalization applies to all instances of `ct:`, `ot:`, `spell:`, `pronounce:` and `IfThenElse:`. However, these are far less frequent than `t:`.

### Adapting or writing custom rules

The tests cover most of the rules in the rule files. However, there can be grammatical things in Portuguese that do not exist in English, and are not covered by the tests. It is therefore a good idea to go through all the rules to check that everything looks correct.

When there are grammatical differences, it will not be enough to simply change the text strings. Then, the rules need to be adapted, or new rules added. It is a good idea to write new tests to cover entirely new rules, to make sure your output is correct. Don't be afraid of trial-and-error until you get it right.

Here follows a crash course in the anatomy of a MathCAT rule. Each rule describes when it applies and what should be spoken. The main parts are:

* `name` — the name of the rule
* `tag` — the MathML element the rule applies to
* `match` — the condition that must be true for the rule to apply
* `replace` — the output and actions MathCAT should use

Inside `replace`, some common commands are:

* `t:` — text to speak
* `ct:` — concatenate text without a space in front
* `ot:` — optional text, used to avoid repeated words
* `x:` — an XPath expression
* `test:` — conditional logic
* `pause:` — a pause in the speech
* `bookmark:` — used for synchronized highlighting

#### Example 1

```yaml
- name: squared
  tag: power
  match: "*[2][self::m:mn][.='2'] and $ClearSpeak_Exponents = 'Auto'"
  replace:
  - x: "*[1]"
  - bookmark: "*[2]/@id"
  - t: "squared"      # phrase(7 'squared' equals 49)
```

This rule states that if a power has exponent 2 and the preference for `ClearSpeak_Exponents` is set to `Auto`, then the output should be “base squared”. For example, $x^2$ would be spoken as “x squared”.

A few things are worth noticing here:

* The `match` says what expressions the rule applies to.
* The expression after `x` is an XPath expression. It retrieves the first child of a power element, which would be the base.
* The expression after `t` is text that must be translated. When the string is translated and verified, capitalize the `t`.

#### Example 2

```yaml
- name: fraction-over-text
  tag: fraction
  match:
  - "not($ClearSpeak_Fractions='General' or $ClearSpeak_Fractions='GeneralEndFrac') and"
  - "( "
  - "  ((*[1][self::m:mi or self::m:mtext][string-length(.)>1]) or "
  - "   (*[1][self::m:mrow][count(*)=3][ "
  - "        *[1][self::m:mn] and "
  - "        *[2][self::m:mo][.='⁢'] and "
  - "        *[3][self::m:mi or self::m:mtext][string-length(.)>1] ]) ) and"
  - "  ((*[2][self::m:mi or self::m:mtext][string-length(.)>1]) or "
  - "   (*[2][self::m:mrow][count(*)=3][ "
  - "        *[1][self::m:mn] and "
  - "        *[2][self::m:mo][.='⁢'] and "
  - "        *[3][self::m:mi or self::m:mtext][string-length(.)>1] ]) )"
  - ")"
  replace:
  - x: "*[1]"
  - t: "over"      # phrase(the fraction 3 'over' 4)
  - x: "*[2]"
  - test:
      if: "$ClearSpeak_Fractions='EndFrac' or $ClearSpeak_Fractions='OverEndFrac'"
      then:
      - pause: short
      - t: "end fraction"      # phrase(7 over 8 'end fraction')
      - pause: short
```

For this rule to apply, several constraints must be satisfied. They are all listed in `match`.

In plain language, the rule says:

* The preference `ClearSpeak_Fractions` should not be `General` or `GeneralEndFrac`, **and**
* the numerator is a text string with length more than 1, or a number invisibly multiplied with a text string with length more than 1, **and**
* the denominator satisfies the same constraints as the numerator.

If all of that is satisfied, the fraction will be spoken as “numerator over denominator”. If `ClearSpeak_Fractions` is set to `EndFrac` or `OverEndFrac`, there will also be a short pause before “end fraction” is spoken.

For example, the output for the expression $\frac{\text{meter}}{\text{second}}$ would be “meter over second”.

---

## Step 7: Review with a mathematician

Once you have a fairly complete translation, it is a very good idea to ask a mathematician, mathematics teacher, or another strong subject expert who is a native speaker of the language to review a representative set of outputs.

A useful way to do this is to prepare a set of example expressions together with transcripts of what MathCAT says. Then ask the reviewer to go through them and flag anything that sounds mathematically odd, ambiguous, or unusual.

This kind of review is important because a translation can be grammatically correct and still not sound like real mathematics. A subject expert can often catch problems in terminology, symbol naming, and the way larger structures are spoken.

That said, you may not always be able to satisfy every request perfectly. Sometimes the way screen readers work, or the way MathCAT is structured, means that the most mathematically elegant wording is not the most usable one.

---

## Step 8: Do user testing

If you are not yourself a proficient screen reader user, user testing is essential.

The best testers are blind users who are comfortable with mathematics, especially STEM users if you can find them. Give them a local version of MathCAT with your translation, together with an HTML file containing representative MathML expressions.

It helps to let users try it on their own first. After that, meet with them and go through more examples together. Ask them not only whether the wording is correct, but whether the structure is clear, whether the pauses help, and whether the output matches what they are used to hearing.

This matters because spoken math is not judged only by correctness. It also has to be usable in real screen reader workflows. In practice, user feedback should often weigh more heavily than theoretical preferences about grammar, especially when the disagreement is really about clarity or navigation rather than mathematical meaning.

---

## Step 9: Submit the translation

Before you open a pull request or share your translation, it’s worth doing one final pass to make sure everything is in good shape. This helps avoid unnecessary back-and-forth and makes the review process smoother.

### Final checklist

Go through the following:

#### Files and structure

* Your language files are up to date with the current English structure
* No rules are missing (audit tool has been run)
* No obvious leftover English text remains

#### Translation status

* All reviewed text has been changed from `t:` to `T:` (and `ct:` → `CT:`, `ot:` → `OT:` etc.)
* You have consciously reviewed wording, not just auto-translated it

#### Speech output

* You have listened to a representative set of expressions in NVDA
* Output sounds natural and unambiguous in your language
* Pauses and structure make sense when listening

#### Tests

* Tests have been added for your language
* All tests pass:

  ```bash
  cargo test Languages::pt
  ```

* Expected outputs in tests match your intended speech (including pauses)

#### Expert review

* A mathematician or subject expert has reviewed representative output
* Major terminology or structure issues have been addressed

#### User testing

* At least some testing has been done with screen reader users
* Feedback on clarity and usability has been considered

### Submitting your translation

Once you are satisfied with the checklist:

1. **Commit your changes** in your branch (for example `pt`)
2. **Push your branch** to GitHub
3. **Open a pull request** against the main MathCAT repository

In your pull request, it helps to include:

* A short description of the translation
* What has been completed (e.g. ClearSpeak, SimpleSpeak, tests)
* Any known limitations or areas that still need work

After that, maintainers will review your work and may suggest changes before merging.

### When will the translation become available for users?

Once your translation is merged into MathCAT:

* It will become part of future MathCAT releases
* Users will be able to select your language in supported screen readers (such as NVDA)

If you want to use it immediately before an official release, you can:

* keep using your local rules folder in NVDA, or
* install a development version of MathCAT that includes your changes

---

## Step 10: Keep the translation up-to-date

A translation is never “finished” in a strict sense. Even after merging, you will likely find small improvements over time—especially as users start working with real content.

Furthermore, the English version is still under active development, and you will want to include new features in your translation. To keep your translation in sync, you may use the [Translation audit tool](https://github.com/daisy/MathCAT/blob/main/PythonScripts/audit_translations/README.md).

<!-- Please add details about notification and translation update procedure here when it has been decided upon. -->

---

## Braille translation

Braille translation follows a similar overall workflow, but there are some important differences.

Before starting work on the Braille rules, it is important to establish what standard is used in your country for writing mathematics. Some countries have established Braille codes, while others use some other kind of notation such as LaTeX or ASCIIMath. You need to decide early which standard MathCAT should follow.

If a Braille standard similar to yours already exists, it may be useful as a starting point. Otherwise, you will probably need to start more or less from scratch.

### Braille files

For a new Braille language, you will need to create three `.yaml` files in `Rules\Braille\your-braille-language`:

1. `xxx_Rules.yaml` — where `xxx` is the name of your Braille code. This contains the rules that translate MathML into Braille.
2. `unicode.yaml` — this contains translations of the more common characters.
3. `unicode-full.yaml` — this contains the rest of the character translations.

The reason for having two Unicode files is practical: keeping the common characters in a shorter file helps startup time.

### An example from Swedish

The Swedish Braille translation followed the official Swedish Braille code for mathematics, which was a 6-dot standard intended for printed Braille. There was no established 8-dot standard for mathematics, so the translation had to be built around the only recognized standard that existed.

The Swedish work also showed that source standards are not always strict enough for software. Some printed standards allow spacing to follow the printed source. In MathCAT, that is not enough. The rules need to be explicit, so you may need to resolve inconsistencies and make firm decisions about spacing.

### Testing Braille

The basic idea is similar to TTS testing, but the tests check Braille Unicode instead of speech strings.

A practical way to work is to write Rust tests with expected Braille output taken from the national standard or from agreed examples. Then run the tests, inspect failures, and refine the rules until the output matches.

The Swedish translation used examples taken straight from the Swedish Braille code, especially for arithmetic, fractions, subscript and superscript, and basic functions. That turned out to be a good way to ground the rules in a real standard.

### Braille-specific challenges

Braille work often raises issues that do not come up in speech translation. For example:

* the standard may be incomplete or inconsistent
* the standard may assume printed context instead of machine-generated output
* some symbols may not have clear established representations
* spacing may need to be made stricter than the source standard suggests

### Braille user testing

Braille user testing is just as important as speech user testing.

In the Swedish work, testers used NVDA together with the Braille viewer so that the output could be monitored during remote sessions. That made it possible to observe the output even when the reviewers themselves were not reading the physical Braille display.

As with speech, the goal is not only technical correctness. The output should also be readable, consistent, and useful in real workflows.

---

## Appendix

### Practical advice for working efficiently

AI tools can be useful when you are drafting translations, exploring alternative phrasings, or trying to understand a difficult rule. They can also help when you need to write or edit rules.

However, they should always be treated as assistants, not authorities. Automatic translations always need checking by a human, and rule changes always need testing with real output.

More generally, it helps to make one change at a time, test often, and discuss difficult cases with other translators or subject experts when you can.

### Troubleshooting

If something does not work as expected, check indentation first. YAML errors are common and can be surprisingly hard to spot. If you use VS Code, it can be useful to download a YAML extension to help you find YAML errors.

Also check the test output carefully and listen again. Many problems are easier to hear than to see.

### XPath

Many parts of a speech rule make use of XPath. This is a popular and well-documented method for selecting parts of an XML document. A web search will turn up many tutorials. Those not familiar with XPath are encouraged to read some. The implementation of XPath used by MathCAT is a slightly extended version of XPath 1.0.

MathCAT usage tends to use only a few features of XPath. It also makes use of some custom functions. Here is a short explanation of common XPath usage:

| usage                           | meaning                                                                                                                                                                                                                                                                   |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `*`                             | matches all children                                                                                                                                                                                                                                                      |
| `[...]`                         | selects nodes from the current match                                                                                                                                                                                                                                      |
| `*[1]`                          | selects first child                                                                                                                                                                                                                                                       |
| `*[self::m:mn]`                 | selects all children that are `mn` elements. Note that `m` is used to indicate that the element is in the MathML namespace.                                                                                                                                               |
| `*[1][self::m:mn]`              | select the first child as long as it is an `mn` element                                                                                                                                                                                                                   |
| `*[1][self::m:mo][text()='-']`  | select the first child as long as it is an `mo` element whose content is ‘-‘. This could also be written as `*[1][text()='-']` because other nodes probably won’t have the content `-`, but an `mtext` element could have that, so specifying the element name is safest. |
| `count(*[2]/*)`                 | the number of children of the second child                                                                                                                                                                                                                                |
| `count(preceding-sibling::*)+1` | add 1 to the number of siblings before the current element                                                                                                                                                                                                                |

MathCAT adds some custom functions to make writing rules easier:

| function                                                         | meaning                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `IsNode(nodes, type)`                                            | Returns true if all of the nodes are of the same type. Type can be one of:<br/> "simple" -- a defined set of elements in ClearSpeak <br/> "leaf" -- one of the MathML leaf elements <br/> "2D" -- a 2D node such as `mfrac` or `mroot`<br/>"modified" – the node has a script or something over/under it<br/>"scripts" – the node has a subscript and/or superscript<br/>"common_fraction" – integer numerator and denominator |
| `ToOrdinal`                                                      |                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `ToCommonFraction`                                               |                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `IsBracketed(openChar, closeChar, requiresComma)`                |                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `BaseNode(node)`                                                 | Returns the base (recursively) of a scripted node                                                                                                                                                                                                                                                                                                                                                                              |
| `IsInDefinition(node, name)`                                     | Returns true if node is a member of the list ‘name’ (defined in definitions.yaml)                                                                                                                                                                                                                                                                                                                                              |
| `IfThenElse(test, then-part, else-part)`                         | Returns `then-part` if the test is true, otherwise `else-part`. All arguments are XPath expressions                                                                                                                                                                                                                                                                                                                                        |
| `DistanceFromLeaf(node, left_side, treat_2d_elements_as_tokens)` | Returns the distance from the current node to the leftmost or rightmost leaf (0 for a character, 1 for a token). If `left_side` is `true`, traversal follows the leftmost child to the leaf; otherwise it follows the right side. If `treat_2d_elements_as_tokens` is `true`, two-dimensional notations such as fractions are treated as single tokens (like leaves).                                                                              |
| `EdgeNode(node, "left"/"right", stopNodeName)`                   | Returns the stopNode if at left/right edge of named ancestor node. `stopNodeName` can also be `"2D"`. The original node is returned if no match is found. Note: if `stopNodeName` is `"math"`, then punctuation is taken into account since it is not really part of the math                                                                                                                                                           |
| `DEBUG(xpath)`                                                   | Really helpful for debugging – it will be added to debug output                                                                                                                                                                                                                                                                                                                                                                |

These are used by Nemeth Rules:

| function       | meaning                                                                                                                             |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `NestingChars` | Used by mfrac, msqrt, and mroot rules to repeat the chars the appropriate number of times                                           |
| `BrailleChars` | Used by token elements to deal with the complicated rearrangement of various Nemeth indicators such as capitalization and font face |
