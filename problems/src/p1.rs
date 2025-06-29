//* Problem 1: Multiples of 3 or 5
//*
//* If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
//* Find the sum of all the multiples of 3 or 5 below 1000.

//! time complexity: O(1)
use euler::prelude::*;

fn sum_multiples(n: u32, limit: u32) -> u32 {
    let count = (limit - 1) / n;
    let sum = count * (count + 1) / 2;
    n * sum
}

fn solve() -> Solution {
    // inclusion exclusion principle
    // https://en.wikipedia.org/wiki/Inclusion%E2%80%93exclusion_principle
    solution!(sum_multiples(3, 1000) + sum_multiples(5, 1000) - sum_multiples(15, 1000))
}

problem!(
    1,
    solve,
    "c0b20f4665d0388d564f0b6ecf3edc9f9480cb15fff87198b95701d9f5fe1f7b"
);
