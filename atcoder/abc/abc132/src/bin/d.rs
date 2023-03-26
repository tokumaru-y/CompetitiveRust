#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
use std::{
    fmt::Debug,
    io::{stdout, Write},
    mem::swap,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}

fn read() -> usize {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<usize>().unwrap()
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 1_000_000_007;
// 繰り返し二乗法
fn modpow(aa: usize, nn: usize, mod_num: usize) -> usize {
    // a^n mod  fermat
    let mut a = aa;
    let mut n = nn;
    let mut res = 1;
    while n > 0 {
        if n & 1 == 1 {
            res = res * a % mod_num;
        }
        a = a * a % mod_num;
        n >>= 1;
    }
    res
}
fn combination_init(mod_num: usize) -> (Vec<usize>, Vec<usize>) {
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
        fac[i] = fac[i - 1] * i % mod_num;
        inv[i] = mod_num - inv[mod_num % i] * (mod_num / i) % mod_num;
        finv[i] = finv[i - 1] * inv[i] % mod_num;
    }
    (fac, finv)
}

fn combination_calc(
    n: usize,
    k: usize,
    mod_num: usize,
    fac: &Vec<usize>,
    finv: &Vec<usize>,
) -> usize {
    if n < k {
        return 0;
    }
    if n < 0 || k < 0 {
        return 0;
    }
    fac[n] * (finv[k] * finv[n - k] % mod_num) % mod_num
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut choice_n = N - K + 1;
    let (fac, finv) = combination_init(MOD);
    for i in 1..=K {
        let mut ans = 1;
        ans *= combination_calc(choice_n, i, MOD, &fac, &finv);
        ans *= combination_calc(K - 1, (i - 1), MOD, &fac, &finv);
        ans %= MOD;

        println!("{}", ans);
    }
}
