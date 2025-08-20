//* Problem 9: Special Pythagorean Triplet
//*
//* A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//* a² + b² = c².
//* For example, 3² + 4² = 9 + 16 = 25 = 5².
//* there exists exactly one Pythagorean triplet for which a + b + c = 1000. Find the product abc.

//! time complexity: O(n)
//! where n is the target sum
// see the book for implementation details
// https://euler.newty.dev/problems/9.html#notes
use euler::prelude::*;

const N: i32 = 1000;

fn triple(a: i32, n: i32) -> (i32, i32) {
    let b = n * (2 * a - n) / 2 / (a - n);
    let c = n - a - b;
    (b, c)
}

fn solve() -> Solution {
    // a must be the smallest side, so we can restrict the search
    for a in 1..=(N / 3) {
        let (b, c) = triple(a, N);

        if a.pow(2) + b.pow(2) == c.pow(2) {
            return solution!(a * b * c);
        }
    }

    error!("No pythagorean triplet found")
}

problem!(
    9,
    solve,
    "d912d9d473ef86f12da1fb2011c5c0c155bd3a0ebdb4bbd7ea275cecdcb63731"
);
