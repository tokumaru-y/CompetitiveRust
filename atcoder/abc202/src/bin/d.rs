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

fn find_kth(A: usize, B: usize, K: usize, dp: &Vec<Vec<usize>>) -> VecDeque<char> {
    if A == 0 {
        return VecDeque::from(vec!['b'; B]);
    }
    if B == 0 {
        return VecDeque::from(vec!['a'; A]);
    }

    if K <= dp[A - 1][B] {
        let mut res = find_kth(A - 1, B, K, dp);
        res.push_front('a');
        return res;
    } else {
        let mut res = find_kth(A, B - 1, K - dp[A - 1][B], dp);
        res.push_front('b');
        return res;
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        K: usize,
    }
    let mut dp = vec![vec![0; 50]; 50];
    dp[0][0] = 1;
    for i in 0..=A {
        for j in 0..=B {
            if i > 0 {
                dp[i][j] += dp[i - 1][j];
            }
            if j > 0 {
                dp[i][j] += dp[i][j - 1];
            }
        }
    }

    let deque = find_kth(A, B, K, &dp);

    let ans = deque.into_iter().join("");
    println!("{}", ans);
}
