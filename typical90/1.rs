// https://atcoder.jp/contests/typical90/tasks/typical90_a
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

fn binary_search(value: usize, div: &Vec<usize>, K: usize) -> bool {
    let mut tmp_sum = 0;
    let mut cut_cnt = 0;
    for d in div.iter() {
        tmp_sum += d;
        if tmp_sum >= value && cut_cnt < K {
            cut_cnt += 1;
            tmp_sum = 0;
        }
    }
    cut_cnt >= K && tmp_sum >= value
}
fn main() {
    input! {
        N: usize,
        L: usize,
        K: usize,
        A: [usize; N]
    }
    let mut div = Vec::new();
    for i in 0..N {
        let len = if i == 0 { A[i] } else { A[i] - A[i - 1] };
        div.push(len);
    }
    div.push(L - A[N - 1]);
    let mut left = 0;
    let mut right = L;
    let mut middle;
    while right - left > 1 {
        middle = (right + left) / 2;
        if binary_search(middle, &div, K) {
            left = middle;
        } else {
            right = middle;
        }
    }
    println!("{}", left);
}
