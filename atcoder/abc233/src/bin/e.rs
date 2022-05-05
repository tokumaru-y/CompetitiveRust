#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        mut X: Chars,
    }
    let mut digit_sum = 0;
    for i in 0..X.len() {
        digit_sum += (X[i] as usize - '0' as usize ) as usize;
    }
    let mut move_up_cnt = 0;
    let mut ans = String::from("");
    for i in (0..X.len()).rev() {
        move_up_cnt += digit_sum;
        digit_sum -= (X[i] as usize - '0' as usize) as usize;
        ans.push_str(&(move_up_cnt%10).to_string());
        move_up_cnt /= 10;
    }

    while move_up_cnt > 0 {
        ans.push_str(&(move_up_cnt%10).to_string());
        move_up_cnt /= 10;
    }
    println!("{}", ans.chars().rev().collect::<String>());

}