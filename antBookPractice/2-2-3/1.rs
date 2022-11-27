// https://atcoder.jp/contests/abc076/tasks/abc076_c

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
        S: Chars,
        T: Chars
    }
    for s_i in (0..(S.len() - T.len() + 1)).rev() {
        if S[s_i] == '?' || S[s_i] == T[0] {
            let mut is_ok = true;
            for add_ind in 1..T.len() {
                is_ok &= (S[s_i + add_ind] == T[add_ind]) || (S[s_i + add_ind] == '?');
            }
            if is_ok {
                let mut ans = S.clone();
                for i in 0..T.len() {
                    ans[s_i + i] = T[i];
                }
                for j in 0..S.len() {
                    if ans[j] == '?' {
                        ans[j] = 'a'
                    };
                }

                println!("{}", ans.iter().collect::<String>());
                exit(0);
            }
        }
    }
    println!("UNRESTORABLE ")
}
