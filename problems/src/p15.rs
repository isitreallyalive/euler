//* Problem 15: Lattice Paths
//* Starting in the top left corner of a 2 × 2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
//* How many such routes are there through a 20 × 20 grid?

//! time complexity: O(N)
// see the book for implementation details
// https://euler.newty.dev/problems/15.html#notes
use euler::prelude::*;

const N: usize = 20;

fn solve() -> Solution {
    let mut result = 1usize;
    for i in 1..=N {
        // can't do result *= (N + i) / i due to loss of precision
        result = result * (N + i) / i;
    }
    solution!(result)
}

problem!(
    15,
    solve,
    "7b8f812ca89e311e1b16b903de76fa7b0800a939b3028d9dc4d35f6fa4050281"
);
