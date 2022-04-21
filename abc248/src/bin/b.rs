use std::{collections::{BinaryHeap, HashMap, VecDeque}, cmp::Reverse};

use proconio::{input, marker::Chars};
fn main() {
    input!{
        A: usize,
        B: usize,
        K: usize,
    }
    let mut ans: usize = 0;
    let mut cnt = A;
    while cnt < B {
        cnt *=K;
        ans += 1;
    }
    println!("{}", ans);
}