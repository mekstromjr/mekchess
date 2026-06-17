# mekchess

A long-term, slow-burn playground for chess engines, Rust, neural nets, and
Lichess bots — with the computationally intensive workloads self-hosted in the
meklab homelab.

It is deliberately polyglot: Rust for the engines and shared chess domain,
Python for neural-net training, TypeScript for the debug UI, and existing
ecosystem tools (fast-chess, OpenBench, lichess-bot) for everything already
solved.

## Architecture

Each engine is a **library crate** (its logic, depending on `chess-core`) plus a
**thin UCI binary**. That split is the keystone of the whole project:

- The in-process **arena** calls engines as Rust libraries, sharing the
  `chess-core` board directly — fast iteration, no text round-trip.
- The same engines compile to **UCI binaries** that plug into fast-chess (for
  SPRT strength testing), external GUIs, and the lichess bot bridge.

So the custom board representation is the shared foundation *inside* the Rust
world, and UCI is the boundary at the *edge* where mekchess meets other tools.

## Layout

| Path | Purpose |
|------|---------|
| `crates/chess-core/` | Shared domain: board representation, move generation, FEN/PGN, Zobrist, perft. |
| `crates/uci/` | Reusable UCI protocol plumbing shared by the engine binaries. |
| `crates/engine-classical/` | Classical alpha-beta engine + handcrafted eval (lib + UCI bin). |
| `crates/engine-nnue/` | Neural-net (NNUE) engine; hand-written quantized inference (lib + UCI bin). |
| `crates/arena/` | In-process engine-vs-engine match runner for fast local iteration. |
| `training/` | Python NNUE training pipeline (nnue-pytorch). |
| `ui/` | Web debug visualizer (Vite + React + chessground). |
| `books/` | Opening books for test games. |

Kubernetes manifests for the hosted workloads (distributed testing, the UI,
the lichess bot) live in `meklab-k8s` and are reconciled by Flux — source
builds here, deploys there.

## Build

```sh
cargo build      # all crates + both UCI binaries
cargo test       # includes the perft correctness gate once Epic A5 lands
```

## Roadmap

Work is tracked as GitLab milestones-as-epics + issues in `meklab/mekchess`.
Dependency order: **A → (B ∥ C) → D → E → F → G**.

| Epic | Theme | Learning goal |
|------|-------|---------------|
| A | Core & Test Harness | board rep + testing (the foundation) |
| B | Debug UI | building/managing UI |
| C | Classical Engine | optimization |
| D | Testing (Local SPRT) | engine-vs-engine comparison |
| E | NNUE Engine | models & neural networks |
| F | Distributed Testing & Data | intensive workloads on k8s |
| G | Lichess Bot | APIs |

Background research that informed these decisions (board representation,
perft, UCI, the Rust ecosystem, NNUE vs AlphaZero, UI/arena/lichess tooling)
is saved in the repo wiki under `research/`.
