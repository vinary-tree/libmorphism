#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
evidence="$root/target/verification"
temporary="$evidence/tmp"
cargo_target="$root/target/cargo"
msrv_target="$root/target/cargo-msrv"

mkdir -p "$temporary" "$cargo_target" "$msrv_target"

if [[ "${LIBMORPHISM_RELEASE_SCOPED:-0}" != "1" ]]; then
  exec systemd-run --user --scope \
    -p MemoryMax=4G \
    -p MemorySwapMax=0 \
    -p CPUQuota=100% \
    -p TasksMax=64 \
    --setenv=LIBMORPHISM_RELEASE_SCOPED=1 \
    --setenv=LIBMORPHISM_RUST_SCOPED=1 \
    --setenv=LIBMORPHISM_FORMAL_SCOPED=1 \
    --setenv=LIBMORPHISM_DOCS_SCOPED=1 \
    --setenv=CARGO_BUILD_JOBS=1 \
    --setenv=CARGO_INCREMENTAL=0 \
    --setenv=CARGO_TARGET_DIR="$cargo_target" \
    --setenv=PYTHONDONTWRITEBYTECODE=1 \
    --setenv=RUFF_CACHE_DIR="$root/target/ruff-cache" \
    --setenv=TMPDIR="$temporary" \
    --setenv=JAVA_TOOL_OPTIONS="-Xmx1024m -Djava.awt.headless=true -Djava.io.tmpdir=$temporary" \
    -- "$root/scripts/verify-release.sh"
fi

export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$cargo_target}"
export PYTHONDONTWRITEBYTECODE=1
export RUFF_CACHE_DIR="${RUFF_CACHE_DIR:-$root/target/ruff-cache}"
export TMPDIR="$temporary"

# Release evidence must not inherit host-local registry patches. Preserve the shared Cargo cache
# while replacing only an ambient config file with an empty, read-only file inside the scope.
if [[ "${LIBMORPHISM_CARGO_CONFIG_ISOLATED:-0}" != "1" ]]; then
  cargo_config="${CARGO_HOME:-$HOME/.cargo}/config.toml"
  if [[ -f "$cargo_config" ]]; then
    command -v bwrap >/dev/null 2>&1 || {
      echo "bwrap is required to isolate the ambient Cargo config" >&2
      exit 1
    }
    empty_config="$evidence/empty-cargo-config"
    install -m 0444 /dev/null "$empty_config"
    exec bwrap \
      --bind / / \
      --dev-bind /dev /dev \
      --proc /proc \
      --ro-bind "$empty_config" "$cargo_config" \
      --chdir "$root" \
      --setenv LIBMORPHISM_CARGO_CONFIG_ISOLATED 1 \
      "$root/scripts/verify-release.sh"
  fi
fi

run_logged() {
  local name="$1"
  shift
  "$@" 2>&1 | tee "$evidence/$name.log"
}

cd "$root"
run_logged shell-syntax bash -n \
  scripts/render-diagrams.sh scripts/verify-docs.sh scripts/verify-formal.sh \
  scripts/verify-release.sh scripts/verify-rust.sh
run_logged shellcheck shellcheck \
  scripts/render-diagrams.sh scripts/verify-docs.sh scripts/verify-formal.sh \
  scripts/verify-release.sh scripts/verify-rust.sh
run_logged yamllint yamllint \
  .github/workflows/ci.yml .github/workflows/release.yml
run_logged ruff-check ruff check scripts/check-refinement.py scripts/check-release-ref.py
run_logged ruff-format ruff format --check scripts/check-refinement.py scripts/check-release-ref.py
run_logged cargo-fmt cargo fmt --all -- --check
run_logged cargo-check cargo check --locked --all-targets --all-features
run_logged cargo-check-no-std cargo check --locked --lib --no-default-features

msrv_cargo="$(rustup which --toolchain 1.85.0 cargo)"
msrv_command=("$msrv_cargo")
run_logged cargo-msrv-no-std env CARGO_TARGET_DIR="$msrv_target" \
  "${msrv_command[@]}" check --locked --lib --no-default-features
run_logged cargo-msrv-std env CARGO_TARGET_DIR="$msrv_target" \
  "${msrv_command[@]}" check --locked --lib

run_logged rust scripts/verify-rust.sh
run_logged release-ref scripts/check-release-ref.py --self-test
run_logged formal scripts/verify-formal.sh all
run_logged docs scripts/verify-docs.sh
run_logged package-list cargo package --locked --allow-dirty --list
run_logged package-first cargo package --locked --allow-dirty
version="$(sed -n 's/^version = "\(.*\)"$/\1/p' Cargo.toml | head -n1)"
archive="$CARGO_TARGET_DIR/package/libmorphism-$version.crate"
sha256sum "$archive" >"$evidence/package-first.sha256"
run_logged package-second cargo package --locked --allow-dirty
sha256sum "$archive" >"$evidence/package-second.sha256"
cmp "$evidence/package-first.sha256" "$evidence/package-second.sha256"
