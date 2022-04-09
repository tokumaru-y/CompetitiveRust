use std::{collections::BinaryHeap, cmp::Reverse};

use proconio::{input, marker::Chars};
fn main() {
    input!{
        N: usize,
        mut sx: usize,
        mut sy: usize,
        mut ex: usize,
        mut ey: usize,
        Grid: [Chars; N],
    }
    let mut dist = vec![vec![100_000_000_usize; N]; N];
    sx -= 1;sy -= 1; ex -=1; ey -=1;
    dist[sx][sy] = 0;
    let move_direction: [[isize; 2]; 4] = [[-1,-1],[1,1],[1,-1],[-1,1]];
    let mut heap = BinaryHeap::new();
    heap.push((0 as isize, sx, sy));
    while heap.len() > 0 {
        let tmp_v = heap.pop().unwrap();
        let now_times = (tmp_v.0 * -1) as usize;let nx = tmp_v.1;let ny = tmp_v.2;
        for mv in &move_direction {
            let mx = &mv[0];let my = &mv[1];
            if mx + (nx as isize) < 0 || my + (ny as isize) < 0 {continue;};
            let mut x = (mx + nx as isize) as usize;
            let mut y = (my + ny as isize) as usize;
            while x <= N-1 && y <= N-1 {
                if Grid[x][y] == '#'{ break;};
                if 1 + now_times >= dist[x][y] {
                    x = (x as isize + mx ) as usize;
                    y = (y as isize + my ) as usize;
                    continue;
                };
                dist[x][y] = 1 + now_times;
                let t = 1 + now_times;
                heap.push((-1 * t as isize, x, y));
                if x as isize + mx < 0 || y as isize + my < 0 {
                    break;
                } else {
                    x = (x as isize + mx ) as usize;
                    y = (y as isize + my ) as usize;
                }
            }
        }
    }
    let ans: isize = if dist[ex][ey] == 100_000_000_usize { -1 } else { dist[ex][ey] as isize };
    println!("{}", ans);
}