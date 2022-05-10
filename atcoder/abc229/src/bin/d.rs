#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars, mut K: usize,
    }
    let mut cnt_list = vec![0; S.len()+1];
    for i in 0..S.len() {
        if S[i] == '.' {
            cnt_list[i+1] = cnt_list[i] + 1;
        } else { 
            cnt_list[i+1] = cnt_list[i];
        }
    }
    let mut ans = 0;
    let mut right = 0;
    for left in 0..S.len() {
        while right < S.len() && cnt_list[right+1] - cnt_list[left] <= K {
            right += 1;
        }
        ans = max(ans, right - left);
    }
    println!("{}",ans);
}