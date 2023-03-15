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
use std::{
    fmt::Debug,
    io::{stdout, Write},
    mem::swap,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}

const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
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
        res
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        ABCD: [(Usize1, char, Usize1, char); M]
    }
    let mut red_end = vec![false; N + 1];
    let mut blue_end = vec![false; N + 1];
    let mut checked = vec![false; N + 1];
    let mut uf = UnionFind::new(N);

    for (a, b, c, d) in ABCD.into_iter() {
        if b == 'R' {
            red_end[a] = true;
        } else {
            blue_end[a] = true;
        }
        if d == 'R' {
            red_end[c] = true;
        } else {
            blue_end[c] = true;
        }

        uf.unite(a, c);
    }

    let mut x = 0;
    let mut y = 0;

    let groups = uf.groups();
    for i in 0..N {
        if checked[i] || groups[i].len() == 0 {
            continue;
        }

        let mut is_cycle = true;
        for j in groups[i].iter() {
            is_cycle &= (red_end[*j] && blue_end[*j]);
            checked[*j] = true;
        }
        if is_cycle {
            x += 1;
        } else {
            y += 1;
        }
    }

    println!("{} {}", x, y)
}
