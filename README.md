# mekchess

A long-term, slow-burn playground for chess engines, Rust, neural nets, and
Lichess bots — with the computationally intensive workloads self-hosted in the
meklab homelab.

## Layout

| Path | Purpose |
|------|---------|
| `Game/` | Core chess domain — board representation, move generation, rules. |
| `Engines/standard/` | Classical search engine (alpha-beta, handcrafted eval). |
| `Engines/neuro/` | Neural-net engine (NNUE-style or policy/value nets) and training. |

## Status

Early scaffolding. No code yet — just the shape of where things will go.
