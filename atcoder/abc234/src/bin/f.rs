#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]

const MOD: usize = 998244353;
fn main() {
    input! {
        S: Chars,
    }
    let alph_cnt = 26;
    let mut alph_cnt_list = vec![0; alph_cnt];
    for i in 0..S.len(){
        alph_cnt_list[S[i] as usize - 'a' as usize] += 1;
    }
    let (fac, finv) = combination_init(&MOD);
    let mut dp = vec![vec![0; 5001]; alph_cnt+1];
    dp[0][0] = 1;
    for i in 0..alph_cnt {
        for j in 0..=S.len() {
            if dp[i][j] == 0 { continue; }
            for k in 0..=alph_cnt_list[i] {
                dp[i+1][j+k] += dp[i][j] * combination_calc(j+k, k, &MOD, &fac, &finv);
                dp[i+1][j+k] %= MOD;
            }
        }
    }
    println!("{}", dp[alph_cnt].iter().skip(1).sum::<usize>() % MOD);
}

// 二項係数
// 参考: https://drken1215.hatenablog.com/entry/2018/06/08/210000

fn combination_init(mod_num: &usize) -> (Vec<usize>, Vec<usize>) {
    let max_len = 510000;
    // 累積積
    let mut fac = vec![0; max_len];
    // 累積積（除算）
    let mut finv = vec![0; max_len];
    // 逆元
    let mut inv = vec![0; max_len];

    fac[0] = 1;fac[1] = 1;
    finv[0] = 1;finv[1] = 1;
    inv[1] = 1;
    for i in 2..max_len {
        fac[i] = fac[i-1] * i % mod_num;
        inv[i] = mod_num - inv[mod_num%i] * (mod_num / i) % mod_num;
        finv[i] = finv[i-1] * inv[i] % mod_num;
    }
    (fac, finv)
}

fn combination_calc(n: usize, k: usize, mod_num: &usize, fac: &Vec<usize>, finv: &Vec<usize>) -> usize {
    if n < k { return 0;}
    if n < 0 || k < 0 { return 0; }
    fac[n] * (finv[k] * finv[n-k] % mod_num) % mod_num
}