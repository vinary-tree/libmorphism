#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
evidence="$root/target/verification"
temporary="$evidence/tmp"
cargo_target="$root/target/cargo"

mkdir -p "$temporary" "$cargo_target"

if [[ "${LIBMORPHISM_RUST_SCOPED:-0}" != "1" ]]; then
  exec systemd-run --user --scope \
    -p MemoryMax=4G \
    -p MemorySwapMax=0 \
    -p CPUQuota=100% \
    -p TasksMax=64 \
    --setenv=LIBMORPHISM_RUST_SCOPED=1 \
    --setenv=CARGO_BUILD_JOBS=1 \
    --setenv=CARGO_INCREMENTAL=0 \
    --setenv=CARGO_TARGET_DIR="$cargo_target" \
    --setenv=PYTHONDONTWRITEBYTECODE=1 \
    --setenv=RUFF_CACHE_DIR="$root/target/ruff-cache" \
    --setenv=TMPDIR="$temporary" \
    -- "$root/scripts/verify-rust.sh"
fi

export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$cargo_target}"
export PYTHONDONTWRITEBYTECODE=1
export RUFF_CACHE_DIR="$root/target/ruff-cache"
export TMPDIR="$temporary"

run_logged() {
  local name="$1"
  shift
  "$@" 2>&1 | tee "$evidence/$name.log"
}

cd "$root"
run_logged shell-syntax bash -n \
  scripts/render-diagrams.sh scripts/verify-docs.sh \
  scripts/verify-formal.sh scripts/verify-release.sh scripts/verify-rust.sh
run_logged shellcheck shellcheck \
  scripts/render-diagrams.sh scripts/verify-docs.sh \
  scripts/verify-formal.sh scripts/verify-release.sh scripts/verify-rust.sh
run_logged ruff-check ruff check scripts/check-refinement.py scripts/check-release-ref.py
run_logged ruff-format ruff format --check scripts/check-refinement.py scripts/check-release-ref.py
run_logged cargo-fmt cargo fmt --all -- --check
run_logged cargo-check cargo check --locked --all-targets --all-features
run_logged cargo-check-no-default cargo check --locked --lib --no-default-features
run_logged cargo-clippy cargo clippy --locked --all-targets --all-features -- -D warnings
run_logged cargo-test cargo test --locked --all-targets --all-features
run_logged cargo-doctest cargo test --locked --doc --all-features
RUSTDOCFLAGS="-D warnings" run_logged cargo-doc cargo doc --locked --no-deps --all-features
run_logged refinement scripts/check-refinement.py
