use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
use std::ops::Bound::Included;
use proconio::{input, marker::Chars};


fn dfs(grid: &Vec<Vec<char>>, passed: &mut Vec<Vec<bool>>,i:isize, j:isize) {
    passed[i as usize][j as usize] = true;
    let dh = [0,1,0,-1];
    let dw = [1,0,-1,0];
    for d in dh.iter().zip(dw.iter()) {
        if i + d.0 < 0 || i + d.0 >= 10 || j + d.1 < 0 || j + d.1 >= 10 {continue;}
        let nh = (i+d.0) as usize;let nw = (j + d.1) as usize;
        if passed[nh][nw] || grid[nh][nw] == 'x' { continue; }
        dfs(grid, passed, nh as isize, nw as isize);
    }
}

fn main() {
    input!{
        mut grid: [Chars; 10],
    }
    for i in 0..10 {
        for j in 0..10 {
            let mut passed = vec![vec![false; 10]; 10];
            passed[i][j] = true;
            dfs(&grid, &mut passed,i as isize,j as isize);
            let mut is_ok = true;
            for h in 0..10 {
                for w in 0..10 {
                    if passed[h][w] == false && grid[h][w] == 'o' { is_ok = false; }
                }
            }
            if is_ok {
                println!("YES");
                std::process::exit(0);
            }
        }
    }
    println!("NO");
}