struct SegmentTree {
    node: Vec<usize>,
    init_value: usize,
    leaf_cnt: usize,
}
// RMQ
// 区間内の最小値を求めている
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
        if n > 1 {
            for i in (0..=(n - 2)).rev() {
                node[i] = min(node[2 * i + 1], node[2 * i + 2]);
            }
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

    pub fn query(&self, a: usize, b: usize) -> usize {
        self._query(a, b, 0, 0, self.leaf_cnt)
    }

    fn _query(&self, a: usize, b: usize, ind: usize, l: usize, r: usize) -> usize {
        if (r <= a || b <= l) {
            return self.init_value;
        }

        if (a <= l && r <= b) {
            self.node[ind]
        } else {
            let chil_v1 = self._query(a, b, 2 * ind + 1, l, (l + r) / 2);
            let chil_v2 = self._query(a, b, 2 * ind + 2, (l + r) / 2, r);

            min(chil_v1, chil_v2)
        }
    }
}
struct LazyAddSegmentTree {
    N: usize,
    node: Vec<usize>,
    lazy: Vec<usize>,
}
// 区間に対する加算セグ木
// queryは区間の合計値を返す
impl LazyAddSegmentTree {
    pub fn new(v: Vec<usize>) -> Self {
        let len = v.len();
        let mut n = 1;
        // assert!(v.len() > 1, "Not allowed Vector's length 1");
        while n < len {
            n *= 2;
        }

        let mut node = vec![0; 2 * n - 1];
        let mut lazy = vec![0; 2 * n - 1];

        for i in (0..len) {
            node[i + n - 1] = v[i];
        }
        if n > 1 {
            for i in (0..(n - 2)).rev() {
                node[i] = node[i * 2 + 1] + node[i * 2 + 2];
            }
        }

        Self {
            N: n,
            node: node,
            lazy: lazy,
        }
    }

    // k番目のノードについて遅延評価
    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy[k] != 0 {
            self.node[k] += self.lazy[k];

            // 末端のノード以外の場合
            if (r - l) > 1 {
                self.lazy[2 * k + 1] += self.lazy[k] / 2;
                self.lazy[2 * k + 2] += self.lazy[k] / 2;
            }

            self.lazy[k] = 0;
        }
    }

    pub fn add(&mut self, a: usize, b: usize, v: usize) {
        self._add(a, b, v, 0, 0, self.N)
    }

    // [a,b)に対して、vを加算する。
    // k:=ノード番号 [l,r):=kのノードがカバーしている範囲
    fn _add(&mut self, a: usize, b: usize, v: usize, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);

        if (b <= l || r <= a) {
            return;
        }

        if (a <= l && r <= b) {
            self.lazy[k] += (r - l) * v;
            self.eval(k, l, r);
        } else {
            self._add(a, b, v, 2 * k + 1, l, (l + r) / 2);
            self._add(a, b, v, 2 * k + 2, (l + r) / 2, r);

            self.node[k] = self.node[2 * k + 1] + self.node[2 * k + 2];
        }
    }

    pub fn query(&mut self, a: usize, b: usize) -> usize {
        self._query(a, b, 0, 0, self.N)
    }

    fn _query(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> usize {
        if (b <= l || r <= a) {
            return 0;
        }

        self.eval(k, l, r);
        if (a <= l && r <= b) {
            return self.node[k];
        }

        let sum_left = self._query(a, b, 2 * k + 1, l, (l + r) / 2);
        let sum_right = self._query(a, b, 2 * k + 2, (l + r) / 2, r);

        sum_left + sum_right
    }
}
struct LazyUpdateSegmentTree {
    N: usize,
    node: Vec<usize>,
    lazy: Vec<usize>,
    lazy_flag: Vec<bool>,
}

// 区間に対する更新セグ木
// findは最小値を返す
impl LazyUpdateSegmentTree {
    pub fn new(v: Vec<usize>) -> Self {
        let len = v.len();
        let mut n = 1;
        while n < len {
            n *= 2;
        }

        let mut node = vec![0; 2 * n - 1];
        let mut lazy = vec![0; 2 * n - 1];

        for i in (0..len) {
            node[i + n - 1] = v[i];
        }
        if n > 1 {
            for i in (0..(n - 2)).rev() {
                node[i] = node[i * 2 + 1] + node[i * 2 + 2];
            }
        }

        Self {
            N: n,
            node: node,
            lazy: lazy,
            lazy_flag: vec![false; 2 * n - 1],
        }
    }

    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy_flag[k] {
            self.node[k] = self.lazy[k];

            if (r - l) > 1 {
                self.lazy[k * 2 + 1] = self.lazy[k];
                self.lazy[k * 2 + 2] = self.lazy[k];
                self.lazy_flag[2 * k + 1] = true;
                self.lazy_flag[2 * k + 2] = true;
            }

            self.lazy_flag[k] = false;
        }
    }

    pub fn update(self, a: usize, b: usize, v: usize) {
        self._update(a, b, v, 0, 0, self.N)
    }

    fn _update(&mut self, a: usize, b: usize, v: usize, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);

        if (b <= l || r <= a) {
            return;
        }

        if (a <= l && r <= b) {
            self.lazy[k] = v;
            self.lazy_flag[k] = true;
            self.eval(k, l, r);
        } else {
            self.update(a, b, v, 2 * k + 1, l, (l + r) / 2);
            self.update(a, b, v, 2 * k + 2, (l + r) / 2, r);

            self.node[k] = min(self.node[2 * k + 1], self.node[2 * k + 2]);
        }
    }

    pub fn find(&self, a: usize, b: usize) -> usize {
        self._find(a, b, 0, 0, self.N);
    }

    fn _find(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> usize {
        self.eval(a, b, k);

        if (b <= l || r <= a) {
            return 10usize.pow(10);
        }

        if (a <= l && r <= b) {
            return self.node[k];
        } else {
            let left = self.find(a, b, 2 * k + 1, l, (l + r) / 2);
            let right = self.find(a, b, 2 * k + 2, (l + r) / 2, r);

            return min(left, right);
        }
    }
}
