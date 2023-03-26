#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let mut num_pos = Vec::new();
    let mut pos_num = Vec::new();
    for i in 0..N {
        num_pos.push(i);pos_num.push(i);
    }
    for _ in 0..Q {
        input! {
            X: Usize1,
        }
        let x_pos = num_pos[X];
        let swap_pos = if x_pos == N - 1 { N - 2 } else { x_pos + 1 };
        let swap_num = pos_num[swap_pos];
        
        pos_num[x_pos] = swap_num;pos_num[swap_pos] = X;
        num_pos[X] = swap_pos;num_pos[swap_num] = x_pos;
    }
    for i in 0..pos_num.len() {
        print!("{} ", pos_num[i]+1);
    }
}