use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};

use proconio::{input, marker::Chars};

fn main() {
    input!{
        N: usize,
    }
    let mut dp:Vec<Vec<usize>> = vec![vec![0;10]; N+1];
    for i in 1..10 {
        dp[1][i] = 1;
    }
    let MOD = 998244353;
    for i in 1..N{
        for j in 1..10 {
            if dp[i][j] >= 1{
                if j >= 2 {
                    dp[i+1][j-1] += dp[i][j];
                    dp[i+1][j-1] %= MOD;
                }
                if j <= 8 {
                    dp[i+1][j+1] += dp[i][j];
                    dp[i+1][j+1] %= MOD;
                }
                dp[i+1][j] += dp[i][j];
                dp[i+1][j] %= MOD;
            }
        }
    }
    let mut ans = 0;
    for i in 1..10 {
        ans += dp[N][i];
        ans %= MOD;
    }
    println!("{}", ans%MOD);
}