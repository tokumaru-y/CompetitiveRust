// https://atcoder.jp/contests/abc012/tasks/abc012_4
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
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        ABC: [(Usize1, Usize1, usize); M]
    }
    let mut dp = vec![vec![FIRST_VALUE as isize; N]; N];
    for i in (0..N) {
        dp[i][i] = 0;
    }

    for (a, b, c) in ABC.iter() {
        dp[*a][*b] = *c as isize;
        dp[*b][*a] = *c as isize;
    }

    for k in (0..N) {
        for i in (0..N) {
            for j in (0..N) {
                dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
            }
        }
    }

    let mut ans = FIRST_VALUE as isize;
    for i in (0..N) {
        let mut tmp_max = 0;
        for j in (0..N) {
            if i == j {
                continue;
            }
            tmp_max = max(tmp_max, dp[i][j]);
        }

        ans = min(ans, tmp_max);
    }

    println!("{}", ans);
}
