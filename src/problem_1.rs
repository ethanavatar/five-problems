/*
Write three functions that compute the sum of the numbers in a given list using a for-loop, a while-loop, and recursion.
*/

pub fn sum_for(list: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in list {
        sum += i;
    }
    sum
}

pub fn sum_while(list: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut new_list = list.clone();
    while let Some(i) = new_list.pop() {
        sum += i;
    }
    sum
}

pub fn sum_recursion(list: Vec<i32>, index: usize) -> i32 {
    if index < list.len() {
        list[index] + sum_recursion(list, index + 1)
    } else {
        0
    }
}