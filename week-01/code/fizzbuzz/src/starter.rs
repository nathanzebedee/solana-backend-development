use crate::check_answer::Counts;

pub mod starter {
    use super::*;

    // define a public function called "starter" that returns a Counts struct
        // Counts { fizzbuzz: 0, fizz: 0, buzz: 0 }
        // loop through 1 to 100 (inclusive)
            // if the number is divisible by 3 and 5, increment fizzbuzz_count
            // if the number is divisible by 3, increment fizz_count
            // if the number is divisible by 5, increment buzz_count
    
    pub fn starter() -> Counts {

        Counts {
            fizzbuzz_count: 0,
            fizz_count: 0,
            buzz_count: 0,
        }
    }
}
