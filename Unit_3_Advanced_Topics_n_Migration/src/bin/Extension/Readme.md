# PyO3 Rust Extension for Python — Command Sequence (uv workflow)

## 1. Initial setup

```bash
uv init Extension
cd Extension
uv add --dev maturin
uv run maturin init
```

## 2. After writing/editing Cargo.toml, pyproject.toml, src/lib.rs

```bash
uv run maturin develop --release
```

Verify the output includes `Compiling Extension v0.1.0 (...)` and a
`Finished 'release' profile [optimized] target(s) in X.Xs` line — if those
lines are missing, the build was skipped and the installed wheel is stale.

## 3. Run the Python test script

```bash
uv run python test.py
```

## 4. If a rebuild is silently skipped or a stale-wheel error persists

```bash
cargo clean
uv run maturin develop --release
uv run python test.py
```

## 5. If uv's own cache seems to be the culprit

```bash
uv cache clean
uv run maturin develop --release
uv run python test.py
```

## Standard edit loop going forward

```bash
# edit src/lib.rs
uv run maturin develop --release
uv run python test.py
```

Never rely on `uv run python test.py` alone to pick up Rust source changes —
always run `uv run maturin develop --release` first after any edit to
`src/lib.rs`, `Cargo.toml`, or `pyproject.toml`.
