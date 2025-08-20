//* Problem 14: Longest Collatz Sequence
//* The following iterative sequence is defined for the set of positive integers:
//* 
//* n → n/2 (n is even)
//* n → 3n + 1 (n is odd)
//* Using the rule above and starting with 13, we generate the following sequence:
//* 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1.
//* It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
//* Which starting number, under one million, produces the longest chain?
//* NOTE: Once the chain starts the terms are allowed to go above one million.

//! time complexity: O(MAX)
use euler::prelude::*;

const MAX: u64 = 1_000_000;

fn solve() -> Solution {
    // memoise lengths of chains
    let mut lengths = vec![0u32; MAX as usize + 1];
    lengths[1] = 1;
    // track the maximum length
    let mut max_start = 1u64;
    let mut max_length = 1u32;

    for start in 2..MAX {
        // build the chain
        let mut n = start;
        let mut chain = Vec::new();
        while n >= lengths.len() as u64 || lengths[n as usize] == 0 {
            chain.push(n);
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        // figure out the length of the chain
        let mut length = if n < lengths.len() as u64 {
            lengths[n as usize]
        } else {
            // for n > MAX, we never memoize, just use the computed length
            // because these values will be far and few, and it's not worth
            // allocating extra memory
            let mut temp_n = n;
            let mut temp_length = 0u32;
            while temp_n >= lengths.len() as u64 || lengths[temp_n as usize] == 0 {
                temp_n = if temp_n % 2 == 0 { temp_n / 2 } else { 3 * temp_n + 1 };
                temp_length += 1;
            }
            temp_length + lengths[temp_n as usize]
        };
        for &k in chain.iter().rev() {
            length += 1;
            if k < lengths.len() as u64 {
                lengths[k as usize] = length;
            }
        }
        // store any new max
        if length > max_length {
            max_length = length;
            max_start = start;
        }
    }
    solution!(max_start)
}

problem!(14, solve, "78a262dd40eba0f7195686ec7f3891a39437523456f8d16fa9065a34409eeac6");
