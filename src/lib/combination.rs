// 二項係数
// 参考: https://drken1215.hatenablog.com/entry/2018/06/08/210000

fn combination_init() -> (Vec<usize>, Vec<usize>) {
    let max_len = 510000;
    // 累積積
    let mut fac = vec![0; max_len];
    // 累積積（除算）
    let mut finv = vec![0; max_len];
    // 逆元
    let mut inv = vec![0; max_len];

    fac[0] = 1;
    fac[1] = 1;
    finv[0] = 1;
    finv[1] = 1;
    inv[1] = 1;
    for i in 2..max_len {
        fac[i] = fac[i - 1] * i % MOD;
        inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
        finv[i] = finv[i - 1] * inv[i] % MOD;
    }
    (fac, finv)
}

fn combination_calc(n: usize, k: usize, fac: &Vec<usize>, finv: &Vec<usize>) -> usize {
    if n < k {
        return 0;
    }
    if n < 0 || k < 0 {
        return 0;
    }
    fac[n] * (finv[k] * finv[n - k] % MOD) % MOD
}
