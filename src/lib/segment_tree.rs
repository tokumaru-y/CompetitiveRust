struct SegmentTree {
    node: Vec<usize>,
    init_value: usize,
    leaf_cnt: usize,
}
// RMQ
impl SegmentTree {
    pub fn new(v: Vec<usize>, init_value: usize) -> Self {
        let len = v.len();
        let mut n = 1;
        // 完全木なので、2冪の葉が必要
        while (n < len) {
            n *= 2;
        }
        let mut node = vec![init_value; 4 * n - 1];

        for i in 0..len {
            node[i + n - 1] = v[i];
        }

        for i in (0..=(n - 2)).rev() {
            node[i] = min(node[2 * i + 1], node[2 * i + 2]);
        }

        Self {
            node: node,
            init_value: init_value,
            leaf_cnt: n,
        }
    }

    pub fn update(&mut self, x: usize, val: usize) {
        let mut ind = x + (self.leaf_cnt - 1);
        self.node[ind] = val;

        while (ind > 0) {
            ind = (ind - 1) / 2;
            self.node[ind] = min(self.node[2 * ind + 1], self.node[2 * ind + 2]);
        }
    }

    pub fn query(&self, a: usize, b: usize, ind: usize, l: usize, r: usize) -> usize {
        if (r <= a || b <= l) {
            return self.init_value;
        }

        if (a <= l && r <= b) {
            self.node[ind]
        } else {
            let chil_v1 = self.query(a, b, 2 * ind + 1, l, (l + r) / 2);
            let chil_v2 = self.query(a, b, 2 * ind + 2, (l + r) / 2, r);

            min(chil_v1, chil_v2)
        }
    }
}
