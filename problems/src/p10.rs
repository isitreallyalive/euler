//* Problem 10: Summation of Primes
//* the sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//* Find the sum of all the primes below two million.

//! time complexity: O(n log log n)
//! where n is the limit for the sieve
use euler::prelude::*;

const N: usize = 2_000_000;

fn solve() -> Solution {
    // sieve of eratosthenes
    // see: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
    let mut a = vec![true; N];
    a[0] = false; // 0 is not prime
    a[1] = false; // 1 is not prime

    // sieve
    let limit = (N as f64).sqrt() as usize;
    for i in 2..=limit {
        if a[i] {
            let mut j = i * i;
            while j < N {
                a[j] = false;
                j += i;
            }
        }
    }

    // sum all primes
    let sum: usize = (0..N).filter(|&i| a[i]).sum();

    solution!(sum)
}

problem!(
    10,
    solve,
    "bed2d160e02f0540f19a64ca738aacb79cfcd08ba7e2421567b16cb6e7e3e90e",
    10
);
