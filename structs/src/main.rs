// fn main() {
//     let mut user = User {
//         email: String::from("someone@example.com"),
//         username: String::from("gordun209"),
//         active: true,
//         sign_in_count: 2,
//     };
//     user.email = String::from("anoitheremail.com");
// }
//
// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
//
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} ssquare pixels.",
        area(&rect1)
    );
    println!("Rect1 is {:?}",rect1)
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
