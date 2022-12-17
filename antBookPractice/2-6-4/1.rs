// https://atcoder.jp/contests/tenka1-2012-qualc/tasks/tenka1_2012_9
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
#[allow(non_snake_case)]
fn eratoshthenes(n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let mut checked_num = vec![false; n + 1];

    if n <= 1 {
        return Vec::new();
    }
    checked_num[0] = true;
    checked_num[1] = true;
    for i in (2..=n) {
        if checked_num[i] {
            continue;
        }

        let mut tmp = i;
        while tmp <= n {
            checked_num[tmp] = true;
            tmp += i;
        }

        res.push(i);
    }

    res
}

fn main() {
    input! {
        N: usize
    }
    let r = eratoshthenes(N - 1);
    println!("{}", r.len());
}
