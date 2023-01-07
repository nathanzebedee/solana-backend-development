use crate::check_answer::Counts;

pub mod finished {
    use super::*;

    pub fn finished() -> Counts {
        let mut fizzbuzz_count = 0;
        let mut buzz_count = 0;
        let mut fizz_count = 0;

        for i in 1..=100 {
            match i {
                i if i % 15 == 0 => fizzbuzz_count += 1,
                i if i % 3 == 0 && i % 5 != 0 => fizz_count += 1,
                i if i % 5 == 0 && i % 3 != 0 => buzz_count += 1,
                _ => (),
            }
        }

        Counts {
            fizz_count,
            buzz_count,
            fizzbuzz_count,
        }
    }
}
