
fn fizzbuzz<'a, T>(nums: T)
    where T: IntoIterator<Item = &'a usize>
{
    for num in nums {
        match (num % 3 == 0, num % 5 == 0) {
            (false, false) => println!("{}", num),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (true, true) => println!("FizzBuzz"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        let nums : Vec<usize> = vec![1,2,3,4,5,15];
        fizzbuzz(&nums);

        let nums: [usize; 6] = [1,2,3,4,5,15];
        fizzbuzz(&nums);
    }
}
