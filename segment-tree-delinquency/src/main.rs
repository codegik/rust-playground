use segment_tree_delinquency::SegmentTree;

fn main() {
    let mut portfolio = SegmentTree::new(8);

    println!("=== Delinquency Monitoring System ===\n");

    println!("Initial portfolio: 8 loans, all current");
    println!("Total delinquent: ${}\n", portfolio.query(0, 7));

    println!("Day 1: Setting initial delinquent amounts");
    portfolio.update(0, 500);
    portfolio.update(2, 1200);
    portfolio.update(4, 300);
    portfolio.update(6, 800);

    println!("Loan 0: $500 delinquent");
    println!("Loan 2: $1200 delinquent");
    println!("Loan 4: $300 delinquent");
    println!("Loan 6: $800 delinquent");
    println!("Total portfolio delinquency: ${}\n", portfolio.query(0, 7));

    println!("Query: Loans 2-5 delinquency");
    println!("Result: ${}\n", portfolio.query(2, 5));

    println!("Query: First half (loans 0-3) delinquency");
    println!("Result: ${}\n", portfolio.query(0, 3));

    println!("Query: Second half (loans 4-7) delinquency");
    println!("Result: ${}\n", portfolio.query(4, 7));

    println!("Day 2: Loan 2 makes payment, delinquency drops to $400");
    portfolio.update(2, 400);
    println!("Total portfolio delinquency: ${}\n", portfolio.query(0, 7));

    println!("Day 3: Loan 6 pays off completely");
    portfolio.update(6, 0);
    println!("Total portfolio delinquency: ${}\n", portfolio.query(0, 7));

    println!("Day 4: Loan 1 becomes delinquent for $750");
    portfolio.update(1, 750);
    println!("Total portfolio delinquency: ${}\n", portfolio.query(0, 7));

    println!("Final query: Loans 0-3 delinquency");
    println!("Result: ${}", portfolio.query(0, 3));
}
