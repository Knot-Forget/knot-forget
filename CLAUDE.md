# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Knot Forget is a lightweight Rust engine for time and reminder management. The project is in early scaffold stage — no source code exists yet.

## Commands

Once `Cargo.toml` is initialized:

```bash
cargo build          # build
cargo run            # run
cargo test           # all tests
cargo test <name>    # single test by name
cargo clippy         # lint
cargo fmt            # format
```

## Git Workflow

GitHub Flow: all feature branches start from `main` and merge back into `main` via PR.

```bash
git checkout main && git pull
git checkout -b <type>/<issue-number>-<short-description>
# work...
git push -u origin <branch>
# open PR → merge into main
```

Each commit must be atomic and scoped to a single concern. When adding a feature, commit the implementation files and the test files separately. The same applies to documentation changes — always in their own commit.