use std::{collections::{BinaryHeap, HashMap, VecDeque}, cmp::Reverse};

use proconio::{input, marker::Chars};

fn calc(list: Vec<usize>, X: usize, Y: usize) -> usize {
    let mut i = 0;
    let mut j = 0;
    let mut count_x = 0;
    let mut count_y = 0;
    let mut res = 0;
    while i != list.len() {
        while j != list.len() && (count_x == 0 || count_y == 0){
            if list[j] == X { count_x += 1 }
            if list[j] == Y { count_y += 1 }
            j+=1;
        }
        if count_x > 0 && count_y > 0{
            res += list.len() + 1 - j;
        }
        if list[i] == X { count_x -= 1 }
        if list[i] == Y { count_y -= 1 }
        i+=1;
    }
    return res;
}

fn main() {
    input!{
        N: usize,
        X: usize,
        Y: usize,
        A: [usize; N],
    }
    let mut ans = 0;
    let mut index = 0;
    while index < N  {
        let mut list = Vec::new();
        while index < N && (A[index] >= Y) && (A[index] <= X) {
            list.push(A[index]);
            index += 1;
        }
        if list.len() > 0 {
            let x = X;
            let y = Y;
            ans += calc(list,x,y);
        }
        index += 1;
    }
    println!("{}", ans);
}