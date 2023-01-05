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
        S: usize,
        AB: [(usize, usize); N]
    }
    let mut dp = vec![vec![false; S + 1]; N + 1];
    dp[0][0] = true;
    for i in 0..N {
        let (a, b) = AB[i];
        for j in 0..=S {
            if dp[i][j] {
                if j + a <= S {
                    dp[i + 1][j + a] = true;
                }
                if j + b <= S {
                    dp[i + 1][j + b] = true;
                }
            }
        }
    }

    if dp[N][S] {
        println!("Yes");
        let mut ans = vec![];
        let mut left = S;
        let mut i = N;
        while left > 0 {
            let (a, b) = AB[i - 1];
            if left >= a && dp[i - 1][left - a] {
                left -= a;
                ans.push("H");
            } else {
                left -= b;
                ans.push("T");
            }
            i -= 1;
        }
        ans.reverse();
        println!("{}", ans.iter().join(""));
    } else {
        println!("No");
    }
}
