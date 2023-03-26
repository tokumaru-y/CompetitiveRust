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
        M: usize,
        ABC: [(Usize1, Usize1, usize); M]
    }

    let mut dp = vec![vec![10usize.pow(10); N]; N];
    for i in 0..N {
        dp[i][i] = 0;
    }

    for (a, b, c) in ABC.into_iter() {
        dp[a][b] = c;
    }

    let mut ans = 0;
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
                if dp[i][j] < 10usize.pow(10) {
                    ans += dp[i][j];
                }
            }
        }
    }

    println!("{}", ans);
}
