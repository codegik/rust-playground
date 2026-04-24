// lang: pub is public and can be used outside its module.
// lang: struct is used to declare a custom data type with named fields.
// lang: The fields in SegmentTree are not public. Only the struct itself is public,
// but its fields (tree, lazy, size) are private and cannot be accessed directly from outside the module.
// lang: The fields tree, lazy, and size are not optional.
pub struct SegmentTree {
    tree: Vec<usize>, // lang: mutable
    lazy: Vec<bool>,
    size: usize,      // lang: immutable
    // lang: usize represent non-negative values, is used for indexing and memory-related operations
    // lang: usize in 64-bit arch is: 2^64 - 1
}

// lang: impl is used to define methods and associated functions for a struct or enum.
// lang: Methods inside impl can access and modify the struct’s fields.
impl SegmentTree {
    // lang: new is the constructor because it is a convention in Rust to provide a function named new
    // that creates and returns an initialized instance of the struct.
    // lang: -> Self means the function returns a value of type Self.
    pub fn new(number: usize) -> Self {
        // algo: Using 4 ensures the internal array is always large enough to store all nodes of the segment tree,
        // even in the worst case when the number of elements is not a power of two.
        // lang: * is an operator
        let size = 4 * number;
        // lang: Self creates a new instance of the struct. Self refers to SegmentTree.
        // lang: Self is not a constructor. Self refers to the type of the struct.
        Self {
            // lang: creates a vector filled with size elements.
            // lang: The exclamation mark means you are invoking a macro, not a function.
            tree: vec![0; size],
            lazy: vec![false; size],
            size: number,
        }
    }

    // lang: push is private. it can only be used inside the SegmentTree implementation.
    // lang: &mut self means the method takes a mutable reference to the instance of the struct.
    // lang: you cannot rename &mut self. Self is a special keyword.
    // lang: push doesn't return a value.
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

    pub fn update(&mut self, left: usize, right: usize) {
        self.update_range(1, 0, self.size - 1, left, right);
    }

    fn update_range(&mut self, node: usize, start: usize, end: usize, left: usize, right: usize) {
        self.push(node, start, end);

        if start > right || end < left {
            return;
        }

        if start >= left && end <= right {
            self.lazy[node] = true;
            self.push(node, start, end);
            return;
        }

        let mid = (start + end) / 2;
        self.update_range(2 * node, start, mid, left, right);
        self.update_range(2 * node + 1, mid + 1, end, left, right);

        self.push(2 * node, start, mid);
        self.push(2 * node + 1, mid + 1, end);
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    pub fn query(&mut self, left: usize, right: usize) -> usize {
        self.query_range(1, 0, self.size - 1, left, right)
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

// lang: #[cfg(test)] means the following code block is only included when running tests.
// lang: I can rename tests, but is not recommented. The Rust test harness looks for a
// module named tests by convention.
#[cfg(test)]
// lang: mod tests declares a module named tests.
mod tests {
    // lang: use super::* brings all items (structs, functions, etc.) from the parent module into scope.
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
