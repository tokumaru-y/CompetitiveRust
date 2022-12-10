// https://atcoder.jp/contests/arc090/tasks/arc090_b
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
// ref: https://at274.hatenablog.com/entry/2018/02/03/140504
struct WeigthedUnionFind {
    node: usize,
    parent: Vec<usize>,
    rank: Vec<usize>,
    weight: Vec<isize>,
}

impl WeigthedUnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            node: size,
            parent: (0..size).collect_vec(),
            rank: vec![0; size],
            // self.weight[i]: side's weight from i to self.parent[i]
            weight: vec![0; size],
        }
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn root(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            return x;
        }
        // attention!
        // self.parent[x] = self.root(self.parent[x])としてはいけない
        // 次の行でself.parent[x]を一つ上の階層のnodeのままにして置かないとweightがrootからの距離になってしまい、距離加算がうまくいかないため
        let res = self.root(self.parent[x]);
        self.weight[x] += self.weight[self.parent[x]];
        self.parent[x] = res;
        self.parent[x]
    }

    pub fn unite(&mut self, x: usize, y: usize, w: isize) {
        let (a, b) = (self.root(x), self.root(y));
        if a == b {
            return;
        }
        if self.rank[a] < self.rank[b] {
            self.parent[a] = b;
            self.weight[a] = w - self.weight[x] + self.weight[y];
        } else {
            self.parent[b] = a;
            self.weight[b] = -w - self.weight[y] + self.weight[x];
            if self.rank[a] == self.rank[b] {
                self.rank[a] += 1;
            }
        }
    }

    pub fn diff(&self, x: usize, y: usize) -> isize {
        self.weight[x] - self.weight[y]
    }
}

const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        LRD: [(Usize1, Usize1, isize); M]
    }
    let mut uf = WeigthedUnionFind::new(N);

    for (l, r, d) in LRD.into_iter() {
        if uf.is_same(l, r) {
            if uf.diff(l, r) != d {
                println!("No");
                exit(0);
            }
        } else {
            uf.unite(l, r, d);
        }
    }

    println!("Yes");
}
