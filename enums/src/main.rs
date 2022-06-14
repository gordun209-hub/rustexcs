// fn main() {
//     let number_list = vec![32, 50, 25, 100, 65];
//     let result = largest(&number_list);
//
//     println!("Largest number is {} ", result);
//
//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
//
//     let result = largest(&number_list);
//     println!("Largest is also : {}", result);
//
//     let integer = Point { x: 5, y: 10 };
//     println!("p.x = {}, p.y = {}", integer.x(), integer.y())
// }
//
// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
//     fn y(&self) -> &T {
//         &self.y
//     }
// }
//
// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course , as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet {}", tweet.summarize())

}
