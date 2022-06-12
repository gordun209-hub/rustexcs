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

fn momo() {
    let mut s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    println!("The length of '{}' is {}.", s1, len);

    println!("{}", s1);
    change(&mut s1);
    println!("{}", s1);
    let la = dangle();
    println!("{}", la)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn dangle() -> String {
    String::from("hello")
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
// structures

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
fn jojo() {
    // --snip--
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    //println!("area of rectangle is {} .", area(&rect1));
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let _user2 = User {
        active: user1.active,
        username: user1.username,
        email: user1.email,
        sign_in_count: user1.sign_in_count,
    };
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    println!("rect1 is {:?}", rect1);

    // }

    // Methods

    #[derive(Debug)]

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    fn mama() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        let _sq = Rectangle::square(3);
        let mq = Rectangle::area(&rect2);
        println!("{}", mq);
        println!("Can rect1 hold rect2? {}", rect1.area());
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect3));
        if rect2.can_hold(&rect3) {
            println!("laa rect2 dha buyk")
        } else {
            println!("laa rect2 dha kuck")
        };
        println!("{} is the truth", can_hold(&rect1, &rect2))
    }

    fn can_hold(rec1: &Rectangle, rect2: &Rectangle) -> bool {
        rec1.can_hold(rect2)
    }
}
