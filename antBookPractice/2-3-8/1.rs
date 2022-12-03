// https://atcoder.jp/contests/abc110/tasks/abc110_d
#[allow(unused_imports)]
use itertools::Itertools;
use num_integer::Roots;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    process::exit,
};
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;

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
    mod_num: &usize,
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
fn make_list(X: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let mut m = X;
    for i in (2..=X.sqrt()) {
        let mut cnt1 = 0;
        while m % i == 0 {
            cnt1 += 1;
            m /= i;
        }
        res.push(cnt1);
    }
    if m != 1 {
        res.push(1);
    }
    res
}

fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut multiple_list = make_list(M);
    let (fac, finv) = combination_init(&MOD);
    let mut ans = 1;
    for i in (0..multiple_list.len()) {
        let cnt = multiple_list[i];
        ans *= combination_calc(N + cnt - 1, N - 1, &MOD, &fac, &finv);
        ans %= MOD;
    }
    println!("{}", ans);
}
