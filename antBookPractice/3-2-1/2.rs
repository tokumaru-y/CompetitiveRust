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
        K: usize,
        S: [usize; N]
    }
    for s in S.iter() {
        if *s == 0 {
            println!("{}", N);
            exit(0);
        }
    }

    let mut ans = 0;
    let mut right = 0;
    let mut sum = 1;
    for left in 0..N {
        while right < N && sum * S[right] <= K {
            sum *= S[right];
            right += 1;
        }

        ans = max(ans, right - left);
        if right == left {
            right += 1;
        } else {
            sum /= S[left];
        }
    }

    println!("{}", ans);
}
