/*
Write a program that outputs all possibilities to put + or - or nothing between the numbers 1, 2, ..., 9 (in this order)
such that the result is always 100. For example: 1 + 2 + 34 – 5 + 67 – 8 + 9 = 100.
*/

pub fn homing_arithmetic() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    check_expression(&mut result, 2, 1, 1, "1".to_string());
    result
}

fn check_expression(out: &mut Vec<String>, iter: i32, sum: i32, f: i32, expr: String) -> () {
    if iter == 10 {
        if sum == 100 {
            out.push(expr.to_string());
        }
        return;
    }
    if f == 1 {
        check_expression(out, iter + 1, sum + iter, 0, expr.to_string() + &iter.to_string());
    } else if f == 0 {
        check_expression(out, iter + 1, sum - iter, 1, expr.to_string() + &iter.to_string());
    } else {
        check_expression(out, iter + 1, sum + iter, 0, expr.to_string() + &iter.to_string());
        check_expression(out, iter + 1, sum - iter, 1, expr.to_string() + &iter.to_string());
    }
}