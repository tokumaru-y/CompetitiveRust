#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;

fn make_left(A: &Vec<isize>, N: usize) -> Vec<isize> {
    let mut res = vec![0; 2*N+1];
    let mut heap = BinaryHeap::new();
    for i in 0..2*N {
        if i < N {
            res[i+1] = res[i] + A[i];
            heap.push(-A[i]);
        } else {
            let n = -*heap.peek().unwrap();
            if n < A[i] {
                heap.pop();
                res[i+1] = res[i] - n + A[i];
                heap.push(-A[i]);
            } else {
                res[i+1] = res[i];
            }
        }
    }
    res
}

fn make_right(A: &Vec<isize>, N: usize) -> Vec<isize> {
    let mut res = vec![0; 2*N+1];
    let mut heap = BinaryHeap::new();
    for i in (N..3*N).rev() {
        let res_index = 3*N - i - 1;
        if i >= 2*N {
            res[res_index+1] = res[res_index] + A[i];
            heap.push(A[i]);
        } else {
            let n = *heap.peek().unwrap();
            if n > A[i] {
                heap.pop();
                res[res_index+1] = res[res_index] - n + A[i];
                heap.push(A[i]);
            } else {
                res[res_index+1] = res[res_index];
            }
        }
    }
    res.reverse();
    res
}

fn main() {
    input! {
        N: usize,
        A: [isize; N*3],
    }
    let mut left = make_left(&A,N);
    let mut right = make_right(&A,N);
    let mut ans = -1000_000_000_000_000_000;
    for i in N..=2*N {
        ans = max(ans, left[i] - right[i - N])
    }
    println!("{}",ans);
}