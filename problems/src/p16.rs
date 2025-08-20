//* Problem 16: Power Digit Sum
//* 2^{15} = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//* What is the sum of the digits of the number 2^{1000}?

//! time complexity: O(nd)
//! where n is the exponent and d is the number of digits in the result
//! n = 1000, d = log_10(2^1000)
use euler::prelude::*;

fn solve() -> Solution {
    // represent the number as a vector of digits, lsd first
    // log_10(2^1000) = 1000 * log_10(2) = 302
    let mut digits = Vec::with_capacity(302);
    digits.push(1);
    for _ in 0..1000 {
        let mut carry = 0;
        for d in &mut digits {
            let tmp = *d * 2 + carry;
            *d = tmp % 10; //
            carry = tmp / 10;
        }
        while carry > 0 {
            digits.push(carry % 10);
            carry /= 10;
        }
    }
    let sum: u32 = digits.iter().sum();
    solution!(sum)
}

problem!(
    16,
    solve,
    "a6f988d30328bd706c66f8ac0d92aac21dd732149cdd69cb31f459dca20c5abe"
);
