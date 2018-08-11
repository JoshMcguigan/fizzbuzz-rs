
fn fizzbuzz<'a, T>(nums: T)
    where T: IntoIterator<Item = &'a usize>
{
    for num in nums {
        println!("{}", num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        let nums : Vec<usize> = vec![1,2,3,4,5,15];

        fizzbuzz(&nums);


    }
}
