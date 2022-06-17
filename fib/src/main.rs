fn main() {
    println!("{}", fibonacci(1));
    println!("{}", fibonacci(3));
    println!("{}", fibonacci(5));
    blastoff_fnlari()
}
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn blastoff_fnlari() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }  
    println!("blastofff")
}
