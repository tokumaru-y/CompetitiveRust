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
struct LazyAddSegmentTree {
    N: usize,
    node: Vec<isize>,
    lazy: Vec<isize>,
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
            node[i + n - 1] = v[i] as isize;
        }
        if n > 1 {
            for i in (0..(n - 2)).rev() {
                node[i] = min(node[i * 2 + 1], node[i * 2 + 2]);
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
            // self.node[k] += self.lazy[k];

            // 末端のノード以外の場合
            if (r - l) > 1 {
                self.lazy[2 * k + 1] += self.lazy[k] / 2;
                self.lazy[2 * k + 2] += self.lazy[k] / 2;
            }

            self.lazy[k] = 0;
        }
    }

    pub fn add(&mut self, a: usize, b: usize, v: isize) {
        self._add(a, b, v, 0, 0, self.N)
    }

    // [a,b)に対して、vを加算する。
    // k:=ノード番号 [l,r):=kのノードがカバーしている範囲
    fn _add(&mut self, a: usize, b: usize, v: isize, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);

        if (b <= l || r <= a) {
            return;
        }

        if (a <= l && r <= b) {
            self.lazy[k] += (r - l) as isize * v;
            self.eval(k, l, r);
        } else {
            self._add(a, b, v, 2 * k + 1, l, (l + r) / 2);
            self._add(a, b, v, 2 * k + 2, (l + r) / 2, r);

            self.node[k] = min(self.node[2 * k + 1], self.node[2 * k + 2]);
        }
    }

    pub fn query(&mut self, a: usize, b: usize) -> isize {
        self._query(a, b, 0, 0, self.N)
    }

    fn _query(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> isize {
        if (b <= l || r <= a) {
            return 0;
        }

        self.eval(k, l, r);
        if (a <= l && r <= b) {
            return self.node[k];
        }

        let sum_left = self._query(a, b, 2 * k + 1, l, (l + r) / 2);
        let sum_right = self._query(a, b, 2 * k + 2, (l + r) / 2, r);

        min(sum_left, sum_right)
    }
}
#[allow(non_snake_case)]
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    let nq: Vec<usize> = buf
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    let N = nq[0];
    let Q = nq[1];

    let mut seg = LazyAddSegmentTree::new(vec![0; N]);

    for _ in (0..Q) {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).ok();
        let qu: Vec<isize> = buf
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();

        if qu[0] == 0 {
            seg.add(qu[1] as usize, qu[2] as usize + 1, qu[3]);
        } else {
            let s = seg.query(qu[1] as usize, qu[2] as usize + 1);
            println!("{}", s);
        }
    }
}
