# Repository Guidelines

## Project Structure & Module Organization

- Workspace root (`Cargo.toml`) aggregates all crates; shared config lives beside it
  (`rustfmt.toml`, `bacon.toml`, `cliff.toml`, `release-plz.toml`, `typos.toml`).
- Main facade crate: `src/lib.rs` re-exports the widget crates behind feature flags.
- Widget crates live in `tui-*` directories (e.g., `tui-prompts`, `tui-scrollview`), each with its
  own `Cargo.toml`, `src/`, `CHANGELOG.md`, `README.md`, and often `examples/`.
- Keep new code and tests inside the relevant crate; avoid adding ad hoc modules under the root
  facade.

## Build, Test, and Development Commands

- List project helper commands: `just --list`.
- Format: `just fmt` (check only: `just fmt-check`). These run `cargo +nightly fmt` because
  `rustfmt.toml` uses unstable formatting options such as comment wrapping and grouped imports.
- Lint: `cargo clippy --all-targets --all-features --workspace` (pedantic/nursery enabled; fix or
  justify warnings).
- Stable Clippy gate: `just clippy-stable` (runs `cargo +stable clippy --all-targets
  --all-features --workspace -- -D warnings`).
- Beta Clippy early-warning gate: `just clippy-beta` (same command on `+beta` to catch lints before
  they reach stable). This repo treats Clippy warnings as a PR-status signal, so beta helps find
  upcoming lint failures while they are still maintenance work instead of inherited breakage on
  unrelated PRs.
- Run both Clippy gates: `just clippy-all`.
- Test full workspace: `cargo test --all-features --workspace`.
- Test a single crate: `cargo test -p tui-prompts --all-features -- --nocapture`.
- Docs smoke test: `cargo doc --all-features --workspace`.

## Coding Style & Naming Conventions

- Rust 2021 edition; follow `rustfmt` defaults (4-space indent, trailing commas where helpful).
- Module/files/functions use `snake_case`; types/traits/enums use `CamelCase`; feature flags match
  crate names (`bar-graph`, `prompts`, etc.).
- Prefer explicit types where clarity matters; keep imports ordered and minimal.
- When adding lints or allow attributes, scope them narrowly and document the reason.

## Documentation Standards

- Target docs.rs quality: prioritize clear behavior explanations, usage guidance, and edge cases.
- Mirror depth seen in `ratatui`, `tokio`, `axum`, and Rust std docs: describe invariants,
  lifetimes, side effects, and feature-flag impacts.
- Include runnable examples where feasible; prefer `///` doc tests that compile and demonstrate
  output.
- Prefer `ratatui_core` types in widget crate docs/examples unless a user-facing example needs
  `ratatui`.
- Use reference-style links for external and internal types (e.g. `[`Rect`]` with a link ref at the
  end of the doc comment).
- Keep crate-level docs updated when adding widgets or major behaviors; link to examples for
  interactive flows.
- Document safety/contracts for unsafe or performance-sensitive code; call out terminal assumptions
  (color, size) and error conditions.

## Testing Guidelines

- Use Rust’s built-in test framework; `rstest` is available for parameterized cases.
- Co-locate unit tests with implementation (use a `mod tests` in the same module file).
- Prefer verb-phrase test names that describe behavior (e.g., `clears_drag_on_pointer_up`).
- Cover happy paths, edge cases (terminal sizing, empty data), and feature-flagged code paths.
- Include examples under `examples/` when a widget or mode benefits from interactive demonstration
  (`cargo run -p tui-big-text --example <name>`).

## Commit & Pull Request Guidelines

- Commit messages follow Conventional Commits (`feat`, `fix`, `docs`, `chore`, `test`, `build`,
  `ci`, `perf`, `refactor`, `revert`) with optional scope (`feat(prompts): add spinner prompt`).
- Keep PRs focused on one change set; include a concise summary, linked issue, and notes on affected
  crate(s) or feature flags.
- State which commands you ran (fmt/clippy/tests) and attach terminal screenshots or gifs when UI
  output changed.
- Do not edit changelogs; release-plz and git-cliff update them during release.
- Document behavior or API changes (README or crate docs) and add tests alongside fixes or features.

## Feature Flags & Workspace Tips

- Default features enable all widgets; use `--no-default-features --features prompts,scrollview` to
  narrow builds.
- Avoid cross-crate coupling; expose shared helpers via crate-local modules rather than the root
  facade unless intended for public API.
