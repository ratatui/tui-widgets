# Changelog

All notable changes to this project will be documented in this file.

## [0.3.5](https://crates.io/crates/tui-bar-graph/0.3.5) - 2026-06-14

- *(bar-graph)* Render by reference ([#289](https://github.com/ratatui/tui-widgets/pull/289))

## [0.3.4](https://crates.io/crates/tui-bar-graph/0.3.4) - 2026-06-14

- Modernize tui-bar-graph examples ([#275](https://github.com/ratatui/tui-widgets/pull/275))
- Document widget examples ([#286](https://github.com/ratatui/tui-widgets/pull/286))

## [0.3.3](https://crates.io/crates/tui-bar-graph/0.3.3) - 2026-04-04

Dependency updates.

## [0.3.2](https://crates.io/crates/tui-bar-graph/0.3.2) - 2026-03-29

Maintenance updates.

## [0.3.1](https://crates.io/crates/tui-bar-graph/0.3.1) - 2025-12-27

- Refresh widget docs ([#148](https://github.com/ratatui/tui-widgets/pull/148))

## [0.3.0](https://crates.io/crates/tui-bar-graph/0.3.0) - 2025-12-27

- *(bar-graph)* **Breaking:** Add block octant characters ([#116](https://github.com/ratatui/tui-widgets/pull/116))
  Update exhaustive matches on `BarStyle` to handle the new `Octant` variant.
- **Breaking:** Migrate to ratatui 0.30 ([#120](https://github.com/ratatui/tui-widgets/pull/120))
  See <https://github.com/joshka/tui-widgets/blob/main/BREAKING_CHANGES.md>

## [0.2.0](https://crates.io/crates/tui-bar-graph/0.2.0) - 2025-11-02

- *(bar-graph)* **Breaking:** Support boxed gradients ([#66](https://github.com/ratatui/tui-widgets/pull/66))
  `BarGraph` now has a lifetime parameter for its gradient. Update explicit type annotations
  to include the lifetime; most inferred usages are unchanged.
- *(bar-graph)* Add Quadrant style ([#80](https://github.com/ratatui/tui-widgets/pull/80))
- Broken bar graph test
- Clippy lints ([#81](https://github.com/ratatui/tui-widgets/pull/81))
- Use f64:midpoint ([#83](https://github.com/ratatui/tui-widgets/pull/83))
- Bump msrv to 1.82.0 ([#74](https://github.com/ratatui/tui-widgets/issues/74))

## [0.1.1](https://crates.io/crates/tui-bar-graph/0.1.1) - 2025-03-05

Maintenance updates.

## [0.1.0](https://crates.io/crates/tui-bar-graph/0.1.0) - 2025-03-04

- Add new tui-bar-graph crate ([#63](https://github.com/ratatui/tui-widgets/pull/63))
