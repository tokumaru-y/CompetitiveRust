// https://atcoder.jp/contests/abc054/tasks/abc054_c
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
        AB: [(usize, usize); M]
    }
    let mut ans = 0;
    let mut linked_list = vec![vec![false; N + 1]; N + 1];
    for (a, b) in AB.into_iter() {
        linked_list[a][b] = true;
        linked_list[b][a] = true;
    }

    for p in (1..=N).permutations(N) {
        if p[0] != 1 {
            continue;
        }
        let mut is_ok = true;
        for ind in (1..N) {
            is_ok &= linked_list[p[ind - 1]][p[ind]];
        }
        if is_ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
