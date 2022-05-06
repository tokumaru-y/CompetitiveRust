#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};

fn dfs(max_h: usize, max_w: usize, h: usize, w: usize, move_cnt: &mut Vec<Vec<i32>>, grid: &Vec<Vec<char>> ){
    let dx = [1,0];let dy = [0,1];
    for (dh, dw) in dx.iter().zip(dy.iter()) {
        let nh = h + *dh;let nw = w + *dw;
        if nh >= max_h || nw >= max_w { continue; }
        if grid[nh][nw] == '#' { continue; }
        if move_cnt[nh][nw] > 0 { continue; }
        move_cnt[nh][nw] = move_cnt[h][w] + 1;
        dfs(max_h, max_w, nh, nw, move_cnt, grid)
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        grid: [Chars; H],
    }
    let mut move_cnt = vec![vec![0; W]; H];
    move_cnt[0][0] = 1;
    dfs(H,W,0, 0, &mut move_cnt, &grid);
    let mut ans = 0;
    for v in move_cnt.iter() {
        for vv in v.iter() {
            ans = max(ans, *vv);
        }
    }
    println!("{}",ans);
}