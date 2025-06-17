//* Problem 5: Smallest Multiple
//*
//* 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//* What is the smallest positive number that is evenly divisble by all of the numbers from 1 to 20?

//! time complexity: O(n log L)
//! where n is the range size and L is the maximum LCM value
use euler::prelude::*;

// https://en.wikipedia.org/wiki/Euclidean_algorithm
fn lcm(mut a: u64, mut b: u64) -> u64 {
    let c = a * b;
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    c / a
}

fn solve() -> Solution {
    solution!((2..=20).fold(1, |acc, n| lcm(acc, n)))
}

problem!(5, solve);
