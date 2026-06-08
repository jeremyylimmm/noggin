# noggin

A chess engine written in Rust, compatible with the [Universal Chess Interface (UCI)](https://backscattering.de/chess/uci/) protocol.

## Building

Requires Rust and Cargo.

```sh
cargo build --release
```

The build script generates magic bitboard tables and Zobrist hash constants at compile time.

## Usage

Run the engine as a UCI-compatible process:

```sh
./target/release/noggin
```

It reads UCI commands from stdin and writes responses to stdout, so it can be plugged into any UCI-compatible GUI (Arena, Cute Chess, etc.).

### Benchmarking

```sh
# Fixed-depth search on a standard test position, reports nodes and nodes/second
./target/release/noggin bench

# Detailed metrics for fixed-depth searches
./target/release/noggin metrics
```

## Architecture

### Board Representation

- 12 `u64` bitboards (6 piece types × 2 sides)
- Incrementally updated [PeSTO](https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function) evaluation
- Zobrist hashing for transposition table lookups and repetition detection

### Move Generation

Pseudo-legal move generation using magic bitboards for sliding pieces (rooks, bishops, queens), with legality checked by testing whether the king is left in check.

### Search

Negamax alpha-beta with:

- Iterative deepening
- Principal variation search (PVS)
- Quiescence search

**Pruning:**
- Null move pruning (NMP)
- Reverse futility pruning (RFP) with improving heuristic
- Futility pruning (FP)
- Late move reductions (LMR)
- SEE-based capture pruning in quiescence

**Move ordering:**
- Transposition table move
- Promotions
- Captures ordered by SEE (good captures before bad)
- Killer moves
- Quiet moves scored by butterfly history + continuation history