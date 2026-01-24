# Changelog

All notable changes to this project will be documented in this file.

## [0.7.2] - 2025-12-27

### 📚 Documentation

- Refresh widget docs ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  > Standardize widget crate docs and README layouts.
  > Unify badges, links, and license references.
  > Add consistent usage sections and link style updates.


## [0.7.1] - 2025-12-27

### ⚙️ Miscellaneous Tasks

- Refresh readmes and rdme check ([#140](https://github.com/ratatui/tui-widgets/issues/140))
  > Regenerate crate READMEs via cargo-rdme and add a CI check to keep
  > workspace readmes in sync.


## [0.7.0] - 2025-12-27

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


## [0.6.2] - 2025-11-02

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

- Remove needless lifetimes ([#60](https://github.com/ratatui/tui-widgets/issues/60))


## [0.6.1] - 2025-11-02

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

- Remove needless lifetimes ([#60](https://github.com/ratatui/tui-widgets/issues/60))


## [0.5.1] - 2024-10-20

### 🐛 Bug Fixes

- Broken links from move to tui-widgets

## [0.5.0] - 2024-08-11

Ratatui-0.28.0 compatible release

## [0.4.7] - 2024-08-09

### ⚙️ Miscellaneous Tasks

- Update Cargo.toml dependencies

## [0.4.6] - 2024-08-06

### Other

- *(deps)* Update crossterm requirement from 0.27.0 to 0.28.1 ([#22](https://github.com/ratatui/tui-widgets/pull/22))
  > Updates the requirements on
  > [crossterm](https://github.com/crossterm-rs/crossterm) to permit the
  > latest version.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/crossterm-rs/crossterm/releases">crossterm's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>0.27.0</h2>
  > <h1>Version 0.27</h1>
  > <h2>Added ⭐</h2>
  > <ul>
  > <li>Add <code>NO_COLOR</code> support (<a
  > href="https://no-color.org/">https://no-color.org/</a>)</li>
  > <li>Add option to force overwrite <code>NO_COLOR</code> (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/802">[#802](https://github.com/ratatui/tui-widgets/pull/802)</a>)</li>
  > <li>Add support for scroll left/right events on windows and unix systems
  > (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/788">[#788](https://github.com/ratatui/tui-widgets/pull/788)</a>).</li>
  > <li>Add <code>window_size</code> function to fetch pixel width/height of
  > screen for more sophisticated rendering in terminals.</li>
  > <li>Add support for deserializing hex color strings to `Color`` e.g
  > #fffff.</li>
  > </ul>
  > <h2>Changes</h2>
  > <ul>
  > <li>Make the events module an optional feature <code>events</code> (to
  > make crossterm more lightweight) (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/776">[#776](https://github.com/ratatui/tui-widgets/pull/776)</a>)</li>
  > </ul>
  > <h2>Breaking ⚠️</h2>
  > <ul>
  > <li>Set minimum rustc version to 1.58 (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/798">[#798](https://github.com/ratatui/tui-widgets/pull/798)</a>)</li>
  > <li>Change all error types to <code>std::io::Result</code> (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/765">[#765](https://github.com/ratatui/tui-widgets/pull/765)</a>)</li>
  > </ul>
  > <p><a href="https://github.com/Gronis"><code>@​Gronis</code></a>, <a
  > href="https://github.com/kevin-vigor"><code>@​kevin-vigor</code></a>, <a
  > href="https://github.com/Wilfred"><code>@​Wilfred</code></a>, <a
  > href="https://github.com/benjajaja"><code>@​benjajaja</code></a>, <a
  > href="https://github.com/blt-r"><code>@​blt-r</code></a>, <a
  > href="https://github.com/Piturnah"><code>@​Piturnah</code></a>, <a
  > href="https://github.com/kdheepak"><code>@​kdheepak</code></a>, <a
  > href="https://github.com/DeathVenom54"><code>@​DeathVenom54</code></a>,
  > <a href="https://github.com/senekor"><code>@​senekor</code></a>, <a
  > href="https://github.com/joseluis"><code>@​joseluis</code></a>, <a
  > href="https://github.com/gibbz00"><code>@​gibbz00</code></a>, <a
  > href="https://github.com/lesleyrs"><code>@​lesleyrs</code></a>, <a
  > href="https://github.com/jhartzell42"><code>@​jhartzell42</code></a></p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/crossterm-rs/crossterm/blob/master/CHANGELOG.md">crossterm's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h1>Unreleased</h1>
  > <h1>Version 0.28.1</h1>
  > <h2>Fixed 🐛</h2>
  > <ul>
  > <li>Fix broken build on linux when using <code>use-dev-tty</code> with
  > (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/906">[#906](https://github.com/ratatui/tui-widgets/pull/906)</a>)</li>
  > </ul>
  > <h2>Breaking ⚠️</h2>
  > <ul>
  > <li>Fix desync with mio and signalhook between repo and published crate.
  > (upgrade to mio 1.0)</li>
  > </ul>
  > <h1>Version 0.28</h1>
  > <h2>Added ⭐</h2>
  > <ul>
  > <li>Capture double click mouse events on windows (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/826">[#826](https://github.com/ratatui/tui-widgets/pull/826)</a>)</li>
  > <li>(De)serialize Reset color (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/824">[#824](https://github.com/ratatui/tui-widgets/pull/824)</a>)</li>
  > <li>Add functions to allow constructing <code>Attributes</code> in a
  > const context (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/817">[#817](https://github.com/ratatui/tui-widgets/pull/817)</a>)</li>
  > <li>Implement <code>Display</code> for <code>KeyCode</code> and
  > <code>KeyModifiers</code> (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/862">[#862](https://github.com/ratatui/tui-widgets/pull/862)</a>)</li>
  > </ul>
  > <h2>Changed ⚙️</h2>
  > <ul>
  > <li>Use Rustix by default instead of libc. Libc can be re-enabled if
  > necessary with the <code>libc</code> feature flag (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/892">[#892](https://github.com/ratatui/tui-widgets/pull/892)</a>)</li>
  > <li><code>FileDesc</code> now requires a lifetime annotation.</li>
  > <li>Improve available color detection (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/885">[#885](https://github.com/ratatui/tui-widgets/pull/885)</a>)</li>
  > <li>Speed up <code>SetColors</code> by ~15-25% (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/879">[#879](https://github.com/ratatui/tui-widgets/pull/879)</a>)</li>
  > <li>Remove unsafe and unnecessary size argument from
  > <code>FileDesc::read()</code> (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/821">[#821](https://github.com/ratatui/tui-widgets/pull/821)</a>)</li>
  > </ul>
  > <h2>Breaking ⚠️</h2>
  > <ul>
  > <li>Fix duplicate bit masks for caps lock and num lock (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/863">[#863](https://github.com/ratatui/tui-widgets/pull/863)</a>).
  > This breaks serialization of <code>KeyEventState</code></li>
  > </ul>
  > <h1>Version 0.27.1</h1>
  > <h2>Added ⭐</h2>
  > <ul>
  > <li>Add support for (de)serializing <code>Reset</code>
  > <code>Color</code></li>
  > </ul>
  > <h1>Version 0.27</h1>
  > <h2>Added ⭐</h2>
  > <ul>
  > <li>Add <code>NO_COLOR</code> support (<a
  > href="https://no-color.org/">https://no-color.org/</a>)</li>
  > <li>Add option to force overwrite <code>NO_COLOR</code> (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/802">[#802](https://github.com/ratatui/tui-widgets/pull/802)</a>)</li>
  > <li>Add support for scroll left/right events on windows and unix systems
  > (<a
  > href="https://redirect.github.com/crossterm-rs/crossterm/issues/788">[#788](https://github.com/ratatui/tui-widgets/pull/788)</a>).</li>
  > <li>Add <code>window_size</code> function to fetch pixel width/height of
  > screen for more sophisticated rendering in terminals.</li>
  > <li>Add support for deserializing hex color strings to
  > <code>Color</code> e.g #fffff.</li>
  > </ul>
  > <h2>Changed ⚙️</h2>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li>See full diff in <a
  > href="https://github.com/crossterm-rs/crossterm/compare/0.27.0...0.27.0">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.
  >
  > [//]:# (dependabot-automerge-start)
  >
  > [//]:# (dependabot-automerge-end)
  >
  > ---
  >
  > <details>
  > <summary>Dependabot commands and options</summary>
  > <br />
  >
  > You can trigger Dependabot actions by commenting on this PR:
  > - `@dependabot rebase` will rebase this PR
  > - `@dependabot recreate` will recreate this PR, overwriting any edits
  > that have been made to it
  > - `@dependabot merge` will merge this PR after your CI passes on it
  > - `@dependabot squash and merge` will squash and merge this PR after
  > your CI passes on it
  > - `@dependabot cancel merge` will cancel a previously requested merge
  > and block automerging
  > - `@dependabot reopen` will reopen this PR if it is closed
  > - `@dependabot close` will close this PR and stop Dependabot recreating
  > it. You can achieve the same result by closing it manually
  > - `@dependabot show <dependency name> ignore conditions` will show all
  > of the ignore conditions of the specified dependency
  > - `@dependabot ignore this major version` will close this PR and stop
  > Dependabot creating any more for this major version (unless you reopen
  > the PR or upgrade to it yourself)
  > - `@dependabot ignore this minor version` will close this PR and stop
  > Dependabot creating any more for this minor version (unless you reopen
  > the PR or upgrade to it yourself)
  > - `@dependabot ignore this dependency` will close this PR and stop
  > Dependabot creating any more for this dependency (unless you reopen the
  > PR or upgrade to it yourself)
  >
  >
  > </details>
  >
  > ---------

## [0.4.5] - 2024-08-02

### 📚 Documentation

- Clean up changelogs ([#17](https://github.com/ratatui/tui-widgets/pull/17))
  > - removed unnecessary footer comments
  > - removed [unreleased] sections
  > - removed duplicate release notes

## [0.4.4] - 2024-07-25

### ⚙️ Miscellaneous Tasks

- Update READMEs and licensing info

## [0.4.3] - 2024-07-25

### ⚙️ Miscellaneous Tasks

- Move tui-popup to its own directory

### Other

- Add tui-popup to widgets

## [0.4.2] - 2024-07-23

- [ef2989b](https://github.com/joshka/tui-popup/commit/ef2989b9e22df64602600ed0f50113d1f7ae9230) feat: add border_set and border_style ([#36](https://github.com/joshka/tui-popup/pull/36))
  >
  > Co-authored-by: Josh McKinney <joshka@users.noreply.github.com>

## [0.4.1] - 2024-07-15

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
  > Signed-off-by: dependabot[bot] <support@github.com>
  > Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>

## [0.4.0] - 2024-07-07

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

## [0.3.4] - 2024-07-07

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

## [0.3.3] - 2024-06-25

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
  > Signed-off-by: dependabot[bot] <support@github.com>
  > Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>

## [0.3.2] - 2024-05-21

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
  > Signed-off-by: dependabot[bot] <support@github.com>
  > Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>

- [dd658c0](https://github.com/joshka/tui-popup/commit/dd658c0c4c7f058d9fa3ffbde07765015810d0ab) --- ([#24](https://github.com/joshka/tui-popup/issues/24))
  >
  > updated-dependencies:
  > - dependency-name: ratatui
  >   dependency-type: direct:production
  >   update-type: version-update:semver-patch
  >   dependency-group: all-dependencies
  > ...
  >
  > Signed-off-by: dependabot[bot] <support@github.com>
  > Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>

## [0.3.1] - 2024-05-01

- [3eb0953](https://github.com/joshka/tui-popup/commit/3eb095357f6a4ea624e6cb2f95056d0d27bda3fd) feat: allow setting borders ([#19](https://github.com/joshka/tui-popup/issues/19))
  >

## [0.3.0] - 2024-04-24

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
  > Signed-off-by: dependabot[bot] <support@github.com>
  > Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>

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

## [0.2.4] - 2024-04-01

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
  > Signed-off-by: dependabot[bot] <support@github.com>
  > Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>

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
  > Signed-off-by: dependabot[bot] <support@github.com>
  > Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>

## [0.2.3] - 2024-03-12

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
  > Signed-off-by: dependabot[bot] <support@github.com>
  > Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>

## [0.2.2] - 2024-01-02

- [88c89a0](https://github.com/joshka/tui-popup/commit/88c89a089a40cb85e23033072df4e10d840b6f66) feat: respond to mouse drag

- [427592d](https://github.com/joshka/tui-popup/commit/427592dd8fc4996b9e75ad961555ca49ed7e5f50) refactor: extract each type to submodule

- [7f96b7e](https://github.com/joshka/tui-popup/commit/7f96b7ee5732977eb7a8e70ef765a246e2111254) docs: add mouse info to readme

- [33a8d33](https://github.com/joshka/tui-popup/commit/33a8d337e144eef70c7f7a9b9c418dde4568044a) feat: add move_to() method to allow manual positioning

- [973f2de](https://github.com/joshka/tui-popup/commit/973f2de3728f911daad3f763b81bc3a3faa7513a) docs: update readme todos

## [0.2.1] - 2023-12-31

- [9b8736f](https://github.com/joshka/tui-popup/commit/9b8736f0a298b86bf04cccee220d122507bdcdcd) feat: add PopupState and state example

## [0.2.0] - 2023-12-31

- [74e6263](https://github.com/joshka/tui-popup/commit/74e62631eb486aac5173988372c4faac3471df09) feat: add popup style

- [4935003](https://github.com/joshka/tui-popup/commit/4935003aaf8878078a8f6fbe75d313cf67c5e21e) docs: add style to readme example

- [78cd3ae](https://github.com/joshka/tui-popup/commit/78cd3ae1e1bd7a0ad8190025ceeb77202756cf45) docs: add doc comments

- [336398b](https://github.com/joshka/tui-popup/commit/336398b119a21cd505dae87daf2506a64b3fc61d) docs: update readme

- [8159e19](https://github.com/joshka/tui-popup/commit/8159e190de559e9ec334638bc058c8856f589f71) fix: mark Popup as non_exhaustive
  >
  > ensures that new fields can be added without breaking compat

- [6b96794](https://github.com/joshka/tui-popup/commit/6b9679424a507a496edd165750b80b1ce3e2b4b9) core: setup clippy lints

## [0.1.1] - 2023-12-31

- [ff8f530](https://github.com/joshka/tui-popup/commit/ff8f53041f129e4f262a3a8756f1dc2f3628a55a) Merge pull request #1 from joshka/release-plz-2023-12-31T02-22-23Z
  >
  > chore: release v0.1.0

- [b87fd77](https://github.com/joshka/tui-popup/commit/b87fd77d0f3cd2d1d0e7efa96c4d6ee5093a0fcb) docs: add badges and license

- [30d9579](https://github.com/joshka/tui-popup/commit/30d9579579c3668fffd07595a806756346e0ab34) Merge pull request #2 from joshka/release-plz-2023-12-31T02-30-22Z
  >
  > chore: release v0.1.1

## [0.1.0] - 2023-12-31

- [5803e9b](https://github.com/joshka/tui-popup/commit/5803e9b46592096da4c1d139940aba57d81cc8e4) feat: initial implementation

- [d4322d8](https://github.com/joshka/tui-popup/commit/d4322d8b35298c27b5b26d614a64dd3e814fe4f1) docs: update readme and demo

- [aad723f](https://github.com/joshka/tui-popup/commit/aad723f835ade62a4f76da43917e504f230799e8) ci: add release-plz and CI
