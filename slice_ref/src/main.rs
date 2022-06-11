fn main() {
    first_word()
}

fn first_word() {
    let s = String::from("Hello world");
    // ref to some part of s 
    let hello  = &s[0..5];
    let world = &s[6..11];
    println!("{}", hello.len());
    println!("{}", world.len())
}
