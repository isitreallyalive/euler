// Problem 4: Largest Palindrome Product
//
// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

use euler::prelude::*;

pub const LOOPS: u8 = 100;
pub struct Problem;

impl Execute for Problem {
    fn execute(&self) -> Result<Return> {
        fn is_palindrome(n: i32) -> bool {
            let s = n.to_string();
            let bytes = s.as_bytes();
            let len = bytes.len();
            for i in 0..len / 2 {
                if bytes[i] != bytes[len - 1 - i] {
                    return false;
                }
            }
            true
        }

        let mut max_palindrome = 0;

        for i in (100..=999).rev() {
            for j in (100..=i).rev() {
                let product = i * j;
                if product <= max_palindrome {
                    break;
                }
                if is_palindrome(product) {
                    max_palindrome = product;
                }
            }
        }

        Ok(Return::i32(max_palindrome))
    }
}
