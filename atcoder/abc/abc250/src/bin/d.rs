use std::collections::HashSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]

fn erathotenes() -> Vec<usize> {
    let total_num = 100_0001;
    let limit = ((total_num as f64).sqrt()) as usize;
    let mut res = Vec::new();

    let mut num_table = vec![true; total_num];
    num_table[0] = false; num_table[1] = false;
    for n in 2..=limit {
        if !num_table[n] { continue; }
        let mut tmp_n = n * 2;
        while tmp_n < total_num {
            num_table[tmp_n] = false;
            tmp_n += n;
        }
    }

    for i in 0..total_num {
        if num_table[i] {
            res.push(i);
        }
    }
    res
}
fn main() {
    input! {
        N: usize,
    }
    let prime_list = erathotenes();
    let mut ans = 0;
    for i in 1..prime_list.len() {
        let q = prime_list[i];
        let q = q * q * q;
        if q * prime_list[0] > N { continue; }

        let search = N / q;
        let mut p_index = match prime_list.binary_search(&search) {
            Ok(x) => x,
            Err(x) => x-1,
        };
        p_index = min(i - 1, p_index);
        ans += p_index + 1;
    }
    println!("{}",ans);
}