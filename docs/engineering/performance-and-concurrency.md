# Performance and Concurrency

## Performance value of categorical laws

Category theory does not make code faster by itself. Its performance value is indirect but
important: lawful composition exposes safe regrouping, typed endpoints prevent wasted invalid
plans, identities eliminate optional-stage branching, homomorphisms permit computation in a more
efficient representation, and explicit effects reveal parallel work.

The production rule is **prove abstractly, execute concretely**. Phantom endpoint markers,
monomorphized adapters, compact enums, and validated witnesses can erase to ordinary Rust data.
Heap-allocated categorical objects, dynamic dispatch in inner loops, and generalized higher-kinded
encodings are out of scope unless measurements establish a net benefit.

## Planning versus execution

The optimizer should construct a dependency directed acyclic graph (DAG), use libvgraph to obtain
deterministic waves, and execute only effect-compatible nodes within a wave concurrently. A
barrier separates waves. Results commit in stable plan order, not completion order.

```text
procedure EXECUTE-PLAN(validated_plan, resource_budget)
    dependency_graph := BUILD-DEPENDENCIES(validated_plan)
    waves := LIBVGRAPH-WAVES(dependency_graph)

    for each wave in deterministic order
        batches := PARTITION-BY-EFFECT-COMPATIBILITY(wave)
        for each batch in deterministic order
            reserve bounded memory and task permits
            execute batch nodes concurrently
            collect outcomes without publishing them
        validate every outcome in stable node order
        commit witnessed outcomes in stable node order
    return the final witnessed object
end procedure
```

This schedule allows pure parsing transforms, independent fiber calculations, disjoint e-class
analyses, and read-only CPG queries to overlap. State writes, shared e-graph mutation, publication,
and evidence streams whose order is observable require explicit serialization or partitioning.

## Algebra-aware parallel reductions

Associativity permits tree-shaped reduction. Commutativity permits arbitrary operand order.
Idempotence permits duplicate suppression. These permissions are cumulative and must never be
inferred from a method name.

| Laws available | Safe transformation |
|---|---|
| Associative only | Parenthesize as a balanced tree while preserving operand order |
| Associative and commutative | Schedule operands in any deterministic order |
| Associative, commutative, and idempotent | Deduplicate repeated operands before reduction |
| Monoid | Partition and reduce chunks using the identity for empty chunks |
| Semiring distributivity | Factor or distribute weighted expressions subject to size budgets |

Floating-point operations, left-biased vector union, diagnostic ordering, and approximate
fixed-point widening can violate one or more laws. Such domains need purpose-specific contracts,
not blanket parallel reduction.

## Equality-saturation concurrency

Replete should own synchronization inside its e-graph representation. lling-llang may parallelize
outside Replete by saturating independent components or running read-only analyses, but it must not
concurrently mutate one e-graph unless Replete's API explicitly guarantees that pattern.

Deterministic equality saturation requires stable rewrite identifiers, stable match ordering at
commit boundaries, deterministic extraction tie-breaking, and explicit iteration and node budgets.
Parallel discovery may be nondeterministic internally if the committed congruence closure and
evidence are deterministic and validated; otherwise the result must declare the nondeterminism.

## Resource model

Every plan node should expose conservative estimates for work, peak resident memory, task count,
and possible expansion. Arithmetic uses checked or saturating operations. Admission fails closed
when estimates overflow or a budget cannot be reserved.

Heavy development commands follow the same discipline:

- formal and documentation commands run in a 4 GiB systemd scope;
- Kani and CBMC use a 2 GiB systemd scope;
- swap is disabled with `MemorySwapMax=0`;
- CPU and task counts are capped; and
- Cargo uses one build job unless a separately reviewed command proves a safer bound.

Heap profiling, when needed, is headless: record with `heaptrack --record-only` and inspect with
`heaptrack_print`. No verification or profiling workflow may launch a graphical user interface.

## Measurement plan

The category contract introduces no performance acceptance claim. Later implementation tasks must
measure end-to-end throughput, peak resident memory, allocation count, cache behavior, plan-build
latency, saturation growth, and deterministic parallel scaling on representative Vinary inputs.
Benchmarks compare an implementation to its prior implementation or an independent oracle; they
must not repeat the already settled Tarjan-versus-Kosaraju study from libcpg.
