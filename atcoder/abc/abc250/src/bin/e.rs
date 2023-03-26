use std::collections::HashSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
        Q: usize,
    }
    let mut map = HashMap::new();
    let mut a_kinds_cnt = vec![0; N];
    let mut b_kinds_cnt = vec![0; N];
    let mut b_max_list = vec![0; N+1];

    let mut kind_cnt = 1;
    for i in 0..N {
        if !map.contains_key(&A[i]) {
            map.insert(A[i], kind_cnt);
            kind_cnt += 1;
        }
        a_kinds_cnt[i] = map.len();
    }

    let mut tmp_set = HashSet::new();
    let mut invalid_num = 1000_000_000_000;
    for i in 0..N {
        tmp_set.insert(B[i]);
        let a_num = if map.contains_key(&B[i]) { map[&B[i]] } else { invalid_num };
        b_max_list[i+1] = max(a_num, b_max_list[i]);
        b_kinds_cnt[i] = tmp_set.len();
    }

    for _ in 0..Q {
        input! {
            X: Usize1, Y: Usize1,
        }
        if a_kinds_cnt[X] == b_kinds_cnt[Y] && b_kinds_cnt[Y] == b_max_list[Y+1] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}