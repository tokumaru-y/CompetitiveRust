// https://atcoder.jp/contests/abc018/tasks/abc018_4
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
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        P: usize,
        Q: usize,
        R: usize,
        XYZ: [(Usize1, Usize1, usize); R]
    }
    let mut happiness_table = vec![vec![0; M]; N];

    for (x, y, z) in XYZ.iter() {
        happiness_table[*x][*y] += *z;
    }

    let mut ans = 0;
    for bit in (0..(1 << N)) {
        let mut tmp_cnt = vec![0; M];
        let mut girl_cnt = 0;

        for i in (0..N) {
            if (bit & 1 << i) >= 1 {
                girl_cnt += 1;
                for j in (0..M) {
                    tmp_cnt[j] += happiness_table[i][j];
                }
            }
        }

        if girl_cnt > P {
            continue;
        }

        tmp_cnt.sort_by(|a, b| b.cmp(a));
        ans = max(ans, tmp_cnt[..Q].iter().sum());
    }

    println!("{}", ans);
}
