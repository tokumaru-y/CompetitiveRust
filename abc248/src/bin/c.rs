use proconio::input;

fn main() {
    input!{
        N: usize,
        M: usize,
        K: usize,
    }
    let MOD = 998244353;
    let mut dp = vec![vec![0; 50*50+1]; 51];
    dp[0][0] = 1;
    for i in 0..N{
        for j in 1..M+1{
            for k in 0..50*50+1{
                if k >= j && dp[i][k-j] >= 1 {
                    dp[i+1][k] += dp[i][k-j];
                    dp[i+1][k] %= MOD;
                }
            }
        }
    }
    let mut ans = dp[N][..K+1].iter().sum::<usize>() % MOD;
    println!("{}", ans);
}
