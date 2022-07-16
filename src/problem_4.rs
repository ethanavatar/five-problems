/*
Write a function that given a list of non negative integers, arranges them such that they form the largest possible number.
For example, given [50, 2, 1, 9], the largest formed number is 95021.
*/

pub fn greatest_concatenation(list: Vec<u32>) -> u32 {
    let mut results: Vec<String> = Vec::new();
    for i in 0..list.len() {
        let mut new_string = list[i].to_string();
        for j in 0..list.len() {
            if i != j {
                new_string.push_str(&list[j].to_string());
            }
        }
        results.push(new_string);
    }
    let mut int_results: Vec<u32> = results.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    int_results.sort();
    int_results.last().unwrap().clone()
}