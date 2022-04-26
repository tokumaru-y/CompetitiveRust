use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
use std::ops::Bound::Included;
use proconio::{input, marker::Chars};
fn main() {
    input! {
        N: usize,
        X: usize,
        AB: [[usize; 2]; N]
    }
    let mut dp = vec![vec![false; 10001]; 101];
    dp[0][0] = true;
    for i in 0..N {
        for j in 0..=10000 {
            let next = &AB[i];
            if (j >= next[0] && (dp[i][j-next[0]])) || (j >= next[1] && (dp[i][j-next[1]])) {
                dp[i+1][j] = true;
            }
        }
    }
    let ans = if dp[N][X] { "Yes" } else { "No"};
    println!("{}",ans);
}