#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        N:usize,
    }
    let mut ans = 0;
    let sq = (N as f64).sqrt() as usize;
    for i in 1..=sq {
        ans += N / i - i;
    }
    ans*=2;
    for i in 1..=sq { ans += 1;}
    println!("{}",ans);
}