mod finished;
mod starter;

use crate::finished::finished::finished;
use crate::starter::starter::starter;


pub fn main() {
    let answer = finished();
    let attempt = starter();

    // fizzbuzz
    assert_eq!(answer.fizzbuzz_count, attempt.fizzbuzz_count);
    // fizz
    assert_eq!(answer.fizz_count, attempt.fizz_count);
    // buzz
    assert_eq!(answer.buzz_count, attempt.buzz_count);
}
