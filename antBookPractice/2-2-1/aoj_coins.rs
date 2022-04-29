#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
#[allow(unused_imports)]
use proconio::{input, marker::Chars};
fn main() {
    input! {
        N: usize,
        M: usize,
        C: [usize; M],
    }
    let mut dp = vec![50_001; 10_001];
    dp[0] = 0;
    for i in 0..10_001 {
        for c in C.iter() {
            let ind = i + *c;
            if ind <= 10_000 && dp[ind] > dp[i] + 1 {
                dp[ind] = dp[i] + 1;
            }
        }
    }
    println!("{}", dp[N]);
}