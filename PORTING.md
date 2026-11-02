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
