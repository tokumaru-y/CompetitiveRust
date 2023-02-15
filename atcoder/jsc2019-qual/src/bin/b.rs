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

fn read() -> usize {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<usize>().unwrap()
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
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
        K: usize,
        A: [usize; N]
    }
    let mut ans = 0;
    for i in 0..N {
        let mut right_cnt = 0;
        let mut total_cnt = 0;
        for j in i..N {
            if A[i] > A[j] {
                right_cnt += 1;
            }
        }
        for j in 0..N {
            if A[i] > A[j] {
                total_cnt += 1;
            }
        }

        ans += right_cnt * K % MOD;
        ans %= MOD;
        ans += (K - 1) * K / 2 % MOD * total_cnt % MOD;
        ans %= MOD;
    }

    println!("{}", ans);
}
