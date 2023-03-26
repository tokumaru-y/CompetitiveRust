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

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut dp = vec![vec![0; 10]; 10];
    for i in 1..=N {
        let char_v = i.to_string().chars().collect::<Vec<char>>();
        let top = char_v[0].to_digit(10u32).unwrap() as usize;
        let bottom = char_v[char_v.len() - 1].to_digit(10).unwrap() as usize;
        dp[top][bottom] += 1;
    }

    let mut ans = 0;
    for i in 1..10 {
        for j in 1..10 {
            ans += dp[i][j] * dp[j][i];
        }
    }

    println!("{}", ans);
}
