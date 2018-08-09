

struct FizzBuzz {
    next: usize,
    max: usize,
}

impl FizzBuzz {
    fn new(min: usize, length: usize) -> Self {
        let max = if length > 0 { min + length - 1 } else { 0 }; // protect from underflow
        FizzBuzz { next: min, max }
    }
}

impl Iterator for FizzBuzz {
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

fn main() {
    for text in FizzBuzz::new(1,100) {
        println!("{}", text);
    }
}
