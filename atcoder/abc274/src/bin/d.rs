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
        X: isize,
        Y: isize,
        A: [isize; N]
    }
    let mut hash_set_x = HashSet::new();
    let mut hash_set_y = HashSet::new();

    hash_set_x.insert(A[0]);
    hash_set_y.insert(0);

    for i in (1..N) {
        let mut tmp_set = HashSet::new();
        if i % 2 == 0 {
            for _x in hash_set_x.iter() {
                tmp_set.insert(_x + A[i]);
                tmp_set.insert(_x - A[i]);
            }
            hash_set_x = tmp_set.clone();
        } else {
            for _y in hash_set_y.iter() {
                tmp_set.insert(_y + A[i]);
                tmp_set.insert(_y - A[i]);
            }
            hash_set_y = tmp_set.clone();
        }
    }

    let is_reachable_x = hash_set_x.contains(&X);
    let is_reachable_y = hash_set_y.contains(&Y);

    let ans = if is_reachable_x && is_reachable_y {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
