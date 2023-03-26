#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        R: usize,
        C: usize,
    }
    let mut ans = 0;
    if R > 1 && H > 1 { ans += 1; }
    if R < H { ans += 1; }
    if C > 1 && W > 1 { ans += 1; }
    if C < W { ans += 1; }
    print!("{}",ans);
}