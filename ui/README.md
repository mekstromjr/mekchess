# ui

Web debug visualizer for the engines (Epic B — `mekchess: Debug UI`).

TypeScript + Vite + React + [chessground](https://github.com/lichess-org/chessground)
(lichess's own board library). Purpose: see what the engine sees — render the
board from a FEN, overlay bitboards (a `u64` mapped to 64 squares), highlight
legal moves, and show live search info.

Bridged to the Rust engine over WebSocket; it consumes the UCI `info string`
debug channel, so the engine exposes internals without any protocol changes.

This is a *debug* tool, not a general chess GUI — for ordinary play and PGN
browsing, use an existing UCI GUI (En Croissant, Cute Chess).
