//
//Smart pointers
//reference counting smart pointer => enables you to have multiple
//owners of data by keeping track of the number of owners and
//when no owners remain, cleaning up data.
//
//difference between references and smart pointers is that references are pinters that
//only borrow data; in contrast, in many cases, smart pointers own the data they point to.
//smart pointers implement the Deref and Drop traits
//Deref = allows an instance of the smart pointer struct to behave like a reference so you can
//write code that works with either references or smart pointers,
//Drop= customize the code that is run when an instance of the smart pointer goes out
//of scope
//
//some of the smart pointers are
// Box<T>
// Rc<T>
// Ref <T>
// RefMut<T>
//
//
// using Box<T> allow you to store data on heap rather than the stack, what remains on the stack
// is the pointer to the heap data.
// we can  store recursive types in box because it points to the heap and
// heap can scale
use std::ops::Deref;
fn main() {
    let x = 5;
    // created reference to x value
    let y = &x;

    assert_eq!(5, x);
    // use dereference for following reference to the copied data
    assert_eq!(5, *y);
    let z = 5;
    let w = Box::new(x);
    let u = 5;
    let g = MyBox::new(x);

    assert_eq!(5, u);
    // rust runs that code -> *(y.deref())
    assert_eq!(5, *g);

    assert_eq!(5, z);
    assert_eq!(5, *w);
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created. ")
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Droppin CustomSmartPointer with data `{}`!", self.data);
    }
}
