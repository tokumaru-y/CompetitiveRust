// https://atcoder.jp/contests/indeednow-finalb-open/tasks/indeednow_2015_finalb_e
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

fn compress(X: &Vec<usize>) -> (Vec<usize>, usize) {
    let mut comp = vec![0];
    for x in X.iter() {
        comp.push(*x);
    }
    comp.sort();

    let mut res = vec![0; X.len()];
    for i in 0..X.len() {
        res[i] = comp.binary_search(&X[i]).unwrap();
    }

    (res, comp.len())
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H: [usize; N]
    }
    let mut unique_h = H.clone();
    unique_h.sort();
    unique_h.dedup();

    if unique_h.len() != N {
        println!("-1");
        exit(0);
    }

    let (comp_h, len_comp_h) = compress(&H);

    let mut bit = BIT::new(N);
    let mut ans = 0;

    for (i, h) in H.into_iter().enumerate() {
        ans += (i - bit.sum(comp_h[i])) * h;
        bit.add(comp_h[i], 1);
    }

    println!("{}", ans);
}
