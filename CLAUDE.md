# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Knot Forget is a lightweight Rust engine for time and reminder management.

## Commands

```bash
cargo build          # build
cargo test           # all tests
cargo test <name>    # single test by name
cargo clippy         # lint
cargo fmt            # format
```

## Git Workflow

GitHub Flow: all feature branches start from `main` and merge back into `main` via PR.

Commits follow [Conventional Commits](https://www.conventionalcommits.org): `type(scope): description`. Valid types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`.

```bash
git checkout main && git pull
git checkout -b <type>/<issue-number>-<short-description>
# work...
git push -u origin <branch>
# open PR → merge into main (merge commit only — squash and rebase disabled)
```

Each commit must be atomic and scoped to a single concern. When adding a feature, commit the implementation files and the test files separately. The same applies to documentation changes — always in their own commit.