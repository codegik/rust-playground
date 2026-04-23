use segment_tree_chessboard::SegmentTree;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();

    let mut st = SegmentTree::new(n);

    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let operation = parts.next().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();

        match operation {
            "Get" => {
                println!("{}", st.query(l, r));
            }
            "Update" => {
                st.update(l, r);
            }
            _ => {}
        }
    }
}
