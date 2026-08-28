#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
evidence="$root/target/verification"
temporary="$evidence/tmp"

mkdir -p "$temporary"

if [[ "${LIBMORPHISM_DOCS_SCOPED:-0}" != "1" ]]; then
  exec systemd-run --user --scope \
    -p MemoryMax=4G \
    -p MemorySwapMax=0 \
    -p CPUQuota=400% \
    -p TasksMax=64 \
    --setenv=LIBMORPHISM_DOCS_SCOPED=1 \
    --setenv=TMPDIR="$temporary" \
    --setenv=JAVA_TOOL_OPTIONS="-Xmx1024m -Djava.awt.headless=true -Djava.io.tmpdir=$temporary" \
    -- "$root/scripts/verify-docs.sh"
fi

mkdir -p "$evidence"
export TMPDIR="$temporary"
export JAVA_TOOL_OPTIONS="${JAVA_TOOL_OPTIONS:--Xmx1024m -Djava.awt.headless=true -Djava.io.tmpdir=$temporary}"
"$root/scripts/render-diagrams.sh"
vinary-doc-lint check "$root" --diagram-tools --format json 2>&1 \
  | tee "$evidence/vinary-doc-lint.json"
jq -e '
  all(.files[];
    ((.diagnostics // []) | length) == 0 and
    ((.changes // []) | length) == 0
  )
' "$evidence/vinary-doc-lint.json" >/dev/null
