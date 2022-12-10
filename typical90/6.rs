// https://atcoder.jp/contests/typical90/tasks/typical90_f
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
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        S: Chars,
    }
    let mut ans: Vec<char> = Vec::new();
    let mut count_table: HashMap<char, VecDeque<usize>> = HashMap::new();

    for (i, c) in S.iter().enumerate() {
        if !count_table.contains_key(c) {
            count_table.insert(*c, VecDeque::new());
        }
        count_table.get_mut(c).unwrap().push_back(i);
    }

    let alpha = (b'a'..=b'z').map(|c| c as char).collect::<Vec<_>>();
    let mut allow_index_first = 0;
    let mut allow_index_last = N - K;

    for i in (0..K) {
        for a in alpha.iter() {
            if !count_table.contains_key(a) {
                continue;
            }

            if count_table[a].front() != None {
                while count_table[a].front() != None
                    && *count_table[a].front().unwrap() < allow_index_first
                {
                    count_table.get_mut(a).unwrap().pop_front();
                }
            }

            if count_table[a].front() != None
                && *count_table[a].front().unwrap() <= allow_index_last
                && *count_table[a].front().unwrap() >= allow_index_first
            {
                ans.push(*a);
                allow_index_first = count_table.get_mut(a).unwrap().pop_front().unwrap() + 1;
                allow_index_last += 1;
                break;
            }
        }
    }

    println!("{}", ans.iter().join(""));
}
