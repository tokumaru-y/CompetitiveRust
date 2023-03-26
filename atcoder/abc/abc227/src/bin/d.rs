use std::collections::HashSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};

fn is_ok(middle: usize, N: usize, K: usize, A: &Vec<usize>) -> bool {
    let mut fix_sum = 0;
    for i in 0..N { fix_sum += min(middle, A[i]); }
    fix_sum >= middle * K
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,K: usize,
        A: [usize; N],
    }
    let mut ok = 0;
    let mut ng = 1000_000_000_000_000_0000;
    while ng - ok > 1 {
        let middle = (ok + ng) / 2;
        if is_ok(middle, N, K, &A) {
            ok = middle;
        } else {
            ng = middle;
        }
    }
    println!("{}",ok);
}