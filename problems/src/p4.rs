//! Problem 4: Largest Palindrome Product
//!
//! A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//! Find the largest palindrome made from the product of two 3-digit numbers.

// time complexity: O(n^2)
use euler::prelude::*;

fn is_palindrome(n: i32) -> bool {
    let (mut original, mut reversed) = (n, 0);

    while original > 0 {
        reversed = reversed * 10 + original % 10; // add last digit to reversed
        original /= 10; // remove last digit from original
    }

    n == reversed
}

fn solve() -> Solution {
    let mut max = 0;

    for i in (100..=999).rev() {
        for j in (100..=i).rev() {
            let product = i * j;
            if product <= max {
                break;
            }
            if is_palindrome(product) {
                max = product;
                break;
            }
        }
    }

    solution!(max)
}

problem!(4, solve);
