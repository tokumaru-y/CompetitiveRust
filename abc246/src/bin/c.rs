use proconio::{input, marker::Chars};
use std::collections::BinaryHeap;
fn main() {
    input! {
        N: usize,
        mut K: usize,
        X: usize,
        A: [usize; N],
    }
    let mut heap = BinaryHeap::from(A);
    while K > 0 {
        let mut max_num = if let Some(x) = heap.pop() {
            x
        } else {
            break;
        };
        if max_num < X {
            K -= 1;
            max_num = 0;
        } else {
            let cnt = if (max_num / X) <= K { max_num / X } else { K };
            K -= cnt;
            max_num -= X * cnt;
        }
        if max_num > 0 {
            heap.push(max_num);
        }
    }
    let ans: usize = heap.into_iter().sum();
    println!("{}", ans);
}