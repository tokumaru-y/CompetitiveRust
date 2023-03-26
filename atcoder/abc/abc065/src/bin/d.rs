use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};

fn main() {
    input! {
        N: usize,
        mut XY: [(Isize1, Isize1); N],
    }
    let mut uf = UnionFind::new(N);
    let mut ans = 0;
    let mut edges: Vec<(isize, usize, usize)> = Vec::new();
    let mut add_index_xy = Vec::new();
    let mut add_index_yx = Vec::new();
    for i in 0..N {
        add_index_xy.push((XY[i].0, XY[i].1, i));
        add_index_yx.push((XY[i].1, XY[i].0, i));
    }
    add_index_xy.sort();
    add_index_yx.sort();
    for i in 0..N-1 {
        let a = add_index_xy[i];let b = add_index_xy[i+1];
        edges.push(((a.0 - b.0).abs(), a.2, b.2));
    }
    for i in 0..N-1 {
        let a = add_index_yx[i];let b = add_index_yx[i+1];
        edges.push(((a.0 - b.0).abs(), a.2, b.2));
    }
    edges.sort();
    for (v, i, j) in edges.iter() {
        if uf.is_same(*i, *j) { continue; }
        uf.unite(*i,*j);
        ans += *v;
    }
    println!("{}",ans);
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
