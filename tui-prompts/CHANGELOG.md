# Changelog

All notable changes to this project will be documented in this file.

## [0.6.7] - 2026-06-18

### 🚀 Features

- *(prompts)* Add select prompt ([#177](https://github.com/ratatui/tui-widgets/issues/177))
  > feat(prompts): add select prompt
  >
  > Add a select prompt for single-choice terminal flows. The new prompt owns an
  > ordered option list while SelectState tracks focus, completion, and abort
  > status. It supports Up/Down navigation, Enter completion, and Escape/Ctrl+C
  > abort.
  >
  > Keep navigation bounded to the rendered options and render a focused window
  > when the list is taller than the available area, so the highlighted option
  > remains visible. Finished prompts ignore further routed key events, preventing
  > selection or status changes after completion or abort.
  >
  > Add SelectOption and SelectOptionList for option construction, including
  > conversions from arrays and vectors of string-like values. Document the select
  > prompt lifecycle in Rustdoc and the README, add regression coverage for key
  > handling and clipping behavior, and update the example plus VHS recording to
  > show the selected value before exit.

### 🐛 Bug Fixes

- *(prompts)* Repair select example CI ([#306](https://github.com/ratatui/tui-widgets/issues/306))
  > ## Summary
  >
  > - repair the `tui-prompts` select example after the local `tui` helper
  > module was removed
  > - align the example with the other `tui-prompts` examples by using
  > `ratatui::run` / `DefaultTerminal`
  > - regenerate the `tui-prompts` README after rustfmt import ordering
  > changed crate-level Rustdoc
  > - clarify AGENTS guidance so formatter-generated crate Rustdoc changes
  > still trigger `just rdme-check`
  >
  > ## Context
  >
  > The open Dependabot PRs are inheriting a `main` CI failure from commit
  > `2d37086b`: `tui-prompts/examples/select.rs` still declares `mod tui;`,
  > but there is no `tui-prompts/examples/tui.rs`. That breaks Check Targets
  > and Clippy, and rustfmt also reports stale import ordering. The README
  > check also failed because `cargo rdme` mirrors the crate-level Rustdoc
  > import ordering into `tui-prompts/README.md`.
  >
  > ## Verification
  >
  > - `just fmt-check`
  > - `cargo check --all-targets --all-features --workspace`
  > - `just clippy-stable`
  > - `just clippy-beta`
  > - `cargo test --all-features --workspace`
  > - `just rdme-check`
  > - `markdownlint-cli2 AGENTS.md tui-prompts/README.md`

### Other

- *(deps)* Bump itertools from 0.14.0 to 0.15.0 ([#305](https://github.com/ratatui/tui-widgets/issues/305))
  > Bumps [itertools](https://github.com/rust-itertools/itertools) from
  > 0.14.0 to 0.15.0.
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/rust-itertools/itertools/blob/master/CHANGELOG.md">itertools's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>0.15.0</h2>
  > <h3>Breaking</h3>
  > <ul>
  > <li>Restructure <code>Position</code> as struct instead of enum (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1042">#1042</a>,
  > <a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1043">#1043</a>)</li>
  > <li>Canonicalize <code>all_equal_value</code>'s error type (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1032">#1032</a>)</li>
  > </ul>
  > <h3>Added</h3>
  > <ul>
  > <li>Add <code>*_with_hasher</code> adaptors (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1007">#1007</a>)</li>
  > <li>Add strip_prefix and strip_prefix_by methods (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1104">#1104</a>)</li>
  > </ul>
  > <h3>Changed</h3>
  > <ul>
  > <li>Remove <code>Clone</code> bounds from
  > <code>tuple_combinations</code> and <code>array_combinations</code>(<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1011">#1011</a>)</li>
  > <li><code>must_use</code> for <code>collect_vec</code> (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1009">#1009</a>)</li>
  > <li>Make <code>izip!</code> temporary friendly (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1021">#1021</a>)</li>
  > <li>Add <code>array_combinations_with_replacement</code> (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1033">#1033</a>)</li>
  > <li>Implement <code>Debug</code> for remaining public types (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1038">#1038</a>)</li>
  > <li>Specialize <code>ExactlyOneError::count</code> (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1046">#1046</a>)</li>
  > <li>Implement <code>PeekingNext</code> for more types, in particular
  > <code>vec::IntoIter</code> (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1059">#1059</a>,
  > <a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1073">#1073</a>)</li>
  > <li>Fix <code>PadUsing::next_back</code> (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1082">#1082</a>)</li>
  > <li>Introduce <code>[circular_]array_windows</code>, deprecate
  > <code>tuple_windows</code> (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1086">#1086</a>)</li>
  > <li>Deprecate <code>tuple_combinations</code> (replaced by
  > <code>array_combinations</code>) (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1085">#1085</a>)</li>
  > </ul>
  > <h3>Notable Internal Changes</h3>
  > <ul>
  > <li>Make <code>into_group_map</code> code more idiomatic (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1027">#1027</a>)</li>
  > <li>Fix clippy lints (<a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1017">#1017</a>,
  > <a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1029">#1029</a>,
  > <a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1076">#1076</a>,
  > <a
  > href="https://redirect.github.com/rust-itertools/itertools/issues/1099">#1099</a>)</li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/rust-itertools/itertools/commit/37bd72aa6d58e594711d127b52418ca5e58b6091"><code>37bd72a</code></a>
  > Update CHANGELOG.md: strip_prefix[_by]</li>
  > <li><a
  > href="https://github.com/rust-itertools/itertools/commit/86ec6359258e57a8af391fafc645d6ffab363bff"><code>86ec635</code></a>
  > Use <code>ControlFlow</code> in <code>fold_while</code>
  > implementation</li>
  > <li><a
  > href="https://github.com/rust-itertools/itertools/commit/d5897f7bb2283a34b736f539a369fe16dc9f6bb4"><code>d5897f7</code></a>
  > refactor(strip_prefix): use try_for_each and drop PartialEq, Eq on
  > StripPrefi...</li>
  > <li><a
  > href="https://github.com/rust-itertools/itertools/commit/b2a978abca1dc57f929f60e0be16d83e58f9558f"><code>b2a978a</code></a>
  > feat(Itertools): add strip_prefix and strip_prefix_by methods</li>
  > <li><a
  > href="https://github.com/rust-itertools/itertools/commit/12b6ec6f91eb83eebe4ac150e53aca14365cd894"><code>12b6ec6</code></a>
  > Update CHANGELOG.md for all_equal_value_error's error type</li>
  > <li><a
  > href="https://github.com/rust-itertools/itertools/commit/121821ed16365a39c015449a3872708fd5dcdfdd"><code>121821e</code></a>
  > AllEqualValueError implements std::error::Error</li>
  > <li><a
  > href="https://github.com/rust-itertools/itertools/commit/adac44e4278b087033d23408652e540aef5b22f0"><code>adac44e</code></a>
  > Introduce AllEqualValueError</li>
  > <li><a
  > href="https://github.com/rust-itertools/itertools/commit/5707384b6a3a675c65e12b96e71335e9ce857b16"><code>5707384</code></a>
  > Update CHANGELOG.md</li>
  > <li><a
  > href="https://github.com/rust-itertools/itertools/commit/df60ff016bf31f5752827808b375d36b0a101e33"><code>df60ff0</code></a>
  > Update CHANGELOG.md</li>
  > <li><a
  > href="https://github.com/rust-itertools/itertools/commit/113b850b9d68406dda7c428d31992002c77eebad"><code>113b850</code></a>
  > Update CHANGELOG.md to include with_hasher</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/rust-itertools/itertools/compare/v0.14.0...v0.15.0">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  > ---------


## [0.6.6] - 2026-06-14

### 🚀 Features

- *(prompts)* Render text prompts by reference ([#291](https://github.com/ratatui/tui-widgets/issues/291))
  > ## Summary
  >
  > - add `StatefulWidget for &TextPrompt`
  > - add `Prompt for &TextPrompt` so stored prompt configuration can be
  > drawn by reference
  > - document the stored prompt pattern and regenerate the README
  >
  > ## Validation
  >
  > - `cargo test -p tui-prompts --all-features`
  > - `cargo clippy -p tui-prompts --all-targets --all-features -- -D
  > warnings`
  > - `cargo rdme --check --manifest-path tui-prompts/Cargo.toml`
  > - `markdownlint-cli2 tui-prompts/README.md`


## [0.6.5] - 2026-06-14

### 📚 Documentation

- Modernize tui-prompts examples ([#281](https://github.com/ratatui/tui-widgets/issues/281))
  > ## Summary
  > - use ratatui::run in the tui-prompts examples
  > - remove the custom example Tui wrapper
  > - install color-eyre before running the prompt examples
  > - handle input with as_key_press_event while preserving prompt focus and
  > submission behavior
  >
  > ## Validation
  > - cargo +nightly fmt --all
  > - cargo check -p tui-prompts --examples --all-features
  > - cargo clippy -p tui-prompts --examples --all-features -- -D warnings
  > - cargo test -p tui-prompts --all-features

- Document widget examples ([#286](https://github.com/ratatui/tui-widgets/issues/286))
  > ## Summary
  >
  > - add run commands and widget-specific context to the example module
  > docs
  > - add targeted inline comments for non-obvious example sizing, state,
  > and rendering choices
  > - fix broken crate-root `Ratatui` reference links that surfaced during
  > docs validation
  >
  > ## Validation
  >
  > - `cargo +nightly fmt --all --check`
  > - `cargo clippy --workspace --examples --all-features -- -D warnings`
  > - `cargo doc --workspace --examples --all-features --no-deps`
  >
  > `cargo doc` still reports the existing `tui-bar-graph` example/lib
  > output filename collision.

### ⚙️ Miscellaneous Tasks

- Migrate workspace to Rust 2024 ([#263](https://github.com/ratatui/tui-widgets/issues/263))
  > ## Summary
  > - migrate the workspace package edition from Rust 2021 to Rust 2024
  > - apply Cargo edition fixes for lifetime capture and macro fragment
  > specifiers
  > - apply mechanical Clippy let-chain fixes needed for the stable -D
  > warnings gate
  > - refresh generated README snippets and document the pre-push README
  > check in AGENTS.md
  >
  > ## Validation
  > - cargo fix --edition --all-features --workspace --allow-dirty
  > --allow-staged
  > - just fmt
  > - just clippy-stable
  > - cargo test --all-features --workspace
  > - just rdme-check
  > - markdownlint-cli2 AGENTS.md README.md tui-*/README.md


## [0.6.4] - 2026-06-11

### 🚀 Features

- *(tui-prompts)* Allow hiding status symbol ([#191](https://github.com/ratatui/tui-widgets/issues/191))
  > Add an opt-in TextPrompt builder for rendering prompts without the
  > status symbol prefix. This supports prompt rows where completion or
  > cancellation state is communicated by surrounding UI instead of the
  > prompt itself.
  >
  > Preserve the existing default rendering behavior by keeping the status
  > symbol visible for new and default prompts. Document that hiding the
  > symbol only changes the rendered prefix: TextState still tracks Status,
  > and cursor placement follows the shortened prompt width.
  >
  > Cover both default rendering and hidden-symbol rendering so the status
  > symbol spacing and cursor behavior stay explicit.

### ⚙️ Miscellaneous Tasks

- Check remaining crate READMEs ([#241](https://github.com/ratatui/tui-widgets/issues/241))
  > ## Summary
  >
  > - add the omitted tui-box-text and tui-prompts manifests to the
  > cargo-rdme CI check
  > - refresh the generated tui-prompts README block so the new check passes
  >
  > ## Validation
  >
  > - cargo rdme --check --manifest-path Cargo.toml
  > - cargo rdme --check --manifest-path tui-bar-graph/Cargo.toml
  > - cargo rdme --check --manifest-path tui-big-text/Cargo.toml
  > - cargo rdme --check --manifest-path tui-box-text/Cargo.toml
  > - cargo rdme --check --manifest-path tui-cards/Cargo.toml
  > - cargo rdme --check --manifest-path tui-popup/Cargo.toml
  > - cargo rdme --check --manifest-path tui-prompts/Cargo.toml
  > - cargo rdme --check --manifest-path tui-qrcode/Cargo.toml
  > - cargo rdme --check --manifest-path tui-scrollbar/Cargo.toml
  > - cargo rdme --check --manifest-path tui-scrollview/Cargo.toml

- Add documentation hygiene checks ([#243](https://github.com/ratatui/tui-widgets/issues/243))
  > ## Summary
  >
  > - add required CI jobs for typos and markdownlint-cli2
  > - exclude generated changelogs from spelling checks
  > - fix small spelling and Markdown hygiene issues caught by the new
  > checks
  >
  > ## Validation
  >
  > - typos
  > - markdownlint-cli2 "**/*.md"
  > - cargo rdme --check --manifest-path tui-big-text/Cargo.toml
  > - cargo rdme --check --manifest-path tui-popup/Cargo.toml
  > - cargo fmt --all -- --check
  > - actionlint -color=false .github/workflows/check.yml


## [0.6.3] - 2026-04-04

### ⚙️ Miscellaneous Tasks

- *(deps)* Lower dependency floors and reduce dependabot noise ([#211](https://github.com/ratatui/tui-widgets/issues/211))
  > ## Summary
  >
  > - lower direct dependency requirements to the broadest semver ranges the
  > workspace actually supports
  > - keep `Cargo.lock` on current compatible releases, including the direct
  > `clap`, `tokio`, `futures`, and `rand` updates that fit this PR's scope
  > - configure Dependabot to group Cargo and GitHub Actions updates and use
  > `increase-if-necessary` to reduce manifest churn
  >
  > ## Details
  >
  > This change validates dependency floors with `cargo minimal-versions` in
  > `--direct` mode so the library manifests reflect honest direct
  > requirements instead of transitive minimum noise.
  >
  > Notable outcomes:
  >
  > - broadened requirements such as `clap = "4"` and `tokio = "1"` after
  > verifying the workspace still compiles and tests against their earliest
  > compatible direct versions
  > - kept real floors where required, such as `crossterm = "0.29"`,
  > `document-features = "0.2.11"`, and `derive_setters = "0.1.9"`
  > - kept the direct `rand` update to `0.10` and adjusted the
  > `tui-bar-graph` examples to generate random `Vec<f64>` values in a `rand
  > 0.10`-compatible way
  > - kept transitive duplicate major versions where they are still required
  > by downstream crates like the Ratatui stack or `lipsum`
  >
  > Dependabot should now produce less noise because grouped update PRs can
  > primarily refresh `Cargo.lock` while leaving `Cargo.toml` alone unless a
  > broader range is truly needed.
  >
  > ## Validation
  >
  > - `cargo minimal-versions check --workspace --direct`
  > - `cargo check --all-features --workspace`
  > - `cargo test --all-features --workspace`
  > - `cargo minimal-versions test --workspace --all-features --direct`
  > - `cargo outdated --workspace --root-deps-only`
  > - `cargo test -p tui-bar-graph --all-features --examples`
  >
  > ## Supersedes
  >
  > This PR should supersede and allow closing the related Dependabot PRs:

### Other

- [codex] Vendor CI workflows into this repository ([#212](https://github.com/ratatui/tui-widgets/issues/212))


## [0.6.2] - 2026-03-29

### ⚙️ Miscellaneous Tasks

- *(project)* Update the repository link


## [0.6.1] - 2025-12-27

### 📚 Documentation

- Refresh widget docs ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  > Standardize widget crate docs and README layouts.
  > Unify badges, links, and license references.
  > Add consistent usage sections and link style updates.


## [0.6.0] - 2025-12-27

### 🚀 Features

- [**breaking**] Migrate to ratatui 0.30 ([#120](https://github.com/ratatui/tui-widgets/issues/120))
  > feat!: migrate to ratatui 0.30
  >
  > - Update workspace deps to ratatui 0.30, ratatui-core, ratatui-widgets,
  > crossterm 0.29
  > - Shift widget crates to ratatui-core/ratatui-widgets imports where
  > needed
  > - Update tui-popup/tui-prompts event handling to use crossterm types
  > - Revise tui-popup rendering/ref semantics and docs to match reference
  > rendering rules
  > - Add rolling breaking changes doc and markdownlint config
  > - Bump direct deps needed for minimal-versions and examples
  > (document-features, colorgrad, unicode-width)

### 🎨 Styling

- *(tui-prompts)* Apply changes from cargo fmt ([#118](https://github.com/ratatui/tui-widgets/issues/118))


## [0.5.2] - 2025-11-02

### 🐛 Bug Fixes

- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/issues/81))
  > Fixes a bunch of lints that are in beta / nursery. A lot of these are
  > opinionated enough that they're not enabled by default, but I figure
  > they generally lead to nicer code, so are worth fixing.

- More clippy lints ([#84](https://github.com/ratatui/tui-widgets/issues/84))

- *(tui-prompts)* Full-width character input in non-multiline prompt ([#93](https://github.com/ratatui/tui-widgets/issues/93)) ([#94](https://github.com/ratatui/tui-widgets/issues/94))

### 🎨 Styling

- Add rustfmt and reformat code

### ⚙️ Miscellaneous Tasks

- Remove needless lifetimes ([#60](https://github.com/ratatui/tui-widgets/issues/60))

- Remove leftover github workflow files ([#73](https://github.com/ratatui/tui-widgets/issues/73))


## [0.5.1] - 2025-11-02

### 🐛 Bug Fixes

- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/issues/81))
  > Fixes a bunch of lints that are in beta / nursery. A lot of these are
  > opinionated enough that they're not enabled by default, but I figure
  > they generally lead to nicer code, so are worth fixing.

- More clippy lints ([#84](https://github.com/ratatui/tui-widgets/issues/84))

- *(tui-prompts)* Full-width character input in non-multiline prompt ([#93](https://github.com/ratatui/tui-widgets/issues/93)) ([#94](https://github.com/ratatui/tui-widgets/issues/94))

### 🎨 Styling

- Add rustfmt and reformat code

### ⚙️ Miscellaneous Tasks

- Remove needless lifetimes ([#60](https://github.com/ratatui/tui-widgets/issues/60))

- Remove leftover github workflow files ([#73](https://github.com/ratatui/tui-widgets/issues/73))


## [0.4.1] - 2024-10-20

### ⚙️ Miscellaneous Tasks

- Update Cargo.toml dependencies

## [0.4.0] - 2024-08-11

Ratatui-0.28.0 compatible release

## [0.3.23] - 2024-08-12

### ⚙️ Miscellaneous Tasks

- Update Cargo.toml dependencies

## [0.3.22] - 2024-08-09

### ⚙️ Miscellaneous Tasks

- Update Cargo.toml dependencies

## [0.3.21] - 2024-08-06

### ⚙️ Miscellaneous Tasks

- Update Cargo.toml dependencies

## [0.3.20] - 2024-08-02

### 📚 Documentation

- Clean up changelogs ([#17](https://github.com/ratatui/tui-widgets/pull/17))
  > - removed unnecessary footer comments
  > - removed [unreleased] sections
  > - removed duplicate release notes

## [0.3.19] - 2024-07-25

### ⚙️ Miscellaneous Tasks

- Update READMEs and licensing info

## [0.3.18] - 2024-07-24

### 🐛 Bug Fixes

- Delete and backspace behavior for multi-byte characters ([#57](https://github.com/ratatui/tui-widgets/pull/57))
- Fixup tui-prompts version to match crates.io

### ⚙️ Miscellaneous Tasks

- Move tui-prompts to its own directory
- Move to tui-widgets repository
- Various fixes / clippy lints ([#6](https://github.com/ratatui/tui-widgets/pull/6))

## [0.3.14](https://github.com/joshka/tui-prompts/compare/v0.3.13...v0.3.14) - 2024-07-02

### Other

- *(deps)* bump clap from 4.5.7 to 4.5.8 ([#53](https://github.com/joshka/tui-prompts/pull/53))
- *(deps)* bump ratatui-macros from 0.4.1 to 0.4.2 ([#54](https://github.com/joshka/tui-prompts/pull/54))

## [0.3.13](https://github.com/joshka/tui-prompts/compare/v0.3.12...v0.3.13) - 2024-06-25

### Other

- *(deps)* update ratatui, itertools, rstest, clap ([#51](https://github.com/joshka/tui-prompts/pull/51))
- simplify text_prompt tests ([#49](https://github.com/joshka/tui-prompts/pull/49))

## [0.3.12](https://github.com/joshka/tui-prompts/compare/v0.3.11...v0.3.12) - 2024-05-11

### Fixed

- *(text_prompt)* use unicode-width instead of char cnt ([#46](https://github.com/joshka/tui-prompts/pull/46))

### Other

- *(text_prompt)* avoid unnecessary clones ([#48](https://github.com/joshka/tui-prompts/pull/48))

## [0.3.11](https://github.com/joshka/tui-prompts/compare/v0.3.10...v0.3.11) - 2024-04-25

### Other

- *(deps)* bump ratatui from 0.26.1 to 0.26.2 ([#44](https://github.com/joshka/tui-prompts/pull/44))

## [0.3.10](https://github.com/joshka/tui-prompts/compare/v0.3.9...v0.3.10) - 2024-04-04

### Fixed

- Fix prompt for multi-byte characters. ([#41](https://github.com/joshka/tui-prompts/pull/41))
- mark broken tests with FIXME ([#42](https://github.com/joshka/tui-prompts/pull/42))

## [0.3.9](https://github.com/joshka/tui-prompts/compare/v0.3.8...v0.3.9) - 2024-04-01

### Other

- *(deps)* bump color-eyre from 0.6.2 to 0.6.3 ([#34](https://github.com/joshka/tui-prompts/pull/34))
- *(deps)* bump indoc from 2.0.4 to 2.0.5 ([#36](https://github.com/joshka/tui-prompts/pull/36))
- *(deps)* bump clap from 4.5.2 to 4.5.4 ([#37](https://github.com/joshka/tui-prompts/pull/37))

## [0.3.8](https://github.com/joshka/tui-prompts/compare/v0.3.7...v0.3.8) - 2024-03-12

### Other

- add workflow names ([#32](https://github.com/joshka/tui-prompts/pull/32))

## [0.3.7](https://github.com/joshka/tui-prompts/compare/v0.3.6...v0.3.7) - 2024-03-12

### Other

- allow release-plz workflow to be manually run ([#30](https://github.com/joshka/tui-prompts/pull/30))

## [0.3.6](https://github.com/joshka/tui-prompts/compare/v0.3.5...v0.3.6) - 2024-03-12

### Other

- *(deps)* bump clap from 4.5.0 to 4.5.2 ([#23](https://github.com/joshka/tui-prompts/pull/23))
- use release-plz from reusable workflows ([#27](https://github.com/joshka/tui-prompts/pull/27))
- add test.yml workflow ([#26](https://github.com/joshka/tui-prompts/pull/26))
- add check.yml workflow ([#24](https://github.com/joshka/tui-prompts/pull/24))

## [0.3.5](https://github.com/joshka/tui-prompts/compare/v0.3.4...v0.3.5) - 2024-02-13

### Other

- *(deps)* bump ratatui from 0.25.0 to 0.26.1 ([#18](https://github.com/joshka/tui-prompts/pull/18))
- *(deps)* bump clap from 4.4.18 to 4.5.0 ([#19](https://github.com/joshka/tui-prompts/pull/19))

## [0.3.4](https://github.com/joshka/tui-prompts/compare/v0.3.3...v0.3.4) - 2024-01-30

### Other

- *(deps)* bump itertools from 0.12.0 to 0.12.1 ([#15](https://github.com/joshka/tui-prompts/pull/15))

## [0.3.3](https://github.com/joshka/tui-prompts/compare/v0.3.2...v0.3.3) - 2024-01-25

### Other

- *(deps)* bump clap from 4.4.8 to 4.4.18 ([#14](https://github.com/joshka/tui-prompts/pull/14))
- *(deps)* bump ratatui from 0.24.0 to 0.25.0 ([#12](https://github.com/joshka/tui-prompts/pull/12))
- *(deps)* bump actions/checkout from 3 to 4 ([#11](https://github.com/joshka/tui-prompts/pull/11))
- Create dependabot.yml

## [0.3.2](https://github.com/joshka/tui-prompts/compare/v0.3.1...v0.3.2) - 2023-12-15

### Other

- *(deps)* bump zerocopy from 0.7.26 to 0.7.31

## [0.3.1](https://github.com/joshka/tui-prompts/compare/v0.3.0...v0.3.1) - 2023-11-17

### Other

- add debug info to examples and update readme

## [0.3.0](https://github.com/joshka/tui-prompts/compare/v0.2.3...v0.3.0) - 2023-11-17

### Other

- deps update and clippy lint fixes
- make TextState fields private

## [0.2.3](https://github.com/joshka/tui-prompts/compare/v0.2.2...v0.2.3) - 2023-07-25

### Other

- simplifiy wrapping

## [0.2.2](https://github.com/joshka/tui-prompts/compare/v0.2.1...v0.2.2) - 2023-07-18

### Other

- add unit tests for soft wrapping single lines
- add release-plz github action

## [0.2.1](https://github.com/joshka/tui-prompts/compare/v0.2.0...v0.2.1) - 2023-07-18

### Other

- add TODO list and key bindings to readme
- add invisible text prompt to readme
- add coverage job to bacon config

## [0.2.0](https://github.com/joshka/tui-prompts/compare/v0.1.1...v0.2.0) - 2023-07-17

### Added

- handle focus and invisible text prompt

### Other

- tweak text example
- Readme badges / license single file
- Update README.md

## [0.1.1](https://github.com/joshka/tui-prompts/compare/v0.1.0...v0.1.1) - 2023-07-11

### Other

- fix cargo.toml categories and keywords
- release

## [0.1.0](https://github.com/joshka/tui-prompts/releases/tag/v0.1.0) - 2023-07-11

### Added

- add text prompt, shared impl
- *(password)* add password prompt

### Fixed

- correct cursor position

### Other

- Revert "chore: add changelog"
- add changelog
- fix readme for release
- remove unrelased ratatui code
- Update readme and add licenses
- improve text example
- replace PasswordPrompt with render_style
- make TextPrompt::new() const
- tidy up text prompt rendering
- extract Status and Symbols to module
- use feat-stylize-all-the-things branch
- add password example to readme
