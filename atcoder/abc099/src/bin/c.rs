use proconio::input;

fn main() {
    todo!();
}
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

const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 1_000_000_007;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut six_v = vec![];
    let mut nine_v = vec![];
    let mut six_num = 6;
    let mut nine_num = 9;
    while six_num <= N {
        six_v.push(six_num);
        six_num *= 6;
    }
    while nine_num <= N {
        nine_v.push(nine_num);
        nine_num *= 9;
    }

    let mut dp = vec![std::usize::MAX; N + 1];
    dp[N] = 0;
    for i in (0..=N).rev() {
        for j in 0..(six_v.len()) {
            if i >= six_v[j] && dp[i] <= N {
                let d = i - six_v[j];
                dp[d] = min(dp[d], dp[i] + 1);
            }
        }
        for j in 0..(nine_v.len()) {
            if i >= nine_v[j] && dp[i] <= N {
                let d = i - nine_v[j];
                dp[d] = min(dp[d], dp[i] + 1);
            }
        }
    }

    let mut ans = std::usize::MAX;
    for i in 0..=N {
        if dp[i] > N {
            continue;
        }
        ans = min(ans, i + dp[i]);
    }

    println!("{}", ans);
}
