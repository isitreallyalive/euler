//* Problem 17: Number Letter Counts
//* If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
//* If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
//* NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.

//! time complexity: O(?)
use euler::prelude::*;

fn solve() -> Solution {
    solution!((1..=1000).map(letter_count).sum::<u16>())
}

const UNITS: [u16; 10] = [0, 3, 3, 5, 4, 4, 3, 5, 5, 4]; // 0 unused
const TEENS: [u16; 10] = [3, 6, 6, 8, 8, 7, 7, 9, 8, 8]; // 10-19
const TENS: [u16; 10] = [0, 0, 6, 6, 5, 5, 5, 7, 6, 6]; // 0,10,20,...

fn letter_count(n: u16) -> u16 {
    match n {
        1..=9 => UNITS[n as usize],
        10..=19 => TEENS[(n - 10) as usize],
        20..=99 => {
            let tens = TENS[(n / 10) as usize];
            let units = UNITS[(n % 10) as usize];
            tens + units
        }
        100..=999 => {
            let hundreds = UNITS[(n / 100) as usize] + 7; // "hundred"
            let rest = n % 100;
            if rest == 0 {
                hundreds
            } else {
                hundreds + 3 + letter_count(rest) // "and"
            }
        }
        1000 => 11, // "one thousand"
        _ => 0,
    }
}

problem!(
    17,
    solve,
    "1a455b216c6e916943acf3fa4c7e57a7a5cac66d97cc51befca810c223ef9c23"
);
