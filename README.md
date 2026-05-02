# MathCAT: Math Capable Assistive Technology

<img src="logo.png" style="position: relative; top: 16px; z-index: -1;" alt="The MathCAT logo with a cat sitting on the text MathCAT">
is a library that supports conversion of MathML to:


* Speech strings (in several languages) with embedded speech engine commands
* Braille (Nemeth, UEB Technical, CMU, and many others)
* Navigation of math (in multiple ways including overviews)


There are four related projects that make use of MathCAT:
- [MathCATDemo](https://nsoiffer.github.io/MathCATDemo/) -- an online demonstration of some of what can be done with MathCAT
- [A python interface for MathCAT](https://github.com/NSoiffer/MathCATForPython) -- used by a [MathCAT NVDA add-on](https://addons.nvda-project.org/addons/MathCAT.en.html).
- [A C/C++ interface for MathCAT](https://github.com/NSoiffer/MathCATForC)
- [A Java interface for MathCAT](https://github.com/mwhapples/MathCAT4J) (thanks to Michael Whapples for working on that)

MathCAT is used in many assistive technologies including NVDA and JAWS.

For more information, see the [full documentation](https://daisy.github.io/MathCAT/).

### Test coverage

Line coverage for the Rust test suite is generated weekly with [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov).

- **Run manually on GitHub:** open [Actions → Test coverage](https://github.com/daisy/MathCAT/actions/workflows/coverage.yml), press **Run workflow**, run on **`main`**. This uploads the **llvm-cov-html** artifact and commits the same HTML into **`llvm-cov/`** on **`main`** (for GitHub Pages; same as the weekly schedule). If the push step fails, check branch protection: allow **GitHub Actions** to push to **`main`**, or use a PAT secret the workflow is allowed to use.
- **Run locally:** install the tool (`cargo install cargo-llvm-cov` and add `llvm-tools-preview` via `rustup component add llvm-tools-preview`), then from the repo root run `cargo llvm-cov --workspace --html --output-dir llvm-cov-html` and open `llvm-cov-html/index.html`.
- **Scheduled runs:** open the same workflow link, pick the latest successful run, and download the **llvm-cov-html** artifact; open `index.html` inside the archive.
- **Browsable URL:** in **Settings → Pages**, use branch **`main`** and **`/(root)`** (same as the docs site). The workflow then keeps coverage at [**https://daisy.github.io/MathCAT/llvm-cov/**](https://daisy.github.io/MathCAT/llvm-cov/) (only files under **`llvm-cov/`** on **`main`** are added or replaced).
