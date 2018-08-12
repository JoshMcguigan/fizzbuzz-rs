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
}
