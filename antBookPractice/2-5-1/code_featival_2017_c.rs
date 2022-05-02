use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};

fn dfs(colors: &mut Vec<isize>, n: usize, c: usize, graph: &Vec<Vec<usize>>) -> bool {
    let mut res = true;
    colors[n] = c as isize;
    for v in graph[n].iter() {
        if n == *v { continue; }
        if colors[*v] == colors[n] { return false; }
        if colors[*v] != -1 { continue; }
        let n_color = (c+1) % 2;
        res &= dfs(colors, *v, n_color, graph);
    }
    res
}

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M]
    }
    let mut graph = vec![Vec::new(); N];
    let mut colors = vec![-1; N];
    for (a,b) in AB.iter() {
        graph[*a].push(*b);graph[*b].push(*a);
    }
    let is_bin = dfs(&mut colors, 0, 0, &graph);
    let white = colors.iter().filter(|x| **x == 0).count();
    let black = colors.iter().filter(|x| **x == 1).count();
    let ans = if is_bin {
        white*black - M
    } else {
        N*(N-1)/2 - M
    };
    println!("{}",ans);
}