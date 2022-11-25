// https://atcoder.jp/contests/joi2010yo/tasks/joi2010yo_d
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    process::exit,
};
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 10_000_000;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        num_list: [usize; N]
    }
    let mut hash = HashMap::new();
    for nums in num_list.iter().permutations(K) {
        let mut sum_num_type_string = nums
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        hash.entry(sum_num_type_string).or_insert(1);
    }
    println!("{}", hash.len());
}
