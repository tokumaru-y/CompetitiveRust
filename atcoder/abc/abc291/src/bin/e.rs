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
    f32::consts::E,
    fmt::Debug,
    io::{stdout, Write},
    mem::swap,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}

fn read() -> usize {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<usize>().unwrap()
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 1_000_000_007;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut XY: [(Usize1, Usize1); M]
    }
    let mut edge_cnt = vec![0; N];
    let mut graph = vec![vec![]; N];
    let mut dag = vec![];
    XY.sort();
    XY.dedup();

    for (x, y) in XY.iter() {
        graph[*y].push(*x);
        edge_cnt[*x] += 1;
    }
    let mut in_list = VecDeque::new();
    for i in 0..N {
        if edge_cnt[i] == 0 {
            in_list.push_back(i);
            dag.push(i);
        }
    }
    while in_list.len() > 0 {
        if in_list.len() >= 2 {
            println!("No");
            exit(0);
        }
        let x = in_list.pop_front().unwrap();
        for nx in graph[x].iter() {
            edge_cnt[*nx] -= 1;
            if edge_cnt[*nx] == 0 {
                in_list.push_back(*nx);
                dag.push(*nx);
            }
        }
    }
    if dag.len() != N {
        println!("No");
        exit(0);
    }

    let mut ans = vec![0; N];
    let mut num = N;
    for i in 0..N {
        ans[dag[i]] = num;
        num -= 1;
    }
    println!("Yes");
    println!("{}", ans.into_iter().join(" "));
}
