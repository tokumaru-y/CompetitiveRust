#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    // TODO: 区間での二分探索パターンも実装
    input! {
        Q: usize,
        TX: [(usize, usize); Q],
    }
    let mod_num = 1048576;
    let mut set = BTreeSet::new();
    for i in 0..mod_num { set.insert(i); }
    let mut num_list = vec![-1; mod_num as usize];
    for (t,x) in TX.into_iter() {
        let mut index = x % mod_num as usize;
        if t == 1 {
            let mut b_index = set.range(index..).next();
            if b_index.is_none() {
                b_index = set.range(0..).next();
            }
            let tmp = *b_index.unwrap();
            num_list[tmp] = x as isize;
            set.remove(&tmp);
        } else {
            println!("{}",num_list[index]);
        }
    }
}
