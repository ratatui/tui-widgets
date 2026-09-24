# Changelog

All notable changes to this project will be documented in this file.

## [0.8.10](https://crates.io/crates/tui-big-text/0.8.10) - 2026-09-24

Maintenance updates.

## [0.8.9](https://crates.io/crates/tui-big-text/0.8.9) - 2026-08-24

- *(big-text)* Add support for rendering all available fonts with `BigText` ([#355](https://github.com/ratatui/tui-widgets/pull/355))

## [0.8.8](https://crates.io/crates/tui-big-text/0.8.8) - 2026-06-18

Dependency updates.

## [0.8.7](https://crates.io/crates/tui-big-text/0.8.7) - 2026-06-14

- *(big-text)* Render by reference ([#288](https://github.com/ratatui/tui-widgets/pull/288))

## [0.8.6](https://crates.io/crates/tui-big-text/0.8.6) - 2026-06-14

- Ignore non-press key events in examples ([#284](https://github.com/ratatui/tui-widgets/pull/284))
- Modernize tui-big-text examples ([#276](https://github.com/ratatui/tui-widgets/pull/276))
- Install color-eyre in stopwatch example ([#280](https://github.com/ratatui/tui-widgets/pull/280))
- Document widget examples ([#286](https://github.com/ratatui/tui-widgets/pull/286))

## [0.8.5](https://crates.io/crates/tui-big-text/0.8.5) - 2026-06-11

Maintenance updates.

## [0.8.4](https://crates.io/crates/tui-big-text/0.8.4) - 2026-04-04

- \[codex\] Vendor CI workflows into this repository ([#212](https://github.com/ratatui/tui-widgets/issues/212))

## [0.8.3](https://crates.io/crates/tui-big-text/0.8.3) - 2026-03-29

- *(big-text)* Add optional Block wrapping for BigText ([#197](https://github.com/ratatui/tui-widgets/pull/197))

## [0.8.2](https://crates.io/crates/tui-big-text/0.8.2) - 2026-02-01

- *(bigtext)* Enable no_std as default ([#190](https://github.com/ratatui/tui-widgets/pull/190))

## [0.8.1](https://crates.io/crates/tui-big-text/0.8.1) - 2025-12-27

- Refresh widget docs ([#148](https://github.com/ratatui/tui-widgets/pull/148))

## [0.8.0](https://crates.io/crates/tui-big-text/0.8.0) - 2025-12-27

- *(big-text)* **Breaking:** Add block octant characters ([#117](https://github.com/ratatui/tui-widgets/pull/117))
  Update exhaustive matches on `PixelSize` to handle the new `Octant` variant.
- **Breaking:** Migrate to ratatui 0.30 ([#120](https://github.com/ratatui/tui-widgets/pull/120))
  See <https://github.com/joshka/tui-widgets/blob/main/BREAKING_CHANGES.md>
- *(big-text)* Fix enum name in field details ([#119](https://github.com/ratatui/tui-widgets/pull/119))

## [0.7.3](https://crates.io/crates/tui-big-text/0.7.3) - 2025-11-02

- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/pull/81))
- More clippy lints ([#84](https://github.com/ratatui/tui-widgets/pull/84))

## [0.7.2](https://crates.io/crates/tui-big-text/0.7.2) - 2025-11-02

- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/pull/81))
- More clippy lints ([#84](https://github.com/ratatui/tui-widgets/pull/84))

## [0.6.1](https://crates.io/crates/tui-big-text/0.6.1) - 2024-10-20

- Broken links from move to tui-widgets

## [0.6.0](https://crates.io/crates/tui-big-text/0.6.0) - 2024-08-11

Ratatui-0.28.0 compatible release

## [0.5.6](https://crates.io/crates/tui-big-text/0.5.6) - 2024-08-09

Dependency updates.

## [0.5.5](https://crates.io/crates/tui-big-text/0.5.5) - 2024-08-09

- Update to ratatui 0.28 ([#24](https://github.com/ratatui/tui-widgets/pull/24))

## [0.5.4](https://crates.io/crates/tui-big-text/0.5.4) - 2024-08-06

Dependency updates.

## [0.5.3](https://crates.io/crates/tui-big-text/0.5.3) - 2024-08-02

- Clean up changelogs ([#17](https://github.com/ratatui/tui-widgets/pull/17))

## [0.5.2](https://crates.io/crates/tui-big-text/0.5.2) - 2024-07-25

Maintenance updates.

## [0.5.1](https://crates.io/crates/tui-big-text/0.5.1) - 2024-07-25

Maintenance updates.

## [0.5.0](https://crates.io/crates/tui-big-text/0.5.0) - 2024-07-25

- *(big-text)* **Breaking:** Make `BigText` builder infallible ([#14](https://github.com/ratatui/tui-widgets/pull/14))
  `BigTextBuilder::build()` now returns `BigText` directly. Remove `?`, `.expect(...)`,
  or `.unwrap()` after `.build()`.
- *(big-text)* Add alignment helper methods
- Simplify tui-big-text examples

## [0.4.7](https://crates.io/crates/tui-big-text/0.4.7) - 2024-07-24

Maintenance updates.

## [0.4.5](https://crates.io/crates/tui-big-text/0.4.5) - 2024-06-25

### ⚙️ Miscellaneous Tasks

- *(deps)* Bump ratatui version ([#45](https://github.com/joshka/tui-big-text/pull/45))
- Use <https://github.com/joshka/github-workflows/>
- Set msrv to 1.74
- Update git cliff config

## [0.4.6](https://crates.io/crates/tui-big-text/0.4.6) - 2024-07-24

- Set msrv to 1.74

## [0.4.4](https://crates.io/crates/tui-big-text/0.4.4) - 2024-05-28

### ⚙️ Miscellaneous Tasks

- *(deps)* Update ratatui to 0.26.3 and itertools to 0.13.0
- Release ([#43](https://github.com/joshka/tui-big-text/issues/43))

## [0.4.3](https://crates.io/crates/tui-big-text/0.4.3) - 2024-04-12

### 🚀 Features

- Add alignment support for BigText ([#41](https://github.com/joshka/tui-big-text/issues/41))

### ⚙️ Miscellaneous Tasks

- Release v0.4.3 ([#42](https://github.com/joshka/tui-big-text/issues/42))

## [0.4.2](https://crates.io/crates/tui-big-text/0.4.2) - 2024-02-26

### 🚀 Features

- Add BigText::builder()

### 📚 Documentation

- Add link to docs.rs in cargo.toml
- Add pixel height example to main readme
- Tweak readme
- Update main demo example and README

### ⚙️ Miscellaneous Tasks

- Release ([#39](https://github.com/joshka/tui-big-text/issues/39))

### Build

- *(deps)* Update derive_builder requirement from 0.13.0 to 0.20.0 ([#38](https://github.com/joshka/tui-big-text/issues/38))

## [0.4.1](https://crates.io/crates/tui-big-text/0.4.1) - 2024-02-15

### ⚙️ Miscellaneous Tasks

- Release ([#37](https://github.com/joshka/tui-big-text/issues/37))

## [0.4.0](https://crates.io/crates/tui-big-text/0.4.0) - 2024-02-08

### 🚀 Features

- Add sextant-based fonts ([#26](https://github.com/joshka/tui-big-text/issues/26))

### 🐛 Bug Fixes

- Typos

### 🚜 Refactor

- Split big_text and pixel_size into modules for readability

### ⚙️ Miscellaneous Tasks

- *(deps)* Reorder cargo.toml and doc
- Release ([#35](https://github.com/joshka/tui-big-text/issues/35))

### Build

- *(deps)* Bump codecov/codecov-action from 3 to 4 ([#34](https://github.com/joshka/tui-big-text/issues/34))

## [0.3.6](https://crates.io/crates/tui-big-text/0.3.6) - 2024-02-05

### ⚙️ Miscellaneous Tasks

- Update ratatui to 0.26 ([#32](https://github.com/joshka/tui-big-text/issues/32))
- Release ([#33](https://github.com/joshka/tui-big-text/issues/33))

## [0.3.5](https://crates.io/crates/tui-big-text/0.3.5) - 2024-01-30

### ⚙️ Miscellaneous Tasks

- *(deps)* Update strum to 0.26.1
- Release ([#31](https://github.com/joshka/tui-big-text/issues/31))

## [0.3.4](https://crates.io/crates/tui-big-text/0.3.4) - 2024-01-29

### ⚙️ Miscellaneous Tasks

- Release ([#30](https://github.com/joshka/tui-big-text/issues/30))

### Build

- *(deps)* Update derive_builder requirement from 0.12.0 to 0.13.0 ([#29](https://github.com/joshka/tui-big-text/issues/29))

## [0.3.3](https://crates.io/crates/tui-big-text/0.3.3) - 2024-01-24

### 🐛 Bug Fixes

- *(doc)* Builder initialization of BigTextBuilder in docs ([#27](https://github.com/joshka/tui-big-text/issues/27))

### ⚙️ Miscellaneous Tasks

- Fix missing changelog entry for PixelSize change
- Create dependabot.yml
- Release ([#25](https://github.com/joshka/tui-big-text/issues/25))

## [0.3.2](https://crates.io/crates/tui-big-text/0.3.2) - 2024-01-12

### 📚 Documentation

- Improve examples

### ⚙️ Miscellaneous Tasks

- *(readme)* Clean up links
- *(readme)* More cleanup
- Release ([#23](https://github.com/joshka/tui-big-text/issues/23))

### Feat

- Add PixelSize option ([#22](https://github.com/joshka/tui-big-text/issues/22))

## [0.3.1](https://crates.io/crates/tui-big-text/0.3.1) - 2023-12-23

### 📚 Documentation

- Update example image ([#20](https://github.com/joshka/tui-big-text/issues/20))

### ⚙️ Miscellaneous Tasks

- Release ([#21](https://github.com/joshka/tui-big-text/issues/21))

## [0.3.0](https://crates.io/crates/tui-big-text/0.3.0) - 2023-12-23

### 📚 Documentation

- Hello world raw mode and screenshot ([#19](https://github.com/joshka/tui-big-text/issues/19))

### ⚙️ Miscellaneous Tasks

- Add check for cargo-rdme to ensure readme is updated when lib.rs docs are ([#16](https://github.com/joshka/tui-big-text/issues/16))
- Release v0.3.0 ([#17](https://github.com/joshka/tui-big-text/issues/17))

## [0.2.1](https://crates.io/crates/tui-big-text/0.2.1) - 2023-10-27

### ⚙️ Miscellaneous Tasks

- Bump release version to 0.2.1

## [0.2.0](https://crates.io/crates/tui-big-text/0.2.0) - 2023-10-27

### 🐛 Bug Fixes

- Update examples to build with ratatui 0.24.0

### ⚙️ Miscellaneous Tasks

- Release v0.1.5 ([#15](https://github.com/joshka/tui-big-text/issues/15))

### Build

- *(ratatui)* Update dependency to ratatui 0.24.0

## [0.1.4](https://crates.io/crates/tui-big-text/0.1.4) - 2023-09-05

### ⚙️ Miscellaneous Tasks

- Update changelog
- Undo release-plz fetch-depth change
- Release ([#13](https://github.com/joshka/tui-big-text/issues/13))

## [0.1.3](https://crates.io/crates/tui-big-text/0.1.3) - 2023-09-05

### 🐛 Bug Fixes

- Add doc test imports ([#8](https://github.com/joshka/tui-big-text/issues/8))

### 🚜 Refactor

- Render fn

### 📚 Documentation

- Tweak readme, licenses, contributing ([#10](https://github.com/joshka/tui-big-text/issues/10))

### 🧪 Testing

- Fix coverage for expected buffers in codecov

### ⚙️ Miscellaneous Tasks

- Add ci.yml ([#6](https://github.com/joshka/tui-big-text/issues/6))
- Add bacon config
- Configure git-cliff ([#11](https://github.com/joshka/tui-big-text/issues/11))
- Configure release-plz fetch depth
- Release ([#12](https://github.com/joshka/tui-big-text/issues/12))

## [0.1.2](https://crates.io/crates/tui-big-text/0.1.2) - 2023-09-05

### 📚 Documentation

- Use cargo-rdme to sync lib.rs to README.md ([#4](https://github.com/joshka/tui-big-text/issues/4))

### ⚙️ Miscellaneous Tasks

- Release ([#5](https://github.com/joshka/tui-big-text/issues/5))

## [0.1.1](https://crates.io/crates/tui-big-text/0.1.1) - 2023-09-05

### 🚀 Features

- Initial implementation

### 🐛 Bug Fixes

- Render correctly when not at the origin

### 📚 Documentation

- Add stopwatch example

### 🎨 Styling

- Readme wrapping

### ⚙️ Miscellaneous Tasks

- Fix repository link ([#1](https://github.com/joshka/tui-big-text/issues/1))
- Release
