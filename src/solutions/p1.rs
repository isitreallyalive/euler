// Problem 1: Multiples of 3 or 5
//
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

use euler::prelude::*;

pub const LOOPS: u8 = 100;
pub struct Problem;

impl Execute for Problem {
    fn execute(&self) -> Result<Return> {
        fn sum_multiples(n: u32, limit: u32) -> u32 {
            let count = (limit - 1) / n;
            let sum = count * (count + 1) / 2;
            n * sum
        }

        let value = sum_multiples(3, 1000) + sum_multiples(5, 1000) - sum_multiples(15, 1000);

        Ok(Return::u32(value))
    }
}
