//! Problem 10: Summation of Primes
//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//! Find the sum of all the primes below two million.

// todo: time complexity: O(n log log n)
use euler::prelude::*;

/// Sieve of Eratosthenes.
/// See: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
fn sieve_sum(n: usize) -> usize {
    let mut a = vec![true; n];
    a[0] = false; // 0 is not prime
    a[1] = false; // 1 is not prime

    // sieve
    let limit = (n as f64).sqrt() as usize;
    for i in 2..=limit {
        if a[i] {
            let mut j = i * i;
            while j < n {
                a[j] = false;
                j += i;
            }
        }
    }

    // sum all primes
    (0..n).filter(|&i| a[i]).sum()
}

fn solve() -> Solution {
    solution!(sieve_sum(2_000_000))
}

problem!(10, 10, solve);
