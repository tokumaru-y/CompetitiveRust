// https://atcoder.jp/contests/arc006/tasks/arc006_3
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
        W: [usize; N]
    }
    let mut list = vec![FIRST_VALUE; N + 1];
    list[0] = W[0];
    for w in W.iter() {
        let b_ind = match list.binary_search(w) {
            Ok(i) => i,
            Err(i) => i,
        };
        list[b_ind] = *w;
    }
    let mut ans = list.iter().filter(|x| **x != FIRST_VALUE).count();
    println!("{}", ans);
}
