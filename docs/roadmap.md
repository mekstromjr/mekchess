# Roadmap

Work is tracked as milestones and issues in the project's private GitLab. The
plan is a sequence of themes, each with a learning goal. Dependency order:
**A, then B and C in parallel, then D, E, F, G**.

| Theme | Scope | Learning goal | State |
|---|---|---|---|
| A. Core and test harness | board representation, FEN, Zobrist hashing, legal move generation, perft, UCI plumbing | board representation and testing | in progress: board + FEN done, move generation next |
| B. Debug UI | web visualizer in `ui/` | building a UI | planned |
| C. Classical engine | alpha-beta search over a handcrafted eval | optimization | planned |
| D. Local SPRT testing | arena + fast-chess matches | engine-vs-engine comparison | planned |
| E. NNUE engine | Python training, Rust inference | models and neural networks | planned |
| F. Distributed testing and data | OpenBench-style distributed runs, self-play data | compute-heavy workloads on Kubernetes | planned |
| G. Lichess bot | lichess-bot bridge to the UCI binaries | APIs | planned |

## Planned chess-core modules

From the crate docs in `crates/chess-core/src/lib.rs`:

- `bitboard`: the `u64`-backed set type and bit-twiddling helpers.
- `board`: today a 64-entry mailbox; planned to become a hybrid of piece
  bitboards, occupancy and mailbox.
- `zobrist`: incremental position hashing.
- `movegen`: magic-bitboard legal move generation.
- `perft`: leaf-node counting and divide. This becomes the correctness gate in
  CI, so no move-generation regression can merge.

For perft cross-validation the workspace plans to use `shakmaty` as a
dev-only oracle, kept under `[dev-dependencies]` so its GPL-3.0 license never
links into the engine binaries.

## Deployment

The compute-heavy pieces (distributed testing, the debug UI, the Lichess bot)
are meant to run on a self-hosted Kubernetes cluster. This repo builds the
source; deployment manifests live in a separate private infrastructure repo
reconciled by Flux.
