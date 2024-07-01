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
use std::{
    fmt::Debug,
    io::{stdout, Write},
    mem::swap,
};
fn main() {
    input! {
        N: usize,
    }
    if N % 2 == 1 {
        println!("");
        return;
    }
    let mut ans = vec![];
    let mut phrase = VecDeque::new();
    dfs(&mut ans, &mut phrase, N);

    ans.sort();
    ans.dedup();
    for a in ans.into_iter() {
        println!("{}", a);
    }
}

fn dfs(ans: &mut Vec<String>, phrase: &mut VecDeque<char>, N: usize) {
    if phrase.len() == N {
        ans.push(phrase.clone().into_iter().collect());
        return;
    }

    let mut same_vec = phrase.clone().into_iter().collect::<VecDeque<char>>();
    let len = same_vec.len();
    if len * 2 <= N && len > 0 {
        phrase.append(&mut same_vec);
        dfs(ans, phrase, N);
        for _ in 0..len {
            phrase.pop_back();
        }
    }

    phrase.push_back('(');
    phrase.push_back(')');
    dfs(ans, phrase, N);
    phrase.pop_back();
    phrase.pop_back();

    phrase.push_front(')');
    phrase.push_front('(');
    dfs(ans, phrase, N);
    phrase.pop_front();
    phrase.pop_front();

    phrase.push_front('(');
    phrase.push_back(')');
    dfs(ans, phrase, N);
    phrase.pop_back();
    phrase.pop_front();
}
