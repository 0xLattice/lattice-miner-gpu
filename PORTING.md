# Platform worker integration

This existing Rust repository is reused at baseline
`0xLattice/lattice-miner-gpu@74285a9d4fe5f4eeb25ce6acb251519cc6f014ed`.
Its assignment/result contract was checked against
`0xLattice/lattice-node@3df9e0a786c7b103a3da88d1c3881efc81706255`, specifically
`docs/mining-workers.md`, `MiningWorkerProcess.swift` and
`LatticeMinerCore/MinerLoopLogic.swift`.

Target parsing now accepts nonempty one-to-64 digit hex, including odd-length
short targets, matching the coordinator parser. An optional lowercase `0x`
prefix is accepted; hex digits may be uppercase. Empty strings previously
silently became zero and are now rejected. The byte-prefix parser remains
separate because its input must contain whole bytes.

Metal device/kernel/pipeline failures return explicit assignment errors. After
waiting, command status must be Completed before reading the result. An error
must never masquerade as an exhausted range. Returned indices are checked
against the dispatched range and the digest is recomputed on the host.

Validation in this checkout:

- CPU target/range tests and real worker-process JSON contract tests pass.
- An actual Metal run matches CPU results for 52 cases across 13 prefix lengths,
  multiple-winner searches, zero targets, and wrapping ranges; empty ranges also
  return without opening a GPU device. The lowest winning assignment offset is
  required so merged-target searches can resume without skipping candidates.
- Default strict Clippy passes. CUDA and OpenCL optional features compile together.
- Actual OpenCL oracle passed all 42 prefix/start cases, a 40-million-attempt wrapped range, and a 100,000-attempt exhausted range on this host (122.62 seconds).
- CUDA runtime validation requires NVIDIA hardware and remains unverified here.

This is a worker compatibility slice. Coordinator template polling, staleness,
merged-target continuation, submission and bounded child-process teardown remain
separate platform work. No live mining, external submission or deployment was
performed. The inherited Metal dependency currently reports a future Rust
compatibility warning in transitive `block` 0.1.6; migration is still outstanding.

## Hardware validation checkpoint — 2026-11-02

The original checkout matched all 21 tracked files in the canonical publication
checkout before this documentation update, including `Cargo.toml` and
`Cargo.lock`.

On macOS arm64, the system Metal device reported **Apple M5** with unified
memory. This existing offline test passed in 0.28 seconds:

```sh
cargo test --offline --locked --release metal_matches_cpu_padding_boundaries_and_lowest_winners -- --ignored --exact contract_tests::metal_matches_cpu_padding_boundaries_and_lowest_winners
```

The test compares 52 Metal searches against the CPU oracle across 13 prefix
lengths, covering SHA-256 padding boundaries, lowest winning assignment offsets,
multiple winners, wrapping nonce ranges and zero-target exhaustion. It also
checks an empty range, which returns without opening the GPU. The executed
release test artifact had SHA-256:

```text
7ee2c0a6053380eb59777d0eb6be2945abfc2b390e3b23e018addeb533bc9efc
```

`cargo test --offline --locked --release` separately passed seven unit tests and
one worker-process contract test; it skips the Metal test unless explicitly
requested as above. No live mining, network submission or deployment occurred.

Remaining hardware coverage:

- CUDA remains compile-checked in CI, without NVIDIA hardware parity evidence.
- OpenCL CI runs the kernel on PoCL's CPU device. The earlier local OpenCL result
  recorded above does not identify its device; this checkpoint did not rerun it
  and does not establish physical-GPU OpenCL coverage.
- The small Metal test does not cross the 16-million-nonce dispatch boundary or
  cover sustained load, device failure injection or GPU worker cancellation.
  CI currently skips this ignored hardware test.
- The inherited throughput figures in the README were not revalidated by this
  port or this correctness checkpoint.
