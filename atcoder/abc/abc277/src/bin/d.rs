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
        M: usize,
        mut A: [usize; N]
    }
    A.sort();
    let mut target_list = A.clone();
    let total_num: usize = A.iter().sum();
    target_list.resize(2 * N + 1, 0);
    for i in (0..N).rev() {
        target_list[N + i] = A[i];
    }

    let mut ans = total_num;
    let mut left = 0;
    let mut sum_num = 0;
    for i in (0..target_list.len()) {
        if left == target_list[i] || (left + 1) % M == target_list[i] {
            sum_num += target_list[i];
        } else {
            sum_num = target_list[i];
            if i >= N {
                sum_num = 0;
                break;
            }
        }
        left = target_list[i];
        ans = min(ans, total_num - sum_num);
        if ans == 0 {
            break;
        }
    }

    println!("{}", ans);
}
