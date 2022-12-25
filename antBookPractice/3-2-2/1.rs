// https://atcoder.jp/contests/arc022/tasks/arc022_2
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
        A: [usize; N]
    }
    let mut ans = 0;
    let mut checked_num = vec![false; 10usize.pow(5) + 1];

    let mut right = 0;
    for left in 0..N {
        while right < N && !checked_num[A[right]] {
            checked_num[A[right]] = true;
            right += 1;
        }

        ans = max(ans, right - left);

        if right == left {
            right += 1;
        } else {
            checked_num[A[left]] = false;
        }
    }

    println!("{}", ans);
}
