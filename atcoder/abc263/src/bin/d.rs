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
const FIRST_VALUE: isize = std::isize::MIN;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        L: isize,
        R: isize,
        A: [isize; N]
    }
    let mut dp_left = vec![vec![std::isize::MAX; 2]; N + 1];
    let mut dp_right = vec![vec![std::isize::MAX; 2]; N + 1];

    dp_left[0][0] = 0;
    dp_left[0][1] = 0;
    dp_right[N][0] = 0;
    dp_right[N][1] = 0;

    for i in 0..N {
        for j in 0..2 {
            if j == 0 {
                dp_left[i + 1][0] = min(dp_left[i][0] + A[i], dp_left[i][1] + A[i]);
            } else {
                dp_left[i + 1][1] = dp_left[i][1] + L;
            }
        }
    }
    for i in (0..N).rev() {
        for j in 0..2 {
            if j == 0 {
                dp_right[i][0] = min(dp_right[i + 1][0] + A[i], dp_right[i + 1][1] + A[i]);
            } else {
                dp_right[i][1] = dp_right[i + 1][1] + R;
            }
        }
    }

    let mut ans = std::isize::MAX;

    for i in 0..=N {
        for j in 0..2 {
            ans = min(ans, dp_right[i][j] + dp_left[i][(j + 1) % 2]);
        }
    }

    println!("{}", ans);
}
