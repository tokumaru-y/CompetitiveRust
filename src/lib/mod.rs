// 繰り返し二乗法
fn modpow(aa: &usize, nn: &u128, mod_num: &u128) -> u128 {
    // a^n mod  fermat
    let mut a = *aa as u128;
    let mut n = *nn;
    let mut res: u128 = 1;
    while n > 0 {
        if n & 1 == 1 {
            res = res * a % mod_num;
        }
        a = a * a % mod_num;
        n >>= 1;
    }
    res
}

// Fermatの小定理
fn modinverse(num: usize) -> usize {
    modpow(num, MOD - 2)
}
