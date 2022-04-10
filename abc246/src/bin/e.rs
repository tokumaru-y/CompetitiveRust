use std::collections::VecDeque;

use proconio::{input, marker::Chars};
struct P {
    x: usize,
    y: usize,
    dir: usize,
}
fn main() {
    input!{
        N: usize,
        mut sx: usize,
        mut sy: usize,
        mut ex: usize,
        mut ey: usize,
        Grid: [Chars; N],
    }
    let mut passed = vec![vec![vec![100_000_000_usize; 4];N];N];
    sx-=1;sy-=1;ex-=1;ey-=1;
    for i in 0..4{passed[sx][sy][i]=0;}
    let move_directions = vec![[-1,-1],[-1,1],[1,1],[1,-1]];
    let mut deque = VecDeque::new();
    for i in 0..4{
        if (sx as isize + move_directions[i][0]) < 0 || (sy as isize + move_directions[i][1]) < 0 {continue;};
        let x = (sx as isize + move_directions[i][0]) as usize;
        let y = (sy as isize + move_directions[i][1]) as usize;
        if x >= N || y >= N || Grid[x][y] == '#'{continue;};
        deque.push_front(P{x,y,dir:i});
        passed[x][y][i] = 1;
    }
    while !deque.is_empty() {
        let tmp_p = deque.pop_front().unwrap();
        if tmp_p.x == ex && tmp_p.y == ey {
            println!("{}", passed[tmp_p.x][tmp_p.y][tmp_p.dir]);
            std::process::exit(0);
        }
        for i in 0..4 {
            let mut nx = tmp_p.x as isize + move_directions[i][0];
            let mut ny = tmp_p.y as isize + move_directions[i][1];
            
            if !(0<= nx) || !(0<= ny) {continue;};
            let nx = nx as usize;let ny = ny as usize;
            if !(nx <= N-1) || !(ny <= N-1) {continue;};
            if Grid[nx][ny] == '#' {continue;};

            let mut cnt = passed[tmp_p.x][tmp_p.y][tmp_p.dir];
            if tmp_p.dir != i { cnt += 1};

            if passed[nx][ny][i] > cnt {
                passed[nx][ny][i] = cnt;
                if tmp_p.dir == i {
                    deque.push_front(P{x:nx,y:ny,dir:i});
                } else {
                    deque.push_back(P{x:nx,y:ny,dir:i});
                }
            }

        }
    }
    println!("-1");
}