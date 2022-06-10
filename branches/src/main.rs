fn main() {
    let number = 3;
    if number < 1 {
        println!("conditon was true");
    } else {
        println!("condition was false");
    }
    check_if_number_is_other_than_zero()
}

fn check_if_number_is_other_than_zero() {
    let number = 3;
    if number != 0 {
        println!("number was something other than zero")
    }
    check_if_divisible_by_four_three_or_two()
}

fn check_if_divisible_by_four_three_or_two() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisilbe by 4");
    } else if number % 3 == 0 {
        println!("number is divisilbe by 3");
    } else if number % 2 == 0 {
        println!("number is divisilbe by 2");
    } else {
        println!("number is not divisible by 4,3 or 2");
    }
    proof_of_if_else_statements_are_expressions()
}

fn proof_of_if_else_statements_are_expressions() {
    let condition = true;
    let number = if condition { 5 } else { 2 };
    println!("the value of number is {}", number)
}
