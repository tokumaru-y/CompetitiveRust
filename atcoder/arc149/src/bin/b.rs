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
// x以上の値の中で、最も左側にあるindexを返す。
// vecに対象が存在しない場合はv.lenを返す
fn lower_bound<T: std::cmp::PartialOrd>(vec: &Vec<T>, x: T) -> usize {
    let mut is_ng: isize = -1;
    let mut is_ok: isize = vec.len() as isize;
    while (is_ok - is_ng).abs() > 1 {
        let mid = (is_ok + is_ng) / 2;

        if x <= vec[mid as usize] {
            is_ok = mid;
        } else {
            is_ng = mid;
        }
    }

    is_ok as usize
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
    }
    let mut dp = vec![FIRST_VALUE; N + 1];
    dp[0] = 0;
    let mut AB = vec![];
    for i in 0..N {
        AB.push((A[i], B[i]));
    }
    AB.sort_by(|a, b| a.0.cmp(&b.0));

    for i in 0..N {
        let target = AB[i].1;
        let ind = lower_bound(&dp, target);
        dp[ind] = target;
    }

    for i in (0..=N).rev() {
        if dp[i] != FIRST_VALUE {
            println!("{}", i + N);
            exit(0);
        }
    }
}
