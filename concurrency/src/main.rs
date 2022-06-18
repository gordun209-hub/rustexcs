// use std::thread;
// use std::time::Duration;
// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//
//     for i in 1..5 {
//         println!("hi number {} from main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     handle.join().unwrap();
// }
// use std::sync::mpsc;
// use std::thread;
//
// fn main() {
//     let (tx, rx) = mpsc::channel();
//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     });
//     // rx.recv blocks main threads execution and wait untill a value is sent down to the channel
//     // once is sent, recv willreturn it in a Result<T,E>
//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }
//
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;
// fn main() {
//     let (tx, rx) = mpsc::channel();
//
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//         // rip val gone but not forgotten
//     });
//     for received in rx {
//         println!("got : {}", received);
// }
//     }
//
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // --snip--
}

