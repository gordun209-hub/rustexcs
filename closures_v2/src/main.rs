fn main() {
    let x = 4;
    // use x as variable even if we dont give any params
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
    // this works too ?
    for val in v1 {
        println!("Got: {}", val);
    }
    iterator_sum();
    let v3: Vec<i32> = vec![1, 2, 3];
    let v4: Vec<_> = v3.iter().map(|x| x + 1).collect();

    println!("{:?}", v4)
}

fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let total: i32 = v1.iter().sum();

    println!("{}", total)
}
