---
artifact_contract: ce-unified-plan/v1
artifact_readiness: implementation-ready
product_contract_source: ce-brainstorm
title: Rust Project Setup - Plan
type: chore
date: 2026-06-29
execution: code
---

# Rust Project Setup — Plan

## Goal Capsule

- **Objective:** Initialize the Knot Forget Rust library crate, configure dev tooling, set up CI, and define the `Knot` domain type — establishing the foundation for all subsequent development.
- **Product authority:** Sam
- **Open blockers:** None
- **Stop condition:** Surface a blocker if `cargo build` fails on the initial scaffold.

---

## Product Contract

### Summary

Initializes the Knot Forget library crate and `Knot` domain type, then establishes lint, format, and CI infrastructure as the foundation for all subsequent development.

### Problem Frame

Knot Forget has no Rust source yet. Before any domain logic can be built, the project needs a compilable crate, a baseline type to anchor the domain model, and tooling that enforces quality from the first commit.

### Requirements

**Crate setup**
- R1. `Cargo.toml` declares a library crate named `knot-forget`, edition 2021, with no binary and no workspace, and a `rust-version` field set to the current stable-2 MSRV.

**Dev tooling**
- R2. `clippy.toml` is present at repo root and sets `msrv` to match `rust-version` in `Cargo.toml`.
- R3. `rustfmt.toml` is present at repo root and sets `edition = "2021"` with any additional style preferences.

**CI**
- R5. `.github/workflows/ci.yml` defines two separate jobs — `lint` and `test` — triggered on `push` and `pull_request` to `main`, both using `ubuntu-latest` and the stable Rust toolchain.
- R6. The `lint` job runs `cargo fmt --check` and `cargo clippy --all-targets --all-features -- -D warnings`.
- R7. The `test` job runs `cargo test`.

**Domain type**
- R8. `src/lib.rs` defines `pub struct Knot` with public fields `pub id: u64`, `pub timestamp: u64` (Unix epoch nanoseconds, caller-provided), and `pub payload: Vec<u8>`.
- R9. `Knot` derives `Debug`, `Clone`, and `PartialEq`.
- R10. `Knot::new(id: u64, timestamp: u64, payload: Vec<u8>) -> Self` is the only constructor; it takes all three fields as arguments with no internal logic.
- R11. A `#[cfg(test)]` module in `src/lib.rs` contains one test that calls `Knot::new()` with known values, asserts all three fields, and also exercises `Clone` and `Debug`.

### Scope Boundaries

