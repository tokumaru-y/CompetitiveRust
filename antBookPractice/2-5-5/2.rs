// https://atcoder.jp/contests/abc073/tasks/abc073_d
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
use std::vec;
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
        R: usize,
        Rs: [Usize1; R],
        ABC: [(Usize1, Usize1, isize); M],
    }
    let mut dp = vec![vec![FIRST_VALUE as isize; N]; N];

    for i in (0..N) {
        dp[i][i] = 0;
    }
    for (a, b, c) in ABC.into_iter() {
        dp[a][b] = c;
        dp[b][a] = c;
    }

    for k in (0..N) {
        for i in (0..N) {
            for j in (0..N) {
                dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
            }
        }
    }

    let mut ans = FIRST_VALUE as isize;
    for way_list in Rs.iter().permutations(R) {
        let mut tmp_way_cost_sum = 0;
        for i in (1..R) {
            let s = way_list[i - 1];
            let e = way_list[i];
            tmp_way_cost_sum += dp[*s][*e];
        }
        ans = min(ans, tmp_way_cost_sum);
    }

    println!("{}", ans);
}
