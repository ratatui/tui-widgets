# Changelog

All notable changes to this project will be documented in this file.

## [0.6.7](https://crates.io/crates/tui-prompts/0.6.7) - 2026-06-18

- *(prompts)* Add select prompt ([#177](https://github.com/ratatui/tui-widgets/pull/177))
- *(prompts)* Repair select example CI ([#306](https://github.com/ratatui/tui-widgets/pull/306))

## [0.6.6](https://crates.io/crates/tui-prompts/0.6.6) - 2026-06-14

- *(prompts)* Render text prompts by reference ([#291](https://github.com/ratatui/tui-widgets/pull/291))

## [0.6.5](https://crates.io/crates/tui-prompts/0.6.5) - 2026-06-14

- Modernize tui-prompts examples ([#281](https://github.com/ratatui/tui-widgets/pull/281))
- Document widget examples ([#286](https://github.com/ratatui/tui-widgets/pull/286))

## [0.6.4](https://crates.io/crates/tui-prompts/0.6.4) - 2026-06-11

- *(prompts)* Allow hiding status symbol ([#191](https://github.com/ratatui/tui-widgets/pull/191))

## [0.6.3](https://crates.io/crates/tui-prompts/0.6.3) - 2026-04-04

- \[codex\] Vendor CI workflows into this repository ([#212](https://github.com/ratatui/tui-widgets/issues/212))

## [0.6.2](https://crates.io/crates/tui-prompts/0.6.2) - 2026-03-29

Maintenance updates.

## [0.6.1](https://crates.io/crates/tui-prompts/0.6.1) - 2025-12-27

- Refresh widget docs ([#148](https://github.com/ratatui/tui-widgets/pull/148))

## [0.6.0](https://crates.io/crates/tui-prompts/0.6.0) - 2025-12-27

- **Breaking:** Migrate to ratatui 0.30 ([#120](https://github.com/ratatui/tui-widgets/pull/120))
  See <https://github.com/joshka/tui-widgets/blob/main/BREAKING_CHANGES.md>

## [0.5.2](https://crates.io/crates/tui-prompts/0.5.2) - 2025-11-02

- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/pull/81))
- More clippy lints ([#84](https://github.com/ratatui/tui-widgets/pull/84))
- *(prompts)* Full-width character input in non-multiline prompt ([#93](https://github.com/ratatui/tui-widgets/issues/93)) ([#94](https://github.com/ratatui/tui-widgets/pull/94))

## [0.5.1](https://crates.io/crates/tui-prompts/0.5.1) - 2025-11-02

- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/pull/81))
- More clippy lints ([#84](https://github.com/ratatui/tui-widgets/pull/84))
- *(prompts)* Full-width character input in non-multiline prompt ([#93](https://github.com/ratatui/tui-widgets/issues/93)) ([#94](https://github.com/ratatui/tui-widgets/pull/94))

## [0.4.1](https://crates.io/crates/tui-prompts/0.4.1) - 2024-10-20

Dependency updates.

## [0.4.0](https://crates.io/crates/tui-prompts/0.4.0) - 2024-08-11

Ratatui-0.28.0 compatible release

## [0.3.23](https://crates.io/crates/tui-prompts/0.3.23) - 2024-08-12

### ⚙️ Miscellaneous Tasks

- Update Cargo.toml dependencies

## [0.3.22](https://crates.io/crates/tui-prompts/0.3.22) - 2024-08-09

Dependency updates.

## [0.3.21](https://crates.io/crates/tui-prompts/0.3.21) - 2024-08-06

Dependency updates.

## [0.3.20](https://crates.io/crates/tui-prompts/0.3.20) - 2024-08-02

- Clean up changelogs ([#17](https://github.com/ratatui/tui-widgets/pull/17))

## [0.3.19](https://crates.io/crates/tui-prompts/0.3.19) - 2024-07-25

Maintenance updates.

## [0.3.18](https://crates.io/crates/tui-prompts/0.3.18) - 2024-07-24

- Delete and backspace behavior for multi-byte characters ([#57](https://github.com/ratatui/tui-widgets/pull/57))
- Fixup tui-prompts version to match crates.io

## [0.3.14](https://crates.io/crates/tui-prompts/0.3.14) - 2024-07-02

### Other

- *(deps)* bump clap from 4.5.7 to 4.5.8 ([#53](https://github.com/joshka/tui-prompts/pull/53))
- *(deps)* bump ratatui-macros from 0.4.1 to 0.4.2 ([#54](https://github.com/joshka/tui-prompts/pull/54))

## [0.3.13](https://crates.io/crates/tui-prompts/0.3.13) - 2024-06-25

### Other

- *(deps)* update ratatui, itertools, rstest, clap ([#51](https://github.com/joshka/tui-prompts/pull/51))
- simplify text_prompt tests ([#49](https://github.com/joshka/tui-prompts/pull/49))

## [0.3.12](https://crates.io/crates/tui-prompts/0.3.12) - 2024-05-11

### Fixed

- *(text_prompt)* use unicode-width instead of char cnt ([#46](https://github.com/joshka/tui-prompts/pull/46))

### Other

- *(text_prompt)* avoid unnecessary clones ([#48](https://github.com/joshka/tui-prompts/pull/48))

## [0.3.11](https://crates.io/crates/tui-prompts/0.3.11) - 2024-04-25

### Other

- *(deps)* bump ratatui from 0.26.1 to 0.26.2 ([#44](https://github.com/joshka/tui-prompts/pull/44))

## [0.3.10](https://crates.io/crates/tui-prompts/0.3.10) - 2024-04-04

### Fixed

- Fix prompt for multi-byte characters. ([#41](https://github.com/joshka/tui-prompts/pull/41))
- mark broken tests with FIXME ([#42](https://github.com/joshka/tui-prompts/pull/42))

## [0.3.9](https://crates.io/crates/tui-prompts/0.3.9) - 2024-04-01

### Other

- *(deps)* bump color-eyre from 0.6.2 to 0.6.3 ([#34](https://github.com/joshka/tui-prompts/pull/34))
- *(deps)* bump indoc from 2.0.4 to 2.0.5 ([#36](https://github.com/joshka/tui-prompts/pull/36))
- *(deps)* bump clap from 4.5.2 to 4.5.4 ([#37](https://github.com/joshka/tui-prompts/pull/37))

## [0.3.8](https://crates.io/crates/tui-prompts/0.3.8) - 2024-03-12

### Other

- add workflow names ([#32](https://github.com/joshka/tui-prompts/pull/32))

## [0.3.7](https://crates.io/crates/tui-prompts/0.3.7) - 2024-03-12

### Other

- allow release-plz workflow to be manually run ([#30](https://github.com/joshka/tui-prompts/pull/30))

## [0.3.6](https://crates.io/crates/tui-prompts/0.3.6) - 2024-03-12

### Other

- *(deps)* bump clap from 4.5.0 to 4.5.2 ([#23](https://github.com/joshka/tui-prompts/pull/23))
- use release-plz from reusable workflows ([#27](https://github.com/joshka/tui-prompts/pull/27))
- add test.yml workflow ([#26](https://github.com/joshka/tui-prompts/pull/26))
- add check.yml workflow ([#24](https://github.com/joshka/tui-prompts/pull/24))

## [0.3.5](https://crates.io/crates/tui-prompts/0.3.5) - 2024-02-13

### Other

- *(deps)* bump ratatui from 0.25.0 to 0.26.1 ([#18](https://github.com/joshka/tui-prompts/pull/18))
- *(deps)* bump clap from 4.4.18 to 4.5.0 ([#19](https://github.com/joshka/tui-prompts/pull/19))

## [0.3.4](https://crates.io/crates/tui-prompts/0.3.4) - 2024-01-30

### Other

- *(deps)* bump itertools from 0.12.0 to 0.12.1 ([#15](https://github.com/joshka/tui-prompts/pull/15))

## [0.3.3](https://crates.io/crates/tui-prompts/0.3.3) - 2024-01-25

### Other

- *(deps)* bump clap from 4.4.8 to 4.4.18 ([#14](https://github.com/joshka/tui-prompts/pull/14))
- *(deps)* bump ratatui from 0.24.0 to 0.25.0 ([#12](https://github.com/joshka/tui-prompts/pull/12))
- *(deps)* bump actions/checkout from 3 to 4 ([#11](https://github.com/joshka/tui-prompts/pull/11))
- Create dependabot.yml

## [0.3.2](https://crates.io/crates/tui-prompts/0.3.2) - 2023-12-15

### Other

- *(deps)* bump zerocopy from 0.7.26 to 0.7.31

## [0.3.1](https://crates.io/crates/tui-prompts/0.3.1) - 2023-11-17

### Other

- add debug info to examples and update readme

## [0.3.0](https://crates.io/crates/tui-prompts/0.3.0) - 2023-11-17

### Other

- deps update and clippy lint fixes
- make TextState fields private

## [0.2.3](https://crates.io/crates/tui-prompts/0.2.3) - 2023-07-25

### Other

- simplifiy wrapping

## [0.2.2](https://crates.io/crates/tui-prompts/0.2.2) - 2023-07-18

### Other

- add unit tests for soft wrapping single lines
- add release-plz github action

## [0.2.1](https://crates.io/crates/tui-prompts/0.2.1) - 2023-07-18

### Other

- add TODO list and key bindings to readme
- add invisible text prompt to readme
- add coverage job to bacon config

## [0.2.0](https://crates.io/crates/tui-prompts/0.2.0) - 2023-07-17

### Added

- handle focus and invisible text prompt

### Other

- tweak text example
- Readme badges / license single file
- Update README.md

## [0.1.1](https://crates.io/crates/tui-prompts/0.1.1) - 2023-07-11

### Other

- fix cargo.toml categories and keywords
- release

## [0.1.0](https://crates.io/crates/tui-prompts/0.1.0) - 2023-07-11

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
