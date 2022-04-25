use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
use std::ops::Bound::Included;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut Q: usize,
    }
    let mut upper_tree = BTreeMap::new();
    while Q > 0 {
        Q -= 1;
        input! {
            q: usize,
        }
        if q == 1 {
            input!{
                target_num : usize,
            }
            *upper_tree.entry(target_num).or_insert(0) += 1;
        } else {
            input! {
                target_num: usize,
                mut cnt: usize,
            }
            if q == 2 {
                let range = upper_tree.range((Included(1usize), Included(target_num))).rev();
                let mut target = 0;
                for (key, value) in range {
                    if cnt <= 0 || cnt <= *value {
                        println!("{}", *key);
                        target = *key;
                        cnt = 0;
                        break;
                    }
                    cnt -= value;
                }
                if cnt > 0 {
                    println!("-1");
                }
            } else {
                let range = upper_tree.range((Included(target_num), Included(1000_000_000_000_000_010usize)));
                let mut target = 0;
                for (key, value) in range {
                    if cnt <= 0 || cnt <= *value {
                        println!("{}", *key);
                        target = *key;
                        cnt = 0;
                        break;
                    }
                    cnt -= value;
                }
                if cnt > 0 {
                    println!("-1");
                }
            }
        }
    }
}