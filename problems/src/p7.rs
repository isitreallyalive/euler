//! Problem 7: 10 001st Prime
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//! What is the 10,001st prime number?

// time complexity: O(n log log n)
use euler::prelude::*;

const N: usize = 10_001;

fn solve() -> Solution {
    // https://en.wikipedia.org/wiki/Prime_number_theorem
    let limit = {
        let n = N as f32;
        let ln_n = n.ln();
        (n * (ln_n + ln_n.ln() - 1.0 + 1.8 * ln_n.ln() / ln_n)) as usize
    };

    // https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
    // only sieve odd numbers (except 2)
    let mut is_prime = vec![true; (limit + 1) / 2];
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
            if prime_count == N {
                return solution!(i);
            }
        }
    }

    error!("Could not find the Nth prime")
}

problem!(7, solve);
