// https://atcoder.jp/contests/abc065/tasks/arc076_b
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;

// reference: https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/dsu.rs
struct UnionFind {
    node: usize,
    parent_or_size: Vec<isize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            node: size,
            // parent_or_size[x] < 0:x is parent
            // parent_or_size[x] > 0:x's parent is parent_or_size[x]
            parent_or_size: vec![-1; size],
        }
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.parent_or_size[x] < 0 {
            return x;
        }
        self.parent_or_size[x] = self.root(self.parent_or_size[x] as usize) as isize;
        self.parent_or_size[x] as usize
    }

    pub fn unite(&mut self, x: usize, y: usize) -> usize {
        let (mut a, mut b) = (self.root(x), self.root(y));
        if a == b {
            return a;
        }
        if -self.parent_or_size[a] < -self.parent_or_size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent_or_size[a] += self.parent_or_size[b];
        self.parent_or_size[b] = a as isize;
        a
    }

    pub fn size(&mut self, x: usize) -> usize {
        let a = self.root(x);
        -self.parent_or_size[a] as usize
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut roots = vec![0; self.node];
        let mut group_size = vec![0; self.node];
        for i in 0..self.node {
            roots[i] = self.root(i);
            group_size[roots[i]] += 1;
        }
        let mut res = vec![Vec::new(); self.node];
        for i in 0..self.node {
            res[i].reserve(group_size[i]);
        }
        for i in 0..self.node {
            res[roots[i]].push(i);
        }

        res.into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        XY: [(isize, isize); N]
    }
    let mut pq = BinaryHeap::new();
    let mut ans = 0;
    let mut uf = UnionFind::new(N);

    let mut sort_x = vec![];
    let mut sort_y = vec![];
    for i in (0..N) {
        sort_x.push((XY[i].0, XY[i].1, i));
        sort_y.push((XY[i].0, XY[i].1, i));
    }

    sort_x.sort();
    sort_y.sort_by(|x, y| x.1.cmp(&y.1));

    for i in 0..N - 1 {
        let v = min(
            (sort_x[i].0 - sort_x[i + 1].0).abs(),
            (sort_x[i].1 - sort_x[i + 1].1).abs(),
        );
        pq.push((-v, sort_x[i].2, sort_x[i + 1].2));
    }
    for i in (0..N - 1) {
        let v = min(
            (sort_y[i].0 - sort_y[i + 1].0).abs(),
            (sort_y[i].1 - sort_y[i + 1].1).abs(),
        );
        pq.push((-v, sort_y[i].2, sort_y[i + 1].2));
    }
    while !pq.is_empty() {
        let (v, a, b) = pq.pop().unwrap();
        if uf.is_same(a, b) {
            continue;
        }
        uf.unite(a, b);
        ans -= v;
    }

    println!("{}", ans);
}
