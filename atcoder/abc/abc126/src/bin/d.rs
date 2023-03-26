use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};

fn dfs(colors: &mut Vec<isize>, p: usize, c: usize, graph: &Vec<Vec<(usize, usize)>>) {
    colors[p] = c as isize;
    for (v, w) in graph[p].iter() {
        if colors[*v] != -1 {continue;}
        let next_color = if w % 2 == 0 { c } else { (c+1) % 2 };
        dfs(colors, *v, next_color, graph);
    }
}

fn main() {
    input!{
        N: usize,
        UVW: [(Usize1, Usize1, usize); N-1],
    }
    let mut granph = vec![Vec::new(); N];
    for (u,v,w) in UVW.iter() {
        granph[*u].push((*v,*w));
        granph[*v].push((*u,*w));
    }
    let mut ans = vec![-1; N];
    dfs(&mut ans, 0, 0, &granph);
    for a in ans.into_iter() {
        println!("{}", a);
    }
}