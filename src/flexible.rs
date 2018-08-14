
fn fizzbuzz<'a, T>(nums: T)
    where T: IntoIterator<Item = &'a u32>
{
    for num in nums {
        let divisible_by_three = num % 3 == 0;
        let divisible_by_five = num % 5 == 0;

        match (divisible_by_three, divisible_by_five) {
            (true, true) => println!("FizzBuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (false, false) => println!("{}", num),
        }
    }
}

fn fizzbuzz_vec(nums: &Vec<u32>) {
    for num in nums {
        let divisible_by_three = num % 3 == 0;
        let divisible_by_five = num % 5 == 0;

        match (divisible_by_three, divisible_by_five) {
            (true, true) => println!("FizzBuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (false, false) => println!("{}", num),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        let nums : Vec<u32> = vec![1,2,3,4,5,15];
        fizzbuzz_vec(&nums);

        let nums : Vec<u32> = vec![1,2,3,4,5,15];
        fizzbuzz(&nums);

        let nums: [u32; 6] = [1,2,3,4,5,15];
        fizzbuzz(&nums);
    }
}
