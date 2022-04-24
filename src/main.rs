use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut T: usize,
    }
    let MOD = 998244353;
    let mut alpha_tabel = Vec::new();
    for b in b'A'..=b'Z' {
        alpha_tabel.push(char::from(b));
    }
    while T > 0 {
        T -= 1;
        input! {
            N: usize,
            S: Chars,
        }
        let limit = if N % 2 == 0 { N / 2 } else { N / 2 + 1 };
        let mut dp = vec![vec![vec![0; 2]; 26]; limit+1];
        for i in 0..26 {
            if alpha_tabel[i] < S[0] && alpha_tabel[i] < S[N-1] {
                dp[1][i][0] += 1;
            } else if (alpha_tabel[i] < S[0] && alpha_tabel[i] == S[N-1]) || (alpha_tabel[i] == S[0] && alpha_tabel[i] < S[N-1]) || (alpha_tabel[i] == S[0] && alpha_tabel[i] == S[N-1]) {
                dp[1][i][1] += 1;
            }
        }
        for i in 1..limit {
            for j in 0..26 {
                for k in 0..26 {
                    for m in 0..2{
                        if m == 0 {
                            // なんでもOK
                            dp[i+1][j][m] += dp[i][k][0] + dp[i][k][1];
                            dp[i+1][j][m] %= MOD;
                        } else {
                            // ギリギリを攻める
                            if alpha_tabel[j] < S[i] && alpha_tabel[j] < S[N-i-1] {
                                dp[i+1][j][0] += dp[i][k][0] + dp[i][k][1];
                                dp[i+1][j][0] %= MOD;
                            } else if (alpha_tabel[j] < S[i] && alpha_tabel[j] == S[N-i-1]) || (alpha_tabel[j] == S[i] && alpha_tabel[j] < S[N-i-1]) || (alpha_tabel[j] == S[i] && alpha_tabel[j] == S[N-i-1]) {
                                dp[i+1][j][1] += dp[i][k][1];
                                dp[i+1][j][1] %= MOD;
                            }
                        }
                    }
                }
            }
        }
        // for i in 1..limit {
        //     for j in 0..26 {
        //         for k in 0..26 {
        //             if alpha_tabel[j] < S[i] && alpha_tabel[j] < S[N-i-1] {
        //                 // dp[i+1][j][0] += dp[i][k][0];
        //                 dp[i+1][j][0] += dp[i][k][1];
        //                 dp[i+1][j][0] %= MOD;
        //             } else if (alpha_tabel[j] < S[i] && alpha_tabel[j] == S[N-i-1]) || (alpha_tabel[j] == S[i] && alpha_tabel[j] < S[N-i-1]) || (alpha_tabel[j] == S[i] && alpha_tabel[j] == S[N-i-1]) {
        //                 dp[i+1][j][1] += dp[i][k][1];
        //                 dp[i+1][j][1] %= MOD;
        //             }
        //             dp[i+1][j][0] += dp[i][k][0];
        //             dp[i+1][j][0] %= MOD;
        //         }
        //     }
        // }
        let mut ans = 0;
        for i in 0..26 {
            ans += dp[limit][i][0] + dp[limit][i][1];
            ans %= MOD;
        }
        println!("{}", ans);
    }
}