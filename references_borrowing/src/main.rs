fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // & means refer without taking ownership
    println!("The lenght of {} is {}", s1, len);
    dangling_example()
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
// Dangling pointer example
fn dangling_example() {
    // let ref_to_nothing = dangle();
    let no_dangle_string = no_dangle();
    println!("{}", no_dangle_string)
}
// Because s is created inside dangle, when the code of dangle is finished, s will be deallocated.
// But we tried to return a reference to it. That means this reference would be pointing to an invalid String.
// That’s no good! Rust won’t let us do this.
// gives error bec references to nothing but function return type is a reference
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
// this works because we return string directly not a pointer
// s
fn no_dangle() -> String {
    String::from("hello")
}
