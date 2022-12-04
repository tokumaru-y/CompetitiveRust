// https://atcoder.jp/contests/abc091/tasks/abc091_b
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
        S: [String; N],
        M: usize,
        T: [String; M]
    }
    let mut map: HashMap<String, i32> = HashMap::new();
    for s in S.into_iter() {
        *map.entry(s).or_insert(0) += 1;
    }
    for t in T.into_iter() {
        *map.entry(t).or_default() -= 1;
    }

    let mut ans = 0;
    for v in map.values() {
        ans = max(ans, *v);
    }
    println!("{}", ans);
}
