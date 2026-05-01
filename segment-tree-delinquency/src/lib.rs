pub struct SegmentTree {
    tree: Vec<u64>,
    size: usize,
}

impl SegmentTree {
    pub fn new(number: usize) -> Self {
        let size = 4 * number;
        Self {
            tree: vec![0; size],
            size: number,
        }
    }

    pub fn update(&mut self, index: usize, value: u64) {
        self.update_point(1, 0, self.size - 1, index, value);
    }

    fn update_point(&mut self, node: usize, start: usize, end: usize, index: usize, value: u64) {
        if start == end {
            self.tree[node] = value;
            return;
        }

        let mid = (start + end) / 2;
        if index <= mid {
            self.update_point(2 * node, start, mid, index, value);
        } else {
            self.update_point(2 * node + 1, mid + 1, end, index, value);
        }

        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    pub fn query(&self, left: usize, right: usize) -> u64 {
        self.query_range(1, 0, self.size - 1, left, right)
    }

    fn query_range(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> u64 {
        if start > r || end < l {
            return 0;
        }

        if start >= l && end <= r {
            return self.tree[node];
        }

        let mid = (start + end) / 2;
        let left = self.query_range(2 * node, start, mid, l, r);
        let right = self.query_range(2 * node + 1, mid + 1, end, l, r);

        left + right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delinquency_basic() {
        let mut st = SegmentTree::new(8);

        st.update(0, 500);
        st.update(2, 1200);
        st.update(4, 300);
        st.update(6, 800);

        assert_eq!(st.query(0, 7), 2800);
    }

    #[test]
    fn test_delinquency_range_query() {
        let mut st = SegmentTree::new(8);

        st.update(0, 500);
        st.update(2, 1200);
        st.update(4, 300);
        st.update(6, 800);

        assert_eq!(st.query(2, 5), 1500);
    }

    #[test]
    fn test_payment_received() {
        let mut st = SegmentTree::new(8);

        st.update(2, 1200);
        assert_eq!(st.query(0, 7), 1200);

        st.update(2, 400);
        assert_eq!(st.query(0, 7), 400);
    }

    #[test]
    fn test_segment_totals() {
        let mut st = SegmentTree::new(8);

        st.update(0, 500);
        st.update(1, 0);
        st.update(2, 1200);
        st.update(3, 0);

        assert_eq!(st.query(0, 3), 1700);
        assert_eq!(st.query(0, 1), 500);
        assert_eq!(st.query(2, 3), 1200);
    }

    #[test]
    fn test_multiple_updates_same_loan() {
        let mut st = SegmentTree::new(4);

        st.update(1, 500);
        assert_eq!(st.query(1, 1), 500);

        st.update(1, 1000);
        assert_eq!(st.query(1, 1), 1000);

        st.update(1, 0);
        assert_eq!(st.query(1, 1), 0);
    }

    #[test]
    fn test_empty_portfolio() {
        let st = SegmentTree::new(100);
        assert_eq!(st.query(0, 99), 0);
    }
}
