fn main() {
    let y =
    // this is a expression bec it doesnt end with ; ex: x + 1 
    {
        let x = 3;
        x + 1
    };
    println!("The value of y is : {}", y);
    println!(
        "the laa is {} and stronger is {}",
        five(),
        five_but_stronger(2)
    );

    let _four = IpAddrKind::V6;
    let _six = IpAddrKind::V4;
}

fn five() -> i32 {
    5
}
fn five_but_stronger(x: i32) -> i32 {
    x + 12
}

enum IpAddrKind {
    V4,
    V6,
}
// enums
