pub mod starter {

    // define a public function called "starter" that returns a Counts struct
        // Counts { fizz_count: i32, buzz_count: i32, fizzbuzz_count: i32 }
        // loop through 1 to 100
            // if the number is divisible by 3 and 5, increment fizzbuzz_count
            // if the number is divisible by 3, increment fizz_count
            // if the number is divisible by 5, increment buzz_count
    
    pub fn starter() -> Counts {
        Counts {
            fizz_count: 0,
            buzz_count: 0,
            fizzbuzz_count: 0,
        }
    }

    pub struct Counts {
        pub fizz_count: i32,
        pub buzz_count: i32,
        pub fizzbuzz_count: i32,
    }
}
