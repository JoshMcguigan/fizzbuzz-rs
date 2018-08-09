use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;

struct FizzBuzz(usize);

impl Display for FizzBuzz {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match (self.0 % 3 == 0, self.0 % 5 == 0) {
            (false, false) => format!("{}", self.0),
            (true, false) => String::from("Fizz"),
            (false, true) => String::from("Buzz"),
            (true, true) => String::from("FizzBuzz"),
        };
        write!(f, "{}", s)
    }
}

fn main() {
    for i in 1usize..=100usize {
        println!("{}", FizzBuzz(i));
    }
}
