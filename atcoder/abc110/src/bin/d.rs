#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut ans = 1;
    let mod_num = 1000_000_007;
    let (fac, finv) = combination_init(&mod_num);
    let div = divisor(&M);
    for d in div.iter() {
        let num = *d.0;let cnt = *d.1;
        ans *= combination_calc(N+cnt-1, N - 1, &mod_num, &fac, &finv) % mod_num;
        ans %= mod_num;
    }
    println!("{}", ans);
}

fn divisor(n: &usize) -> HashMap<usize, usize>{
    let mut res = HashMap::new();
    let mut num = *n;
    for i in 2..=((*n as f64).sqrt() as usize) {
        while num % i == 0 {
            *res.entry(i).or_insert(0) += 1;
            num /= i
        }
    }
    if num != 1 && num > 0 { *res.entry(num).or_insert(0) += 1; }
    res
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