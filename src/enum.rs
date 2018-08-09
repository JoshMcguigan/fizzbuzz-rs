#[derive(Debug)]
enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Other(usize),
}

impl From<usize> for FizzBuzz {
    fn from(item: usize) -> Self {
        match (item % 3 == 0, item % 5 == 0) {
            (false, false) => FizzBuzz::Other(item),
            (true, false) => FizzBuzz::Fizz,
            (false, true) => FizzBuzz::Buzz,
            (true, true) => FizzBuzz::FizzBuzz,
        }
    }
}

fn main() {
    for i in 1..=100 {
        println!("{:?}", FizzBuzz::from(i));
    }
}
