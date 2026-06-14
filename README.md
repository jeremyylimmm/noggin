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

### Training Data Generation

```sh
./target/release/noggin datagen
```

Runs self-play games in parallel (via Rayon) and writes packed binary training data to the `data/` directory. Each shard contains 10,000 games. Games start with 10 random plies to diversify positions, then both sides play at a fixed node limit of 5,000 nodes per move.

## Architecture

### Board Representation

- 12 `u64` bitboards (6 piece types × 2 sides)
- Zobrist hashing for transposition table lookups and repetition detection

### Move Generation

Pseudo-legal move generation using magic bitboards for sliding pieces (rooks, bishops, queens), with legality checked by testing whether the king is left in check.

### Evaluation

A small NNUE (Efficiently Updatable Neural Network) with:

- **Input:** 768 features (12 piece types × 64 squares), maintained as an incrementally updated accumulator
- **Hidden layer:** 128 neurons (64 per perspective), using SCReLU activation
- **Output:** 1 scalar centipawn score, evaluated from both the white and black perspectives

The network weights are compiled directly into the binary at build time.

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
