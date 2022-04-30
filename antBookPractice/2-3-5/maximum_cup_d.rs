#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        L: usize,
        X: usize,
        A: [usize; N],
    }
    let unreach = 10_001;
    let mut dp = vec![vec![unreach; M]; N+1];
    dp[0][0] = 1;
    for i in 0..N {
        for j in 0..M {
            if dp[i][j] != unreach {
                let position = (j+A[i]) % M;
                let cnt = (j+A[i]) / M;
                dp[i+1][position] = min(dp[i+1][position], dp[i][j] + cnt);
            }
            dp[i+1][j] = min(dp[i+1][j], dp[i][j]);
        }
    }
    let mut ans = unreach;
    for i in (1..=N).rev() {
        ans = min(ans, dp[i][L]);
    }
    println!("{}",if ans <= X { "Yes" } else { "No" });
}