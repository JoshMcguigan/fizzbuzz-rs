use std::convert::From;
use std::ops::Deref;

#[derive(Debug)]
struct OutputText {
    text: String,
}

impl From<usize> for OutputText {
    fn from(item: usize) -> Self {
        let s = match (item % 3 == 0, item % 5 == 0) {
            (false, false) => format!("{}", item),
            (true, false) => String::from("Fizz"),
            (false, true) => String::from("Buzz"),
            (true, true) => String::from("FizzBuzz"),
        };
        OutputText { text: String::from(s) }
    }
}

impl Deref for OutputText {
    type Target = String;

    fn deref(&self) -> &String {
        &self.text
    }
}

trait Printer {
   fn print(&self);
}

impl Printer for String {
    fn print(&self){
        println!("{}", self);
    }
}

fn fizzbuzz(text: OutputText) {
    text.print();
}

fn main() {
    for i in 1usize..=100usize {
        fizzbuzz(i.into());
    }
}
