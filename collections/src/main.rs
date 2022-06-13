use std::collections::HashMap;
use std::fs::File;
use std::net::IpAddr;
fn main() {
    let k = vec![1, 2, 3, 4, 5];

    let third: &i32 = &k[2];
    println!("The third element is {}", third);

    match k.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    let s1 = String::from(
        " Hello, 
",
    );
    let s2 = String::from("World!");
    let s3 = String::from("") + &s1 + &s2;
    println!("{} , {}", s2, s3);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}",scores);
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10,50];

    let scor:HashMap<_, _> = 
    teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}",scor);

    let f = File::open("~/Desktop/coding/projects/rust/collections/src/hello.txt").unwrap();
    println!("{:?}", f);

    let _home : IpAddr = "127.0.0.1".parse().unwrap();
}
