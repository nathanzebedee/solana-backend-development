pub mod finished {
    pub fn finished() {
        let mut count = 0;
        for i in 1..=100 {
            if i % 15 == 0 {
                count += 1;
                println!("FizzBuzz", count);
            }
            else if i % 3 == 0 {
                println!("Fizz");
            }
            else if i % 5 == 0 {
                println!("Buzz");
            }
        }
        println!("FizzBuzz count: {}", count);
        return count;
    }
}
