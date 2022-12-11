// https://atcoder.jp/contests/indeednow-finalb-open/tasks/indeednow_2015_finalb_d
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

fn make_heap(
    P: &Vec<Vec<isize>>,
    pq: &mut BinaryHeap<(isize, (usize, usize), (usize, usize))>,
    H: usize,
    W: usize,
) {
    for (h, h_line) in P.iter().enumerate() {
        for (w, value) in h_line.iter().enumerate() {
            for [hi, wi] in [[0, 1], [0, -1], [1, 0], [-1, 0]].iter() {
                let h = h as isize;
                let w = w as isize;
                if h + hi >= 0 && h + hi < H as isize && w + wi >= 0 && w + wi < W as isize {
                    pq.push((
                        (*value) * (P[(h + hi) as usize][(w + wi) as usize]),
                        (h as usize, w as usize),
                        ((h + hi) as usize, (w + wi) as usize),
                    ));
                }
            }
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        S: (Usize1, Usize1),
        G: (Usize1, Usize1),
        P: [[isize; W]; H],
    }
    let mut pq = BinaryHeap::new();
    let mut ans = 0;
    let mut uf = UnionFind::new(100000 * 100);

    for h in 0..H {
        ans += P[h].iter().fold(0, |sum, x| sum + x);
    }

    make_heap(&P, &mut pq, H, W);

    while !pq.is_empty() {
        let (cost, a, b) = pq.pop().unwrap();
        let x = 1000 * a.0 + a.1;
        let y = 1000 * b.0 + b.1;

        if uf.is_same(x, y) {
            continue;
        }

        uf.unite(x, y);
        ans += cost;
    }

    println!("{}", ans);
}
