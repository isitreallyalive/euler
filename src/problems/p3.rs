//! Problem 3: Largest Prime Factor
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//! What is the largest prime factor of the number 600851475143?

// time complexity: O(sqrt n)
// https://en.wikipedia.org/wiki/Trial_division
use crate::prelude::*;

const NUMBER: u64 = 600_851_475_143;

fn solve() -> Result<u64> {
    let mut largest = 1;
    let mut num = NUMBER;

    // get rid of factors of 2
    while num % 2 == 0 {
        largest = 2;
        num /= 2;
    }

    // check odd factors
    let mut i = 3;
    while i * i <= num {
        while num % i == 0 {
            largest = i;
            num /= i
        }
        i += 2;
    }

    // if num > 1, then it is prime
    if num > 1 {
        largest = num;
    }

    Ok(largest)
}

problem!(3, solve);
