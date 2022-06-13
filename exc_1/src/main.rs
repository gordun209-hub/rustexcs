// fn main() {
//     let y =
//     // this is a expression bec it doesnt end with ; ex: x + 1
//     {
//         let x = 3;
//         x + 1
//     };
//     println!("The value of y is : {}", y);
//     println!(
//         "the laa is {} and stronger is {}",
//         five(),
//         five_but_stronger(2)
//     );
//
//     let _four = IpAddrKind::V6;
//     let _six = IpAddrKind::V4;
// }
//
// fn five() -> i32 {
//     5
// }
// fn five_but_stronger(x: i32) -> i32 {
//     x + 12
// }
//
// enum IpAddrKind {
//     V4,
//     V6,
// }
// // enums
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
//
// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6,
//     }
//     // allow deadcode
//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }
//
//     let _home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//
//     let _loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }
//
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeCOlor(i32, i32, i32),
// }
//
// struct QuitMessage;
//
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String);
//
// struct ChangeColorMessage(i32, i32, i32);
//
// impl Message {
//     fn call(&self) {}
// }
//
// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
//     let _some_number = Some(5);
//     let _some_string = Some("a string");
//     let _absent_number : Option<i32> = None;
// }
//
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    let five = Some(5);
    let six = plus_one(five);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => laa31(),
        _ => mqq(),
    }
    println!("six is : {}", six.unwrap());
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("some , :{}", max),
        _ => (),
    }
}

fn mqq() {}
fn add_fancy_hat() {}
fn laa31() {}
