use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
use std::ops::Bound::Included;
use proconio::{input, marker::Chars};

fn divisor(n: &usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 1..=(*n as f64).sqrt() as usize {
        if n % i == 0 {
            res.push(i);
            if n/i != i {res.push(n/i);};
        }
    }
    res
}
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut cnt = vec![0usize; 2*10_0001];
    let mut ans: usize = 0;
    for i in A.iter() { cnt[*i] += 1; }
    for i in 0..N {
        let div_list = divisor(&A[i]);
        for div in div_list {
            if A[i] % div == 0 {
                ans += cnt[div] * cnt[A[i] / div];
            }
        }
    }
    println!("{}", ans);
}