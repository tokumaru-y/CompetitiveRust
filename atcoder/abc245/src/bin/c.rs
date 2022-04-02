use proconio::input;

fn main() {
    input!{
        N: usize,
        K: usize,
        A: [usize; N],
        B: [usize; N],
    }
    let mut dp = vec![vec![false; 2]; N];
    dp[0][0] = true;
    dp[0][1] = true;
    for i in 1..N {
        for pre in 0..2 {
            for now in 0..2 {
                if dp[i-1][pre] {
                    let pre_num = if pre == 0 { A[i-1] } else { B[i-1] };
                    let now_num = if now == 0 { A[i] } else { B[i] };
                    let dif = if pre_num >= now_num { pre_num - now_num } else { now_num - pre_num };
                    dp[i][now] = if dif <= K { true || dp[i][now] } else { false || dp[i][now] };
                }
            }
        }
    }
    if dp[N-1][0] || dp[N-1][1] {
        println!("Yes");
    } else {
        println!("No");
    }
}