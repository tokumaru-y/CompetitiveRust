use either::Either::Left;
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

fn is_winner_left(left: char, right: char) -> bool {
    if left == 'P' {
        return (right == 'R' || right == 'P');
    } else if left == 'R' {
        return (right == 'S' || right == 'R');
    } else {
        return (right == 'P' || right == 'S');
    }
}

fn solve(
    N: usize,
    S: &Vec<char>,
    K: usize,
    offset: usize,
    two_index: &Vec<usize>,
    dp: &mut Vec<Vec<char>>,
) -> char {
    if K == 0 {
        return S[offset];
    }
    if dp[K][offset] != '?' {
        return dp[K][offset];
    }

    let offset2 = (offset + two_index[K - 1]) % N;
    let left = solve(N, S, K - 1, offset, two_index, dp);
    let right = solve(N, S, K - 1, offset2, two_index, dp);

    dp[K][offset] = if is_winner_left(left, right) {
        left
    } else {
        right
    };
    dp[K][offset]
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        S: Chars
    }
    let mut two_index = vec![1; K + 1];
    for i in 0..K {
        two_index[i + 1] = two_index[i] * 2 % N;
    }
    let mut dp = vec![vec!['?'; N + 1]; K + 1];
    let ans = solve(N, &S, K, 0, &two_index, &mut dp);

    println!("{}", ans);
}
