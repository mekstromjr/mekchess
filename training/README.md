# training

Neural-net training pipeline for the NNUE engine (Epic E — `mekchess: NNUE Engine`).

Python, kept deliberately separate from the Rust workspace — this is where the
general ML ecosystem (PyTorch) lives. Training happens here; **inference** is
hand-written in Rust in `crates/engine-nnue`.

Planned contents:
- `nnue-pytorch` setup (the standard Stockfish-lineage trainer).
- self-play data generation + `.binpack` export.
- bootstrap from the Lichess eval database (CC0).

Tooling: `uv` for the Python environment. Weights/checkpoints are data, not
source — they live in Garage S3, not git (see `.gitignore`).
