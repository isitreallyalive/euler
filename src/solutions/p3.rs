// Problem 3: Largest Prime Factor
//
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143?

use euler::prelude::*;

pub const LOOPS: u8 = 100;
pub struct Problem;
const NUMBER: u64 = 600851475143;

impl Execute for Problem {
    fn execute(&self) -> Result<Return> {
        let mut primes = Vec::new();
        let mut num = NUMBER;
        let mut i = 3;

        while num != 1 {
            if num % i == 0 {
                num /= i;
                primes.push(i);
            } else {
                // next odd number
                i += 2;
            }
        }

        Ok(Return::u64(primes[primes.len() - 1]))
    }
}
