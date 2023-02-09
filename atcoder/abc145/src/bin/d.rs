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
// 二項係数
// 参考: https://drken1215.hatenablog.com/entry/2018/06/08/210000

fn combination_init(mod_num: usize) -> (Vec<usize>, Vec<usize>) {
    let max_len = 1000000;
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
        X: isize,
        Y: isize
    }
    let mut ans = 0;
    let (fanc, finv) = combination_init(MOD);
    for a in 0..=(X as usize) {
        let mut b = X - a as isize;
        if b % 2 == 1 {
            continue;
        }
        b /= 2;
        if (2 * a as isize + b == Y) {
            ans = max(ans, combination_calc(a + b as usize, a, MOD, &fanc, &finv))
        }
    }

    println!("{}", ans)
}
