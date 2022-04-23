use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        S: usize,
        T: usize,
        X: usize,
        UV: [[usize; 2]; M],
    }
    let MOD = 998244353;
    let mut list = vec![Vec::new(); N];
    for i in 0..M {
        let n = &UV[i];
        let a = n[0] - 1 ;let b = n[1] - 1;
        list[a].push(b);
        list[b].push(a);
    }
    let mut dp = vec![vec![vec![0; 2]; N]; K+1];
    if S == X {
        dp[0][S-1][1] = 1;
    } else {
        dp[0][S-1][0] = 1;
    }
    for k in 0..K{
        for j in 0..N {
            for i in 0..2 {
                for l_ind in 0..list[j].len() {
                    let n_ind = list[j][l_ind];
                    if n_ind + 1 == X {
                        dp[k+1][n_ind][i] += dp[k][j][(i+1)%2];
                    } else {
                        dp[k+1][n_ind][i] += dp[k][j][i];
                    }
                    dp[k+1][n_ind][i] %= MOD;
                }
            }
        }
    }
    println!("{}", dp[K][T-1][0] % MOD);
}