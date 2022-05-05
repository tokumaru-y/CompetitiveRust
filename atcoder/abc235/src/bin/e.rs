use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        Q: usize,
        ABC: [(Usize1, Usize1, usize); M],
        UVM: [(Usize1, Usize1, usize); Q],
    }
    let mut uf = UnionFind::new(N);
    let mut queries = Vec::new();
    for (a,b,c) in ABC.into_iter() {
        queries.push((c,a,b,1, 0));
    }
    for i in 0..Q {
        let (u,v,m) = UVM[i];
        queries.push((m,u,v,0,i));
    }
    let mut ans = vec!["no"; Q];
    queries.sort();
    for (cost, x, y, is_query, index) in queries.into_iter() {
        if is_query == 1 {
            if uf.is_same(x, y) { continue; }
            uf.unite(x,y);
        } else {
            ans[index] = match uf.is_same(x, y) {
                true => "No",
                false => "Yes",
            }
        }
    }
    for i in 0..Q { println!("{}", ans[i]); }
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
