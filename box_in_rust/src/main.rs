use List::{Cons, Nil};
use std::ops::Deref;
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct  MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
      MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {} !", self.data);
    }
}

fn main() {
    // shows how to use a box to store an i32 value on the heap.
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    let x = 5;
    //let y = &x;
    let y = Box::new(x);
    assert_eq!(5, x);    
    assert_eq!(5, *y);   

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);    
    assert_eq!(5, *y);           //Will not work until we implement deref trait because we have not implemented Deref trait

    /**
     *  When we use * to dereference it actually calls .deref()
     */

    let m = MyBox::new(String::from("Rust"));
    hello(&m);  //it will automatically deref the argument until it becomes the string slice

    // deref is automatically done by the rust if it does not do this we have to write below code

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");

    let _c = CustomSmartPointer { data: String::from("some data")};
    println!("CustomSmartPointer created");
    drop(_c);
    println!("CustomSmartPointer dropped before the end of the main");

}
