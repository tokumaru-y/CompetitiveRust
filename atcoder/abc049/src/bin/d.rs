use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};
// reference: https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/dsu.rs
struct UnionFind {
    node: usize,
    parent_or_size: Vec<isize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            node: size,
            parent_or_size: vec![-1; size]
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
}

fn main() {
    input! {
        N: usize,
        K: usize,
        L: usize,
        PQ: [(Usize1, Usize1); K],
        RS: [(Usize1, Usize1); L],
    }
    let mut roads = UnionFind::new(N);
    let mut rails = UnionFind::new(N);
    for (p, q) in PQ.into_iter() {
        roads.unite(p, q);
    }
    for (r,s) in RS.into_iter() {
        rails.unite(r, s);
    }
    let mut map = BTreeMap::new();
    for i in 0..N{
        let x = roads.root(i);
        let y = rails.root(i);
        *map.entry((x,y)).or_insert(0) += 1;
    }
    for i in 0..N {
        print!("{} ", map.get(&(roads.root(i), rails.root(i))).unwrap());
    }
}