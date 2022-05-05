#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N],
    }
    let mut sum_list = vec![0; N+1];
    for i in 0..N { sum_list[i+1] = sum_list[i] + A[i]; }
    let mut map = BTreeMap::new();
    // annotationをしないと,i32型になる。
    // そうした場合、Σ20000の時にオーバーフローしてしまうのでusizeに変更
    let mut ans:usize = 0;
    for i in 0..=N{
        ans += map.get(&(sum_list[i])).unwrap_or_else(|| &0);
        *map.entry(sum_list[i]+K).or_insert(0) += 1;
    }
    println!("{}",ans);
}