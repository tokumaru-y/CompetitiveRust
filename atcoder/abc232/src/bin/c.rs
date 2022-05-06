#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M],
        CD: [(Usize1, Usize1); M],
    }
    let mut graph_n = vec![vec![false; N]; N];
    let mut graph_m = vec![vec![false; N]; N];
    for (a,b) in AB.into_iter() {
        graph_n[a][b]=true;graph_n[b][a]=true;
    }
    for (c,d) in CD.into_iter() {
        graph_m[c][d] = true;graph_m[d][c] = true;
    }
    let mut num_list: Vec<usize> = (0..N).collect();
    loop {
        let mut is_ok = true;
        for i in 0..N { 
            for j in i+1..N{
                if graph_n[i][j] && !graph_m[num_list[i]][num_list[j]]{
                    is_ok = false;
                }
            }
        }
        if is_ok {
            println!("Yes");
            std::process::exit(0);
        }
        if !num_list.next_permutation() { break; }
    }
    println!("No");
}

/// https://github.com/bluss/permutohedron/blob/master/src/lexical.rs
/// Permute a slice into its next or previous permutation (in lexical order).
///
/// ```
/// use permutohedron::LexicalPermutation;
///
/// let mut data = [1, 2, 3];
/// let mut permutations = Vec::new();
///
/// loop {
///     permutations.push(data.to_vec());
///     if !data.next_permutation() {
///         break;
///     }
/// }
///
/// assert_eq!(permutations, &[&[1, 2, 3], &[1, 3, 2],
///                            &[2, 1, 3], &[2, 3, 1],
///                            &[3, 1, 2], &[3, 2, 1]]);
///
/// // `data` has been mutated in-place:
/// assert_eq!(data, [3, 2, 1]);
/// ```
pub trait LexicalPermutation {
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the last ordered permutation.
    fn next_permutation(&mut self) -> bool;
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the first ordered permutation.
    fn prev_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T] where T: Ord {
    /// Original author in Rust: Thomas Backman <serenity@exscape.org>
    fn next_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 { return false; }

        // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] >= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the last-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Find the rightmost element larger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i-1]  {
            j -= 1;
        }

        // Step 3: Swap that element with the pivot
        self.swap(j, i-1);

        // Step 4: Reverse the (previously) weakly decreasing part
        self[i..].reverse();

        true
    }

    fn prev_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 { return false; }

        // Step 1: Identify the longest, rightmost weakly increasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] <= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the first-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Reverse the weakly increasing part
        self[i..].reverse();

        // Step 3: Find the rightmost element equal to or bigger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j-1] < self[i-1]  {
            j -= 1;
        }

        // Step 4: Swap that element with the pivot
        self.swap(i-1, j);

        true
    }

}