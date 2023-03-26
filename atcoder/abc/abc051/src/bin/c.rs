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

const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 1_000_000_007;

#[allow(non_snake_case)]
fn main() {
    input! {
        sx: isize,
        sy: isize,
        tx: isize,
        ty: isize,
    }
    let mut ans = vec![];

    ans.push(vec!['R'; (tx - sx) as usize].into_iter().join(""));
    ans.push(vec!['U'; (ty - sy) as usize].into_iter().join(""));
    ans.push(vec!['L'; (tx - sx) as usize].into_iter().join(""));
    ans.push(vec!['D'; (ty - sy) as usize].into_iter().join(""));

    ans.push(String::from("D"));
    ans.push(vec!['R'; (tx - sx + 1) as usize].into_iter().join(""));
    ans.push(vec!['U'; (ty - sy + 1) as usize].into_iter().join(""));
    ans.push(String::from("L"));
    ans.push(String::from("U"));
    ans.push(vec!['L'; (tx - sx + 1) as usize].into_iter().join(""));
    ans.push(vec!['D'; (ty - sy + 1) as usize].into_iter().join(""));
    ans.push(String::from("R"));

    println!("{}", ans.into_iter().join(""));
}
