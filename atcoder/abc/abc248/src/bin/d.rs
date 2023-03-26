use std::{collections::{BinaryHeap, HashMap, VecDeque}, cmp::Reverse};

use proconio::{input, marker::Chars};

fn main() {
    input!{
        N: usize,
        A: [usize; N],
        Q: usize,
        queries: [[usize; 3]; Q],
    }
    let mut number_list = vec![vec![]; 2 * 10_0001];
    for i in 0..N {
        number_list[A[i]].push(i);
    }
    for iter in queries.iter() {
        let left = iter[0] - 1;
        let right = iter[1] - 1;
        let target = iter[2];
        
        let left_ind = match number_list[target].binary_search(&left) {
            Ok(x) => x+1,
            Err(x) => x+1,
        };
        let right_ind = match number_list[target].binary_search(&right) {
            Ok(x) => x+2,
            Err(x) => x+1,
        };
        println!("{}", right_ind - left_ind);
    }
}   