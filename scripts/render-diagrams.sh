#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
evidence="$root/target/verification"

if [[ "${LIBMORPHISM_DOCS_SCOPED:-0}" != "1" ]]; then
  exec systemd-run --user --scope \
    -p MemoryMax=4G \
    -p MemorySwapMax=0 \
    -p CPUQuota=400% \
    -p TasksMax=64 \
    --setenv=LIBMORPHISM_DOCS_SCOPED=1 \
    -- "$root/scripts/render-diagrams.sh"
fi

mkdir -p "$evidence"
export JAVA_TOOL_OPTIONS="${JAVA_TOOL_OPTIONS:--Xmx1024m -Djava.awt.headless=true}"

plantuml -tsvg \
  "$root/docs/diagrams/algebra-boundaries.puml" \
  "$root/docs/diagrams/precision-lifecycle.puml" \
  "$root/docs/diagrams/ownership-boundaries.puml" \
  "$root/docs/diagrams/optimization-pipeline.puml" \
  "$root/docs/diagrams/formal-first-sequence.puml" \
  2>&1 | tee "$evidence/plantuml.log"
