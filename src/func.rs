struct FizzBuzzer<FnFizz, FnBuzz, FnFizzBuzz, FnOther> {
    fn_fizz: FnFizz,
    fn_buzz: FnBuzz,
    fn_fizzbuzz: FnFizzBuzz,
    fn_other: FnOther,
}

impl<FnFizz, FnBuzz, FnFizzBuzz, FnOther> FizzBuzzer<FnFizz, FnBuzz, FnFizzBuzz, FnOther>
    where
        FnFizz: Fn(),
        FnBuzz: Fn(),
        FnFizzBuzz: Fn(),
        FnOther: Fn(u32),
{
    fn new(fn_fizz: FnFizz, fn_buzz: FnBuzz, fn_fizzbuzz: FnFizzBuzz, fn_other: FnOther) -> Self {
        Self { fn_fizz, fn_buzz, fn_fizzbuzz, fn_other }
    }

    fn eval(&self, num: u32) {
        match (num % 3 == 0, num % 5 == 0) {
            (false, false) => (self.fn_other)(num),
            (true, false) => (self.fn_fizz)(),
            (false, true) => (self.fn_buzz)(),
            (true, true) => (self.fn_fizzbuzz)(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let fizzbuzzer = FizzBuzzer::new(
            || println!("Fizz"),
            || println!("Buzz"),
            || println!("FizzBuzz"),
            |num| println!("{}", num),
        );

        for i in 1..=100 {
            fizzbuzzer.eval(i);
        }
    }
}
