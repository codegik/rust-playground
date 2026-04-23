# Segment Tree: Chessboard Pieces Range Query

Rust implementation of a segment tree with lazy propagation to solve the chessboard pieces problem.

## Problem

Given N chessboard pieces initially colored white, support two operations:

1. **Update [L, R]**: Invert colors of all pieces in range (white → black, black → white)
2. **Query [L, R]**: Count black pieces in range

## Solution

Uses a segment tree with lazy propagation to achieve O(log N) time complexity per operation.

### Key Components

- **tree[]**: Stores count of black pieces for each segment
- **lazy[]**: Marks segments pending inversion
- **push()**: Propagates lazy updates down the tree
- **update()**: Inverts colors in a range
- **query()**: Counts black pieces in a range

## Build

```bash
cargo build --release
```

## Run Tests

```bash
cargo test
```

## Run

```bash
cargo run
```

### Input Format

```
N Q
operation L R
operation L R
...
```

Where:
- N = number of pieces
- Q = number of queries
- operation = "Get" or "Update"
- L, R = range bounds (0-indexed)

### Usage

```bash
echo "4 5
Get 0 3
Update 1 2
Get 0 1
Update 0 3
Get 0 3" | cargo run
```

Output:
```
0
1
2
```

## Complexity

- Time: O(Q × log N)
- Space: O(N)

## Reference

Based on: https://www.geeksforgeeks.org/dsa/range-update-query-chessboard-pieces/
