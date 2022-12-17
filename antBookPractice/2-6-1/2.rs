// https://atcoder.jp/contests/abc070/tasks/abc070_c
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
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
fn gcd(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    if x < y {
        std::mem::swap(&mut x, &mut y);
    }
    if y == 0 {
        return x;
    }
    return gcd(y, x % y);
}

fn lcm(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    if x < y {
        std::mem::swap(&mut x, &mut y);
    }
    x / gcd(x, y) * y
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: [usize; N],
    }
    let mut ans = 1;
    for t in T.into_iter() {
        ans = lcm(ans, t);
    }

    println!("{}", ans);
}
