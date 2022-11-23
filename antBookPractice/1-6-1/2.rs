// https://atcoder.jp/contests/abc051/tasks/abc051_b
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
};
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        S: usize,
    }
    let mut ans = 0;
    for x in 0..=K {
        for y in 0..=K {
            if S < x + y {
                continue;
            } else if S - (x + y) <= K {
                ans += 1;
            }
        }
    }

    println!("{}", ans)
}
