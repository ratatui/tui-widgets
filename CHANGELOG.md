# Changelog

All notable changes to this project will be documented in this file.

## [0.7.1] - 2026-02-14

### 🚀 Features

- *(scrollbar)* Update glyph previews and tests ([#169](https://github.com/ratatui/tui-widgets/issues/169))
  > Default ScrollBar renders without arrow endcaps and uses a dark gray
  > background with a blank (space) track.
  >
  > Add glyph set variants and improve the Unicode-only fallback. Document
  > glyph sets with a 1/8-step horizontal thumb walk, and add snapshot
  > render tests to keep the glyph combinations stable.

- *(scrollbar)* Support crossterm 0.28 ([#172](https://github.com/ratatui/tui-widgets/issues/172))
  > Add versioned crossterm feature flags and re-export the selected version
  > as `tui_scrollbar::crossterm`.
  >
  > Add CI checks for the feature matrix and a docs.rs-style build.
  >
  > ---------

- *(tui-bigtext)* Enable no_std as default ([#190](https://github.com/ratatui/tui-widgets/issues/190))
  > This enables using `tui-bigtext` in a no_std environment
  >
  > e.g. https://github.com/ratatui/mousefood/pull/159

- *(tui-big-text)* Add optional Block wrapping for BigText ([#197](https://github.com/ratatui/tui-widgets/issues/197))
  > # Summary
  > Add optional block wrapping for the big text widget so it can render
  > within a border and title area.
  >
  > # Motivation
  > This enables composing the big text widget with standard block
  > decorations, which is useful for dashboards and status displays.
  > E. g. I'm making layout with BigText stopwatch and I want to use block
  > title as a place for status. Additionally it fits into my dashboard
  > design.
  > I think it might be useful.
  >
  > My example:
  >
  > ![Screenshot_5](https://github.com/user-attachments/assets/0fdbbdeb-8f05-4588-ad39-ba5133295717)
  >
  >
  > # Changes
  > - Added optional block support in the widget configuration and rendering
  > flow.
  > - Added a test to validate rendering with a bordered block and title.
  >
  > # Testing
  > `cargo test -p tui-big-text --all-features -- --nocapture`

### 📚 Documentation

- *(license)* Add ratatui developers to the license

### ⚙️ Miscellaneous Tasks

- *(tui-scrollbar)* Release v0.2.1 ([#170](https://github.com/ratatui/tui-widgets/issues/170))
  > ## 🤖 New release
  >
  > * `tui-scrollbar`: 0.2.0 -> 0.2.1 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.2.1] - 2026-01-05
  >
  > ### 🚀 Features
  >
  > - *(scrollbar)* Update glyph previews and tests
  > ([#169](https://github.com/joshka/tui-widgets/issues/169))
  >   > Default ScrollBar renders without arrow endcaps and uses a dark gray
  >   > background with a blank (space) track.
  >   >
  > > Add glyph set variants and improve the Unicode-only fallback. Document
  >   > glyph sets with a 1/8-step horizontal thumb walk, and add snapshot
  >   > render tests to keep the glyph combinations stable.
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- *(tui-scrollbar)* Release v0.2.2 ([#173](https://github.com/ratatui/tui-widgets/issues/173))
  > ## 🤖 New release
  >
  > * `tui-scrollbar`: 0.2.1 -> 0.2.2 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.2.2] - 2026-01-05
  >
  > ### 🚀 Features
  >
  > - *(scrollbar)* Support crossterm 0.28
  > ([#172](https://github.com/joshka/tui-widgets/issues/172))
  > > Add versioned crossterm feature flags and re-export the selected
  > version
  >   > as `tui_scrollbar::crossterm`.
  >   >
  >   > Add CI checks for the feature matrix and a docs.rs-style build.
  >   >
  >   > ---------
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- *(project)* Update the repository link

- *(ci)* Check for the ratatui organization for releases

- Bump MSRV to 1.88.0 ([#192](https://github.com/ratatui/tui-widgets/issues/192))
  > ```
  > error: rustc 1.87.0 is not supported by the following packages:
  >   darling@0.23.0 requires rustc 1.88.0
  >   darling_core@0.23.0 requires rustc 1.88.0
  >   darling_macro@0.23.0 requires rustc 1.88.0
  >   instability@0.3.11 requires rustc 1.88
  >   instability@0.3.11 requires rustc 1.88
  >   instability@0.3.11 requires rustc 1.88
  >   time@0.3.46 requires rustc 1.88.0
  >   time-core@0.1.8 requires rustc 1.88.0
  > ```

- *(tui-big-text)* Release v0.8.2 ([#176](https://github.com/ratatui/tui-widgets/issues/176))
  > ## 🤖 New release
  >
  > * `tui-big-text`: 0.8.1 -> 0.8.2 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.8.2] - 2026-02-01
  >
  > ### 🚀 Features
  >
  > - *(tui-bigtext)* Enable no_std as default
  > ([#190](https://github.com/ratatui/tui-widgets/issues/190))
  >   > This enables using `tui-bigtext` in a no_std environment
  >   >
  >   > e.g. https://github.com/ratatui/mousefood/pull/159
  >
  > ### ⚙️ Miscellaneous Tasks
  >
  > - *(project)* Update the repository link
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

### 🛡️ Security

- *(deps)* Bump tokio from 1.48.0 to 1.49.0 ([#175](https://github.com/ratatui/tui-widgets/issues/175))
  > Bumps [tokio](https://github.com/tokio-rs/tokio) from 1.48.0 to 1.49.0.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/tokio-rs/tokio/releases">tokio's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>Tokio v1.49.0</h2>
  > <h1>1.49.0 (January 3rd, 2026)</h1>
  > <h3>Added</h3>
  > <ul>
  > <li>net: add support for <code>TCLASS</code> option on IPv6 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7781">#7781</a>)</li>
  > <li>runtime: stabilize <code>runtime::id::Id</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7125">#7125</a>)</li>
  > <li>task: implement <code>Extend</code> for <code>JoinSet</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7195">#7195</a>)</li>
  > <li>task: stabilize the <code>LocalSet::id()</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7776">#7776</a>)</li>
  > </ul>
  > <h3>Changed</h3>
  > <ul>
  > <li>net: deprecate <code>{TcpStream,TcpSocket}::set_linger</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7752">#7752</a>)</li>
  > </ul>
  > <h3>Fixed</h3>
  > <ul>
  > <li>macros: fix the hygiene issue of <code>join!</code> and
  > <code>try_join!</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7766">#7766</a>)</li>
  > <li>runtime: revert &quot;replace manual vtable definitions with
  > Wake&quot; (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7699">#7699</a>)</li>
  > <li>sync: return <code>TryRecvError::Disconnected</code> from
  > <code>Receiver::try_recv</code> after <code>Receiver::close</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7686">#7686</a>)</li>
  > <li>task: remove unnecessary trait bounds on the <code>Debug</code>
  > implementation (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7720">#7720</a>)</li>
  > </ul>
  > <h3>Unstable</h3>
  > <ul>
  > <li>fs: handle <code>EINTR</code> in <code>fs::write</code> for io-uring
  > (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7786">#7786</a>)</li>
  > <li>fs: support io-uring with <code>tokio::fs::read</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7696">#7696</a>)</li>
  > <li>runtime: disable io-uring on <code>EPERM</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7724">#7724</a>)</li>
  > <li>time: add alternative timer for better multicore scalability (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7467">#7467</a>)</li>
  > </ul>
  > <h3>Documented</h3>
  > <ul>
  > <li>docs: fix a typos in <code>bounded.rs</code> and
  > <code>park.rs</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7817">#7817</a>)</li>
  > <li>io: add <code>SyncIoBridge</code> cross-references to
  > <code>copy</code> and <code>copy_buf</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7798">#7798</a>)</li>
  > <li>io: doc that <code>AsyncWrite</code> does not inherit from
  > <code>std::io::Write</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7705">#7705</a>)</li>
  > <li>metrics: clarify that <code>num_alive_tasks</code> is not strongly
  > consistent (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7614">#7614</a>)</li>
  > <li>net: clarify the cancellation safety of the
  > <code>TcpStream::peek</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7305">#7305</a>)</li>
  > <li>net: clarify the drop behavior of <code>unix::OwnedWriteHalf</code>
  > (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7742">#7742</a>)</li>
  > <li>net: clarify the platform-dependent backlog in
  > <code>TcpSocket</code> docs (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7738">#7738</a>)</li>
  > <li>runtime: mention <code>LocalRuntime</code> in
  > <code>new_current_thread</code> docs (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7820">#7820</a>)</li>
  > <li>sync: add missing period to <code>mpsc::Sender::try_send</code> docs
  > (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7721">#7721</a>)</li>
  > <li>sync: clarify the cancellation safety of
  > <code>oneshot::Receiver</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7780">#7780</a>)</li>
  > <li>sync: improve the docs for the <code>errors</code> of mpsc (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7722">#7722</a>)</li>
  > <li>task: add example for <code>spawn_local</code> usage on local
  > runtime (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7689">#7689</a>)</li>
  > </ul>
  > <p><a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7125">#7125</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7125">tokio-rs/tokio#7125</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7195">#7195</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7195">tokio-rs/tokio#7195</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7305">#7305</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7305">tokio-rs/tokio#7305</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7467">#7467</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7467">tokio-rs/tokio#7467</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7614">#7614</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7614">tokio-rs/tokio#7614</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7686">#7686</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7686">tokio-rs/tokio#7686</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7689">#7689</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7689">tokio-rs/tokio#7689</a></p>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/e3b89bbefa7564e2eba2fb9f849ef7bf87d60fad"><code>e3b89bb</code></a>
  > chore: prepare Tokio v1.49.0 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7824">#7824</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/4f577b84e939c8d427d79fdc73919842d8735de2"><code>4f577b8</code></a>
  > Merge 'tokio-1.47.3' into 'master'</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/f320197693ee09e28f1fca0e55418081adcdfc25"><code>f320197</code></a>
  > chore: prepare Tokio v1.47.3 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7823">#7823</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/ea6b144cd1042d6841a7830b18f2df77c3db904b"><code>ea6b144</code></a>
  > ci: freeze rustc on nightly-2025-01-25 in <code>netlify.toml</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7652">#7652</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/264e703296bccd6783a438815d91055d4517099b"><code>264e703</code></a>
  > Merge <code>tokio-1.43.4</code> into <code>tokio-1.47.x</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7822">#7822</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/dfb0f00838ca1986dee04a54a6299d35b0a4072c"><code>dfb0f00</code></a>
  > chore: prepare Tokio v1.43.4 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7821">#7821</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/4a91f197b03dc335010fffcf0e0c14e1f4011b42"><code>4a91f19</code></a>
  > ci: fix wasm32-wasip1 tests (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7788">#7788</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/601c383ab6def5a6d2f95a434c95a97b65059628"><code>601c383</code></a>
  > ci: upgrade FreeBSD from 14.2 to 14.3 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7758">#7758</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/484cb52d8d21cb8156decbeba9569651fcc09d0d"><code>484cb52</code></a>
  > sync: return <code>TryRecvError::Disconnected</code> from
  > <code>Receiver::try_recv</code> after `Re...</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/16f20c34ed9bc11eb1e7cdec441ab844b198d2cd"><code>16f20c3</code></a>
  > rt: mention <code>LocalRuntime</code> in <code>new_current_thread</code>
  > docs (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7820">#7820</a>)</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/tokio-rs/tokio/compare/tokio-1.48.0...tokio-1.49.0">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=tokio&package-manager=cargo&previous-version=1.48.0&new-version=1.49.0)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump clap from 4.5.53 to 4.5.54 ([#174](https://github.com/ratatui/tui-widgets/issues/174))
  > Bumps [clap](https://github.com/clap-rs/clap) from 4.5.53 to 4.5.54.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/clap-rs/clap/releases">clap's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>v4.5.54</h2>
  > <h2>[4.5.54] - 2026-01-02</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(help)</em> Move <code>[default]</code> to its own paragraph
  > when <code>PossibleValue::help</code> is present in
  > <code>--help</code></li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/clap-rs/clap/blob/master/CHANGELOG.md">clap's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[4.5.54] - 2026-01-02</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(help)</em> Move <code>[default]</code> to its own paragraph
  > when <code>PossibleValue::help</code> is present in
  > <code>--help</code></li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/194c676f60b916506f94f70decdbf319af5d1ec6"><code>194c676</code></a>
  > chore: Release</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/44838f6606fa015140c65a2d35971c1e9b269e26"><code>44838f6</code></a>
  > docs: Update changelog</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/0f59d55ff6b132cd59cd252442ce47078494be07"><code>0f59d55</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/6027">#6027</a>
  > from Alpha1337k/master</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/e2aa2f07d1cd50412de51b51a7cc897e80e0b92f"><code>e2aa2f0</code></a>
  > Feat: Add catch-all on external subcommands for zsh</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/b9c0aee9f28c5ad72932225bd730260f9bbe1fc6"><code>b9c0aee</code></a>
  > Feat: Add external subcommands test to suite</li>
  > <li>See full diff in <a
  > href="https://github.com/clap-rs/clap/compare/clap_complete-v4.5.53...clap_complete-v4.5.54">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=clap&package-manager=cargo&previous-version=4.5.53&new-version=4.5.54)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.


## [0.7.0] - 2026-01-02

### 🚀 Features

- *(scrollbar)* Add tui-scrollbar crate ([#164](https://github.com/ratatui/tui-widgets/issues/164))
  > ## Summary
  >
  > Introduce `tui-scrollbar`, a new widget crate for Ratatui that renders
  > smooth, fractional scrollbars
  > with precise thumb feedback and stateless input handling. The crate
  > focuses on clear geometry via
  > `ScrollMetrics`, configurable glyph sets (including legacy computing
  > symbols), and ergonomic
  > examples for keyboard and mouse interaction.
  >
  > ![ScrollBar demo](https://vhs.charm.sh/vhs-7c6j0GG4Up47YEHK5XLKoR.gif)
  >
  > ## Why
  >
  > Ratatui’s built-in scrollbar favors full-cell glyphs and stateful use.
  > This crate prioritizes
  > fractional thumbs for more accurate feedback, exposes pure metrics for
  > testing/composition, and
  > keeps scroll state in the application for predictable input behavior.
  >
  > ## Docs and Examples
  >
  > The crate-level docs include a quick start, API map, and input-handling
  > guidance. Two examples
  > show the fractional glyph sweep and an interactive mouse/keyboard demo.
  >
  > ```rust
  > use ratatui_core::buffer::Buffer;
  > use ratatui_core::layout::Rect;
  > use ratatui_core::widgets::Widget;
  > use tui_scrollbar::{ScrollBar, ScrollBarArrows, ScrollLengths};
  >
  > let area = Rect::new(0, 0, 1, 6);
  > let lengths = ScrollLengths {
  >     content_len: 120,
  >     viewport_len: 30,
  > };
  > let scrollbar = ScrollBar::vertical(lengths)
  >     .arrows(ScrollBarArrows::Both)
  >     .offset(45);
  >
  > let mut buffer = Buffer::empty(area);
  > scrollbar.render(area, &mut buffer);
  > ```

- *(scrollbar)* Add styled defaults ([#168](https://github.com/ratatui/tui-widgets/issues/168))
  > Set default track/thumb/arrow styles, style the demos with a filled
  > track, and update arrow glyphs and demo links.
  >
  > ![ScrollBar demo](https://vhs.charm.sh/vhs-21HzyozMOar6SYjVDBrpOb.gif)

### ⚙️ Miscellaneous Tasks

- *(tui-scrollbar)* Release v0.2.0 ([#165](https://github.com/ratatui/tui-widgets/issues/165))
  > ## 🤖 New release
  >
  > * `tui-scrollbar`: 0.1.0 -> 0.2.0 (⚠ API breaking changes)
  >
  > ### ⚠ `tui-scrollbar` breaking changes
  >
  > ```text
  > --- failure inherent_method_missing: pub method removed or renamed ---


## [0.6.2] - 2025-12-27

### 📚 Documentation

- Refresh widget docs ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  > Standardize widget crate docs and README layouts.
  > Unify badges, links, and license references.
  > Add consistent usage sections and link style updates.

### ⚙️ Miscellaneous Tasks

- *(tui-box-text)* Release v0.3.1 ([#149](https://github.com/ratatui/tui-widgets/issues/149))
  > ## 🤖 New release
  >
  > * `tui-box-text`: 0.3.0 -> 0.3.1 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.3.1] - 2025-12-27
  >
  > ### 📚 Documentation
  >
  > - Refresh widget docs
  > ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  >   > Standardize widget crate docs and README layouts.
  >   > Unify badges, links, and license references.
  >   > Add consistent usage sections and link style updates.
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- *(tui-cards)* Release v0.3.1 ([#150](https://github.com/ratatui/tui-widgets/issues/150))
  > ## 🤖 New release
  >
  > * `tui-cards`: 0.3.0 -> 0.3.1 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.3.1] - 2025-12-27
  >
  > ### 📚 Documentation
  >
  > - Refresh widget docs
  > ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  >   > Standardize widget crate docs and README layouts.
  >   > Unify badges, links, and license references.
  >   > Add consistent usage sections and link style updates.
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- *(tui-bar-graph)* Release v0.3.1 ([#151](https://github.com/ratatui/tui-widgets/issues/151))
  > ## 🤖 New release
  >
  > * `tui-bar-graph`: 0.3.0 -> 0.3.1 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.3.1] - 2025-12-27
  >
  > ### 📚 Documentation
  >
  > - Refresh widget docs
  > ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  >   > Standardize widget crate docs and README layouts.
  >   > Unify badges, links, and license references.
  >   > Add consistent usage sections and link style updates.
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- *(tui-qrcode)* Release v0.2.2 ([#152](https://github.com/ratatui/tui-widgets/issues/152))
  > ## 🤖 New release
  >
  > * `tui-qrcode`: 0.2.1 -> 0.2.2 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.2.2] - 2025-12-27
  >
  > ### 📚 Documentation
  >
  > - Refresh widget docs
  > ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  >   > Standardize widget crate docs and README layouts.
  >   > Unify badges, links, and license references.
  >   > Add consistent usage sections and link style updates.
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- *(tui-popup)* Release v0.7.2 ([#155](https://github.com/ratatui/tui-widgets/issues/155))
  > ## 🤖 New release
  >
  > * `tui-popup`: 0.7.1 -> 0.7.2 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.7.2] - 2025-12-27
  >
  > ### 📚 Documentation
  >
  > - Refresh widget docs
  > ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  >   > Standardize widget crate docs and README layouts.
  >   > Unify badges, links, and license references.
  >   > Add consistent usage sections and link style updates.
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- *(tui-big-text)* Release v0.8.1 ([#154](https://github.com/ratatui/tui-widgets/issues/154))
  > ## 🤖 New release
  >
  > * `tui-big-text`: 0.8.0 -> 0.8.1 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.8.1] - 2025-12-27
  >
  > ### 📚 Documentation
  >
  > - Refresh widget docs
  > ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  >   > Standardize widget crate docs and README layouts.
  >   > Unify badges, links, and license references.
  >   > Add consistent usage sections and link style updates.
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- *(tui-scrollview)* Release v0.6.2 ([#156](https://github.com/ratatui/tui-widgets/issues/156))
  > ## 🤖 New release
  >
  > * `tui-scrollview`: 0.6.1 -> 0.6.2 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.6.2] - 2025-12-27
  >
  > ### 📚 Documentation
  >
  > - Refresh widget docs
  > ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  >   > Standardize widget crate docs and README layouts.
  >   > Unify badges, links, and license references.
  >   > Add consistent usage sections and link style updates.
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- *(tui-prompts)* Release v0.6.1 ([#161](https://github.com/ratatui/tui-widgets/issues/161))
  > ## 🤖 New release
  >
  > * `tui-prompts`: 0.6.0 -> 0.6.1 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.6.1] - 2025-12-27
  >
  > ### 📚 Documentation
  >
  > - Refresh widget docs
  > ([#148](https://github.com/ratatui/tui-widgets/issues/148))
  >   > Standardize widget crate docs and README layouts.
  >   > Unify badges, links, and license references.
  >   > Add consistent usage sections and link style updates.
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).


## [0.6.1] - 2025-12-27

### ⚙️ Miscellaneous Tasks

- Refresh readmes and rdme check ([#140](https://github.com/ratatui/tui-widgets/issues/140))
  > Regenerate crate READMEs via cargo-rdme and add a CI check to keep
  > workspace readmes in sync.

- *(tui-qrcode)* Release v0.2.1 ([#141](https://github.com/ratatui/tui-widgets/issues/141))
  > ## 🤖 New release
  >
  > * `tui-qrcode`: 0.2.0 -> 0.2.1 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.2.1] - 2025-12-27
  >
  > ### ⚙️ Miscellaneous Tasks
  >
  > - Refresh readmes and rdme check
  > ([#140](https://github.com/ratatui/tui-widgets/issues/140))
  >   > Regenerate crate READMEs via cargo-rdme and add a CI check to keep
  >   > workspace readmes in sync.
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).


## [0.6.0] - 2025-12-27

### 🚀 Features

- *(tui-bar-graph)* [**breaking**] Add block octant characters ([#116](https://github.com/ratatui/tui-widgets/issues/116))
  > Since Unicode 16.0 was published on September 10, 2024, support for
  > block octant characters (U+1CD00 to U+1CDE5,
  > [PDF](https://www.unicode.org/charts/PDF/Unicode-16.0/U160-1CC00.pdf))
  > has been improving in fonts. We should enable users of `tui-widgets` to
  > use these characters in addition to existing options.

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

- Add AGENTS.md repository guidelines ([#108](https://github.com/ratatui/tui-widgets/issues/108))

- *(tui-big-text)* Fix enum name in field details ([#119](https://github.com/ratatui/tui-widgets/issues/119))

### 🎨 Styling

- *(tui-prompts)* Apply changes from cargo fmt ([#118](https://github.com/ratatui/tui-widgets/issues/118))

### ⚙️ Miscellaneous Tasks

- Enable trusted publishing via release-plz ([#110](https://github.com/ratatui/tui-widgets/issues/110))

- Run release-plz per package ([#121](https://github.com/ratatui/tui-widgets/issues/121))
  > - Run release-plz release-pr in a per-crate matrix
  > - Keep release job as a single workspace publish

- *(tui-box-text)* Release v0.3.0 ([#106](https://github.com/ratatui/tui-widgets/issues/106))
  > ## 🤖 New release
  >
  > * `tui-box-text`: 0.2.2 -> 0.3.0 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.3.0] - 2025-12-27
  >
  > ### 🚀 Features
  >
  > - [**breaking**] Migrate to ratatui 0.30
  > ([#120](https://github.com/ratatui/tui-widgets/issues/120))
  >   > feat!: migrate to ratatui 0.30
  >   >
  > > - Update workspace deps to ratatui 0.30, ratatui-core,
  > ratatui-widgets,
  >   > crossterm 0.29
  >   > - Shift widget crates to ratatui-core/ratatui-widgets imports where
  >   > needed
  >   > - Update tui-popup/tui-prompts event handling to use crossterm types
  > > - Revise tui-popup rendering/ref semantics and docs to match reference
  >   > rendering rules
  >   > - Add rolling breaking changes doc and markdownlint config
  >   > - Bump direct deps needed for minimal-versions and examples
  >   > (document-features, colorgrad, unicode-width)
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- Fix release-plz matrix concurrency ([#123](https://github.com/ratatui/tui-widgets/issues/123))
  > Avoid matrix jobs canceling each other by including the package name in
  > the concurrency group.

- *(tui-big-text)* Release v0.8.0 ([#122](https://github.com/ratatui/tui-widgets/issues/122))
  > ## 🤖 New release
  >
  > * `tui-big-text`: 0.7.3 -> 0.8.0 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.8.0] - 2025-12-27
  >
  > ### 🚀 Features
  >
  > - *(tui-big-text)* [**breaking**] Add block octant characters
  > ([#117](https://github.com/ratatui/tui-widgets/issues/117))
  >   > The changes in this PR add the following enum variants:
  >   >
  >   > * `PixelSize::QuarterHeight` and `PixelSize::Octant` to `PixelSize`
  >   > found in the `tui-big-text` crate.
  >   >
  > > Documentation and tests are included, and the examples (along with the
  >   > VHS tapes and screenshots) have been updated to show the new
  >   > **two-row-tall text** styles in action and how they compare to text
  >   > rendered with other `PixelSize` settings.
  >
  > - [**breaking**] Migrate to ratatui 0.30
  > ([#120](https://github.com/ratatui/tui-widgets/issues/120))
  >   > feat!: migrate to ratatui 0.30
  >   >
  > > - Update workspace deps to ratatui 0.30, ratatui-core,
  > ratatui-widgets,
  >   > crossterm 0.29
  >   > - Shift widget crates to ratatui-core/ratatui-widgets imports where
  >   > needed
  >   > - Update tui-popup/tui-prompts event handling to use crossterm types
  > > - Revise tui-popup rendering/ref semantics and docs to match reference
  >   > rendering rules
  >   > - Add rolling breaking changes doc and markdownlint config
  >   > - Bump direct deps needed for minimal-versions and examples
  >   > (document-features, colorgrad, unicode-width)
  >
  > ### 📚 Documentation
  >
  > - *(tui-big-text)* Fix enum name in field details
  > ([#119](https://github.com/ratatui/tui-widgets/issues/119))
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- Separate release-pr branches ([#126](https://github.com/ratatui/tui-widgets/issues/126))
  > Configure per-package release-plz branch prefixes so matrix jobs target
  > distinct PR branches instead of clobbering one.

- *(tui-cards)* Release v0.3.0 ([#125](https://github.com/ratatui/tui-widgets/issues/125))
  > ## 🤖 New release
  >
  > * `tui-cards`: 0.2.4 -> 0.3.0 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.3.0] - 2025-12-27
  >
  > ### 🚀 Features
  >
  > - [**breaking**] Migrate to ratatui 0.30
  > ([#120](https://github.com/ratatui/tui-widgets/issues/120))
  >   > feat!: migrate to ratatui 0.30
  >   >
  > > - Update workspace deps to ratatui 0.30, ratatui-core,
  > ratatui-widgets,
  >   > crossterm 0.29
  >   > - Shift widget crates to ratatui-core/ratatui-widgets imports where
  >   > needed
  >   > - Update tui-popup/tui-prompts event handling to use crossterm types
  > > - Revise tui-popup rendering/ref semantics and docs to match reference
  >   > rendering rules
  >   > - Add rolling breaking changes doc and markdownlint config
  >   > - Bump direct deps needed for minimal-versions and examples
  >   > (document-features, colorgrad, unicode-width)
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- Template per-package release-pr config ([#127](https://github.com/ratatui/tui-widgets/issues/127))
  > Generate a per-package release-plz config in CI so each matrix job uses
  > a unique PR branch prefix without unsupported package settings.

- Add per-package release-plz configs ([#128](https://github.com/ratatui/tui-widgets/issues/128))
  > Commit per-package release-plz config files and point the workflow at
  > them so each matrix job uses a unique PR branch prefix.

- *(tui-popup)* Release v0.7.0 ([#129](https://github.com/ratatui/tui-widgets/issues/129))
  > ## 🤖 New release
  >
  > * `tui-popup`: 0.6.2 -> 0.7.0 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.7.0] - 2025-12-27
  >
  > ### 🚀 Features
  >
  > - [**breaking**] Migrate to ratatui 0.30
  > ([#120](https://github.com/ratatui/tui-widgets/issues/120))
  >   > feat!: migrate to ratatui 0.30
  >   >
  > > - Update workspace deps to ratatui 0.30, ratatui-core,
  > ratatui-widgets,
  >   > crossterm 0.29
  >   > - Shift widget crates to ratatui-core/ratatui-widgets imports where
  >   > needed
  >   > - Update tui-popup/tui-prompts event handling to use crossterm types
  > > - Revise tui-popup rendering/ref semantics and docs to match reference
  >   > rendering rules
  >   > - Add rolling breaking changes doc and markdownlint config
  >   > - Bump direct deps needed for minimal-versions and examples
  >   > (document-features, colorgrad, unicode-width)
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- *(tui-scrollview)* Release v0.6.0 ([#132](https://github.com/ratatui/tui-widgets/issues/132))
  > ## 🤖 New release
  >
  > * `tui-scrollview`: 0.5.3 -> 0.6.0 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.6.0] - 2025-12-27
  >
  > ### 🚀 Features
  >
  > - [**breaking**] Migrate to ratatui 0.30
  > ([#120](https://github.com/ratatui/tui-widgets/issues/120))
  >   > feat!: migrate to ratatui 0.30
  >   >
  > > - Update workspace deps to ratatui 0.30, ratatui-core,
  > ratatui-widgets,
  >   > crossterm 0.29
  >   > - Shift widget crates to ratatui-core/ratatui-widgets imports where
  >   > needed
  >   > - Update tui-popup/tui-prompts event handling to use crossterm types
  > > - Revise tui-popup rendering/ref semantics and docs to match reference
  >   > rendering rules
  >   > - Add rolling breaking changes doc and markdownlint config
  >   > - Bump direct deps needed for minimal-versions and examples
  >   > (document-features, colorgrad, unicode-width)
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- *(tui-bar-graph)* Release v0.3.0 ([#131](https://github.com/ratatui/tui-widgets/issues/131))
  > ## 🤖 New release
  >
  > * `tui-bar-graph`: 0.2.0 -> 0.3.0 (⚠ API breaking changes)
  >
  > ### ⚠ `tui-bar-graph` breaking changes
  >
  > ```text
  > --- failure enum_variant_added: enum variant added on exhaustive enum ---

- *(tui-prompts)* Release v0.6.0 ([#130](https://github.com/ratatui/tui-widgets/issues/130))
  > ## 🤖 New release
  >
  > * `tui-prompts`: 0.5.2 -> 0.6.0 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.6.0] - 2025-12-27
  >
  > ### 🚀 Features
  >
  > - [**breaking**] Migrate to ratatui 0.30
  > ([#120](https://github.com/ratatui/tui-widgets/issues/120))
  >   > feat!: migrate to ratatui 0.30
  >   >
  > > - Update workspace deps to ratatui 0.30, ratatui-core,
  > ratatui-widgets,
  >   > crossterm 0.29
  >   > - Shift widget crates to ratatui-core/ratatui-widgets imports where
  >   > needed
  >   > - Update tui-popup/tui-prompts event handling to use crossterm types
  > > - Revise tui-popup rendering/ref semantics and docs to match reference
  >   > rendering rules
  >   > - Add rolling breaking changes doc and markdownlint config
  >   > - Bump direct deps needed for minimal-versions and examples
  >   > (document-features, colorgrad, unicode-width)
  >
  > ### 🎨 Styling
  >
  > - *(tui-prompts)* Apply changes from cargo fmt
  > ([#118](https://github.com/ratatui/tui-widgets/issues/118))
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

- *(tui-qrcode)* Release v0.2.0 ([#133](https://github.com/ratatui/tui-widgets/issues/133))
  > ## 🤖 New release
  >
  > * `tui-qrcode`: 0.1.3 -> 0.2.0 (✓ API compatible changes)
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > <blockquote>
  >
  > ## [0.2.0] - 2025-12-27
  >
  > ### 🚀 Features
  >
  > - [**breaking**] Migrate to ratatui 0.30
  > ([#120](https://github.com/ratatui/tui-widgets/issues/120))
  >   > feat!: migrate to ratatui 0.30
  >   >
  > > - Update workspace deps to ratatui 0.30, ratatui-core,
  > ratatui-widgets,
  >   > crossterm 0.29
  >   > - Shift widget crates to ratatui-core/ratatui-widgets imports where
  >   > needed
  >   > - Update tui-popup/tui-prompts event handling to use crossterm types
  > > - Revise tui-popup rendering/ref semantics and docs to match reference
  >   > rendering rules
  >   > - Add rolling breaking changes doc and markdownlint config
  >   > - Bump direct deps needed for minimal-versions and examples
  >   > (document-features, colorgrad, unicode-width)
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/release-plz/release-plz/).

### 🛡️ Security

- *(deps)* Bump indoc from 2.0.6 to 2.0.7 ([#115](https://github.com/ratatui/tui-widgets/issues/115))
  > Bumps [indoc](https://github.com/dtolnay/indoc) from 2.0.6 to 2.0.7.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/dtolnay/indoc/releases">indoc's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>2.0.7</h2>
  > <ul>
  > <li>Support C-string literals <code>indoc! {c&quot;...&quot;}</code>,
  > <code>indoc! {cr&quot;...&quot;}</code> (<a
  > href="https://redirect.github.com/dtolnay/indoc/issues/67">#67</a>)</li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/dtolnay/indoc/commit/8d78216b3f127f523d198475ea44090f8f6894d5"><code>8d78216</code></a>
  > Release 2.0.7</li>
  > <li><a
  > href="https://github.com/dtolnay/indoc/commit/23472ff7f3d2523ea1f5b396c7ea135c02054ee2"><code>23472ff</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/dtolnay/indoc/issues/67">#67</a> from
  > dtolnay/cstring</li>
  > <li><a
  > href="https://github.com/dtolnay/indoc/commit/8d05562cbe8fe71e15afe7e6da602c1265217fd7"><code>8d05562</code></a>
  > Hide C-string tests from old toolchain versions</li>
  > <li><a
  > href="https://github.com/dtolnay/indoc/commit/7c92efb7180eeabde698c2db22c24d189f07ab31"><code>7c92efb</code></a>
  > Recognize C-string literals</li>
  > <li><a
  > href="https://github.com/dtolnay/indoc/commit/fe39de460f2e30f3eaeea0891aec5cf412c65d72"><code>fe39de4</code></a>
  > Generalize Error constructors</li>
  > <li><a
  > href="https://github.com/dtolnay/indoc/commit/27e015160e5aa8da3ce33af7ca7da2e0f2c13869"><code>27e0151</code></a>
  > Add C-string tests</li>
  > <li><a
  > href="https://github.com/dtolnay/indoc/commit/57f6fbb4dab9277638bd4cbf358b31dab3a4512c"><code>57f6fbb</code></a>
  > Sort tests</li>
  > <li><a
  > href="https://github.com/dtolnay/indoc/commit/170a0795587a3010785e05ee2240c98f49b02bf2"><code>170a079</code></a>
  > Raise minimum tested compiler to rust 1.76</li>
  > <li><a
  > href="https://github.com/dtolnay/indoc/commit/2f6ef0452d0495c1a3abde4293934d293d9c2c5d"><code>2f6ef04</code></a>
  > Opt in to generate-macro-expansion when building on docs.rs</li>
  > <li><a
  > href="https://github.com/dtolnay/indoc/commit/ce1bed41bb48d6071e2a15bf8dae8d801c500b92"><code>ce1bed4</code></a>
  > Update ui test suite to nightly-2025-09-12</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/dtolnay/indoc/compare/2.0.6...2.0.7">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=indoc&package-manager=cargo&previous-version=2.0.6&new-version=2.0.7)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump rstest from 0.25.0 to 0.26.1 ([#114](https://github.com/ratatui/tui-widgets/issues/114))
  > Bumps [rstest](https://github.com/la10736/rstest) from 0.25.0 to 0.26.1.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/la10736/rstest/releases">rstest's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>0.26.1</h2>
  > <p>Fix Docs</p>
  > <p><strong>Full Changelog</strong>: <a
  > href="https://github.com/la10736/rstest/compare/v0.26.0...v0.26.1">https://github.com/la10736/rstest/compare/v0.26.0...v0.26.1</a></p>
  > <h2>0.26.0</h2>
  > <h2>What's Changed</h2>
  > <ul>
  > <li>docs: fix filemode examples by <a
  > href="https://github.com/lucascool12"><code>@​lucascool12</code></a> in
  > <a
  > href="https://redirect.github.com/la10736/rstest/pull/301">la10736/rstest#301</a></li>
  > <li>Issue <a
  > href="https://redirect.github.com/la10736/rstest/issues/306">#306</a>.
  > Ignore folders by <a
  > href="https://github.com/Obito-git"><code>@​Obito-git</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/307">la10736/rstest#307</a></li>
  > <li>Hide generated items in documentation by <a
  > href="https://github.com/wiktor-k"><code>@​wiktor-k</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/309">la10736/rstest#309</a></li>
  > <li>313_fix by <a
  > href="https://github.com/la10736"><code>@​la10736</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/314">la10736/rstest#314</a></li>
  > <li>fix: do not depend by default on <code>async-std</code> by <a
  > href="https://github.com/coriolinus"><code>@​coriolinus</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/311">la10736/rstest#311</a></li>
  > <li>Add permission for empty_structs_with_brackets in fixture by <a
  > href="https://github.com/bugRanger"><code>@​bugRanger</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/317">la10736/rstest#317</a></li>
  > <li>Touch up indentation used for examples in the README by <a
  > href="https://github.com/fgimian"><code>@​fgimian</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/318">la10736/rstest#318</a></li>
  > <li>Make #[files(...)] work on Windows by <a
  > href="https://github.com/twz123"><code>@​twz123</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/322">la10736/rstest#322</a></li>
  > <li>Finalize <a
  > href="https://redirect.github.com/la10736/rstest/issues/311">#311</a> by
  > <a href="https://github.com/la10736"><code>@​la10736</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/323">la10736/rstest#323</a></li>
  > <li>Make clippy happy by <a
  > href="https://github.com/la10736"><code>@​la10736</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/324">la10736/rstest#324</a></li>
  > </ul>
  > <h2>New Contributors</h2>
  > <ul>
  > <li><a href="https://github.com/Obito-git"><code>@​Obito-git</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/la10736/rstest/pull/307">la10736/rstest#307</a></li>
  > <li><a href="https://github.com/wiktor-k"><code>@​wiktor-k</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/la10736/rstest/pull/309">la10736/rstest#309</a></li>
  > <li><a
  > href="https://github.com/coriolinus"><code>@​coriolinus</code></a> made
  > their first contribution in <a
  > href="https://redirect.github.com/la10736/rstest/pull/311">la10736/rstest#311</a></li>
  > <li><a href="https://github.com/bugRanger"><code>@​bugRanger</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/la10736/rstest/pull/317">la10736/rstest#317</a></li>
  > <li><a href="https://github.com/fgimian"><code>@​fgimian</code></a> made
  > their first contribution in <a
  > href="https://redirect.github.com/la10736/rstest/pull/318">la10736/rstest#318</a></li>
  > <li><a href="https://github.com/twz123"><code>@​twz123</code></a> made
  > their first contribution in <a
  > href="https://redirect.github.com/la10736/rstest/pull/322">la10736/rstest#322</a></li>
  > </ul>
  > <p><strong>Full Changelog</strong>: <a
  > href="https://github.com/la10736/rstest/compare/v0.25.0...v0.26.0">https://github.com/la10736/rstest/compare/v0.25.0...v0.26.0</a></p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/la10736/rstest/blob/master/CHANGELOG.md">rstest's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[0.26.1] 2025/7/27</h2>
  > <h3>Fixed</h3>
  > <ul>
  > <li>Docs</li>
  > </ul>
  > <h2>[0.26.0] 2025/7/26</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>The <code>#[files(...)]</code> attribute now ignores matched
  > directory paths by default.
  > See <a
  > href="https://redirect.github.com/la10736/rstest/pull/306">#306</a>
  > thanks to <a
  > href="https://github.com/Obito-git"><code>@​Obito-git</code></a>.</li>
  > </ul>
  > <h3>Add</h3>
  > <ul>
  > <li>Introduced the <code>#[dirs]</code> attribute, which can be used
  > with <code>#[files(...)]</code> to explicitly include directory paths.
  > See <a
  > href="https://redirect.github.com/la10736/rstest/pull/306">#306</a>
  > thanks to <a
  > href="https://github.com/Obito-git"><code>@​Obito-git</code></a>.</li>
  > <li>The CI now runs builds and tests on Windows, as well.</li>
  > <li><code>#[test_attr]</code> to define test attribute explicit and also
  > enable the use of
  > <code>#[macro_rules_attribute::apply(&lt;macro&gt;)]</code>: naw also
  > <code>smol</code> works. See
  > <a href="https://redirect.github.com/la10736/rstest/pull/303">#303</a>
  > <a href="https://redirect.github.com/la10736/rstest/pull/311">#311</a>
  > <a href="https://redirect.github.com/la10736/rstest/pull/315">#315</a>
  > thanks to <a
  > href="https://github.com/coriolinus"><code>@​coriolinus</code></a>.</li>
  > </ul>
  > <h3>Fixed</h3>
  > <ul>
  > <li>Removed unsued trait and impl spotted out on
  > <code>1.89.0-nightly</code></li>
  > <li>Add missed tests about ignore attribute's args in
  > <code>rstest</code> expansion.
  > See <a
  > href="https://redirect.github.com/la10736/rstest/pull/313">#313</a></li>
  > <li>The <code>#[files(...)]</code> attribute now works reliably on
  > Windows.</li>
  > <li>Now global attributes can go everywhere in the list also where case
  > is used</li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/la10736/rstest/commit/7a326c87e882d2da1f3f97c40e6b04757f085679"><code>7a326c8</code></a>
  > Should rollback version used to test</li>
  > <li><a
  > href="https://github.com/la10736/rstest/commit/a16a8025817ba001853687879ce95729b5f4a487"><code>a16a802</code></a>
  > Should rollback version used to test</li>
  > <li><a
  > href="https://github.com/la10736/rstest/commit/98e886756fa687cd807380c347debda1f2b5422b"><code>98e8867</code></a>
  > Merge remote-tracking branch 'origin/master'</li>
  > <li><a
  > href="https://github.com/la10736/rstest/commit/f4447880ce1ab1468430fbbd41313e2079008b5b"><code>f444788</code></a>
  > Bump version 0.26.1</li>
  > <li><a
  > href="https://github.com/la10736/rstest/commit/5eab9ac46128e23c366929c5e9180e19b3380140"><code>5eab9ac</code></a>
  > Bump version 0.26.1</li>
  > <li><a
  > href="https://github.com/la10736/rstest/commit/971f6ad05232b1fc3ca5a7b0e2830d476d683307"><code>971f6ad</code></a>
  > Bump version 0.26.1</li>
  > <li><a
  > href="https://github.com/la10736/rstest/commit/edfdd89b3ecd8f5d2172308b3e10bcf831db4772"><code>edfdd89</code></a>
  > Fixed docs and readme</li>
  > <li><a
  > href="https://github.com/la10736/rstest/commit/ab24b5bc03e93e6dd7c334db1c21e6a8249f4ccd"><code>ab24b5b</code></a>
  > Bump version 0.26.0-dev</li>
  > <li><a
  > href="https://github.com/la10736/rstest/commit/e18375bfd4c639ab88f0a7a8b0f47149c347b5c3"><code>e18375b</code></a>
  > Bump Version 0.26.0</li>
  > <li><a
  > href="https://github.com/la10736/rstest/commit/dcea54f01a8a8b498f4c7d47e6dfc29fa3f282cf"><code>dcea54f</code></a>
  > Make clippy happy</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/la10736/rstest/compare/v0.25.0...v0.26.1">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=rstest&package-manager=cargo&previous-version=0.25.0&new-version=0.26.1)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump document-features from 0.2.11 to 0.2.12 ([#113](https://github.com/ratatui/tui-widgets/issues/113))
  > Bumps [document-features](https://github.com/slint-ui/document-features)
  > from 0.2.11 to 0.2.12.
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/slint-ui/document-features/blob/master/CHANGELOG.md">document-features's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>0.2.12 - 2025-10-24</h2>
  > <ul>
  > <li>Update litrs dependency to 1.0.0</li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/slint-ui/document-features/commit/721e708012254b81760bd2befdae0970e7a2615e"><code>721e708</code></a>
  > Prepare for release</li>
  > <li><a
  > href="https://github.com/slint-ui/document-features/commit/21ec1eb99cb96ced6892a5e41c5ffbd683f6f1e6"><code>21ec1eb</code></a>
  > Update MSRV</li>
  > <li><a
  > href="https://github.com/slint-ui/document-features/commit/1c70d0aa83ee9da30be3e3c91a3b89b881aa3f0b"><code>1c70d0a</code></a>
  > chore: fix wrong test function name (<a
  > href="https://redirect.github.com/slint-ui/document-features/issues/35">#35</a>)</li>
  > <li><a
  > href="https://github.com/slint-ui/document-features/commit/21cdfeccbcfcba8ec61b9d56efce082c214b8c41"><code>21cdfec</code></a>
  > Update litrs dependency to v1.0</li>
  > <li><a
  > href="https://github.com/slint-ui/document-features/commit/bb0dea8b10c642978c1f797c6942dcc72cfd5eac"><code>bb0dea8</code></a>
  > Fix typo in CHANGELOG.md</li>
  > <li>See full diff in <a
  > href="https://github.com/slint-ui/document-features/compare/v0.2.11...v0.2.12">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=document-features&package-manager=cargo&previous-version=0.2.11&new-version=0.2.12)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump clap from 4.5.48 to 4.5.53 ([#111](https://github.com/ratatui/tui-widgets/issues/111))
  > Bumps [clap](https://github.com/clap-rs/clap) from 4.5.48 to 4.5.53.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/clap-rs/clap/releases">clap's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>v4.5.53</h2>
  > <h2>[4.5.53] - 2025-11-19</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Add <code>default_values_if</code>,
  > <code>default_values_ifs</code></li>
  > </ul>
  > <h2>v4.5.52</h2>
  > <h2>[4.5.52] - 2025-11-17</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li>Don't panic when <code>args_conflicts_with_subcommands</code>
  > conflicts with an <code>ArgGroup</code></li>
  > </ul>
  > <h2>v4.5.51</h2>
  > <h2>[4.5.51] - 2025-10-29</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(help)</em> Correctly calculate padding for short flags that
  > take a value</li>
  > <li><em>(help)</em> Don't panic on short flags using
  > <code>ArgAction::Count</code></li>
  > </ul>
  > <h2>v4.5.50</h2>
  > <h2>[4.5.50] - 2025-10-20</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Accept <code>Cow</code> where <code>String</code> and
  > <code>&amp;str</code> are accepted</li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/clap-rs/clap/blob/master/CHANGELOG.md">clap's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[4.5.53] - 2025-11-19</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Add <code>default_values_if</code>,
  > <code>default_values_ifs</code></li>
  > </ul>
  > <h2>[4.5.52] - 2025-11-17</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li>Don't panic when <code>args_conflicts_with_subcommands</code>
  > conflicts with an <code>ArgGroup</code></li>
  > </ul>
  > <h2>[4.5.51] - 2025-10-29</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(help)</em> Correctly calculate padding for short flags that
  > take a value</li>
  > <li><em>(help)</em> Don't panic on short flags using
  > <code>ArgAction::Count</code></li>
  > </ul>
  > <h2>[4.5.50] - 2025-10-20</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Accept <code>Cow</code> where <code>String</code> and
  > <code>&amp;str</code> are accepted</li>
  > </ul>
  > <h2>[4.5.49] - 2025-10-13</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(help)</em> Correctly wrap when ANSI escape codes are
  > present</li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/3716f9f4289594b43abec42b2538efd1a90ff897"><code>3716f9f</code></a>
  > chore: Release</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/613b69a6b7bff729b7a363fa0c91fd03f48d12c3"><code>613b69a</code></a>
  > docs: Update changelog</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/d117f7acdeedebaf5fd7847debb15c834423f159"><code>d117f7a</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/6028">#6028</a>
  > from epage/arg</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/cb8255d2f3c7f12ebf07ec1c55ac98b6848599a9"><code>cb8255d</code></a>
  > feat(builder): Allow quoted id's for arg macro</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/1036060f1319412d3d50d821a7b39a0a0122f0f7"><code>1036060</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/6025">#6025</a>
  > from AldaronLau/typos-in-faq</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/2fcafc0aee6380e1f0c44a3e927cef1bfc88930e"><code>2fcafc0</code></a>
  > docs: Fix minor grammar issues in FAQ</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/a380b65fe9eceade90bce8aeb13c205265fcceee"><code>a380b65</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/6023">#6023</a>
  > from epage/template</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/4d7ab1483cd0f0849668d274aa2fb6358872eca9"><code>4d7ab14</code></a>
  > chore: Update from _rust/main template</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/b8a7ea49d973a35bb6b3f43506b8319f340a20a4"><code>b8a7ea4</code></a>
  > chore(deps): Update Rust Stable to v1.87 (<a
  > href="https://redirect.github.com/clap-rs/clap/issues/18">#18</a>)</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/f9842b3b3f920ef64c5fc06298b4762018d88809"><code>f9842b3</code></a>
  > chore: Avoid MSRV problems out of the box</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/clap-rs/clap/compare/clap_complete-v4.5.48...clap_complete-v4.5.53">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=clap&package-manager=cargo&previous-version=4.5.48&new-version=4.5.53)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

### Other

- *(deps)* Bump tokio from 1.47.1 to 1.48.0 ([#112](https://github.com/ratatui/tui-widgets/issues/112))


## [0.5.0] - 2025-11-02

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

- Calculate area of QRCodeWidget ([#68](https://github.com/ratatui/tui-widgets/issues/68))

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

- More clippy lints ([#84](https://github.com/ratatui/tui-widgets/issues/84))

- *(tui-prompts)* Full-width character input in non-multiline prompt ([#93](https://github.com/ratatui/tui-widgets/issues/93)) ([#94](https://github.com/ratatui/tui-widgets/issues/94))

### 🚜 Refactor

- Simplify BarGraph rendering logic

- Simplify color / gradient handling logic

### 🎨 Styling

- Format doc comments

- Add rustfmt and reformat code

### ⚙️ Miscellaneous Tasks

- Remove needless lifetimes ([#60](https://github.com/ratatui/tui-widgets/issues/60))

- Fix git-cliff config ([#61](https://github.com/ratatui/tui-widgets/issues/61))

- Prepare tui-bar-graph 0.1.1

- Remove leftover github workflow files ([#73](https://github.com/ratatui/tui-widgets/issues/73))

- Commit cargo.lock file

- *(tui-big-text)* Support disabling crossterm ([#70](https://github.com/ratatui/tui-widgets/issues/70))

- Use semver compatible dependency versions ([#77](https://github.com/ratatui/tui-widgets/issues/77))
  > Use 0.x and x.y instead of 0.x.y and x.y.z for deps to reduce
  > incompatibilities

- *(deps)* Use less specific versions of color-eyre and clap ([#82](https://github.com/ratatui/tui-widgets/issues/82))

### 🛡️ Security

- *(deps)* Bump derive_setters from 0.1.7 to 0.1.8 ([#86](https://github.com/ratatui/tui-widgets/issues/86))
  > Bumps [derive_setters](https://github.com/Lymia/derive_setters) from
  > 0.1.7 to 0.1.8.
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li>See full diff in <a
  > href="https://github.com/Lymia/derive_setters/commits/v0.1.8">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=derive_setters&package-manager=cargo&previous-version=0.1.7&new-version=0.1.8)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump clap from 4.5.40 to 4.5.41 ([#87](https://github.com/ratatui/tui-widgets/issues/87))
  > Bumps [clap](https://github.com/clap-rs/clap) from 4.5.40 to 4.5.41.
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/clap-rs/clap/blob/master/CHANGELOG.md">clap's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[4.5.41] - 2025-07-09</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Add <code>Styles::context</code> and
  > <code>Styles::context_value</code> to customize the styling of
  > <code>[default: value]</code> like notes in the <code>--help</code></li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/92fcd83b7687a16005f91465ad64ca647929e76f"><code>92fcd83</code></a>
  > chore: Release</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/aca91b99c1f934c1f6b29924bb052e2c51854d05"><code>aca91b9</code></a>
  > docs: Update changelog</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/8434510cee78d9591277c187c128c6ca7db8acc1"><code>8434510</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5869">#5869</a>
  > from tw4452852/patch-1</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/33b1fc304ec6f551e0f2b082eafe1b6f44212179"><code>33b1fc3</code></a>
  > fix(complete): Fix env leakage in elvish dynamic completion</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/e5f1f4884c48fd472529baa253c6384929f2ac0d"><code>e5f1f48</code></a>
  > chore: Release</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/9466a552fbf938f7969245f5bac99c38ea446e9b"><code>9466a55</code></a>
  > docs: Update changelog</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/d74b79351212ad10eb89b9f842e678b8b2fdbee9"><code>d74b793</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5865">#5865</a>
  > from gifnksm/nushell-completion-value-types</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/ecbc775d3b4d8874786738fa4f342e6796446ff0"><code>ecbc775</code></a>
  > fix(nu): Set argument type based on <code>ValueHint</code></li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/6784054536a18549d90221ecd300084f02ca6386"><code>6784054</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5857">#5857</a>
  > from epage/empty</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/cca5f32b3a9dc0982fbc63e856a49ad3c7688b68"><code>cca5f32</code></a>
  > test(complete): Show empty option-value behavior</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/clap-rs/clap/compare/clap_complete-v4.5.40...clap_complete-v4.5.41">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=clap&package-manager=cargo&previous-version=4.5.40&new-version=4.5.41)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump strum from 0.27.1 to 0.27.2 ([#89](https://github.com/ratatui/tui-widgets/issues/89))
  > Bumps [strum](https://github.com/Peternator7/strum) from 0.27.1 to
  > 0.27.2.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/Peternator7/strum/releases">strum's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>v0.27.2</h2>
  > <h2>What's Changed</h2>
  > <ul>
  > <li>Adding support for doc comments on <code>EnumDiscriminants</code>
  > generated type… by <a
  > href="https://github.com/linclelinkpart5"><code>@​linclelinkpart5</code></a>
  > in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/141">Peternator7/strum#141</a></li>
  > <li>Drop needless <code>rustversion</code> dependency by <a
  > href="https://github.com/paolobarbolini"><code>@​paolobarbolini</code></a>
  > in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/446">Peternator7/strum#446</a></li>
  > <li>Upgrade <code>phf</code> to v0.12 by <a
  > href="https://github.com/paolobarbolini"><code>@​paolobarbolini</code></a>
  > in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/448">Peternator7/strum#448</a></li>
  > <li>allow discriminants on empty enum by <a
  > href="https://github.com/crop2000"><code>@​crop2000</code></a> in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/435">Peternator7/strum#435</a></li>
  > <li>Remove broken link to EnumTable docs by <a
  > href="https://github.com/schneems"><code>@​schneems</code></a> in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/427">Peternator7/strum#427</a></li>
  > <li>Change enum table callbacks to FnMut. by <a
  > href="https://github.com/ClaytonKnittel"><code>@​ClaytonKnittel</code></a>
  > in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/443">Peternator7/strum#443</a></li>
  > <li>Add <code>#[automatically_derived]</code> to the <code>impl</code>s
  > by <a
  > href="https://github.com/dandedotdev"><code>@​dandedotdev</code></a> in
  > <a
  > href="https://redirect.github.com/Peternator7/strum/pull/444">Peternator7/strum#444</a></li>
  > <li>Implement a <code>suffix</code> attribute for serialization of enum
  > variants by <a
  > href="https://github.com/amogh-dambal"><code>@​amogh-dambal</code></a>
  > in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/440">Peternator7/strum#440</a></li>
  > <li>Expound upon use_phf docs by <a
  > href="https://github.com/Peternator7"><code>@​Peternator7</code></a> in
  > <a
  > href="https://redirect.github.com/Peternator7/strum/pull/449">Peternator7/strum#449</a></li>
  > </ul>
  > <h2>New Contributors</h2>
  > <ul>
  > <li><a
  > href="https://github.com/paolobarbolini"><code>@​paolobarbolini</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/446">Peternator7/strum#446</a></li>
  > <li><a href="https://github.com/crop2000"><code>@​crop2000</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/435">Peternator7/strum#435</a></li>
  > <li><a href="https://github.com/schneems"><code>@​schneems</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/427">Peternator7/strum#427</a></li>
  > <li><a
  > href="https://github.com/ClaytonKnittel"><code>@​ClaytonKnittel</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/443">Peternator7/strum#443</a></li>
  > <li><a
  > href="https://github.com/dandedotdev"><code>@​dandedotdev</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/444">Peternator7/strum#444</a></li>
  > <li><a
  > href="https://github.com/amogh-dambal"><code>@​amogh-dambal</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/440">Peternator7/strum#440</a></li>
  > </ul>
  > <p><strong>Full Changelog</strong>: <a
  > href="https://github.com/Peternator7/strum/compare/v0.27.1...v0.27.2">https://github.com/Peternator7/strum/compare/v0.27.1...v0.27.2</a></p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/Peternator7/strum/blob/master/CHANGELOG.md">strum's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>0.27.2</h2>
  > <ul>
  > <li>
  > <p><a
  > href="https://redirect.github.com/Peternator7/strum/pull/141">#141</a>:
  > Adding support for doc comments on <code>EnumDiscriminants</code>
  > generated type.</p>
  > <ul>
  > <li>The doc comment will be copied from the variant on the type
  > itself.</li>
  > </ul>
  > </li>
  > <li>
  > <p><a
  > href="https://redirect.github.com/Peternator7/strum/pull/435">#435</a>:allow
  > discriminants on empty enum.</p>
  > </li>
  > <li>
  > <p><a
  > href="https://redirect.github.com/Peternator7/strum/pull/443">#443</a>:
  > Change enum table callbacks to FnMut.</p>
  > </li>
  > <li>
  > <p><a
  > href="https://redirect.github.com/Peternator7/strum/pull/444">#444</a>:
  > Add <code>#[automatically_derived]</code> to the <code>impl</code>s by
  > <a href="https://github.com/dandedotdev"><code>@​dandedotdev</code></a>
  > in <a
  > href="https://redirect.github.com/Peternator7/strum/pull/444">Peternator7/strum#444</a></p>
  > <ul>
  > <li>This should make the linter less noisy with warnings in generated
  > code.</li>
  > </ul>
  > </li>
  > <li>
  > <p><a
  > href="https://redirect.github.com/Peternator7/strum/pull/440">#440</a>:
  > Implement a <code>suffix</code> attribute for serialization of enum
  > variants.</p>
  > <pre lang="rust"><code>#[derive(strum::Display)]
  > #[strum(suffix=&quot;.json&quot;)]
  > #[strum(serialize_all=&quot;snake_case&quot;)]
  > enum StorageConfiguration {
  >   PostgresProvider,
  >   S3StorageProvider,
  >   AzureStorageProvider,
  > }
  > <p>fn main() {
  > let response = SurveyResponse::Other(&quot;It was good&quot;.into());
  > println!(&quot;Loading configuration from: {}&quot;,
  > StorageConfiguration::PostgresProvider);
  > // prints: Loaded Configuration from: postgres_provider.json
  > }
  > </code></pre></p>
  > </li>
  > <li>
  > <p><a
  > href="https://redirect.github.com/Peternator7/strum/pull/446">#446</a>:
  > Drop needless <code>rustversion</code> dependency.</p>
  > </li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/38f66210e7ca0bb156f3632dcf24a2548959c379"><code>38f6621</code></a>
  > Expound upon use_phf docs (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/449">#449</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/bb1339026b44773e395913340f4e60972fa5e6a1"><code>bb13390</code></a>
  > Implement a <code>suffix</code> attribute for serialization of enum
  > variants (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/440">#440</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/c9e52bfd2865c8c766e0379f9e7bf57621a104e3"><code>c9e52bf</code></a>
  > Add <code>#[automatically_derived]</code> to the <code>impl</code>s (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/444">#444</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/1b00f899e52f43fa35c4d406c901d33b1e9645e2"><code>1b00f89</code></a>
  > Change enum table callbacks to FnMut. (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/443">#443</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/6e2ca25fba8ebdfa403ada6a2bf2f3b15403b2cf"><code>6e2ca25</code></a>
  > Remove broken link to EnumTable docs (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/427">#427</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/95037811412792c9cd70586598aa88d7f514c0ac"><code>9503781</code></a>
  > allow discriminants on empty enum (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/435">#435</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/8553ba2845989337d88a7170f7f0c419945bf156"><code>8553ba2</code></a>
  > Upgrade <code>phf</code> to v0.12 (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/448">#448</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/2eba5c2a5c0b827317bafcb1f545af67b5ce9110"><code>2eba5c2</code></a>
  > Drop needless <code>rustversion</code> dependency (<a
  > href="https://redirect.github.com/Peternator7/strum/issues/446">#446</a>)</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/f301b67d9122b271e0531ab7f167c4585cefa484"><code>f301b67</code></a>
  > Merge branch 'linclelinkpart5-master-2'</li>
  > <li><a
  > href="https://github.com/Peternator7/strum/commit/455b2bf859640dc27442b9d38f58ce8da7e3bd6e"><code>455b2bf</code></a>
  > Merge branch 'master' of <a
  > href="https://github.com/linclelinkpart5/strum">https://github.com/linclelinkpart5/strum</a>
  > into lincle...</li>
  > <li>See full diff in <a
  > href="https://github.com/Peternator7/strum/compare/v0.27.1...v0.27.2">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=strum&package-manager=cargo&previous-version=0.27.1&new-version=0.27.2)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump rand from 0.9.1 to 0.9.2 ([#88](https://github.com/ratatui/tui-widgets/issues/88))
  > Bumps [rand](https://github.com/rust-random/rand) from 0.9.1 to 0.9.2.
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/rust-random/rand/blob/master/CHANGELOG.md">rand's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[0.9.2 — 2025-07-20]</h2>
  > <h3>Deprecated</h3>
  > <ul>
  > <li>Deprecate <code>rand::rngs::mock</code> module and
  > <code>StepRng</code> generator (<a
  > href="https://redirect.github.com/rust-random/rand/issues/1634">#1634</a>)</li>
  > </ul>
  > <h3>Additions</h3>
  > <ul>
  > <li>Enable <code>WeightedIndex&lt;usize&gt;</code> (de)serialization (<a
  > href="https://redirect.github.com/rust-random/rand/issues/1646">#1646</a>)</li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/d3dd4157052e5431ce42e157b544968560a68b95"><code>d3dd415</code></a>
  > Prepare rand_core 0.9.2 (<a
  > href="https://redirect.github.com/rust-random/rand/issues/1605">#1605</a>)</li>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/99fabd20e9b39d0af7c2ed6c31dbcad83a1703fd"><code>99fabd2</code></a>
  > Prepare rand_core 0.9.2</li>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/c7fe1c43b5ba53aacad5fbac94a8b88788564049"><code>c7fe1c4</code></a>
  > rand: re-export <code>rand_core</code> (<a
  > href="https://redirect.github.com/rust-random/rand/issues/1604">#1604</a>)</li>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/db2b1e0bb41b0a1435b9fecaa1b7bdb531183204"><code>db2b1e0</code></a>
  > rand: re-export <code>rand_core</code></li>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/ee1d96f9f527dbe6f873c8a5ccf47d60a6b8f7b7"><code>ee1d96f</code></a>
  > rand_core: implement reborrow for <code>UnwrapMut</code> (<a
  > href="https://redirect.github.com/rust-random/rand/issues/1595">#1595</a>)</li>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/e0eb2ee0fcc0b07afb901465f4a8ba7f07128f87"><code>e0eb2ee</code></a>
  > rand_core: implement reborrow for <code>UnwrapMut</code></li>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/975f602f5dbbdab0a024e0c5e8b527907426bd8c"><code>975f602</code></a>
  > fixup clippy 1.85 warnings</li>
  > <li><a
  > href="https://github.com/rust-random/rand/commit/775b05be1b8a4fdef17c6601cd223551fbf67edc"><code>775b05b</code></a>
  > Relax <code>Sized</code> requirements for blanket impls (<a
  > href="https://redirect.github.com/rust-random/rand/issues/1593">#1593</a>)</li>
  > <li>See full diff in <a
  > href="https://github.com/rust-random/rand/compare/rand_core-0.9.1...rand_core-0.9.2">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=rand&package-manager=cargo&previous-version=0.9.1&new-version=0.9.2)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump tokio from 1.45.1 to 1.46.1 ([#85](https://github.com/ratatui/tui-widgets/issues/85))
  > Bumps [tokio](https://github.com/tokio-rs/tokio) from 1.45.1 to 1.46.1.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/tokio-rs/tokio/releases">tokio's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>Tokio v1.46.1</h2>
  > <h1>1.46.1 (July 4th, 2025)</h1>
  > <p>This release fixes incorrect spawn locations in runtime task hooks
  > for tasks spawned using <code>tokio::spawn</code> rather than
  > <code>Runtime::spawn</code>. This issue only effected the spawn location
  > in <code>TaskMeta::spawned_at</code>, and did not effect task locations
  > in Tracing events.</p>
  > <h2>Unstable</h2>
  > <ul>
  > <li>runtime: add <code>TaskMeta::spawn_location</code> tracking where a
  > task was spawned (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7440">#7440</a>)</li>
  > </ul>
  > <p><a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7440">#7440</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7440">tokio-rs/tokio#7440</a></p>
  > <h2>Tokio v1.46.0</h2>
  > <h1>1.46.0 (July 2nd, 2025)</h1>
  > <h3>Fixed</h3>
  > <ul>
  > <li>net: fixed <code>TcpStream::shutdown</code> incorrectly returning an
  > error on macOS (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7290">#7290</a>)</li>
  > </ul>
  > <h2>Added</h2>
  > <ul>
  > <li>sync: <code>mpsc::OwnedPermit::{same_channel,
  > same_channel_as_sender}</code> methods (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7389">#7389</a>)</li>
  > <li>macros: <code>biased</code> option for <code>join!</code> and
  > <code>try_join!</code>, similar to <code>select!</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7307">#7307</a>)</li>
  > <li>net: support for cygwin (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7393">#7393</a>)</li>
  > <li>net: support <code>pope::OpenOptions::read_write</code> on Android
  > (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7426">#7426</a>)</li>
  > <li>net: add <code>Clone</code> implementation for
  > <code>net::unix::SocketAddr</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7422">#7422</a>)</li>
  > </ul>
  > <h2>Changed</h2>
  > <ul>
  > <li>runtime: eliminate unnecessary lfence while operating on
  > <code>queue::Local&lt;T&gt;</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7340">#7340</a>)</li>
  > <li>task: disallow blocking in <code>LocalSet::{poll,drop}</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7372">#7372</a>)</li>
  > </ul>
  > <h2>Unstable</h2>
  > <ul>
  > <li>runtime: add <code>TaskMeta::spawn_location</code> tracking where a
  > task was spawned (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7417">#7417</a>)</li>
  > <li>runtime: removed borrow from <code>LocalOptions</code> parameter to
  > <code>runtime::Builder::build_local</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7346">#7346</a>)</li>
  > </ul>
  > <h2>Documented</h2>
  > <ul>
  > <li>io: clarify behavior of seeking when <code>start_seek</code> is not
  > used (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7366">#7366</a>)</li>
  > <li>io: document cancellation safety of
  > <code>AsyncWriteExt::flush</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7364">#7364</a>)</li>
  > <li>net: fix docs for <code>recv_buffer_size</code> method (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7336">#7336</a>)</li>
  > <li>net: fix broken link of <code>RawFd</code> in <code>TcpSocket</code>
  > docs (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7416">#7416</a>)</li>
  > <li>net: update <code>AsRawFd</code> doc link to current Rust stdlib
  > location (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7429">#7429</a>)</li>
  > <li>readme: fix double period in reactor description (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7363">#7363</a>)</li>
  > <li>runtime: add doc note that <code>on_*_task_poll</code> is unstable
  > (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7311">#7311</a>)</li>
  > <li>sync: update broadcast docs on allocation failure (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7352">#7352</a>)</li>
  > <li>time: add a missing panic scenario of <code>time::advance</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7394">#7394</a>)</li>
  > </ul>
  > <p><a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7290">#7290</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7290">tokio-rs/tokio#7290</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7307">#7307</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7307">tokio-rs/tokio#7307</a></p>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/ab3ff69cf2258a8c696b2dca89a2cef4ff114c1c"><code>ab3ff69</code></a>
  > chore: prepare to release v1.46.1 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7444">#7444</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/a0d5b8ab308bbeaa8090d411550d6c887d699096"><code>a0d5b8a</code></a>
  > runtime(unstable): fix task hook spawn locations for
  > <code>tokio::spawn</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7440">#7440</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/a1ee3ef218894f2441b5719812ab218ae0539c8d"><code>a1ee3ef</code></a>
  > chore: fix some minor typos in the comments (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7442">#7442</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/171cd148a37da40dcbb8b06bf2c67634b2ba1f87"><code>171cd14</code></a>
  > changelog: fix typo in <code>pipe::OpenOptions</code> for 1.46.0 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7439">#7439</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/3f1f268583a16c11560f8e310d5a35e9aa55b547"><code>3f1f268</code></a>
  > chore: prepare Tokio v1.46.0 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7437">#7437</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/3e890cc0171ddb210acdcfec831b7c7bcbb0d2d9"><code>3e890cc</code></a>
  > rt(unstable): add spawn <code>Location</code> to <code>TaskMeta</code>
  > (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7417">#7417</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/69290a64327a017fd9a0cedefaac60c4993c3b54"><code>69290a6</code></a>
  > net: derive <code>Clone</code> for <code>net::unix::SocketAddr</code>
  > (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7422">#7422</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/e2b175848b2cb25e99cd3a0486e506f889379db5"><code>e2b1758</code></a>
  > fuzz: cfg fuzz tests under cfg(test) (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7428">#7428</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/b7a75b5be349aab2cee9b224c0610d7cf4fea73e"><code>b7a75b5</code></a>
  > net: update <code>AsRawFd</code> doc link to current Rust stdlib
  > location (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7429">#7429</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/6b705b3053d2c777e05cb60c758202ff9d4b2e7d"><code>6b705b3</code></a>
  > net: allow <code>pipe::OpenOptions::read_write</code> on Android (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7426">#7426</a>)</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/tokio-rs/tokio/compare/tokio-1.45.1...tokio-1.46.1">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=tokio&package-manager=cargo&previous-version=1.45.1&new-version=1.46.1)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump clap from 4.5.41 to 4.5.43 ([#97](https://github.com/ratatui/tui-widgets/issues/97))
  > Bumps [clap](https://github.com/clap-rs/clap) from 4.5.41 to 4.5.43.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/clap-rs/clap/releases">clap's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>v4.5.43</h2>
  > <h2>[4.5.43] - 2025-08-06</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(help)</em> In long help, list Possible Values before defaults,
  > rather than after, for a more consistent look</li>
  > </ul>
  > <h2>v4.5.42</h2>
  > <h2>[4.5.42] - 2025-07-30</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li>Include subcommand visible long aliases in <code>--help</code></li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/clap-rs/clap/blob/master/CHANGELOG.md">clap's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[4.5.43] - 2025-08-06</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(help)</em> In long help, list Possible Values before defaults,
  > rather than after, for a more consistent look</li>
  > </ul>
  > <h2>[4.5.42] - 2025-07-30</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li>Include subcommand visible long aliases in <code>--help</code></li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/c4105bd90c314ac21dd9e008de8b88ab0175fdf7"><code>c4105bd</code></a>
  > chore: Release</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/a029b20be631aab1d3a963872df2158b97f61427"><code>a029b20</code></a>
  > docs: Update changelog</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/cf15d48b59cf39cafc3e3797dec293edaf9cf533"><code>cf15d48</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5893">#5893</a>
  > from 8LWXpg/patch-2</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/7e54542de972c4af98d3035377dcde83c5a5734e"><code>7e54542</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5892">#5892</a>
  > from 8LWXpg/patch-1</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/6ffc88f8c97be82e71d5d6101c98e1042708ab69"><code>6ffc88f</code></a>
  > fix(complete): Check if help string is empty</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/7d8470ed9cf1d5503482938cea62f8f363579f12"><code>7d8470e</code></a>
  > fix(complete): Fix single quote escaping in PowerShell</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/eadcc8f66c128272ea309fed3d53d45b9c700b6f"><code>eadcc8f</code></a>
  > chore: Release</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/7ce0f7bea34011ca888a762bdd95d2371006c97a"><code>7ce0f7b</code></a>
  > docs: Update changelog</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/fea7c5487bb602a9b7151c40069afc6f34bda442"><code>fea7c54</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5888">#5888</a>
  > from epage/tut</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/c297ddd56e2601d9d1b0a0ba13a9086e8f3ac43c"><code>c297ddd</code></a>
  > docs(tutorial): Experiment with a flat layout</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/clap-rs/clap/compare/clap_complete-v4.5.41...clap_complete-v4.5.43">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=clap&package-manager=cargo&previous-version=4.5.41&new-version=4.5.43)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump tokio from 1.46.1 to 1.47.1 ([#96](https://github.com/ratatui/tui-widgets/issues/96))
  > Bumps [tokio](https://github.com/tokio-rs/tokio) from 1.46.1 to 1.47.1.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/tokio-rs/tokio/releases">tokio's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>Tokio v1.47.1</h2>
  > <h1>1.47.1 (August 1st, 2025)</h1>
  > <h3>Fixed</h3>
  > <ul>
  > <li>process: fix panic from spurious pidfd wakeup (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7494">#7494</a>)</li>
  > <li>sync: fix broken link of Python <code>asyncio.Event</code> in
  > <code>SetOnce</code> docs (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7485">#7485</a>)</li>
  > </ul>
  > <p><a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7485">#7485</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7485">tokio-rs/tokio#7485</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7494">#7494</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7494">tokio-rs/tokio#7494</a></p>
  > <h2>Tokio v1.47.0</h2>
  > <h1>1.47.0 (July 25th, 2025)</h1>
  > <p>This release adds <code>poll_proceed</code> and
  > <code>cooperative</code> to the <code>coop</code> module for
  > cooperative scheduling, adds <code>SetOnce</code> to the
  > <code>sync</code> module which provides
  > similar functionality to [<code>std::sync::OnceLock</code>], and adds a
  > new method
  > <code>sync::Notify::notified_owned()</code> which returns an
  > <code>OwnedNotified</code> without
  > a lifetime parameter.</p>
  > <h2>Added</h2>
  > <ul>
  > <li>coop: add <code>cooperative</code> and <code>poll_proceed</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7405">#7405</a>)</li>
  > <li>sync: add <code>SetOnce</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7418">#7418</a>)</li>
  > <li>sync: add <code>sync::Notify::notified_owned()</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7465">#7465</a>)</li>
  > </ul>
  > <h2>Changed</h2>
  > <ul>
  > <li>deps: upgrade windows-sys 0.52 → 0.59 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7117">#7117</a>)</li>
  > <li>deps: update to socket2 v0.6 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7443">#7443</a>)</li>
  > <li>sync: improve <code>AtomicWaker::wake</code> performance (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7450">#7450</a>)</li>
  > </ul>
  > <h2>Documented</h2>
  > <ul>
  > <li>metrics: fix listed feature requirements for some metrics (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7449">#7449</a>)</li>
  > <li>runtime: improve safety comments of <code>Readiness&lt;'_&gt;</code>
  > (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7415">#7415</a>)</li>
  > </ul>
  > <p><a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7405">#7405</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7405">tokio-rs/tokio#7405</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7415">#7415</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7415">tokio-rs/tokio#7415</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7418">#7418</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7418">tokio-rs/tokio#7418</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7449">#7449</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7449">tokio-rs/tokio#7449</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7450">#7450</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7450">tokio-rs/tokio#7450</a>
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7465">#7465</a>:
  > <a
  > href="https://redirect.github.com/tokio-rs/tokio/pull/7465">tokio-rs/tokio#7465</a></p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/be8ee45b3fc2d107174e586141b1cb12c93e2ddf"><code>be8ee45</code></a>
  > chore: prepare Tokio v1.47.1 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7504">#7504</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/d9b19166cde30b8d4a65f31a94b5ee09d2dd7b8c"><code>d9b1916</code></a>
  > Merge 'tokio-1.43.2' into 'tokio-1.47.x' (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7503">#7503</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/db8edc620fb369f6cc92dd9dcfdd03b832c2b02f"><code>db8edc6</code></a>
  > chore: prepare Tokio v1.43.2 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7502">#7502</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/4730984d66e708b36efe84245cbf15bd483a886f"><code>4730984</code></a>
  > readme: add 1.47 as LTS release (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7497">#7497</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/1979615cbf1cc4b4d296814957394703827362d0"><code>1979615</code></a>
  > process: fix panic from spurious pidfd wakeup (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7494">#7494</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/f669a609cf1eaa94d2bc135212f57ff913eca898"><code>f669a60</code></a>
  > ci: add lockfile for LTS branch</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/ce41896f8dcbc6249df3279600f45f7a65915cf6"><code>ce41896</code></a>
  > sync: fix broken link of Python <code>asyncio.Event</code> in
  > <code>SetOnce</code> docs (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7485">#7485</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/c8ab78a84fff284958dc84b77b5222fecd0f44b2"><code>c8ab78a</code></a>
  > changelog: fix incorrect PR number for 1.47.0 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7484">#7484</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/3911cb8523f190142f61c64b66881c07c0d3e7be"><code>3911cb8</code></a>
  > chore: prepare Tokio v1.47.0 (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7482">#7482</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tokio/commit/d545aa2601e3008ce49c8c0191b0f172ce577452"><code>d545aa2</code></a>
  > sync: add <code>sync::Notify::notified_owned()</code> (<a
  > href="https://redirect.github.com/tokio-rs/tokio/issues/7465">#7465</a>)</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/tokio-rs/tokio/compare/tokio-1.46.1...tokio-1.47.1">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=tokio&package-manager=cargo&previous-version=1.46.1&new-version=1.47.1)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump clap from 4.5.43 to 4.5.48 ([#103](https://github.com/ratatui/tui-widgets/issues/103))
  > Bumps [clap](https://github.com/clap-rs/clap) from 4.5.43 to 4.5.48.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/clap-rs/clap/releases">clap's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>v4.5.48</h2>
  > <h2>[4.5.48] - 2025-09-19</h2>
  > <h3>Documentation</h3>
  > <ul>
  > <li>Add a new CLI Concepts document as another way of framing clap</li>
  > <li>Expand the <code>typed_derive</code> cookbook entry</li>
  > </ul>
  > <h2>v4.5.47</h2>
  > <h2>[4.5.47] - 2025-09-02</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Added <code>impl FromArgMatches for ()</code></li>
  > <li>Added <code>impl Args for ()</code></li>
  > <li>Added <code>impl Subcommand for ()</code></li>
  > <li>Added <code>impl FromArgMatches for Infallible</code></li>
  > <li>Added <code>impl Subcommand for Infallible</code></li>
  > </ul>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(derive)</em> Update runtime error text to match
  > <code>clap</code></li>
  > </ul>
  > <h2>v4.5.46</h2>
  > <h2>[4.5.46] - 2025-08-26</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Expose <code>StyledStr::push_str</code></li>
  > </ul>
  > <h2>v4.5.45</h2>
  > <h2>[4.5.45] - 2025-08-12</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(unstable-v5)</em> <code>ValueEnum</code> variants now use the
  > full doc comment, not summary, for <code>PossibleValue::help</code></li>
  > </ul>
  > <h2>v4.5.44</h2>
  > <h2>[4.5.44] - 2025-08-11</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Add <code>Command::mut_subcommands</code></li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/clap-rs/clap/blob/master/CHANGELOG.md">clap's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[4.5.48] - 2025-09-19</h2>
  > <h3>Documentation</h3>
  > <ul>
  > <li>Add a new CLI Concepts document as another way of framing clap</li>
  > <li>Expand the <code>typed_derive</code> cookbook entry</li>
  > </ul>
  > <h2>[4.5.47] - 2025-09-02</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Added <code>impl FromArgMatches for ()</code></li>
  > <li>Added <code>impl Args for ()</code></li>
  > <li>Added <code>impl Subcommand for ()</code></li>
  > <li>Added <code>impl FromArgMatches for Infallible</code></li>
  > <li>Added <code>impl Subcommand for Infallible</code></li>
  > </ul>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(derive)</em> Update runtime error text to match
  > <code>clap</code></li>
  > </ul>
  > <h2>[4.5.46] - 2025-08-26</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Expose <code>StyledStr::push_str</code></li>
  > </ul>
  > <h2>[4.5.45] - 2025-08-12</h2>
  > <h3>Fixes</h3>
  > <ul>
  > <li><em>(unstable-v5)</em> <code>ValueEnum</code> variants now use the
  > full doc comment, not summary, for <code>PossibleValue::help</code></li>
  > </ul>
  > <h2>[4.5.44] - 2025-08-11</h2>
  > <h3>Features</h3>
  > <ul>
  > <li>Add <code>Command::mut_subcommands</code></li>
  > </ul>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/c3a1ddc1182fa7cf2cfe6d6dba4f76db83d48178"><code>c3a1ddc</code></a>
  > chore: Release</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/4460ff44b375c8d596fb70b848ff401fe12942c0"><code>4460ff4</code></a>
  > docs: Update changelog</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/54947a1b4bc70745cd5787fb92a830081c6ed291"><code>54947a1</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5981">#5981</a>
  > from mernen/fix-bash-clap-complete-space</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/fd3f6d271defef2aa2f111555a005689f71f6acb"><code>fd3f6d2</code></a>
  > fix(complete): Restore nospace in bash</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/2f6a1083d94b832af96b791fc934beb043a969cb"><code>2f6a108</code></a>
  > test(complete): Demonstrate current behavior</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/f88be5738e33018f3298fabb7b67835eefbc55e0"><code>f88be57</code></a>
  > style: Ensure consistent newlines</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/f209bce2203498e743b171b7ac64f0fb9d3ae590"><code>f209bce</code></a>
  > chore: Release</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/f33ff7f81ab78c227a127fbd2dbd0fed1455a6fb"><code>f33ff7f</code></a>
  > docs: Update changelog</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/bf06e6f8f6efc5af03a52c5e4cfea39c682aa500"><code>bf06e6f</code></a>
  > Merge pull request <a
  > href="https://redirect.github.com/clap-rs/clap/issues/5974">#5974</a>
  > from kryvashek/support-clearing-args-matches</li>
  > <li><a
  > href="https://github.com/clap-rs/clap/commit/5d357ada532d430290c2de14c918833564f12795"><code>5d357ad</code></a>
  > feat(parser): Added ArgMatches::try_clear_id()</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/clap-rs/clap/compare/clap_complete-v4.5.43...clap_complete-v4.5.48">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=clap&package-manager=cargo&previous-version=4.5.43&new-version=4.5.48)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump tracing-subscriber from 0.3.19 to 0.3.20 ([#100](https://github.com/ratatui/tui-widgets/issues/100))
  > Bumps [tracing-subscriber](https://github.com/tokio-rs/tracing) from
  > 0.3.19 to 0.3.20.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/tokio-rs/tracing/releases">tracing-subscriber's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>tracing-subscriber 0.3.20</h2>
  > <p><strong>Security Fix</strong>: ANSI Escape Sequence Injection
  > (CVE-TBD)</p>
  > <h2>Impact</h2>
  > <p>Previous versions of tracing-subscriber were vulnerable to ANSI
  > escape sequence injection attacks. Untrusted user input containing ANSI
  > escape sequences could be injected into terminal output when logged,
  > potentially allowing attackers to:</p>
  > <ul>
  > <li>Manipulate terminal title bars</li>
  > <li>Clear screens or modify terminal display</li>
  > <li>Potentially mislead users through terminal manipulation</li>
  > </ul>
  > <p>In isolation, impact is minimal, however security issues have been
  > found in terminal emulators that enabled an attacker to use ANSI escape
  > sequences via logs to exploit vulnerabilities in the terminal
  > emulator.</p>
  > <h2>Solution</h2>
  > <p>Version 0.3.20 fixes this vulnerability by escaping ANSI control
  > characters in when writing events to destinations that may be printed to
  > the terminal.</p>
  > <h2>Affected Versions</h2>
  > <p>All versions of tracing-subscriber prior to 0.3.20 are affected by
  > this vulnerability.</p>
  > <h2>Recommendations</h2>
  > <p>Immediate Action Required: We recommend upgrading to
  > tracing-subscriber 0.3.20 immediately, especially if your
  > application:</p>
  > <ul>
  > <li>Logs user-provided input (form data, HTTP headers, query parameters,
  > etc.)</li>
  > <li>Runs in environments where terminal output is displayed to
  > users</li>
  > </ul>
  > <h2>Migration</h2>
  > <p>This is a patch release with no breaking API changes. Simply update
  > your Cargo.toml:</p>
  > <pre lang="toml"><code>[dependencies]
  > tracing-subscriber = &quot;0.3.20&quot;
  > </code></pre>
  > <h2>Acknowledgments</h2>
  > <p>We would like to thank <a href="http://github.com/zefr0x">zefr0x</a>
  > who responsibly reported the issue at
  > <code>security@tokio.rs</code>.</p>
  > <p>If you believe you have found a security vulnerability in any
  > tokio-rs project, please email us at <code>security@tokio.rs</code>.</p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/4c52ca5266a3920fc5dfeebda2accf15ee7fb278"><code>4c52ca5</code></a>
  > fmt: fix ANSI escape sequence injection vulnerability (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3368">#3368</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/f71cebe41e4c12735b1d19ca804428d4ff7d905d"><code>f71cebe</code></a>
  > subscriber: impl Clone for EnvFilter (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3360">#3360</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/3a1f571102b38bcdca13d59f3c454989d179055d"><code>3a1f571</code></a>
  > Fix CI (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3361">#3361</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/e63ef57f3d686abe3727ddd586eb9af73d6715b7"><code>e63ef57</code></a>
  > chore: prepare tracing-attributes 0.1.30 (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3316">#3316</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/6e59a13b1a7bcdd78b8b5a7cbcf70a0b2cdd76f0"><code>6e59a13</code></a>
  > attributes: fix tracing::instrument regression around shadowing (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3311">#3311</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/e4df76127538aa8370d7dee32a6f84bbec6bbf10"><code>e4df761</code></a>
  > tracing: update core to 0.1.34 and attributes to 0.1.29 (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3305">#3305</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/643f392ebb73c4fb856f56a78c066c82582dd22c"><code>643f392</code></a>
  > chore: prepare tracing-attributes 0.1.29 (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3304">#3304</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/d08e7a6eea1833810ea527e18ea03b08cd402c9d"><code>d08e7a6</code></a>
  > chore: prepare tracing-core 0.1.34 (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3302">#3302</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/6e70c571d319a033d5f37c885ccf99aa675a9eac"><code>6e70c57</code></a>
  > tracing-subscriber: count numbers of enters in <code>Timings</code> (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/2944">#2944</a>)</li>
  > <li><a
  > href="https://github.com/tokio-rs/tracing/commit/c01d4fd9def2fb061669a310598095c789ca0a32"><code>c01d4fd</code></a>
  > fix docs and enable CI on <code>main</code> branch (<a
  > href="https://redirect.github.com/tokio-rs/tracing/issues/3295">#3295</a>)</li>
  > <li>Additional commits viewable in <a
  > href="https://github.com/tokio-rs/tracing/compare/tracing-subscriber-0.3.19...tracing-subscriber-0.3.20">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=tracing-subscriber&package-manager=cargo&previous-version=0.3.19&new-version=0.3.20)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- *(deps)* Bump actions/checkout from 4 to 5 ([#99](https://github.com/ratatui/tui-widgets/issues/99))
  > Bumps [actions/checkout](https://github.com/actions/checkout) from 4 to
  > 5.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/actions/checkout/releases">actions/checkout's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>v5.0.0</h2>
  > <h2>What's Changed</h2>
  > <ul>
  > <li>Update actions checkout to use node 24 by <a
  > href="https://github.com/salmanmkc"><code>@​salmanmkc</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2226">actions/checkout#2226</a></li>
  > <li>Prepare v5.0.0 release by <a
  > href="https://github.com/salmanmkc"><code>@​salmanmkc</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2238">actions/checkout#2238</a></li>
  > </ul>
  > <h2>⚠️ Minimum Compatible Runner Version</h2>
  > <p><strong>v2.327.1</strong><br />
  > <a
  > href="https://github.com/actions/runner/releases/tag/v2.327.1">Release
  > Notes</a></p>
  > <p>Make sure your runner is updated to this version or newer to use this
  > release.</p>
  > <p><strong>Full Changelog</strong>: <a
  > href="https://github.com/actions/checkout/compare/v4...v5.0.0">https://github.com/actions/checkout/compare/v4...v5.0.0</a></p>
  > <h2>v4.3.0</h2>
  > <h2>What's Changed</h2>
  > <ul>
  > <li>docs: update README.md by <a
  > href="https://github.com/motss"><code>@​motss</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1971">actions/checkout#1971</a></li>
  > <li>Add internal repos for checking out multiple repositories by <a
  > href="https://github.com/mouismail"><code>@​mouismail</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1977">actions/checkout#1977</a></li>
  > <li>Documentation update - add recommended permissions to Readme by <a
  > href="https://github.com/benwells"><code>@​benwells</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2043">actions/checkout#2043</a></li>
  > <li>Adjust positioning of user email note and permissions heading by <a
  > href="https://github.com/joshmgross"><code>@​joshmgross</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2044">actions/checkout#2044</a></li>
  > <li>Update README.md by <a
  > href="https://github.com/nebuk89"><code>@​nebuk89</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2194">actions/checkout#2194</a></li>
  > <li>Update CODEOWNERS for actions by <a
  > href="https://github.com/TingluoHuang"><code>@​TingluoHuang</code></a>
  > in <a
  > href="https://redirect.github.com/actions/checkout/pull/2224">actions/checkout#2224</a></li>
  > <li>Update package dependencies by <a
  > href="https://github.com/salmanmkc"><code>@​salmanmkc</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2236">actions/checkout#2236</a></li>
  > <li>Prepare release v4.3.0 by <a
  > href="https://github.com/salmanmkc"><code>@​salmanmkc</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2237">actions/checkout#2237</a></li>
  > </ul>
  > <h2>New Contributors</h2>
  > <ul>
  > <li><a href="https://github.com/motss"><code>@​motss</code></a> made
  > their first contribution in <a
  > href="https://redirect.github.com/actions/checkout/pull/1971">actions/checkout#1971</a></li>
  > <li><a href="https://github.com/mouismail"><code>@​mouismail</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/actions/checkout/pull/1977">actions/checkout#1977</a></li>
  > <li><a href="https://github.com/benwells"><code>@​benwells</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/actions/checkout/pull/2043">actions/checkout#2043</a></li>
  > <li><a href="https://github.com/nebuk89"><code>@​nebuk89</code></a> made
  > their first contribution in <a
  > href="https://redirect.github.com/actions/checkout/pull/2194">actions/checkout#2194</a></li>
  > <li><a href="https://github.com/salmanmkc"><code>@​salmanmkc</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/actions/checkout/pull/2236">actions/checkout#2236</a></li>
  > </ul>
  > <p><strong>Full Changelog</strong>: <a
  > href="https://github.com/actions/checkout/compare/v4...v4.3.0">https://github.com/actions/checkout/compare/v4...v4.3.0</a></p>
  > <h2>v4.2.2</h2>
  > <h2>What's Changed</h2>
  > <ul>
  > <li><code>url-helper.ts</code> now leverages well-known environment
  > variables by <a href="https://github.com/jww3"><code>@​jww3</code></a>
  > in <a
  > href="https://redirect.github.com/actions/checkout/pull/1941">actions/checkout#1941</a></li>
  > <li>Expand unit test coverage for <code>isGhes</code> by <a
  > href="https://github.com/jww3"><code>@​jww3</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1946">actions/checkout#1946</a></li>
  > </ul>
  > <p><strong>Full Changelog</strong>: <a
  > href="https://github.com/actions/checkout/compare/v4.2.1...v4.2.2">https://github.com/actions/checkout/compare/v4.2.1...v4.2.2</a></p>
  > <h2>v4.2.1</h2>
  > <h2>What's Changed</h2>
  > <ul>
  > <li>Check out other refs/* by commit if provided, fall back to ref by <a
  > href="https://github.com/orhantoy"><code>@​orhantoy</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1924">actions/checkout#1924</a></li>
  > </ul>
  > <h2>New Contributors</h2>
  > <ul>
  > <li><a href="https://github.com/Jcambass"><code>@​Jcambass</code></a>
  > made their first contribution in <a
  > href="https://redirect.github.com/actions/checkout/pull/1919">actions/checkout#1919</a></li>
  > </ul>
  > <p><strong>Full Changelog</strong>: <a
  > href="https://github.com/actions/checkout/compare/v4.2.0...v4.2.1">https://github.com/actions/checkout/compare/v4.2.0...v4.2.1</a></p>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/actions/checkout/blob/main/CHANGELOG.md">actions/checkout's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h1>Changelog</h1>
  > <h2>V5.0.0</h2>
  > <ul>
  > <li>Update actions checkout to use node 24 by <a
  > href="https://github.com/salmanmkc"><code>@​salmanmkc</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2226">actions/checkout#2226</a></li>
  > </ul>
  > <h2>V4.3.0</h2>
  > <ul>
  > <li>docs: update README.md by <a
  > href="https://github.com/motss"><code>@​motss</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1971">actions/checkout#1971</a></li>
  > <li>Add internal repos for checking out multiple repositories by <a
  > href="https://github.com/mouismail"><code>@​mouismail</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1977">actions/checkout#1977</a></li>
  > <li>Documentation update - add recommended permissions to Readme by <a
  > href="https://github.com/benwells"><code>@​benwells</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2043">actions/checkout#2043</a></li>
  > <li>Adjust positioning of user email note and permissions heading by <a
  > href="https://github.com/joshmgross"><code>@​joshmgross</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2044">actions/checkout#2044</a></li>
  > <li>Update README.md by <a
  > href="https://github.com/nebuk89"><code>@​nebuk89</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2194">actions/checkout#2194</a></li>
  > <li>Update CODEOWNERS for actions by <a
  > href="https://github.com/TingluoHuang"><code>@​TingluoHuang</code></a>
  > in <a
  > href="https://redirect.github.com/actions/checkout/pull/2224">actions/checkout#2224</a></li>
  > <li>Update package dependencies by <a
  > href="https://github.com/salmanmkc"><code>@​salmanmkc</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/2236">actions/checkout#2236</a></li>
  > </ul>
  > <h2>v4.2.2</h2>
  > <ul>
  > <li><code>url-helper.ts</code> now leverages well-known environment
  > variables by <a href="https://github.com/jww3"><code>@​jww3</code></a>
  > in <a
  > href="https://redirect.github.com/actions/checkout/pull/1941">actions/checkout#1941</a></li>
  > <li>Expand unit test coverage for <code>isGhes</code> by <a
  > href="https://github.com/jww3"><code>@​jww3</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1946">actions/checkout#1946</a></li>
  > </ul>
  > <h2>v4.2.1</h2>
  > <ul>
  > <li>Check out other refs/* by commit if provided, fall back to ref by <a
  > href="https://github.com/orhantoy"><code>@​orhantoy</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1924">actions/checkout#1924</a></li>
  > </ul>
  > <h2>v4.2.0</h2>
  > <ul>
  > <li>Add Ref and Commit outputs by <a
  > href="https://github.com/lucacome"><code>@​lucacome</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1180">actions/checkout#1180</a></li>
  > <li>Dependency updates by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a>- <a
  > href="https://redirect.github.com/actions/checkout/pull/1777">actions/checkout#1777</a>,
  > <a
  > href="https://redirect.github.com/actions/checkout/pull/1872">actions/checkout#1872</a></li>
  > </ul>
  > <h2>v4.1.7</h2>
  > <ul>
  > <li>Bump the minor-npm-dependencies group across 1 directory with 4
  > updates by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1739">actions/checkout#1739</a></li>
  > <li>Bump actions/checkout from 3 to 4 by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1697">actions/checkout#1697</a></li>
  > <li>Check out other refs/* by commit by <a
  > href="https://github.com/orhantoy"><code>@​orhantoy</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1774">actions/checkout#1774</a></li>
  > <li>Pin actions/checkout's own workflows to a known, good, stable
  > version. by <a href="https://github.com/jww3"><code>@​jww3</code></a> in
  > <a
  > href="https://redirect.github.com/actions/checkout/pull/1776">actions/checkout#1776</a></li>
  > </ul>
  > <h2>v4.1.6</h2>
  > <ul>
  > <li>Check platform to set archive extension appropriately by <a
  > href="https://github.com/cory-miller"><code>@​cory-miller</code></a> in
  > <a
  > href="https://redirect.github.com/actions/checkout/pull/1732">actions/checkout#1732</a></li>
  > </ul>
  > <h2>v4.1.5</h2>
  > <ul>
  > <li>Update NPM dependencies by <a
  > href="https://github.com/cory-miller"><code>@​cory-miller</code></a> in
  > <a
  > href="https://redirect.github.com/actions/checkout/pull/1703">actions/checkout#1703</a></li>
  > <li>Bump github/codeql-action from 2 to 3 by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1694">actions/checkout#1694</a></li>
  > <li>Bump actions/setup-node from 1 to 4 by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1696">actions/checkout#1696</a></li>
  > <li>Bump actions/upload-artifact from 2 to 4 by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1695">actions/checkout#1695</a></li>
  > <li>README: Suggest <code>user.email</code> to be
  > <code>41898282+github-actions[bot]@users.noreply.github.com</code> by <a
  > href="https://github.com/cory-miller"><code>@​cory-miller</code></a> in
  > <a
  > href="https://redirect.github.com/actions/checkout/pull/1707">actions/checkout#1707</a></li>
  > </ul>
  > <h2>v4.1.4</h2>
  > <ul>
  > <li>Disable <code>extensions.worktreeConfig</code> when disabling
  > <code>sparse-checkout</code> by <a
  > href="https://github.com/jww3"><code>@​jww3</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1692">actions/checkout#1692</a></li>
  > <li>Add dependabot config by <a
  > href="https://github.com/cory-miller"><code>@​cory-miller</code></a> in
  > <a
  > href="https://redirect.github.com/actions/checkout/pull/1688">actions/checkout#1688</a></li>
  > <li>Bump the minor-actions-dependencies group with 2 updates by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1693">actions/checkout#1693</a></li>
  > <li>Bump word-wrap from 1.2.3 to 1.2.5 by <a
  > href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
  > href="https://redirect.github.com/actions/checkout/pull/1643">actions/checkout#1643</a></li>
  > </ul>
  > <h2>v4.1.3</h2>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li><a
  > href="https://github.com/actions/checkout/commit/08c6903cd8c0fde910a37f88322edcfb5dd907a8"><code>08c6903</code></a>
  > Prepare v5.0.0 release (<a
  > href="https://redirect.github.com/actions/checkout/issues/2238">#2238</a>)</li>
  > <li><a
  > href="https://github.com/actions/checkout/commit/9f265659d3bb64ab1440b03b12f4d47a24320917"><code>9f26565</code></a>
  > Update actions checkout to use node 24 (<a
  > href="https://redirect.github.com/actions/checkout/issues/2226">#2226</a>)</li>
  > <li>See full diff in <a
  > href="https://github.com/actions/checkout/compare/v4...v5">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > [![Dependabot compatibility
  > score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=actions/checkout&package-manager=github_actions&previous-version=4&new-version=5)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

### Other

- *(deps)* Update rstest requirement from 0.24.0 to 0.25.0 ([#62](https://github.com/ratatui/tui-widgets/issues/62))
  > Updates the requirements on [rstest](https://github.com/la10736/rstest)
  > to permit the latest version.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/la10736/rstest/releases">rstest's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>0.24.0</h2>
  > <h2>What's Changed</h2>
  > <ul>
  > <li>refactor: use <code>core</code> instead of <code>std</code> by <a
  > href="https://github.com/rnbguy"><code>@​rnbguy</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/283">la10736/rstest#283</a></li>
  > <li>Fix msrv and complete no_std support by <a
  > href="https://github.com/la10736"><code>@​la10736</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/285">la10736/rstest#285</a></li>
  > <li>replace futures with futures-util by <a
  > href="https://github.com/mati865"><code>@​mati865</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/288">la10736/rstest#288</a></li>
  > <li>Introduce Context by <a
  > href="https://github.com/la10736"><code>@​la10736</code></a> in <a
  > href="https://redirect.github.com/la10736/rstest/pull/289">la10736/rstest#289</a></li>
  > </ul>
  > <h2>New Contributors</h2>
  > <ul>
  > <li><a href="https://github.com/rnbguy"><code>@​rnbguy</code></a> made
  > their first contribution in <a
  > href="https://redirect.github.com/la10736/rstest/pull/283">la10736/rstest#283</a></li>
  > <li><a href="https://github.com/mati865"><code>@​mati865</code></a> made
  > their first contribution in <a
  > href="https://redirect.github.com/la10736/rstest/pull/288">la10736/rstest#288</a></li>
  > </ul>
  > <p><strong>Full Changelog</strong>: <a
  > href="https://github.com/la10736/rstest/compare/v0.23.0...v0.24.0">https://github.com/la10736/rstest/compare/v0.23.0...v0.24.0</a></p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/la10736/rstest/blob/master/CHANGELOG.md">rstest's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[0.24.0] 2025/1/1</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>MSRV to 1.70.0 (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/284">#284</a>
  > thanks to <a
  > href="https://github.com/rnbguy"><code>@​rnbguy</code></a>)</li>
  > </ul>
  > <h3>Add</h3>
  > <ul>
  > <li><code>#![no_std]</code> support: now you can use <code>rstest</code>
  > also in <code>no_std</code> lib
  > (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/282">#282</a>
  > thanks to <a
  > href="https://github.com/rnbguy"><code>@​rnbguy</code></a>)</li>
  > <li><code>#[context]</code> to have test function name and other useful
  > thighs on
  > the tip of your fingers (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/177">#177</a>)</li>
  > </ul>
  > <h2>[0.23.0] 2024/9/29</h2>
  > <h3>Add</h3>
  > <ul>
  > <li>You can now use environment variables in <code>#[files]</code> with
  > an optional default value (see <a
  > href="https://redirect.github.com/la10736/rstest/pull/277">#277</a>).</li>
  > <li>You can now set a base_dir for <code>#[files]</code> with the
  > <code>$[base_dir = &quot;...&quot;]</code> attribute (see <a
  > href="https://redirect.github.com/la10736/rstest/pull/277">#277</a>).</li>
  > </ul>
  > <h2>[0.22.0] 2024/8/4</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>Now it's possible destructuring input values both for cases, values
  > and fixtures. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/231">#231</a>
  > for details</li>
  > </ul>
  > <h3>Add</h3>
  > <ul>
  > <li>Implemented <code>#[ignore]</code> attribute to ignore test
  > parameters during fixtures resolution/injection. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/228">#228</a>
  > for details</li>
  > </ul>
  > <h3>Fixed</h3>
  > <ul>
  > <li>Lot of typo in code</li>
  > </ul>
  > <h2>[0.21.0] 2024/6/1</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>Add feature <code>crate-name</code> enabled by default to opt-in
  > crate rename
  > support. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/258">#258</a></li>
  > </ul>
  > <h2>[0.20.0] 2024/5/30</h2>
  > <h3>Add</h3>
  > <ul>
  > <li>Implemented <code>#[by_ref]</code> attribute to take get a local
  > lifetime for test arguments.
  > See <a
  > href="https://redirect.github.com/la10736/rstest/issues/241">#241</a>
  > for more details. Thanks to
  > <a href="https://github.com/narpfel"><code>@​narpfel</code></a> for
  > suggesting it and useful discussions.</li>
  > <li>Support for import <code>rstest</code> with another name. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/221">#221</a></li>
  > </ul>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li>See full diff in <a
  > href="https://github.com/la10736/rstest/compare/v0.24.0...v0.24.0">compare
  > view</a></li>
  > </ul>
  > </details>
  > <br />
  >
  >
  > Dependabot will resolve any conflicts with this PR as long as you don't
  > alter it yourself. You can also trigger a rebase manually by commenting
  > `@dependabot rebase`.

- Added render_stateful_widget method to ScrollView ([#65](https://github.com/ratatui/tui-widgets/issues/65))

- Bump msrv to 1.82.0 ([#74](https://github.com/ratatui/tui-widgets/issues/74))


## [0.4.1] - 2024-11-23

### ⚙️ Miscellaneous Tasks

- Updated the following local packages: tui-scrollview

## [0.3.1] - 2024-10-20

### 🚀 Features

- *(cards)* Add new tui-cards library for playing cards

### 🐛 Bug Fixes

- Broken links from move to tui-widgets

### Other

- Remove patch from main Cargo.toml file that was pointing at a local path ([#38](https://github.com/ratatui/tui-widgets/pull/38))

- *(deps)* Update rstest requirement from 0.22.0 to 0.23.0 ([#41](https://github.com/ratatui/tui-widgets/pull/41))
  > Updates the requirements on [rstest](https://github.com/la10736/rstest)
  > to permit the latest version.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/la10736/rstest/releases">rstest's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>Version 0.22.0</h2>
  > <p>Destructuring input data</p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/la10736/rstest/blob/master/CHANGELOG.md">rstest's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[0.22.0] 2024/8/4</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>Now it's possible destructuring input values both for cases, values
  > and fixtures. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/231">[#231](https://github.com/ratatui/tui-widgets/pull/231)</a>
  > for details</li>
  > </ul>
  > <h3>Add</h3>
  > <ul>
  > <li>Implemented <code>#[ignore]</code> attribute to ignore test
  > parameters during fixtures resolution/injection. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/228">[#228](https://github.com/ratatui/tui-widgets/pull/228)</a>
  > for details</li>
  > </ul>
  > <h3>Fixed</h3>
  > <ul>
  > <li>Lot of typo in code</li>
  > </ul>
  > <h2>[0.21.0] 2024/6/1</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>Add feature <code>crate-name</code> enabled by default to opt-in
  > crate rename
  > support. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/258">[#258](https://github.com/ratatui/tui-widgets/pull/258)</a></li>
  > </ul>
  > <h2>[0.20.0] 2024/5/30</h2>
  > <h3>Add</h3>
  > <ul>
  > <li>Implemented <code>#[by_ref]</code> attribute to take get a local
  > lifetime for test arguments.
  > See <a
  > href="https://redirect.github.com/la10736/rstest/issues/241">[#241](https://github.com/ratatui/tui-widgets/pull/241)</a>
  > for more details. Thanks to
  > <a href="https://github.com/narpfel"><code>@​narpfel</code></a> for
  > suggesting it and useful discussions.</li>
  > <li>Support for import <code>rstest</code> with another name. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/221">[#221](https://github.com/ratatui/tui-widgets/pull/221)</a></li>
  > </ul>
  > <h3>Fixed</h3>
  > <ul>
  > <li>Don't remove Lifetimes from test function if any. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/230">[#230](https://github.com/ratatui/tui-widgets/pull/230)</a>
  > <a href="https://redirect.github.com/la10736/rstest/issues/241">[#241](https://github.com/ratatui/tui-widgets/pull/241)</a>
  > for more details.</li>
  > <li><a
  > href="https://doc.rust-lang.org/std/path/struct.PathBuf.html"><code>PathBuf</code></a>
  > does no longer need to be
  > in scope when using <code>#[files]</code> (see <a
  > href="https://redirect.github.com/la10736/rstest/pull/242">[#242](https://github.com/ratatui/tui-widgets/pull/242)</a>)</li>
  > <li><code>#[from(now::accept::also::path::for::fixture)]</code> See <a
  > href="https://redirect.github.com/la10736/rstest/issues/246">[#246](https://github.com/ratatui/tui-widgets/pull/246)</a>
  > for more details</li>
  > </ul>
  > <h2>[0.19.0] 2024/4/9</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>Defined <code>rust-version</code> for each crate (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/227">[#227](https://github.com/ratatui/tui-widgets/pull/227)</a>)</li>
  > </ul>
  > <h3>Fixed</h3>
  > <ul>
  > <li><code>#[once]</code> fixtures now require the returned type to be
  > <a
  > href="https://doc.rust-lang.org/std/marker/trait.Sync.html"><code>Sync</code></a>
  > to prevent UB
  > when tests are executed in parallel. (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/235">[#235](https://github.com/ratatui/tui-widgets/pull/235)</a></li>
  > </ul>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li>See full diff in <a
  > href="https://github.com/la10736/rstest/compare/v0.22.0...v0.22.0">compare
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

## [0.3.0] - 2024-08-12

Ratatui-0.28.0 compatible release

### ⚙️ Miscellaneous Tasks

- Bump ratatui-macros to 0.5.0
- Bump tui-big-text to 0.6.0
- Bump tui-popup to 0.5.0
- Bump tui-prompts to 0.4.0
- Bump tui-scrollview to 0.4.0

## [0.2.6] - 2024-08-09

### ⚙️ Miscellaneous Tasks

- *(deps)* Bump ratatui-macros and re-enable multiple versions lint

## [0.2.5] - 2024-08-09

### 🐛 Bug Fixes

- Add missing cfg ([#28](https://github.com/ratatui/tui-widgets/pull/28))

### ⚙️ Miscellaneous Tasks

- *(tui-big-text)* Release v0.5.5 ([#25](https://github.com/ratatui/tui-widgets/pull/25))

  > ## 🤖 New release
  >
  > - `tui-big-text`: 0.5.4 -> 0.5.5
  >
  > <details><summary><i><b>Changelog</b></i></summary><p>
  >
  > ## `tui-big-text`
  >
  > <blockquote>
  >
  > ## [0.5.5] - 2024-08-09
  >
  > ### 🐛 Bug Fixes
  >
  > - Update to ratatui 0.28
  > ([[#24](https://github.com/ratatui/tui-widgets/pull/24)](<https://github.com/ratatui/tui-widgets/pull/24>))
  > > Note that for projects that rely on crossterm, Ratatui 0.28.0 now
  > relies internally on Crossterm 0.28.0.
  > > Ratatui release notes highlights: <https://ratatui.rs/highlights/v028/>
  > > See <https://github.com/ratatui-org/ratatui/issues/1298> for notes about
  > crossterm compatibility
  > </blockquote>
  >
  >
  > </p></details>
  >
  > ---
  > This PR was generated with
  > [release-plz](https://github.com/MarcoIeni/release-plz/).

## [0.2.4] - 2024-08-06

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

- *(deps)* Update rstest requirement from 0.21.0 to 0.22.0 ([#21](https://github.com/ratatui/tui-widgets/pull/21))
  > Updates the requirements on [rstest](https://github.com/la10736/rstest)
  > to permit the latest version.
  > <details>
  > <summary>Release notes</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/la10736/rstest/releases">rstest's
  > releases</a>.</em></p>
  > <blockquote>
  > <h2>0.21.0</h2>
  > <p>Use <code>crate-name</code> feature to enable the crate rename
  > support (enabled by default)</p>
  > </blockquote>
  > </details>
  > <details>
  > <summary>Changelog</summary>
  > <p><em>Sourced from <a
  > href="https://github.com/la10736/rstest/blob/master/CHANGELOG.md">rstest's
  > changelog</a>.</em></p>
  > <blockquote>
  > <h2>[0.21.0] 2024/6/1</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>Add feature <code>crate-name</code> enabled by default to opt-in
  > crate rename
  > support. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/258">[#258](https://github.com/ratatui/tui-widgets/pull/258)</a></li>
  > </ul>
  > <h2>[0.20.0] 2024/5/30</h2>
  > <h3>Add</h3>
  > <ul>
  > <li>Implemented <code>#[by_ref]</code> attribute to take get a local
  > lifetime for test arguments.
  > See <a
  > href="https://redirect.github.com/la10736/rstest/issues/241">[#241](https://github.com/ratatui/tui-widgets/pull/241)</a>
  > for more details. Thanks to
  > <a href="https://github.com/narpfel"><code>@​narpfel</code></a> for
  > suggesting it and useful discussions.</li>
  > <li>Support for import <code>rstest</code> with another name. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/221">[#221](https://github.com/ratatui/tui-widgets/pull/221)</a></li>
  > </ul>
  > <h3>Fixed</h3>
  > <ul>
  > <li>Don't remove Lifetimes from test function if any. See <a
  > href="https://redirect.github.com/la10736/rstest/issues/230">[#230](https://github.com/ratatui/tui-widgets/pull/230)</a>
  > <a href="https://redirect.github.com/la10736/rstest/issues/241">[#241](https://github.com/ratatui/tui-widgets/pull/241)</a>
  > for more details.</li>
  > <li><a
  > href="https://doc.rust-lang.org/std/path/struct.PathBuf.html"><code>PathBuf</code></a>
  > does no longer need to be
  > in scope when using <code>#[files]</code> (see <a
  > href="https://redirect.github.com/la10736/rstest/pull/242">[#242](https://github.com/ratatui/tui-widgets/pull/242)</a>)</li>
  > <li><code>#[from(now::accept::also::path::for::fixture)]</code> See <a
  > href="https://redirect.github.com/la10736/rstest/issues/246">[#246](https://github.com/ratatui/tui-widgets/pull/246)</a>
  > for more details</li>
  > </ul>
  > <h2>[0.19.0] 2024/4/9</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>Defined <code>rust-version</code> for each crate (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/227">[#227](https://github.com/ratatui/tui-widgets/pull/227)</a>)</li>
  > </ul>
  > <h3>Fixed</h3>
  > <ul>
  > <li>
  > <p><code>#[once]</code> fixtures now require the returned type to be
  > <a
  > href="https://doc.rust-lang.org/std/marker/trait.Sync.html"><code>Sync</code></a>
  > to prevent UB
  > when tests are executed in parallel. (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/235">[#235](https://github.com/ratatui/tui-widgets/pull/235)</a>
  > for more details)</p>
  > </li>
  > <li>
  > <p><code>#[future(awt)]</code> and <code>#[awt]</code> now properly
  > handle mutable (<code>mut</code>) parameters by treating futures as
  > immutable and
  > treating the awaited rebinding as mutable.</p>
  > </li>
  > </ul>
  > <h2>[0.18.2] 2023/8/13</h2>
  > <h3>Changed</h3>
  > <ul>
  > <li>Now <code>#[files]</code> accept also parent folders (see <a
  > href="https://redirect.github.com/la10736/rstest/issues/205">[#205](https://github.com/ratatui/tui-widgets/pull/205)</a>
  > for more details).</li>
  > </ul>
  > <h2>[0.18.1] 2023/7/5</h2>
  > <!-- raw HTML omitted -->
  > </blockquote>
  > <p>... (truncated)</p>
  > </details>
  > <details>
  > <summary>Commits</summary>
  > <ul>
  > <li>See full diff in <a
  > href="https://github.com/la10736/rstest/compare/v0.21.0...v0.21.0">compare
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

## [0.2.3] - 2024-08-02

### 📚 Documentation

- Clean up changelogs ([#17](https://github.com/ratatui/tui-widgets/pull/17))
  > - removed unnecessary footer comments
  > - removed [unreleased] sections
  > - removed duplicate release notes

### ⚙️ Miscellaneous Tasks

- Remove changelog footer ([#19](https://github.com/ratatui/tui-widgets/pull/19))
  > wrt <https://github.com/ratatui/tui-widgets/pull/18/files#r1701302921>
  >
  > not working as expected with `release-plz`

## [0.2.2] - 2024-07-25

### ⚙️ Miscellaneous Tasks

- Updated the following local packages: tui-big-text

## [0.2.1] - 2024-07-25

### 📚 Documentation

- Update readme / lib.rs links

### ⚙️ Miscellaneous Tasks

- Remove anyhow dependency
  > Replaced with color_eyre generally

- Update bacon config

- Update READMEs and licensing info

## [0.2.0] - 2024-07-25

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

- Fixup readme

- Simplify tui-big-text examples

### ⚙️ Miscellaneous Tasks

- Include commit body in changelog

## [0.1.5] - 2024-07-25

### Other

- Add tui-popup to widgets

## [0.1.4] - 2024-07-24

### 🐛 Bug Fixes

- Remove cargo.lock file
- Delete and backspace behavior for multi-byte characters ([#57](https://github.com/ratatui/tui-widgets/pull/57))
- Fixup tui-prompts version to match crates.io

### ⚙️ Miscellaneous Tasks

- Various fixes / clippy lints ([#6](https://github.com/ratatui/tui-widgets/pull/6))

## [0.1.3](https://github.com/ratatui/tui-widgets/compare/tui-widgets-v0.1.2...tui-widgets-v0.1.3) - 2024-07-24

### Fixed

- *(deps)* update minimal version for futures

### Other

- add workflows and dependabot settings
- Move to tui-widgets repository
