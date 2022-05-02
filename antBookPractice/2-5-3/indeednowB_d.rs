use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};

fn main() {
    input! {
        H: usize,
        W: usize,
        S: [Isize1; 2],
        G: [Isize1; 2],
        Grid: [[isize; W]; H],
    }
    let mut ans = 0isize;
    for g in Grid.iter() {
        ans += (*g).iter().sum::<isize>();
    }
    let mut edges: Vec<(isize, (usize, usize), (usize, usize))> = Vec::new();
    for h in 0..H {
        for w in 0..W {
            if h < H-1 { edges.push(( - (Grid[h+1][w] * Grid[h][w]), (h+1,w), (h,w))); }
            if w < W-1 { edges.push(( - (Grid[h][w+1] * Grid[h][w]), (h,w+1), (h,w))); }
        }
    }
    edges.sort();
    let mut uf = UnionFind::new(1000_000 + W);
    for (v, hw1, hw2) in edges.iter() {
        let i = 1000*hw1.0 + hw1.1; let j = 1000*hw2.0 + hw2.1;
        if uf.is_same(i, j) { continue; }
        uf.unite(i,j);
        ans += -(*v);
    }
    println!("{}", ans);
}

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
