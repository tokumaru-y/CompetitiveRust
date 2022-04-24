use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};

use proconio::{input, marker::Chars};


fn check_h(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let len = grid.len();
    let mut used_cnt = 0;
    for h in 0..6 {
        if i+h >= len {
            return false;
        }
        if grid[h+i][j] == '.' {
            if used_cnt >= 2 {
                return false;
            }
            used_cnt+=1;
        }
    }
    true
}

fn check_w(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let len = grid.len();
    let mut used_cnt = 0;
    for w in 0..6 {
        if j+w >= len {
            return false;
        }
        if grid[i][j+w] == '.' {
            if used_cnt >= 2 {
                return false;
            }
            used_cnt+=1;
        }
    }
    true
}

fn check_hw(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let len = grid.len();
    let mut used_cnt = 0;
    for k in 0..6 {
        if i+k >= len || j+k >= len {
            return false;
        }
        if grid[i+k][j+k] == '.' {
            if used_cnt >= 2 {
                return false;
            }
            used_cnt+=1;
        }
    }
    true
}

fn check_wh(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let len = grid.len();
    let mut used_cnt = 0;
    for k in 0..6 {
        if i+k >= len || j < k {
            return false;
        }
        if grid[i+k][j-k] == '.' {
            if used_cnt >= 2 {
                return false;
            }
            used_cnt+=1;
        }
    }
    true
}

fn is_acceptable(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    check_h(grid, i, j) || check_w(grid, i, j) || check_hw(grid, i, j) || check_wh(grid, i, j)
}

fn main() {
    input! {
        N: usize,
        Grid: [Chars; N],
    }
    for i in 0..N{
        for j in 0..N {
            if is_acceptable(&Grid, i, j) {
                println!("Yes");
                std::process::exit(0);
            }
        }
    }
    println!("No");
}