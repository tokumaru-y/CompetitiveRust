// https://atcoder.jp/contests/joisc2011/tasks/joisc2011_bookshelf
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
use std::fmt::Debug;
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
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
            self.node[ind] = max(self.node[2 * ind + 1], self.node[2 * ind + 2]);
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

            max(chil_v1, chil_v2)
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: [usize; N],
        order: [Usize1; N]
    }
    let total_weight = W.iter().sum::<usize>();
    let mut seg = SegmentTree::new(vec![0; N], 0);

    for book in order.into_iter() {
        let max = seg.query(0, book);
        seg.update(book, max + W[book]);
    }

    println!("{}", (total_weight - seg.query(0, N)) * 2);
}
