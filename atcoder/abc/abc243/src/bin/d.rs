use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        mut X: usize,
        S: Chars,
    }
    let mut s = VecDeque::new();
    let mut skip_cnt = 0;
    for i in (0..N).rev() {
        match S[i] {
            'U' => {
                skip_cnt += 1;
            }
            _ => {
                if skip_cnt > 0 {
                    skip_cnt -=1;
                } else {
                    s.push_front(S[i]);
                }
            }
        }
    }
    while skip_cnt > 0 {
        X /= 2;
        skip_cnt -=1;
    }
    for c in s.iter() {
        if *c == 'L' {
            X *= 2;
        } else {
            X *= 2;
            X += 1;
        }
    }
    println!("{}", X);
}