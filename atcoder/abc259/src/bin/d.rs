#[allow(unused_imports)]
use itertools::Itertools;
use num_integer::Roots;
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

fn read() -> usize {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<usize>().unwrap()
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

        res.into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        sx: isize,
        sy: isize,
        ex: isize,
        ey: isize,
        XYR: [(isize, isize, usize); N]
    }
    let mut uf = UnionFind::new(N);
    let mut start_ind = N + 1;
    let mut end_ind = N + 1;
    for i in 0..N {
        let (a, b, c) = XYR[i];
        if ((sx - a) * (sx - a) + (sy - b) * (sy - b)) as usize == c * c {
            start_ind = i;
        }
        if (((ex - a) * (ex - a) + (ey - b) * (ey - b)) as usize) == c * c {
            end_ind = i;
        }
        for j in (i + 1)..N {
            let (x, y, z) = XYR[j];
            let dif = (((x - a) * (x - a)) + ((b - y) * (b - y))) as usize;
            let r1 = min(c, z);
            let r2 = max(c, z);
            if dif > (r1 + r2) * (r1 + r2) || dif < (r2 - r1) * (r2 - r1) {
                continue;
            }
            uf.unite(i, j);
        }
    }

    let ans = if uf.is_same(start_ind, end_ind) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
