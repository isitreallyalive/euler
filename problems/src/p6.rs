//* Problem 6: Sum Square Difference
//*
//* the sum of the squares of the first ten natural numbers is,
//* 1² + 2² + ... + 10² = 385.
//* the square of the sum of the first ten natural numbers is,
//* (1 + 2 + ... + 10)² = 55² = 3025.
//* Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.
//* Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

//! time complexity: O(1)
use euler::prelude::*;

const N: u64 = 100;

fn solve() -> Solution {
    // see the derivation in: notes/p6.rs
    solution!(N * (N + 1) * (3 * N + 2) * (N - 1) / 12)
}

problem!(6, solve);
