use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
use std::ops::Bound::Included;
use proconio::{input, marker::Chars};

fn modpow(aa:  &usize, nn: &u128, mod_num: &u128) -> u128 {
    // a^n mod  fermat
    let mut a = *aa as u128;
    let mut n = *nn;
    let mut res: u128 = 1;
    while n > 0 {
        if n & 1  == 1 { res = res * a % mod_num; }
        a = a * a % mod_num;
        n >>= 1;
    }
    res
}

fn main() {
    input! {
        N: String,
    }
    let len = N.len();
    let n = N.parse::<usize>().unwrap();
    let mut ans = 0;
    let mut cnt = 9;
    let MOD: u128 = 998244353;
    for i in 0..(len - 1) {
        let mut tmp = (cnt) % MOD;
        tmp *= (cnt+1) % MOD;tmp %= MOD;
        tmp *= modpow(&2, &(MOD - 2), &MOD);tmp %= MOD;
        ans += tmp;
        ans %= MOD;
        cnt *= 10;
    }
    let mut left = n - 10usize.pow((len - 1) as u32) + 1;
    let mut tmp: u128 = left as u128;
    tmp *= (1+left) as u128 % MOD;tmp %= MOD;
    tmp *= modpow(&2, &(MOD - 2), &MOD);tmp %= MOD;
    ans += tmp;
    println!("{}",ans % MOD);
}