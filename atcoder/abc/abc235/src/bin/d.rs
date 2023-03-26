use std::{collections::BTreeSet, sync::mpsc::channel, fmt::format};
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};

fn exchage_num(num: usize) -> usize {
    let mut copy_num = num;
    let mut num_list = Vec::new();
    while copy_num > 0 {
        let t = copy_num % 10;
        num_list.push(t);
        copy_num -= t;
        copy_num /= 10;
    }
    let mut res = 0;
    for i in 1..num_list.len() {
        res += num_list[i] * (10usize.pow((i-1) as u32));
    }
    res += num_list[0] * 10usize.pow((num_list.len() - 1) as u32);
    res
}

fn main() {
    input!{
        a: usize,
        N: usize,
    }
    let limit = 1000_000;
    let mut chage_count = vec![-1; limit];
    chage_count[1] = 0;
    let mut deque = VecDeque::new();
    deque.push_back(1);
    while deque.len() > 0 {
        let num = deque.pop_front().unwrap();
        let format_num = num * a;
        if format_num < limit && chage_count[format_num] == -1 {
            chage_count[format_num] = chage_count[num] + 1;
            deque.push_back(format_num);
        }

        if num >= 10 && num % 10 != 0 {
            let ex_num = exchage_num(num);
            if ex_num < limit && chage_count[ex_num] == -1 {
                chage_count[ex_num] = chage_count[num] + 1;
                deque.push_back(ex_num);
            }
        }
    }
    println!("{}", chage_count[N]);
}