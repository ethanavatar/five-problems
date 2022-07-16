/*
Write a function that combines two lists by alternatingly taking elements.
For example: given the two lists [a, b, c] and [1, 2, 3], the function should return [a, 1, b, 2, c, 3].
*/

pub fn alternating_take(list1: Vec<i32>, list2: Vec<i32>) -> Vec<i32> {
    let mut new_list = Vec::new();
    let mut index = 0;
    while index < list1.len() && index < list2.len() {
        new_list.push(list1[index]);
        new_list.push(list2[index]);
        index += 1;
    }
    new_list
}