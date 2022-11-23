// https://atcoder.jp/contests/abc051/tasks/abc051_b
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
use std::collections::HashSet;
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        XY: [(i32, i32); N]
    }
    let mut ans = 0;
    for i in 0..N {
        let (x, y) = XY[i];
        for j in (i + 1)..N {
            let (a, b) = XY[j];
            ans = ans.max((x - a).pow(2) + (y - b).pow(2));
        }
    }

    println!("{}", (ans as f64).sqrt())
}
