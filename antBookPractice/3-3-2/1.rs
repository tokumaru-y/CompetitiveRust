// https://atcoder.jp/contests/chokudai_S001/tasks/chokudai_S001_j
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
use std::fmt::Debug;
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
struct BIT {
    // 1-indexed
    node: Vec<usize>,
    N: usize,
}

impl BIT {
    pub fn new(size: usize) -> Self {
        Self {
            node: vec![0; size + 1],
            N: size,
        }
    }

    pub fn add(&mut self, ind: usize, val: usize) {
        let mut x = ind;
        while x <= self.N {
            self.node[x] += val;
            x += x & x.wrapping_neg();
        }
    }

    pub fn sum(&self, ind: usize) -> usize {
        let mut res = 0;
        let mut x = ind;

        while (0 < x) {
            res += self.node[x];
            x -= x & x.wrapping_neg();
        }

        res
    }
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }
    let mut binary_indexed_tree = BIT::new(N);
    let mut ans = 0;

    for (i, a) in A.into_iter().enumerate() {
        ans += i - binary_indexed_tree.sum(a);
        binary_indexed_tree.add(a, 1);
    }

    println!("{}", ans);
}
