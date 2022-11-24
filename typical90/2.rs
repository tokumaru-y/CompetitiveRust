// https://atcoder.jp/contests/typical90/tasks/typical90_b
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
};

fn dfs(mut left: usize, mut right: usize, ans_vct: &mut Vec<String>, str_b: String) {
    if left == 0 && right == 0 {
        ans_vct.push(str_b.clone());
    } else {
        let brancket = ["(", ")"];
        for b in brancket.into_iter() {
            if *b == "(" {
                if left == 0 {
                    continue;
                }
                left -= 1;
                right += 1;
                let s = str_b.clone() + b;
                dfs(left, right, ans_vct, s);
                left += 1;
                right -= 1;
            } else {
                if right == 0 {
                    continue;
                }
                right -= 1;
                let s = str_b.clone() + b;
                dfs(left, right, ans_vct, s);
                right += 1;
            }
        }
    }
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize
    }
    if N % 2 == 1 {
        std::process::exit(0)
    }
    let mut ans = Vec::new();
    let mut left_branckets = N / 2;
    let mut right_branckets = 0;
    dfs(left_branckets, right_branckets, &mut ans, String::from(""));
    for v in ans.into_iter() {
        println!("{}", v);
    }
}
