# Changelog

All notable changes to this project will be documented in this file.

## [0.6.2] - 2025-12-27

### 📚 Documentation

- Refresh widget docs ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  > Standardize widget crate docs and README layouts.
  > Unify badges, links, and license references.
  > Add consistent usage sections and link style updates.


## [0.6.1] - 2025-12-27

### ⚙️ Miscellaneous Tasks

- Refresh readmes and rdme check ([#140](https://github.com/ratatui/tui-widgets/issues/140))
  > Regenerate crate READMEs via cargo-rdme and add a CI check to keep
  > workspace readmes in sync.


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


## [0.5.3] - 2025-11-02

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

- Remove leftover github workflow files ([#73](https://github.com/ratatui/tui-widgets/issues/73))

- Use semver compatible dependency versions ([#77](https://github.com/ratatui/tui-widgets/issues/77))
  > Use 0.x and x.y instead of 0.x.y and x.y.z for deps to reduce
  > incompatibilities

### Other

- Added render_stateful_widget method to ScrollView ([#65](https://github.com/ratatui/tui-widgets/issues/65))


## [0.5.2] - 2025-11-02

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

- Remove leftover github workflow files ([#73](https://github.com/ratatui/tui-widgets/issues/73))

- Use semver compatible dependency versions ([#77](https://github.com/ratatui/tui-widgets/issues/77))
  > Use 0.x and x.y instead of 0.x.y and x.y.z for deps to reduce
  > incompatibilities

### Other

- Added render_stateful_widget method to ScrollView ([#65](https://github.com/ratatui/tui-widgets/issues/65))


## [0.5.1] - 2024-11-23

### 🚀 Features

- *(tui-scrollview)* Add scrollbars visibility handling ([#45](https://github.com/ratatui/tui-widgets/pull/45))
  > Scrollbars can now be set to never display, always display or automatically display (default).

## [0.4.1] - 2024-10-20

### 🐛 Bug Fixes

- Broken links from move to tui-widgets

- *(tui-scrollview)* Make scroll_view buffer area the same as its content ([#37](https://github.com/ratatui/tui-widgets/pull/37))
  > This makes the scroll_view buffer area the same as its content by
  > default.
  > The previous behavior is easily reproduced by adding empty space to the
  > buffer. =)
  > Most tests remain the same with the scrollbar updated to match the new
  > size.
  >
  > Fixes:[#35](https://github.com/ratatui/tui-widgets/pull/35)

### 📚 Documentation

- Use ratatui 0.28.1 methods for examples

- Demo horizontal scrolling and mark TODO as done

## [0.4.0] - 2024-08-11

Ratatui-0.28.0 compatible release

## [0.3.13] - 2024-08-06

### ⚙️ Miscellaneous Tasks

- Update Cargo.toml dependencies

## [0.3.12] - 2024-08-02

### 📚 Documentation

- Clean up changelogs ([#17](https://github.com/ratatui/tui-widgets/pull/17))
  > - removed unnecessary footer comments
  > - removed [unreleased] sections
  > - removed duplicate release notes

## [0.3.11] - 2024-07-25

### ⚙️ Miscellaneous Tasks

- Update READMEs and licensing info

## [0.3.10] - 2024-07-25

### 🐛 Bug Fixes

- Use ratatui::crossterm instead of crossterm::

## [0.3.9] - 2024-07-24

### ⚙️ Miscellaneous Tasks

- Various fixes / clippy lints ([#6](https://github.com/ratatui/tui-widgets/pull/6))

## [0.3.8](https://github.com/ratatui/tui-widgets/compare/tui-scrollview-v0.3.7...tui-scrollview-v0.3.8) - 2024-07-24

### Other

- Move to tui-widgets repository
- move tui-scrollview to its own directory

## [0.3.7] - 2024-06-25

### ⚙️ Miscellaneous Tasks

- *(deps)* Bump rstest in the all-dependencies group ([#34](https://github.com/joshka/tui-scrollview/pull/34))
- *(deps)* Bump ratatui in the all-dependencies group ([#36](https://github.com/joshka/tui-scrollview/pull/36))

## [0.3.6] - 2024-05-21

### 📚 Documentation

- *(examples)* Add tabs / state example

### ⚙️ Miscellaneous Tasks

- Update git cliff to include deps, unconventional commits, and link PRs
- Cargo update

### Other

- --- ([#33](https://github.com/joshka/tui-scrollview/pull/33))

## [0.3.5] - 2024-04-26

### 🚀 Features

- Add horizontal scrollbars and hide scrollbars if there is enough space ([#30](https://github.com/joshka/tui-scollview/issues/30))

### 📚 Documentation

- Ensure that horizontal scrollbar is not shown for example
- Update git cliff config

### ⚙️ Miscellaneous Tasks

- *(deps)* Bump eyre from 0.6.11 to 0.6.12 ([#26](https://github.com/joshka/tui-scollview/issues/26))
- Group dependabot updates
- *(deps)* Bump ratatui in the all-dependencies group ([#28](https://github.com/joshka/tui-scollview/issues/28))

## [0.3.4] - 2024-04-01

### 🐛 Bug Fixes

- Scroll_to_bottom scrolls the y offset not the x offset ([#24](https://github.com/joshka/tui-scollview/issues/24))

## [0.3.3] - 2024-03-28

### 📚 Documentation

- Improve docs and example code ([#22](https://github.com/joshka/tui-scollview/issues/22))

### ⚙️ Miscellaneous Tasks

- *(deps)* Bump lipsum from 0.9.0 to 0.9.1 ([#17](https://github.com/joshka/tui-scollview/issues/17))
- *(deps)* Bump color-eyre from 0.6.2 to 0.6.3 ([#18](https://github.com/joshka/tui-scollview/issues/18))
- *(deps)* Bump indoc from 2.0.4 to 2.0.5 ([#21](https://github.com/joshka/tui-scollview/issues/21))

## [0.3.2] - 2024-03-12

### ⚙️ Miscellaneous Tasks

- Use joshka/github-workflows ([#15](https://github.com/joshka/tui-scollview/issues/15))
- *(deps)* Bump mio from 0.8.10 to 0.8.11 ([#14](https://github.com/joshka/tui-scollview/issues/14))

## [0.3.1] - 2024-02-13

### ⚙️ Miscellaneous Tasks

- *(deps)* Bump ratatui from 0.26.0 to 0.26.1 ([#12](https://github.com/joshka/tui-scollview/issues/12))

## [0.3.0] - 2024-02-11

### 🚀 Features

- Add PageUp/PageDown and rework Demo

### ⚙️ Miscellaneous Tasks

- *(deps)* Bump ratatui to 0.26.0
- *(deps)* Bump codecov/codecov-action from 3 to 4 ([#9](https://github.com/joshka/tui-scollview/issues/9))

## [0.2.1] - 2024-01-30

### 📚 Documentation

- Describe demo

### ⚙️ Miscellaneous Tasks

- Create dependabot.yml
- *(deps)* Bump ratatui from 0.26.0-alpha.2 to 0.26.0-alpha.3 ([#8](https://github.com/joshka/tui-scollview/issues/8))

## [0.2.0] - 2024-01-24

### 🚀 Features

- [**breaking**] Use Position instead of tuple for offset

### 📚 Documentation

- Update readme todo list

## [0.1.5] - 2024-01-18

### 🐛 Bug Fixes

- Clippy lint
- Revert "docs: longer demo time"

### 🚜 Refactor

- Refactor demo

### 📚 Documentation

- Update readme and gif
- Longer demo time

## [0.1.4] - 2024-01-18

### 🐛 Bug Fixes

- Allow scroll to scroll until the last item is just visible

### 📚 Documentation

- Add demo gif

## [0.1.3] - 2024-01-18

### 🚀 Features

- Derive Copy for ScrollViewState
- Add movement functinos to ScrollViewState
- Ensure that the offset is clamped to only show the buffer area
- Add vertical scrollbar

### 🐛 Bug Fixes

- Don't crash when scroll area is out of visible area
- Overflow error in scroll to bottom due to u16:Max * 2
- Add scrollbar to tests

### 📚 Documentation

- Add example

## [0.1.2] - 2024-01-18

### 📚 Documentation

- Add todos to readme
- Run cargo-rdme

## [0.1.1] - 2024-01-18

### 🐛 Bug Fixes

- Fix bugs in doc comments

### ⚙️ Miscellaneous Tasks

- Add changelog
- Configure release-plz

## [0.1.0] - 2024-01-18

### ⚙️ Miscellaneous Tasks

- Add missing fields to cargo.toml
- Typo in cargo.toml

### Other

- Initial implementation
