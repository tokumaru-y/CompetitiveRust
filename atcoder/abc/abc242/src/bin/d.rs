use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};

use proconio::{input, marker::Chars};
fn get_char(t:usize, k:usize, S: &Vec<char>) -> char {
    if t == 0 { return S[k] }
    if k == 0 { return calc_char(S[0], t) }
    calc_char(get_char(t-1, k/2, S), k%2+1)
}

fn calc_char(c: char, ind: usize) -> char {
    let chars = ['A', 'B', 'C'];
    let now_ind = match c {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        _ => 100,
    };
    chars[(now_ind + ind)%3]
}

fn main() {
    input!{
        S: Chars,
        Q: usize,
        TK: [[usize; 2]; Q],
    }
    for i in 0..Q{
        let t = TK[i][0];
        let k = TK[i][1];
        let ans = get_char(t, k-1, &S);
        println!("{}", ans);
    }
}