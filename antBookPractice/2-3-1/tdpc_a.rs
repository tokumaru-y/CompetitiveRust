#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
#[allow(unused_imports)]
use proconio::{input, marker::Chars};
fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }
    let mut dp = vec![false; 100_001];
    dp[0] = true;
    for p in P.into_iter() {
        for i in (0..=100_00).rev() { dp[i+p] |= dp[i] };
    }
    let ans = dp.iter().filter(|x| **x == true).count();
    println!("{}", ans);
}