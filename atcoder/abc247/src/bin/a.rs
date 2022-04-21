use std::{collections::BinaryHeap, cmp::Reverse};

use proconio::{input, marker::Chars};
fn main() {
    input!{
        mut S: Chars,
    }
    S.insert(0, '0');
    let ans: String = S[..4].iter().collect();
    println!("{}", ans);
}