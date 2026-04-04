# Changelog

All notable changes to this project will be documented in this file.

## [0.3.3] - 2026-04-04

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

- *(deps)* Support strum 0.28 ([#224](https://github.com/ratatui/tui-widgets/issues/224))
  > ## Summary
  >
  > This PR fixes the `strum 0.28` dependency bump proposed in
  > [#210](https://github.com/ratatui/tui-widgets/pull/210) and adds
  > explicit `cargo check --all-targets --all-features --workspace` coverage
  > so example-target compile failures are caught directly in CI instead of
  > being missed by the existing workspace check.
  >
  > ## Root Cause
  >
  > The failure came from the `tui-bar-graph` example, where clap derives a
  > parser for `BarStyle`.
  >
  > `BarStyle` still derives `FromStr`, and this was not caused by the
  > `#[strum(default)]` behavior change from [strum PR
  > #476](https://github.com/Peternator7/strum/pull/476). The actual issue
  > was dependency resolution plus feature unification.
  >
  > Before the bump, `tui-bar-graph` and `ratatui-core` both used `strum
  > 0.27`, so Cargo unified them and `ratatui-core`'s `strum/std` activation
  > effectively carried `std` into the `strum` instance used by
  > `tui-bar-graph`. After
  > [#210](https://github.com/ratatui/tui-widgets/pull/210), `tui-bar-graph`
  > used `strum 0.28` while `ratatui-core` still used `strum 0.27`, so the
  > graph split into separate `strum` packages.
  >
  > At that point, `tui-bar-graph`'s own `strum 0.28` was built with
  > `default-features = false`, which meant `strum::ParseError` no longer
  > implemented `std::error::Error` in that context. That was enough to
  > break clap's inferred parser for `BarStyle`.
  >
  > ## Code Changes
  >
  > `tui-bar-graph` now declares an explicit `std` feature and uses it to
  > enable `strum/std`. `std` remains enabled by default, so the crate's
  > normal ergonomics do not change.
  >
  > The interactive example is marked with `required-features = ["std"]` so
  > its compile requirements are explicit instead of depending on accidental
  > transitive feature behavior.
  >
  > `colorgrad` now uses `default-features = false`.
  >
  > ## Why Only `strum/std`
  >
  > `strum/std` matters here because it affects trait-based downstream
  > integration on a public enum type. `BarStyle` derives traits whose
  > interaction with clap depends on the trait set of `strum::ParseError`.
  >
  > `colorgrad/std` does not play the same role in this crate today, so the
  > `std` feature does not forward `colorgrad/std`.
  >
  > ## Docs
  >
  > The crate now uses `document-features` so the feature contract is
  > rendered in generated docs and on docs.rs.
  >
  > ## CI
  >
  > The issue exposed by
  > [#210](https://github.com/ratatui/tui-widgets/pull/210) was a normal
  > compile failure in a non-lib target.
  >
  > The workspace already ran `cargo check --all-features --workspace`, but
  > that does not compile examples and other non-lib targets. `cargo check
  > --all-targets --all-features --workspace` reproduces the failure
  > directly.
  >
  > This PR adds explicit all-targets compile coverage so example-target
  > breakage is caught as a normal build/check failure.
  >
  > ## Verification
  >
  > ```shell
  > cargo check --all-targets --all-features --workspace
  > cargo clippy --all-targets --all-features --workspace -- -D warnings
  > cargo check -p tui-bar-graph --lib --no-default-features
  > cargo doc -p tui-bar-graph --all-features
  > cargo rdme --check --manifest-path tui-bar-graph/Cargo.toml
  > ```
  >
  > ## Follow-up
  >
  > A broader `no_std` audit of the other widget crates was done during this
  > work and recorded separately on
  > [#102](https://github.com/ratatui/tui-widgets/issues/102).


## [0.3.2] - 2026-03-29

### ⚙️ Miscellaneous Tasks

- *(project)* Update the repository link


## [0.3.1] - 2025-12-27

### 📚 Documentation

- Refresh widget docs ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  > Standardize widget crate docs and README layouts.
  > Unify badges, links, and license references.
  > Add consistent usage sections and link style updates.


## [0.3.0] - 2025-12-27

### 🚀 Features

- *(tui-bar-graph)* [**breaking**] Add block octant characters ([#116](https://github.com/ratatui/tui-widgets/issues/116))
  > Since Unicode 16.0 was published on September 10, 2024, support for
  > block octant characters (U+1CD00 to U+1CDE5,
  > [PDF](https://www.unicode.org/charts/PDF/Unicode-16.0/U160-1CC00.pdf))
  > has been improving in fonts. We should enable users of `tui-widgets` to
  > use these characters in addition to existing options.

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


## [0.2.0] - 2025-11-02

### 🚀 Features

- *(tui-bar-graph)* [**breaking**] Support boxed gradients ([#66](https://github.com/ratatui/tui-widgets/issues/66))
  > This patch adds support for boxed gradients in the `BarGraph` widget.
  > This makes it possible to choose gradients of different types at runtime
  > without having to change the type of the `BarGraph` struct.

- *(tui-bar-graph)* Add Quadrant style ([#80](https://github.com/ratatui/tui-widgets/issues/80))
  > This style uses the block drawing 2x2 quadrant characters.
  > In contrast to the braille style, it renders solid rather than dots.
  > In contrast to the solid style, it renders two columns and rows per bar.
  >
  > ![Quadrant Magma](https://vhs.charm.sh/vhs-1rx6XQ9mLiO8qybSBXRGwn.gif)

### 🐛 Bug Fixes

- Broken bar graph test

- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/issues/81))
  > Fixes a bunch of lints that are in beta / nursery. A lot of these are
  > opinionated enough that they're not enabled by default, but I figure
  > they generally lead to nicer code, so are worth fixing.

- Use f64:midpoint ([#83](https://github.com/ratatui/tui-widgets/issues/83))
  > MSRV is now 1.87

### 🚜 Refactor

- Simplify color / gradient handling logic

### 🎨 Styling

- Add rustfmt and reformat code

### Other

- Bump msrv to 1.82.0 ([#74](https://github.com/ratatui/tui-widgets/issues/74))


## [tui-bar-graph-v0.1.1] - 2025-03-05

### 🚜 Refactor

- Simplify BarGraph rendering logic

## [tui-bar-graph-v0.1.0] - 2025-03-04

### 🚀 Features

- Add new tui-bar-graph crate ([#63](https://github.com/ratatui/tui-widgets/issues/63))
  > ![Braille demo](https://vhs.charm.sh/vhs-3H7bFj0M1kj0GoHcc4EIJ4.gif)
  >
  > ![Solid demo](https://vhs.charm.sh/vhs-5XMtSFgX3vqOhKcKl8fEQK.gif)
  >
  > ```rust
  > use tui_bar_graph::{BarGraph, BarStyle, ColorMode};
  >
  > let data = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
  > let bar_graph = BarGraph::new(data)
  >     .with_gradient(colorgrad::preset::turbo())
  >     .with_bar_style(BarStyle::Braille)
  >     .with_color_mode(ColorMode::VerticalGradient);
  > frame.render_widget(bar_graph, area);
  > ```
