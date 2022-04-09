use std::collections::HashMap;

use proconio::{input, marker::Chars};
fn check(h: HashMap<isize, usize>) -> isize {
    for map in h.into_iter() {
        if map.1 == 1 {
            return map.0;
        }
    }
    return -1;
}

fn main() {
    input! {
        XY: [[isize; 2]; 3],
    }
    let mut x_map: HashMap<isize, usize> = HashMap::new();
    let mut y_map: HashMap<isize, usize> = HashMap::new();
    for xy in XY {
        let cnt_x = x_map.entry(xy[0]).or_insert(0);
        let cnt_y = y_map.entry(xy[1]).or_insert(0);
        *cnt_x += 1;
        *cnt_y += 1;
    }
    let ans_x = check(x_map);
    let ans_y = check(y_map);
    println!("{} {}", ans_x, ans_y);
    
}