**Explicitly deferred:**
- Scheduling or event system
- Serialization or deserialization of `Knot`
- Persistence layer
- Auto-generation of `id` (caller's responsibility)
- Auto-generation of `timestamp` (caller provides epoch nanoseconds)
- Field validation

**Deferred to Follow-Up Work:**
- codegraph initialization — deferred until sufficient codebase exists to benefit from the index

**Outside this project's identity:**
- Binary crate or workspace structure
- Any external crate dependencies beyond the Rust standard library

### Assumptions

- `id` uniqueness is the caller's concern; the struct does not enforce or generate it.
- `timestamp` semantics (epoch nanoseconds as `u64`) are defined here and not revisited until a scheduling layer is needed; `u64` nanoseconds covers ~584 years from epoch and aligns with `std::time::SystemTime` native precision.
- MSRV is set to current stable at time of `cargo init` and follows an N-2 ongoing policy (latest stable minus 2 releases).

---

## Planning Contract

### Summary

Four units in dependency order: crate init → domain type → tooling config → CI. The `Knot` struct comes before tooling config so clippy has something to check in U3.

### Key Technical Decisions

KTD1. **`-D warnings` as a CI flag, not `#![deny(warnings)]` in source.**
`#![deny(warnings)]` in source breaks local builds whenever upstream lints change. Passing `-- -D warnings` to the CI command keeps strict checking in CI without poisoning developer builds.

KTD2. **`msrv` in `clippy.toml` mirrors `rust-version` in `Cargo.toml`.**
Clippy uses `msrv` to suppress lint suggestions that require a newer Rust version. Keeping the two fields in sync avoids spurious warnings on patterns that are valid at the declared MSRV.

KTD3. **`--all-targets --all-features` in the clippy command.**
Ensures the `#[cfg(test)]` module is also linted, not just the library target.

### Sequencing

U1 (crate init) must come first. U2 (Knot struct) and U3 (tooling config) both depend only on U1 and can be done in either order. U4 (CI) depends on U1–U3 being in place.

**Product Contract preservation:** R8 updated to make field visibility explicit (`pub`); R11 updated to clarify the single test also exercises `Clone` and `Debug`; R6 updated to add `cargo fmt --check` to the lint job (format enforcement was in the Verification Contract but missing from CI). codegraph (brainstorm R4) moved to Deferred to Follow-Up Work at user's direction.

---

## Implementation Units

### U1. Initialize Cargo library crate

**Goal:** Create `Cargo.toml` as a library crate with the correct name, edition, and MSRV.

**Requirements:** R1

**Dependencies:** none

**Files:** `Cargo.toml`

**Approach:** Run `cargo init --lib` then edit `Cargo.toml` to set `name = "knot-forget"`, `edition = "2021"`, and `rust-version` to the current stable-2 release. Verify that only a `[lib]` target is present and no `[[bin]]` entry exists.

To determine the MSRV: run `rustc --version`, identify the current stable minor version, subtract 2.

**Test scenarios:**
- `cargo build` exits 0 with no errors.
- `cargo metadata --no-deps` shows a single `lib` target; no `bin` targets present.
- `Cargo.toml` contains `edition = "2021"` and a non-empty `rust-version` field.

**Verification:** `cargo build` exits 0.

---

### U2. Define Knot struct, constructor, and unit test

**Goal:** Define the `Knot` domain type in `src/lib.rs` with its constructor and the required unit test.

**Requirements:** R8, R9, R10, R11

**Dependencies:** U1

**Files:** `src/lib.rs`

**Approach:** Replace the entire contents of `src/lib.rs` generated by `cargo init --lib` with the `Knot` definition. Define `pub struct Knot` with three public fields. Apply `#[derive(Debug, Clone, PartialEq)]`. Implement `pub fn new(id: u64, timestamp: u64, payload: Vec<u8>) -> Self` as a plain struct literal with no internal logic. Add a `#[cfg(test)]` module with one `#[test]` function that covers field values, clone equality, and debug formatting.

**Test scenarios (all in one `#[test]` function):**
- `Knot::new(1, 1_000_000_000, vec![0x42])` produces a struct where `id == 1`, `timestamp == 1_000_000_000`, and `payload == vec![0x42]`.
- `knot.clone() == knot` holds (exercises `Clone` and `PartialEq`).
- `format!("{:?}", knot)` does not panic and contains the struct name (exercises `Debug`).

**Verification:** `cargo test` exits 0 with one test passing.

---

### U3. Configure Clippy and Rustfmt

**Goal:** Add `clippy.toml` and `rustfmt.toml` at repo root.

**Requirements:** R2, R3

**Dependencies:** U1

**Files:** `clippy.toml`, `rustfmt.toml`

**Approach:**
- `clippy.toml`: set `msrv` to the same value as `rust-version` in `Cargo.toml`.
- `rustfmt.toml`: set `edition = "2021"`; add other formatting preferences (e.g. `max_width`) as the developer sees fit.

**Patterns to follow:** `edition` in `rustfmt.toml` must match `Cargo.toml`; `msrv` in `clippy.toml` must match `rust-version` in `Cargo.toml`.

**Test scenarios:**
- `cargo clippy --all-targets --all-features -- -D warnings` exits 0 (no warnings on the initial codebase).
- `cargo fmt --check` exits 0 (no formatting diffs on the initial codebase).

**Verification:** both commands above exit 0.

---

### U4. Set up GitHub Actions CI

**Goal:** Create `.github/workflows/ci.yml` with separate `lint` and `test` jobs triggered on push and PR to `main`.

**Requirements:** R5, R6, R7

**Dependencies:** U1, U2, U3

**Files:** `.github/workflows/ci.yml`

**Approach:** Two top-level jobs so they can fail independently. Both use `ubuntu-latest` and `dtolnay/rust-toolchain@stable`. Each job must include `actions/checkout@v4` as its first step before the toolchain action. Trigger on `push` and `pull_request` with `branches: [main]`.
- `lint` job: `cargo fmt --check`, then `cargo clippy --all-targets --all-features -- -D warnings`
- `test` job: `cargo test`

No matrix builds or caching required for the initial setup.

**Test scenarios:**
- Pushing this branch with a PR targeting `main` triggers both jobs.
- A clippy warning causes `lint` to fail without blocking `test`.
- A failing test causes `test` to fail without blocking `lint`.
- Both jobs pass on the current codebase.

**Verification:** CI is green on first push.

---

## Verification Contract

| Check | Command | Done signal |
|---|---|---|
| Compiles | `cargo build` | Exit 0, no errors |
| Tests pass | `cargo test` | Exit 0, one test green |
| Lint clean | `cargo clippy --all-targets --all-features -- -D warnings` | Exit 0, no warnings |
| Format consistent | `cargo fmt --check` | Exit 0, no diffs |
| CI green | Push branch, open PR targeting `main` | Both `lint` and `test` jobs pass |

---

## Definition of Done

- `cargo build` exits 0.
- `cargo test` exits 0 with one test passing.
- `cargo clippy --all-targets --all-features -- -D warnings` exits 0.
- `cargo fmt --check` exits 0.
- CI is green on the PR for this branch.
- No dead-end or experimental code left in the diff.
