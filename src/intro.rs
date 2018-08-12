

#[cfg(test)]
mod tests {
    #[test]
    fn using_if_else(){
        for i in 1..=100 {

            let divisible_by_three = i % 3 == 0;
            let divisible_by_five = i % 5 == 0;

            if divisible_by_three && divisible_by_five {
                println!("FizzBuzz");
            } else if divisible_by_three {
                println!("Fizz");
            } else if divisible_by_five {
                println!("Buzz");
            } else {
                println!("{}", i);
            }

        }
    }

    #[test]
    fn using_match(){
        for i in 1..=100 {

            let divisible_by_three = i % 3 == 0;
            let divisible_by_five = i % 5 == 0;

            match (divisible_by_three, divisible_by_five) {
                (true, true) => println!("FizzBuzz"),
                (true, false) => println!("Fizz"),
                (false, true) => println!("Buzz"),
                (false, false) => println!("{}", i),
            }

        }
    }
}
