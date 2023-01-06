use crate::check_answer::Counts;

pub mod starter {
    use super::*;

    // TODO: Define a public function called "starter" that returns a Counts struct
        // Counts { fizzbuzz_count, fizz_count, buzz_count }
        // TODO: Loop through 1 to 100 (inclusive)
            // TODO: If the number is divisible by 3 and 5, increment fizzbuzz_count
            // TODO: If the number is divisible by 3, increment fizz_count
            // TODO: If the number is divisible by 5, increment buzz_count
    
    pub fn starter() -> Counts {
        Counts {
            fizzbuzz_count: 0,
            fizz_count: 0,
            buzz_count: 0,
        }
    }
}
