mod finished;
mod starter;

use crate::finished::finished::finished;
use crate::starter::starter::starter;

pub mod check_answer {
    use super::*;

    pub fn check_answer() {

        let answer = finished();
        let attempt = starter();

        // fizzbuzz
        assert_eq!(answer.fizzbuzz_count, attempt.fizzbuzz_count);
        // fizz
        assert_eq!(answer.fizz_count, attempt.fizz_count);
        // buzz
        assert_eq!(answer.buzz_count, attempt.buzz_count);
    }

    pub struct Counts {
        pub fizzbuzz_count: u32,
        pub fizz_count: u32,
        pub buzz_count: u32,
    }
}

pub fn main() {
    check_answer::check_answer();
}
