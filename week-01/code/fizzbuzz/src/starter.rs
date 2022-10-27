use mod finished

pub mod starter {
    pub fn starter() {
        // write your Fizz Buzz program here
            // iterate from 1 to 100
            // if the number is divisible by 3, print "Fizz"
            // if the number is divisible by 5, print "Buzz"
            // if the number is divisible by 3 and 5, print "FizzBuzz"
            // count the number of times you print "FizzBuzz" and print this number at the end
            // return the number of times you print "FizzBuzz"
    }

    let attempt = starter();
    let answer = finished::finished();
    assert_eq!(answer, attempt);
}
