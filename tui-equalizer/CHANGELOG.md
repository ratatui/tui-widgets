# Changelog

All notable changes to this project will be documented in this file.

## [0.2.3] - 2026-06-18

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


## [0.2.2] - 2026-06-14

### 🚀 Features

- *(equalizer)* Render by reference ([#290](https://github.com/ratatui/tui-widgets/issues/290))
  > ## Summary
  >
  > - add `Widget for &Equalizer`
  > - make band rendering borrow each band instead of consuming it
  > - cover borrowed rendering against owned rendering
  >
  > ## Validation
  >
  > - `cargo test -p tui-big-text -p tui-bar-graph -p tui-equalizer -p
  > tui-prompts -p tui-scrollview --all-features`
  > - `cargo clippy -p tui-big-text -p tui-bar-graph -p tui-equalizer -p
  > tui-prompts -p tui-scrollview --all-targets --all-features -- -D
  > warnings`


## [0.2.1] - 2026-06-14

### 📚 Documentation

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

