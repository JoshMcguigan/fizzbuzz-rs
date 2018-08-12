struct FizzBuzzer {
    next: usize,
    max: usize,
}

impl FizzBuzzer {
    fn new(min: usize, length: usize) -> Self {
        let max = if length > 0 { min + length - 1 } else { 0 }; // protect from underflow
        FizzBuzzer { next: min, max }
    }
}

impl Iterator for FizzBuzzer {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.next > self.max { return None }

        let s = match (self.next % 3 == 0, self.next % 5 == 0) {
            (false, false) => format!("{}", self.next),
            (true, false) => String::from("Fizz"),
            (false, true) => String::from("Buzz"),
            (true, true) => String::from("FizzBuzz"),
        };

        self.next += 1;

        Some(String::from(s))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        for text in FizzBuzzer::new(1, 100) {
            println!("{}", text);
        }
    }
}
