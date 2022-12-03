// https://atcoder.jp/contests/code-thanks-festival-2017-open/tasks/code_thanks_festival_2017_c
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
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    process::exit,
};
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        AB: [(isize, isize); N]
    }
    let mut priority_queue = BinaryHeap::new();
    for (a, b) in AB.into_iter() {
        priority_queue.push((-a, b));
    }
    let mut ans = 0;
    for i in 0..K {
        let (a, b) = priority_queue.pop().unwrap();
        ans += -a;
        priority_queue.push((a - b, b));
    }
    println!("{}", ans);
}
