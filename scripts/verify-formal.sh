#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
target="${1:-all}"
evidence="$root/target/verification"

if [[ "${LIBMORPHISM_FORMAL_SCOPED:-0}" != "1" ]]; then
  exec systemd-run --user --scope \
    -p MemoryMax=4G \
    -p MemorySwapMax=0 \
    -p CPUQuota=400% \
    -p TasksMax=64 \
    --setenv=LIBMORPHISM_FORMAL_SCOPED=1 \
    --setenv=CARGO_BUILD_JOBS=1 \
    -- "$root/scripts/verify-formal.sh" "$target"
fi

mkdir -p "$evidence"
export JAVA_TOOL_OPTIONS="${JAVA_TOOL_OPTIONS:--Xmx1024m -Djava.awt.headless=true}"

require_tool() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "required tool is unavailable: $1" >&2
    exit 1
  fi
}

verify_rocq() {
  require_tool coqc
  local log="$evidence/rocq.log"
  (
    cd "$root/formal/rocq"
    for source in Composition.v Algebra.v IndexedFamilies.v Countermodels.v; do
      coqc -Q . Libmorphism "$source"
    done
  ) 2>&1 | tee "$log"

  local closed_count
  closed_count="$(rg -c '^Closed under the global context$' "$log")"
  if [[ "$closed_count" != "12" ]]; then
    echo "expected 12 axiom-free Rocq assumption reports; found $closed_count" >&2
    exit 1
  fi

  if rg -n '\b(Axiom|Parameter|Admitted|admit)\b' "$root/formal/rocq"/*.v; then
    echo "forbidden Rocq trust escape found" >&2
    exit 1
  fi
}

verify_tla() {
  require_tool tla2sany
  require_tool tlc
  local log="$evidence/tla.log"
  (
    cd "$root/formal/tla"
    tla2sany PrecisionLifecycle.tla
    tlc -workers 1 -config PrecisionLifecycle.cfg PrecisionLifecycle.tla
  ) 2>&1 | tee "$log"

  rg -q '^Model checking completed\. No error has been found\.$' "$log"
  rg -q '^26 states generated, 25 distinct states found, 0 states left on queue\.$' "$log"
}

verify_tlaps() {
  require_tool tlapm
  local log="$evidence/tlaps.log"
  (
    cd "$root/formal/tla"
    tlapm PrecisionLifecycle.tla
  ) 2>&1 | tee "$log"

  rg -q 'All 3 obligations proved\.' "$log"
}

verify_z3() {
  require_tool z3
  local log="$evidence/z3.log"
  z3 -smt2 "$root/formal/smt/contracts.smt2" 2>&1 | tee "$log"

  mapfile -t actual < <(rg '^(un)?sat$' "$log")
  local expected=(unsat unsat sat sat)
  if [[ "${actual[*]:-}" != "${expected[*]}" ]]; then
    echo "unexpected solver result sequence: ${actual[*]:-<none>}" >&2
    exit 1
  fi

  for case_name in \
    exact-promotion-is-impossible \
    unconfirmed-exact-validation-is-impossible \
    semiring-times-can-differ-from-meet \
    noninjective-map-can-fail-order-reflection; do
    rg -q "^CASE ${case_name}$" "$log"
  done
}

case "$target" in
  rocq) verify_rocq ;;
  tla) verify_tla ;;
  tlaps) verify_tlaps ;;
  z3) verify_z3 ;;
  all)
    verify_rocq
    verify_tla
    verify_tlaps
    verify_z3
    ;;
  *)
    echo "usage: $0 {rocq|tla|tlaps|z3|all}" >&2
    exit 2
    ;;
esac
