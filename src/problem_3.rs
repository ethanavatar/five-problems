/*
Write a function that computes the list of the first 100 Fibonacci numbers.
By definition, the first two numbers in the Fibonacci sequence are 0 and 1, and each subsequent number is the sum of the previous two.
As an example, here are the first 10 Fibonnaci numbers: 0, 1, 1, 2, 3, 5, 8, 13, 21, and 34.
*/

pub fn fib(first_n: u32) -> Vec<u128> {
    let mut fib_list = vec![0, 1];
    for _ in 2..first_n {
        let new_fib = fib_list[fib_list.len() - 1] + fib_list[fib_list.len() - 2];
        fib_list.push(new_fib);
    }
    fib_list
}