fn recursive_fizzbuzz(current: usize, max: usize){
    let s = match (current % 3 == 0, current % 5 == 0) {
        (false, false) => format!("{}", current),
        (true, false) => String::from("Fizz"),
        (false, true) => String::from("Buzz"),
        (true, true) => String::from("FizzBuzz"),
    };
    println!("{}", s);
    if current < max { recursive_fizzbuzz(current + 1, max) };
}

fn main() {
    recursive_fizzbuzz(1, 100);
}
