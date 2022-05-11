#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, K: Usize1,
        P: [[usize; 3]; N],
    }
    let mut sum_list = Vec::new();
    for i in 0..N {
        let point_sum = P[i].iter().sum::<usize>();
        sum_list.push(point_sum);
    }
    let mut copy_sum = sum_list.to_vec();
    copy_sum.sort_by_key(|x| Reverse(*x));
    for i in 0..N {
        if sum_list[i] + 300 >= copy_sum[K] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}