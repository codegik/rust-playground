# Segment Tree for Delinquency Monitoring

## Structure

Each leaf node represents ONE loan in the portfolio.
Each internal node stores the SUM of delinquent amounts in its range.

```
Portfolio: 8 loans with delinquent amounts (in dollars)

Loan Index:     0      1      2      3      4      5      6      7
Delinquent:     $500   $0     $1200  $0     $300   $0     $800   $0

Segment Tree Structure:

                    [0-7]: $2800
                    /            \
            [0-3]: $1700        [4-7]: $1100
            /        \           /        \
        [0-1]: $500  [2-3]: $1200  [4-5]: $300  [6-7]: $800
        /    \       /    \        /    \       /    \
      [0]   [1]    [2]   [3]     [4]   [5]    [6]   [7]
      $500  $0    $1200  $0     $300   $0    $800   $0
```

## Operations

### Query: "What's total delinquent amount for loans 2-5?"

```
Query(2, 5) traverses:

                    [0-7]: $2800
                    /            \
            [0-3]: $1700        [4-7]: $1100
            /        \           /        \
        [0-1]: $500  [2-3]: $1200  [4-5]: $300  [6-7]: $800
                     ^^^^^^       ^^^^^^
                     fully in     fully in
                     range        range

Result: $1200 + $300 = $1500
```

### Update: "Loan 2 payment received, delinquency drops to $400"

```
Update(2, -$800) propagates upward:

Before:                          After:
        [0-3]: $1700                    [0-3]: $900
        /        \                      /        \
    [0-1]: $500  [2-3]: $1200      [0-1]: $500  [2-3]: $400
                 /    \                           /    \
               [2]   [3]                        [2]   [3]
              $1200  $0                        $400   $0

Root [0-7] also updates: $2800 → $2000
```

## Real-World Queries

1. **Total portfolio delinquency**: Query(0, n-1)
2. **Q1 2025 loans**: Query(loans_originated_jan_mar_2025)
3. **High-risk segment**: Query(subprime_loan_indices)
4. **Geographic**: Query(california_loan_indices)

## Performance

- **Query**: O(log n) - 20 operations for 1M loans
- **Update**: O(log n) - 20 operations for 1M loans
- **Space**: O(4n) - ~32MB for 1M loans (8 bytes per node)

## Data Model

```rust
struct Loan {
    id: u64,
    index: usize,              // position in segment tree
    delinquent_amount: u64,    // cents
    days_past_due: u16,
    origination_date: Date,
}

struct DelinquencyTree {
    tree: SegmentTree<u64>,    // stores delinquent amounts
    loan_index_map: HashMap<u64, usize>,  // loan_id -> tree index
}
```

## Use Cases

1. **Daily dashboard**: Query delinquency across origination cohorts
2. **Risk alerts**: Detect when segment exceeds threshold
3. **Reporting**: Sum delinquency by state/product/vintage
4. **Stress testing**: Quickly recalculate totals after simulated defaults
5. **Rebalancing**: Find segments with min/max delinquency rates

## Why Segment Tree Wins Here

- **Updates frequent**: Daily payment processing affects individual loans
- **Queries frequent**: Real-time dashboards, risk monitoring, compliance reports
- **Flexible ranges**: Need arbitrary date ranges, not just fixed buckets
- **Low latency**: O(log n) vs O(n) matters when querying 100K+ loan portfolio

## Alternative: If reads >> writes

If you query constantly but only update nightly:
- Precompute aggregates in PostgreSQL
- Use materialized views
- Segment tree overhead not justified
