// https://atcoder.jp/contests/arc098/tasks/arc098_b

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
        A: [usize; N]
    }
    let mut ans = 0;
    let mut right = 0;
    let mut xor_sum = 0;
    let mut sum = 0;
    for left in 0..N {
        while right < N && ((xor_sum ^ A[right]) == sum + A[right] || right <= left) {
            xor_sum ^= A[right];
            sum += A[right];
            right += 1;
        }
        ans += right - left;
        if right == left {
            right += 1;
        } else {
            xor_sum ^= A[left];
            sum -= A[left];
        }
    }

    println!("{}", ans);
}
