use std::fmt::Display;
use std::fmt::Formatter;
use core::fmt;

#[derive(Debug)]
enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Other(u32),
}

impl From<u32> for FizzBuzz {
    fn from(item: u32) -> Self {
        match (item % 3 == 0, item % 5 == 0) {
            (false, false) => FizzBuzz::Other(item),
            (true, false) => FizzBuzz::Fizz,
            (false, true) => FizzBuzz::Buzz,
            (true, true) => FizzBuzz::FizzBuzz,
        }
    }
}

impl Display for FizzBuzz {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FizzBuzz::Other(n) => write!(f, "{}", n),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from(){
        for i in 1..=100 {
            println!("{:?}", FizzBuzz::from(i));
        }
    }

    #[test]
    fn into(){
        for i in 1..=100 {
            let fizzbuzz : FizzBuzz = i.into();
            println!("{:?}", fizzbuzz);
        }
    }

    #[test]
    fn display() {
        for i in 1..=100 {
            println!("{}", FizzBuzz::from(i));
        }
    }
}
