//* Problem 7: 10 001st Prime
//*
//* By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//* What is the 10,001st prime number?

//! time complexity: O(n log log n)
//! where n is the limit for the sieve
use euler::prelude::*;

const N: f32 = 10_001.;

/// Uses the Prime Number Theorem (PMT)
/// See: https://en.wikipedia.org/wiki/Prime_number_theorem
fn limit() -> f32 {
    let ln = N.ln();
    N * (ln + ln.ln() - 1.0 + 1.8 * ln.ln() / ln)
}

fn solve() -> Solution {
    let limit = limit() as usize;

    // see: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
    // only sieve odd numbers (except 2)
    let mut is_prime = vec![true; limit.div_ceil(2)];
    let mut prime_count = 1;
    let mut i = 3;
    while i * i <= limit {
        if is_prime[i / 2] {
            // mark odd multiples of i starting from i²
            let mut j = i * i;
            while j <= limit {
                is_prime[j / 2] = false;
                j += 2 * i; // skip even multiples
            }
        }
        i += 2;
    }

    // count primes until we reach the nth
    for i in (3..=limit).step_by(2) {
        if is_prime[i / 2] {
            prime_count += 1;
            if prime_count == N as i32 {
                return solution!(i);
            }
        }
    }

    error!("Could not find the Nth prime")
}

problem!(
    7,
    solve,
    "ecbe74e25cfa4763dbc304ccac2ffb9912e9625cd9993a84bd0dd6d7dc0ca021"
);
