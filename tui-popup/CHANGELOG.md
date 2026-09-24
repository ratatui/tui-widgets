# Changelog

All notable changes to this project will be documented in this file.

## [0.7.7](https://crates.io/crates/tui-popup/0.7.7) - 2026-09-24

Maintenance updates.

## [0.7.6](https://crates.io/crates/tui-popup/0.7.6) - 2026-06-14

- Modernize tui-popup examples ([#279](https://github.com/ratatui/tui-widgets/pull/279))
- Document widget examples ([#286](https://github.com/ratatui/tui-widgets/pull/286))

## [0.7.5](https://crates.io/crates/tui-popup/0.7.5) - 2026-06-11

Maintenance updates.

## [0.7.4](https://crates.io/crates/tui-popup/0.7.4) - 2026-04-04

Dependency updates.

## [0.7.3](https://crates.io/crates/tui-popup/0.7.3) - 2026-03-29

Maintenance updates.

## [0.7.2](https://crates.io/crates/tui-popup/0.7.2) - 2025-12-27

- Refresh widget docs ([#148](https://github.com/ratatui/tui-widgets/pull/148))

## [0.7.1](https://crates.io/crates/tui-popup/0.7.1) - 2025-12-27

Maintenance updates.

## [0.7.0](https://crates.io/crates/tui-popup/0.7.0) - 2025-12-27

- **Breaking:** Migrate to ratatui 0.30 ([#120](https://github.com/ratatui/tui-widgets/pull/120))
  See <https://github.com/joshka/tui-widgets/blob/main/BREAKING_CHANGES.md>

## [0.6.2](https://crates.io/crates/tui-popup/0.6.2) - 2025-11-02

- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/pull/81))
- More clippy lints ([#84](https://github.com/ratatui/tui-widgets/pull/84))

## [0.6.1](https://crates.io/crates/tui-popup/0.6.1) - 2025-11-02

- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/pull/81))
- More clippy lints ([#84](https://github.com/ratatui/tui-widgets/pull/84))

## [0.5.1](https://crates.io/crates/tui-popup/0.5.1) - 2024-10-20

- Broken links from move to tui-widgets

## [0.5.0](https://crates.io/crates/tui-popup/0.5.0) - 2024-08-11

Ratatui-0.28.0 compatible release

## [0.4.7](https://crates.io/crates/tui-popup/0.4.7) - 2024-08-09

Dependency updates.

## [0.4.6](https://crates.io/crates/tui-popup/0.4.6) - 2024-08-06

Dependency updates.

## [0.4.5](https://crates.io/crates/tui-popup/0.4.5) - 2024-08-02

- Clean up changelogs ([#17](https://github.com/ratatui/tui-widgets/pull/17))

## [0.4.4](https://crates.io/crates/tui-popup/0.4.4) - 2024-07-25

Maintenance updates.

## [0.4.3](https://crates.io/crates/tui-popup/0.4.3) - 2024-07-25

- Add tui-popup to widgets

## [0.4.2](https://crates.io/crates/tui-popup/0.4.2) - 2024-07-23

- [ef2989b](https://github.com/joshka/tui-popup/commit/ef2989b9e22df64602600ed0f50113d1f7ae9230) feat: add border_set and border_style ([#36](https://github.com/joshka/tui-popup/pull/36))
  >
  > Co-authored-by: Josh McKinney <joshka@users.noreply.github.com>

## [0.4.1](https://crates.io/crates/tui-popup/0.4.1) - 2024-07-15

- [a909878](https://github.com/joshka/tui-popup/commit/a909878c5936e3f935b5f27e13ec160dc6a89242) chore(deps): bump document-features in the all-dependencies group ([#34](https://github.com/joshka/tui-popup/pull/34))
  >
  > Bumps the all-dependencies group with 1 update: [document-features](https://github.com/slint-ui/document-features).
  >
  >
  > Updates `document-features` from 0.2.8 to 0.2.10
  > - [Release notes](https://github.com/slint-ui/document-features/releases)
  > - [Changelog](https://github.com/slint-ui/document-features/blob/master/CHANGELOG.md)
  > - [Commits](https://github.com/slint-ui/document-features/commits)
  >
  > ---
  > updated-dependencies:
  > - dependency-name: document-features
  >   dependency-type: direct:production
  >   update-type: version-update:semver-patch
  >   dependency-group: all-dependencies
  > ...
  >
  > Signed-off-by: dependabot\[bot\] <support@github.com>
  > Co-authored-by: dependabot\[bot\] <49699333+dependabot\[bot\]@users.noreply.github.com>

## [0.4.0](https://crates.io/crates/tui-popup/0.4.0) - 2024-07-07

- [1c79c1c](https://github.com/joshka/tui-popup/commit/1c79c1c462520901443d44fe474d9bee474a4d61) feat!: remove title from Popup::new ([#31](https://github.com/joshka/tui-popup/pull/31))
  >
  > BREAKING CHANGE: The `Popup::new` method no longer accepts a title
  > parameter. Instead, the title can be set using the `title` method on the
  > `Popup` instance.
  >
  > ```diff
  > - let popup = Popup::new("tui-popup demo", "Press any key to exit")
  > + let popup = Popup::new("Press any key to exit")
  > +     .title("tui-popup demo")
  >       .style(Style::new().white().on_blue());
  > ```

- [0ee7505](https://github.com/joshka/tui-popup/commit/0ee750501b5056c6a4dd42533ef023a231f6c76d) feat!: rename `MouseState` to `DragState`
  >
  > BREAKING CHANGE: The `MouseState` enum has been renamed to `DragState`
  > and the corresponding field on `PopupState` has been renamed as well.
  > The variant names are now `NotDragging` and `Dragging` instead of `None`
  > and `Dragging`.
  > The `PopupState` fields are now also `pub(crate)` instead of `pub`.
  >
  > ```diff
  > - PopupState::default().mouse_state()
  > + PopupState::default().drag_state()
  > ```

## [0.3.4](https://crates.io/crates/tui-popup/0.3.4) - 2024-07-07

- [3433aec](https://github.com/joshka/tui-popup/commit/3433aec13d3ef3179b5bdf2267d9dd6e59407f95) chore(examples): simplify examples ([#27](https://github.com/joshka/tui-popup/pull/27))
  >
  > - Move terminal setup / restore and error handling to module
  > - Smaller more focused methods
  > - Extract App struct for state example

- [3a7fd10](https://github.com/joshka/tui-popup/commit/3a7fd10034ffef21a48b0cef9c3a0bbfaf65c8de) feat(mouse): add PopupState::handle_mouse_event() ([#29](https://github.com/joshka/tui-popup/pull/29))
  >
  > Converts a crossterm mouse event into mouse movement. Requires the
  > `crossterm` feature to be enabled.
  >
  > ```rust
  > if let Event::Mouse(event) = event::read()? {
  >     popup_state.handle_mouse_event(event);
  > }
  > ```

- [c89b142](https://github.com/joshka/tui-popup/commit/c89b14292a17f0184cd3a2c6109c2d952a42b0a3) chore: create git cliff config ([#30](https://github.com/joshka/tui-popup/pull/30))

- [29008c4](https://github.com/joshka/tui-popup/commit/29008c4695e8fd38ea6398ae7cfee3d0fa1bcb03) chore: update changelog

- [81dcd7f](https://github.com/joshka/tui-popup/commit/81dcd7f89339fc99e8807c8aaade4a0c29c16a89) chore: use cliff.toml in release.plz

- [eeeb8b1](https://github.com/joshka/tui-popup/commit/eeeb8b125ce9b1c1d1f591bc3e78b68cef33149d) ci: config git-cliff to better handle PR links

## [0.3.3](https://crates.io/crates/tui-popup/0.3.3) - 2024-06-25

- [4cd3786](https://github.com/joshka/tui-popup/commit/4cd3786d195f315974d1ad3fbfb3f8c8211c7748) chore(deps): bump ratatui in the all-dependencies group ([#25](https://github.com/joshka/tui-popup/issues/25))
  >
  > Bumps the all-dependencies group with 1 update: [ratatui](https://github.com/ratatui-org/ratatui).
  >
  >
  > Updates `ratatui` from 0.26.3 to 0.27.0
  > - [Release notes](https://github.com/ratatui-org/ratatui/releases)
  > - [Changelog](https://github.com/ratatui-org/ratatui/blob/main/CHANGELOG.md)
  > - [Commits](https://github.com/ratatui-org/ratatui/compare/v0.26.3...v0.27.0)
  >
  > ---
  > updated-dependencies:
  > - dependency-name: ratatui
  >   dependency-type: direct:production
  >   update-type: version-update:semver-minor
  >   dependency-group: all-dependencies
  > ...
  >
  > Signed-off-by: dependabot\[bot\] <support@github.com>
  > Co-authored-by: dependabot\[bot\] <49699333+dependabot\[bot\]@users.noreply.github.com>

## [0.3.2](https://crates.io/crates/tui-popup/0.3.2) - 2024-05-21

- [7fffe3f](https://github.com/joshka/tui-popup/commit/7fffe3f7f6419ea3b1d9b75d0b84761e888ce9a5) chore: cleanup clippy lints

- [8ea131f](https://github.com/joshka/tui-popup/commit/8ea131f9e9754edd1e656f0fcd65e3bbafbff5b4) chore(deps): bump derive-getters in the all-dependencies group ([#22](https://github.com/joshka/tui-popup/issues/22))
  >
  > Bumps the all-dependencies group with 1 update: derive-getters.
  >
  >
  > Updates `derive-getters` from 0.3.0 to 0.4.0
  >
  > ---
  > updated-dependencies:
  > - dependency-name: derive-getters
  >   dependency-type: direct:production
  >   update-type: version-update:semver-minor
  >   dependency-group: all-dependencies
  > ...
  >
  > Signed-off-by: dependabot\[bot\] <support@github.com>
  > Co-authored-by: dependabot\[bot\] <49699333+dependabot\[bot\]@users.noreply.github.com>

- [dd658c0](https://github.com/joshka/tui-popup/commit/dd658c0c4c7f058d9fa3ffbde07765015810d0ab) --- ([#24](https://github.com/joshka/tui-popup/issues/24))
  >
  > updated-dependencies:
  > - dependency-name: ratatui
  >   dependency-type: direct:production
  >   update-type: version-update:semver-patch
  >   dependency-group: all-dependencies
  > ...
  >
  > Signed-off-by: dependabot\[bot\] <support@github.com>
  > Co-authored-by: dependabot\[bot\] <49699333+dependabot\[bot\]@users.noreply.github.com>

## [0.3.1](https://crates.io/crates/tui-popup/0.3.1) - 2024-05-01

- [3eb0953](https://github.com/joshka/tui-popup/commit/3eb095357f6a4ea624e6cb2f95056d0d27bda3fd) feat: allow setting borders ([#19](https://github.com/joshka/tui-popup/issues/19))
  >

## [0.3.0](https://crates.io/crates/tui-popup/0.3.0) - 2024-04-24

- [a9744d4](https://github.com/joshka/tui-popup/commit/a9744d49905f5727311bb33870d02ad6f2b78ba8) chore(deps): bump eyre from 0.6.11 to 0.6.12 ([#14](https://github.com/joshka/tui-popup/issues/14))
  >
  > Bumps [eyre](https://github.com/eyre-rs/eyre) from 0.6.11 to 0.6.12.
  > - [Commits](https://github.com/eyre-rs/eyre/commits)
  >
  > ---
  > updated-dependencies:
  > - dependency-name: eyre
  >   dependency-type: indirect
  > ...
  >
  > Signed-off-by: dependabot\[bot\] <support@github.com>
  > Co-authored-by: dependabot\[bot\] <49699333+dependabot\[bot\]@users.noreply.github.com>

- [1be3247](https://github.com/joshka/tui-popup/commit/1be3247bc66493cccaf752738544344b9b4996d1) ci: group dependabot updates

- [2e5bc7a](https://github.com/joshka/tui-popup/commit/2e5bc7ab054e11f19e825dd06f8b4aafaba56ff8) feat!: make popup accept any widget as the body ([#18](https://github.com/joshka/tui-popup/issues/18))
  >
  > Popup is now a generic over the body widget, allowing for more complex
  > widgets to be used as the body of the popup. This change also allows for
  > the popup to be rendered multiple times without needing to clone it.
  >
  > Fixes: <https://github.com/joshka/tui-popup/issues/16>
  >
  > BREAKING CHANGE: The Popup widget now accepts any widget as the body
  > instead of just a Text. This allows for more complex widgets to be used
  > as the body of the popup, such as Paragraph or ScrollView.
  >
  > Additionally the Popup widget now implements `WidgetRef` and
  > `StatefulWidgetRef` instead of having a separate PopupWidget type. This
  > allows for saving a reference to the popup and rendering it multiple
  > times without needing to clone it. To update your code, replace
  > `frame.render_widget(popup.to_widget(), area)` with
  > `frame.render_widget(&popup, area)` and replace
  > `frame.render_stateful_widget(popup.to_widget(), area, state)`
  > with `frame.render_stateful_widget_ref(&popup, area, state)`.

## [0.2.4](https://crates.io/crates/tui-popup/0.2.4) - 2024-04-01

- [684429d](https://github.com/joshka/tui-popup/commit/684429df40903af880a664b1951ab2031d833747) chore(deps): bump lipsum from 0.9.0 to 0.9.1 ([#12](https://github.com/joshka/tui-popup/issues/12))
  >
  > Bumps [lipsum](https://github.com/mgeisler/lipsum) from 0.9.0 to 0.9.1.
  > - [Release notes](https://github.com/mgeisler/lipsum/releases)
  > - [Commits](https://github.com/mgeisler/lipsum/compare/0.9.0...0.9.1)
  >
  > ---
  > updated-dependencies:
  > - dependency-name: lipsum
  >   dependency-type: direct:production
  >   update-type: version-update:semver-patch
  > ...
  >
  > Signed-off-by: dependabot\[bot\] <support@github.com>
  > Co-authored-by: dependabot\[bot\] <49699333+dependabot\[bot\]@users.noreply.github.com>

- [92097b4](https://github.com/joshka/tui-popup/commit/92097b4c87f6a162e41162e2ec72add9e65bc63e) chore(deps): bump color-eyre from 0.6.2 to 0.6.3 ([#11](https://github.com/joshka/tui-popup/issues/11))
  >
  > Bumps [color-eyre](https://github.com/eyre-rs/eyre) from 0.6.2 to 0.6.3.
  > - [Commits](https://github.com/eyre-rs/eyre/compare/v0.6.2...color-eyre-v0.6.3)
  >
  > ---
  > updated-dependencies:
  > - dependency-name: color-eyre
  >   dependency-type: direct:production
  >   update-type: version-update:semver-patch
  > ...
  >
  > Signed-off-by: dependabot\[bot\] <support@github.com>
  > Co-authored-by: dependabot\[bot\] <49699333+dependabot\[bot\]@users.noreply.github.com>

## [0.2.3](https://crates.io/crates/tui-popup/0.2.3) - 2024-03-12

- [c36f77f](https://github.com/joshka/tui-popup/commit/c36f77f1a281adcf98b2ea55ba8da235aaf6310d) chore: Create dependabot.yml

- [67d210f](https://github.com/joshka/tui-popup/commit/67d210f8db41575b61492cf8061ffd3c8e74a319) chore(deps): bump ratatui from 0.25.0 to 0.26.1 ([#8](https://github.com/joshka/tui-popup/issues/8))
  >

- [7273147](https://github.com/joshka/tui-popup/commit/727314715cfbfbc4b57a5251b2e9223455e3acdb) ci: use joshka/github-workflows ([#10](https://github.com/joshka/tui-popup/issues/10))
  >
  > <https://github.com/joshka/github-workflows>

- [e0d7ba4](https://github.com/joshka/tui-popup/commit/e0d7ba4d24f96738ba345ed998923be266b88135) chore(deps): bump mio from 0.8.10 to 0.8.11 ([#9](https://github.com/joshka/tui-popup/issues/9))
  >
  > Bumps [mio](https://github.com/tokio-rs/mio) from 0.8.10 to 0.8.11.
  > - [Release notes](https://github.com/tokio-rs/mio/releases)
  > - [Changelog](https://github.com/tokio-rs/mio/blob/master/CHANGELOG.md)
  > - [Commits](https://github.com/tokio-rs/mio/compare/v0.8.10...v0.8.11)
  >
  > ---
  > updated-dependencies:
  > - dependency-name: mio
  >   dependency-type: indirect
  > ...
  >
  > Signed-off-by: dependabot\[bot\] <support@github.com>
  > Co-authored-by: dependabot\[bot\] <49699333+dependabot\[bot\]@users.noreply.github.com>

## [0.2.2](https://crates.io/crates/tui-popup/0.2.2) - 2024-01-02

- [88c89a0](https://github.com/joshka/tui-popup/commit/88c89a089a40cb85e23033072df4e10d840b6f66) feat: respond to mouse drag

- [427592d](https://github.com/joshka/tui-popup/commit/427592dd8fc4996b9e75ad961555ca49ed7e5f50) refactor: extract each type to submodule

- [7f96b7e](https://github.com/joshka/tui-popup/commit/7f96b7ee5732977eb7a8e70ef765a246e2111254) docs: add mouse info to readme

- [33a8d33](https://github.com/joshka/tui-popup/commit/33a8d337e144eef70c7f7a9b9c418dde4568044a) feat: add move_to() method to allow manual positioning

- [973f2de](https://github.com/joshka/tui-popup/commit/973f2de3728f911daad3f763b81bc3a3faa7513a) docs: update readme todos

## [0.2.1](https://crates.io/crates/tui-popup/0.2.1) - 2023-12-31

- [9b8736f](https://github.com/joshka/tui-popup/commit/9b8736f0a298b86bf04cccee220d122507bdcdcd) feat: add PopupState and state example

## [0.2.0](https://crates.io/crates/tui-popup/0.2.0) - 2023-12-31

- [74e6263](https://github.com/joshka/tui-popup/commit/74e62631eb486aac5173988372c4faac3471df09) feat: add popup style

- [4935003](https://github.com/joshka/tui-popup/commit/4935003aaf8878078a8f6fbe75d313cf67c5e21e) docs: add style to readme example

- [78cd3ae](https://github.com/joshka/tui-popup/commit/78cd3ae1e1bd7a0ad8190025ceeb77202756cf45) docs: add doc comments

- [336398b](https://github.com/joshka/tui-popup/commit/336398b119a21cd505dae87daf2506a64b3fc61d) docs: update readme

- [8159e19](https://github.com/joshka/tui-popup/commit/8159e190de559e9ec334638bc058c8856f589f71) fix: mark Popup as non_exhaustive
  >
  > ensures that new fields can be added without breaking compat

- [6b96794](https://github.com/joshka/tui-popup/commit/6b9679424a507a496edd165750b80b1ce3e2b4b9) core: setup clippy lints

## [0.1.1](https://crates.io/crates/tui-popup/0.1.1) - 2023-12-31

- [ff8f530](https://github.com/joshka/tui-popup/commit/ff8f53041f129e4f262a3a8756f1dc2f3628a55a) Merge pull request #1 from joshka/release-plz-2023-12-31T02-22-23Z
  >
  > chore: release v0.1.0

- [b87fd77](https://github.com/joshka/tui-popup/commit/b87fd77d0f3cd2d1d0e7efa96c4d6ee5093a0fcb) docs: add badges and license

- [30d9579](https://github.com/joshka/tui-popup/commit/30d9579579c3668fffd07595a806756346e0ab34) Merge pull request #2 from joshka/release-plz-2023-12-31T02-30-22Z
  >
  > chore: release v0.1.1

## [0.1.0](https://crates.io/crates/tui-popup/0.1.0) - 2023-12-31

- [5803e9b](https://github.com/joshka/tui-popup/commit/5803e9b46592096da4c1d139940aba57d81cc8e4) feat: initial implementation

- [d4322d8](https://github.com/joshka/tui-popup/commit/d4322d8b35298c27b5b26d614a64dd3e814fe4f1) docs: update readme and demo

- [aad723f](https://github.com/joshka/tui-popup/commit/aad723f835ade62a4f76da43917e504f230799e8) ci: add release-plz and CI
