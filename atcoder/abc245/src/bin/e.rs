use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};

use proconio::{input, marker::Chars};

enum ValueType {
    Boxes(usize),
    Chocolates(usize),
}

fn main() {
    input!{
        N: usize,
        M: usize,
        A: [isize; N],
        B: [isize; N],
        C: [isize; M],
        D: [isize; M],
    }
    let mut options = Vec::new();
    for i in 0..N {
        options.push((-A[i], ValueType::Chocolates(2), B[i]));
    }
    for i in 0..M{
        options.push((-C[i], ValueType::Boxes(1), D[i]));
    }
    options.sort();
    let mut b_tree = BTreeMap::new();
    for i in 0..options.len() {
        let tuple = options[i];
        if tuple.1 == 1 {
            *b_tree.entry(tuple.2).or_insert(0) += 1;
        } else if tuple.1 == 2 {
            let first = b_tree.range(tuple.2..).nth(0);
            if first.is_none() {
                println!("No");
                std::process::exit(0);
            }
            let (&k, &v) = first.unwrap();
            if v > 1 {
                b_tree.insert(k, v-1);
            } else {
                b_tree.remove(&k);
            }
        }
    }
    println!("Yes");
}