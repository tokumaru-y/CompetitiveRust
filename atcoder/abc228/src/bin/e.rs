#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: u128,K: u128, M: u128,
    }
    let mod_num = 998244353;
    let list_cnt = modpow(&K, &N, &(mod_num-1));
    let mut ans = modpow(&M, &list_cnt, &mod_num);
    if M%mod_num == 0 { ans = 0; }
    println!("{}",ans%mod_num);
}

fn modpow(aa:  &u128, nn: &u128, mod_num: &u128) -> u128 {
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