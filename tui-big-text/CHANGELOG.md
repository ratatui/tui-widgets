# Changelog

All notable changes to this project will be documented in this file.

## [0.8.2] - 2026-02-01

### 🚀 Features

- *(tui-bigtext)* Enable no_std as default ([#190](https://github.com/ratatui/tui-widgets/issues/190))
  > This enables using `tui-bigtext` in a no_std environment
  >
  > e.g. https://github.com/ratatui/mousefood/pull/159

### ⚙️ Miscellaneous Tasks

- *(project)* Update the repository link


## [0.8.1] - 2025-12-27

### 📚 Documentation

- Refresh widget docs ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  > Standardize widget crate docs and README layouts.
  > Unify badges, links, and license references.
  > Add consistent usage sections and link style updates.


## [0.8.0] - 2025-12-27

### 🚀 Features

- *(tui-big-text)* [**breaking**] Add block octant characters ([#117](https://github.com/ratatui/tui-widgets/issues/117))
  > The changes in this PR add the following enum variants:
  >
  > * `PixelSize::QuarterHeight` and `PixelSize::Octant` to `PixelSize`
  > found in the `tui-big-text` crate.
  >
  > Documentation and tests are included, and the examples (along with the
  > VHS tapes and screenshots) have been updated to show the new
  > **two-row-tall text** styles in action and how they compare to text
  > rendered with other `PixelSize` settings.

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

### 📚 Documentation

- *(tui-big-text)* Fix enum name in field details ([#119](https://github.com/ratatui/tui-widgets/issues/119))


## [0.7.3] - 2025-11-02

### 🐛 Bug Fixes

- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/issues/81))
  > Fixes a bunch of lints that are in beta / nursery. A lot of these are
  > opinionated enough that they're not enabled by default, but I figure
  > they generally lead to nicer code, so are worth fixing.

- More clippy lints ([#84](https://github.com/ratatui/tui-widgets/issues/84))

### 🎨 Styling

- Format doc comments

- Add rustfmt and reformat code

### ⚙️ Miscellaneous Tasks

- *(tui-big-text)* Support disabling crossterm ([#70](https://github.com/ratatui/tui-widgets/issues/70))


## [0.7.2] - 2025-11-02

### 🐛 Bug Fixes

- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/issues/81))
  > Fixes a bunch of lints that are in beta / nursery. A lot of these are
  > opinionated enough that they're not enabled by default, but I figure
  > they generally lead to nicer code, so are worth fixing.

- More clippy lints ([#84](https://github.com/ratatui/tui-widgets/issues/84))

### 🎨 Styling

- Format doc comments

- Add rustfmt and reformat code

### ⚙️ Miscellaneous Tasks

- *(tui-big-text)* Support disabling crossterm ([#70](https://github.com/ratatui/tui-widgets/issues/70))


## [0.6.1] - 2024-10-20

### 🐛 Bug Fixes

- Broken links from move to tui-widgets

## [0.6.0] - 2024-08-11

Ratatui-0.28.0 compatible release

## [0.5.6] - 2024-08-09

### ⚙️ Miscellaneous Tasks

- Update Cargo.toml dependencies

## [0.5.5] - 2024-08-09

### 🐛 Bug Fixes

- Update to ratatui 0.28 ([#24](https://github.com/ratatui/tui-widgets/pull/24))
  > Note that for projects that rely on crossterm, Ratatui 0.28.0 now relies internally on Crossterm 0.28.0.
  > Ratatui release notes highlights: <https://ratatui.rs/highlights/v028/>
  > See <https://github.com/ratatui-org/ratatui/issues/1298> for notes about crossterm compatibility

## [0.5.4] - 2024-08-06

### ⚙️ Miscellaneous Tasks

- Update Cargo.toml dependencies

## [0.5.3] - 2024-08-02

### 📚 Documentation

- Clean up changelogs ([#17](https://github.com/ratatui/tui-widgets/pull/17))
  > - removed unnecessary footer comments
  > - removed [unreleased] sections
  > - removed duplicate release notes

## [0.5.2] - 2024-07-25

### ⚙️ Miscellaneous Tasks

- Update tui-big-text images

## [0.5.1] - 2024-07-25

### ⚙️ Miscellaneous Tasks

- Remove anyhow dependency
  > Replaced with color_eyre generally

- Update READMEs and licensing info

## [0.5.0] - 2024-07-25

### 🚀 Features

- *(tui-big-text)* Add alignment helper methods
  > Adds helper methods to the `BigTextBuilder` struct to set the alignment
  > of the text. This makes it simpler to set the alignment of the text.
  >
  > ```rust
  > let left = BigText::builder()
  >     .left_aligned()
  >     .lines(vec!["Left".white().into()])
  >     .build()?;
  >
  > let right = BigText::builder()
  >     .right_aligned()
  >     .lines(vec!["Right".green().into()])
  >     .build()?;
  >
  > let centered = BigText::builder()
  >     .centered()
  >     .lines(vec!["Centered".red().into()])
  >     .build()?;
  > ```

- *(tui-big-text)* [**breaking**] Make `BigText` builder infallible ([#14](https://github.com/ratatui/tui-widgets/pull/14))
  > BigTextBuilder.build() no longer returns a Result. Instead it returns
  > the BigText widget directly. This change is made to simplify rendering
  > code which often otherwise doesn't have any error conditions.
  >
  > This also makes the fields on BigText public (marked as non-exhaustive)
  >
  > BREAKING CHANGE:BigTextBuilder.build() no longer returns a Result.
  >
  > Remove the `?` / `expect` / `unwrap` calls code which calls the build
  > method.
  >
  > ```diff
  >  let big_text = BigText::builder()
  >      .lines(vec![Line::from("SingleLine")])
  > -    .build()?;
  > +    .build();
  > ```

### 📚 Documentation

- Simplify tui-big-text examples

## [0.4.7] - 2024-07-24

### ⚙️ Miscellaneous Tasks

- Various fixes / clippy lints ([#6](https://github.com/ratatui/tui-widgets/pull/6))

## [0.4.5] - 2024-06-25

### ⚙️ Miscellaneous Tasks

- *(deps)* Bump ratatui version ([#45](https://github.com/joshka/tui-big-text/pull/45))
- Use <https://github.com/joshka/github-workflows/>
- Set msrv to 1.74
- Update git cliff config

## [0.4.6](https://github.com/ratatui/tui-widgets/compare/tui-big-text-v0.4.5...tui-big-text-v0.4.6) - 2024-07-24

### Other

- Move to tui-widgets repository

### ⚙️ Miscellaneous Tasks

- Use faster release-plz
- *(deps)* Bump ratatui version ([#45](https://github.com/joshka/tui-big-text/issues/45))
- Use <https://github.com/joshka/github-workflows/>
- Set msrv to 1.74

## [0.4.4] - 2024-05-28

### ⚙️ Miscellaneous Tasks

- *(deps)* Update ratatui to 0.26.3 and itertools to 0.13.0
- Release ([#43](https://github.com/joshka/tui-big-text/issues/43))

## [0.4.3] - 2024-04-12

### 🚀 Features

- Add alignment support for BigText ([#41](https://github.com/joshka/tui-big-text/issues/41))

### ⚙️ Miscellaneous Tasks

- Release v0.4.3 ([#42](https://github.com/joshka/tui-big-text/issues/42))

## [0.4.2] - 2024-02-26

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

## [0.4.1] - 2024-02-15

### ⚙️ Miscellaneous Tasks

- Release ([#37](https://github.com/joshka/tui-big-text/issues/37))

## [0.4.0] - 2024-02-08

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

## [0.3.6] - 2024-02-05

### ⚙️ Miscellaneous Tasks

- Update ratatui to 0.26 ([#32](https://github.com/joshka/tui-big-text/issues/32))
- Release ([#33](https://github.com/joshka/tui-big-text/issues/33))

## [0.3.5] - 2024-01-30

### ⚙️ Miscellaneous Tasks

- *(deps)* Update strum to 0.26.1
- Release ([#31](https://github.com/joshka/tui-big-text/issues/31))

## [0.3.4] - 2024-01-29

### ⚙️ Miscellaneous Tasks

- Release ([#30](https://github.com/joshka/tui-big-text/issues/30))

### Build

- *(deps)* Update derive_builder requirement from 0.12.0 to 0.13.0 ([#29](https://github.com/joshka/tui-big-text/issues/29))

## [0.3.3] - 2024-01-24

### 🐛 Bug Fixes

- *(doc)* Builder initialization of BigTextBuilder in docs ([#27](https://github.com/joshka/tui-big-text/issues/27))

### ⚙️ Miscellaneous Tasks

- Fix missing changelog entry for PixelSize change
- Create dependabot.yml
- Release ([#25](https://github.com/joshka/tui-big-text/issues/25))

## [0.3.2] - 2024-01-12

### 📚 Documentation

- Improve examples

### ⚙️ Miscellaneous Tasks

- *(readme)* Clean up links
- *(readme)* More cleanup
- Release ([#23](https://github.com/joshka/tui-big-text/issues/23))

### Feat

- Add PixelSize option ([#22](https://github.com/joshka/tui-big-text/issues/22))

## [0.3.1] - 2023-12-23

### 📚 Documentation

- Update example image ([#20](https://github.com/joshka/tui-big-text/issues/20))

### ⚙️ Miscellaneous Tasks

- Release ([#21](https://github.com/joshka/tui-big-text/issues/21))

## [0.3.0] - 2023-12-23

### 📚 Documentation

- Hello world raw mode and screenshot ([#19](https://github.com/joshka/tui-big-text/issues/19))

### ⚙️ Miscellaneous Tasks

- Add check for cargo-rdme to ensure readme is updated when lib.rs docs are ([#16](https://github.com/joshka/tui-big-text/issues/16))
- Release v0.3.0 ([#17](https://github.com/joshka/tui-big-text/issues/17))

## [0.2.1] - 2023-10-27

### ⚙️ Miscellaneous Tasks

- Bump release version to 0.2.1

## [0.2.0] - 2023-10-27

### 🐛 Bug Fixes

- Update examples to build with ratatui 0.24.0

### ⚙️ Miscellaneous Tasks

- Release v0.1.5 ([#15](https://github.com/joshka/tui-big-text/issues/15))

### Build

- *(ratatui)* Update dependency to ratatui 0.24.0

## [0.1.4] - 2023-09-05

### ⚙️ Miscellaneous Tasks

- Update changelog
- Undo release-plz fetch-depth change
- Release ([#13](https://github.com/joshka/tui-big-text/issues/13))

## [0.1.3] - 2023-09-05

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

## [0.1.2] - 2023-09-05

### 📚 Documentation

- Use cargo-rdme to sync lib.rs to README.md ([#4](https://github.com/joshka/tui-big-text/issues/4))

### ⚙️ Miscellaneous Tasks

- Release ([#5](https://github.com/joshka/tui-big-text/issues/5))

## [0.1.1] - 2023-09-05

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
