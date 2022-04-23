use std::{collections::{BinaryHeap, HashMap, VecDeque}, cmp::Reverse};

use proconio::{input, marker::Chars};
fn main() {
    input!{
        S: Chars,
    }
    let mut used = vec![false; 10];
    for s in S.iter() {
        let s_usize = s.to_digit(10).unwrap() as usize;
        used[s_usize] = true;
    }
    for i in 0..10{
        if used[i] == false {
            println!("{}", i);
        }
    }
}