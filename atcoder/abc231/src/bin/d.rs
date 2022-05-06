#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        M: usize,
        AB:[(Usize1, Usize1); M],
    }
    let mut hash = HashMap::new();
    let mut uf = UnionFind::new(N);
    let mut is_ok = true;
    for (a,b) in AB.into_iter() {
        *hash.entry(a).or_insert_with(|| 0) += 1;
        *hash.entry(b).or_insert_with(|| 0) += 1;
        if uf.is_same(a, b) { is_ok = false; }
        uf.unite(a,b);
    }
    for v in hash.values() {
        if *v > 2 { is_ok = false; }
    }
    let ans = match is_ok {
        true => "Yes",
        false => "No",
    };
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
