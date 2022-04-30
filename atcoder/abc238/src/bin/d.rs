#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;

fn main() {
    input!{
        mut T: usize,
    }
    while T > 0 {
        input! {
            A: usize,
            S: usize,
        }
        let mut is_ok = false;
        if 2 * A <= S {
            let d = S - 2 * A;
            if d & A == 0 { is_ok = true; }
        }
        if is_ok {
            println!("Yes");
        } else {
            println!("No")
        }
        T -= 1;
    }
}