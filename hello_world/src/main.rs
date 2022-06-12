#![allow(unused)]
fn main() {
    let a = [1, 2, 3, 4, 5];

    // addition
    let sum = 5 + 10;

    //subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;

    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    //char
    let c = 'z';

    // Tuple types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructure tuplees
    let (x, y, z) = tup;
    println!("The values of tuplee is {} , {}, {}", x, y, z);

    // acccessing tuplees with dot notation

    let y: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = y.0;
    let six_point_four = y.1;
    let one = y.2;

    println!(
        "five hundred : {}, six point four : {}, one : {}",
        five_hundred, six_point_four, one
    );
    // Arrays
    let months = [
        "January", "February", "March", "April", "May", "June", "Augst", "...",
    ];
    // i32 typesinde 5 tane number war dion
    let g: [i32; 5] = [1, 2, 3, 4, 5];
    // 5 tane 3 yap dion
    let w = [3; 5];

    // println!("{:?}", w) --> [3,3,3,3,3,3]

    let s = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("first is {}, second is {}", first, second);
    let qw = 2;
    another_function(qw)
}

fn another_function(x: i32) {
    let x = x - 2;
    println!("The value is : {}", x + 2)
}
