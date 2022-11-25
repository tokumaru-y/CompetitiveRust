// https://atcoder.jp/contests/abc103/tasks/abc103_d
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
const FIRST_VALUE: usize = 10_000_000;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut AB: [(usize, usize); M]
    }
    let mut ans = 0;
    let mut remove_bridge_num = 0;
    AB.sort_by(|a, b| a.1.partial_cmp(&(b.1)).unwrap());
    for (a, b) in AB.into_iter() {
        if remove_bridge_num > a {
            continue;
        }
        remove_bridge_num = b;
        ans += 1;
    }

    println!("{}", ans);
}
