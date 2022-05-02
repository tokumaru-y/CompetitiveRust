#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut K: usize,
        AB: [[usize; 2]; N],
    }
    let mut binary_heap = BinaryHeap::new();
    for i in 0..N {
        let time = AB[i][0] as isize;
        binary_heap.push((-time, i));
    }
    let mut ans = 0;
    while K > 0 {
        let (time, index) = binary_heap.pop().unwrap();
        ans += -time;
        binary_heap.push((time - (AB[index][1] as isize), index));
        K-=1;
    }
    println!("{}",ans);
}