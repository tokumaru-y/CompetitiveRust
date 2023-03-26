#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
struct MyRange {
    left: i128,
    right: i128,
}

fn check_conditions(value: i128, first: MyRange, second: MyRange) -> bool {
    let first_value = (first.left <= value && value <= first.right);
    let second_value = (second.left <= value && value <= second.right);
    first_value || second_value
}

fn return_white_grid(h1: i128, h2: i128, w1: i128, w2: i128) {
    for i in h1..=h2 {
        for j in w1..=w2 {
            print!(".");
        }
        println!("");
    }
    std::process::exit(0);
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: i128,
        A: i128,
        B: i128,
        P: i128, Q: i128, R: i128, S: i128,
    }
    let first_conditions = MyRange {
        left: max(1 - A, 1 - B),
        right: min(N - A, N - B),
    };
    let second_conditions = MyRange{
        left: max(1 - A, B - N),
        right: min(N - A, B - 1),
    };
    if (P > A + first_conditions.right && Q < A + first_conditions.left) && (R > B + second_conditions.right && S < B + second_conditions.left) {
        return_white_grid(P, Q, R, S);
    }

    let mut grid = vec![vec!['.'; (S - R + 1) as usize]; (Q - P + 1) as usize];
    // first
    let left = max(first_conditions.left, P - A);
    let right = min(first_conditions.right, Q - A);
    for h in left..=right {
        let nh = A + h;let nw = B + h;
        if P<= nh && nh <= Q && R <= nw && nw <= S {
            let nh = nh - P;let nw = nw - R;
            grid[nh as usize][nw as usize] = '#';
        }
    }
    // second
    let left = max(second_conditions.left, P - A);
    let right = min(second_conditions.right, Q - A);
    for h in left..=right {
        let nh = A + h;let nw = B - h;
        if P<= nh && nh <= Q && R <= nw && nw <= S {
            let nh = nh - P; let nw = nw - R;
            grid[nh as usize][nw as usize] = '#';
        }
    }
    if P <= A && A <= Q && R <= B && B <= R {
        grid[(A - P) as usize][(B - R) as usize] = '#';
    }

    for i in 0..(Q - P + 1){
        for j in 0..(S - R + 1){
            print!("{}", grid[i as usize][j as usize])
        }
        println!("");
    }
}