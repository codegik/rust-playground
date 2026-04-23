pub struct SegmentTree {
    tree: Vec<usize>,
    lazy: Vec<bool>,
    n: usize,
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        let size = 4 * n;
        Self {
            tree: vec![0; size],
            lazy: vec![false; size],
            n,
        }
    }

    fn push(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node] {
            self.tree[node] = (end - start + 1) - self.tree[node];

            if start != end {
                self.lazy[2 * node] = !self.lazy[2 * node];
                self.lazy[2 * node + 1] = !self.lazy[2 * node + 1];
            }

            self.lazy[node] = false;
        }
    }

    pub fn update(&mut self, l: usize, r: usize) {
        self.update_range(1, 0, self.n - 1, l, r);
    }

    fn update_range(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize) {
        self.push(node, start, end);

        if start > r || end < l {
            return;
        }

        if start >= l && end <= r {
            self.lazy[node] = true;
            self.push(node, start, end);
            return;
        }

        let mid = (start + end) / 2;
        self.update_range(2 * node, start, mid, l, r);
        self.update_range(2 * node + 1, mid + 1, end, l, r);

        self.push(2 * node, start, mid);
        self.push(2 * node + 1, mid + 1, end);
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    pub fn query(&mut self, l: usize, r: usize) -> usize {
        self.query_range(1, 0, self.n - 1, l, r)
    }

    fn query_range(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize) -> usize {
        if start > r || end < l {
            return 0;
        }

        self.push(node, start, end);

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
    fn test_basic_operations() {
        let mut st = SegmentTree::new(4);

        assert_eq!(st.query(0, 3), 0);

        st.update(1, 2);
        assert_eq!(st.query(0, 1), 1);

        st.update(0, 3);
        assert_eq!(st.query(0, 3), 2);
    }

    #[test]
    fn test_single_piece() {
        let mut st = SegmentTree::new(1);

        assert_eq!(st.query(0, 0), 0);

        st.update(0, 0);
        assert_eq!(st.query(0, 0), 1);

        st.update(0, 0);
        assert_eq!(st.query(0, 0), 0);
    }

    #[test]
    fn test_multiple_updates_same_range() {
        let mut st = SegmentTree::new(5);

        st.update(1, 3);
        assert_eq!(st.query(1, 3), 3);

        st.update(1, 3);
        assert_eq!(st.query(1, 3), 0);

        st.update(1, 3);
        assert_eq!(st.query(1, 3), 3);
    }

    #[test]
    fn test_overlapping_updates() {
        let mut st = SegmentTree::new(6);

        st.update(0, 2);
        assert_eq!(st.query(0, 5), 3);

        st.update(2, 4);
        assert_eq!(st.query(0, 5), 4);
        assert_eq!(st.query(0, 1), 2);
        assert_eq!(st.query(2, 2), 0);
        assert_eq!(st.query(3, 4), 2);
    }

    #[test]
    fn test_partial_queries() {
        let mut st = SegmentTree::new(8);

        st.update(2, 5);

        assert_eq!(st.query(0, 1), 0);
        assert_eq!(st.query(2, 5), 4);
        assert_eq!(st.query(6, 7), 0);
        assert_eq!(st.query(3, 4), 2);
        assert_eq!(st.query(0, 7), 4);
    }

    #[test]
    fn test_complex_sequence() {
        let mut st = SegmentTree::new(10);

        st.update(0, 4);
        assert_eq!(st.query(0, 9), 5);

        st.update(5, 9);
        assert_eq!(st.query(0, 9), 10);

        st.update(2, 7);
        assert_eq!(st.query(0, 9), 4);
        assert_eq!(st.query(0, 1), 2);
        assert_eq!(st.query(2, 7), 0);
        assert_eq!(st.query(8, 9), 2);
    }

    #[test]
    fn test_geeksforgeeks_example() {
        let mut st = SegmentTree::new(4);

        assert_eq!(st.query(0, 3), 0);

        st.update(1, 2);

        assert_eq!(st.query(0, 1), 1);

        st.update(0, 3);

        assert_eq!(st.query(0, 3), 2);
    }
}
