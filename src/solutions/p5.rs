// Problem 5: Smallest Multiple
//
// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisble by all of the numbers from 1 to 20?

use euler::prelude::*;

pub const LOOPS: u8 = 100;
pub struct Problem;

impl Execute for Problem {
    fn execute(&self) -> Result<Return> {
        fn lcm(mut a: u64, mut b: u64) -> u64 {
            let c = a * b;
            while b != 0 {
                let tmp = b;
                b = a % b;
                a = tmp;
            }
            c / a
        }

        let value = (2..=20).fold(1, |acc, n| lcm(acc, n));

        Ok(Return::u64(value))
    }
}
