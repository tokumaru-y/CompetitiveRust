#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        N: usize,
        LR: Chars,
    }
    let mut deque = VecDeque::new();
    deque.push_back(N);
    for i in (0..N).rev(){
        if LR[i] == 'R' {
            deque.push_front(i);
        } else {
            deque.push_back(i);
        }
    }
    let ans = deque.iter().map(|x| (*x).to_string()).join(" ");
    println!("{}",ans);
}