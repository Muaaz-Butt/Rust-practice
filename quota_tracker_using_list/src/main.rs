use std::{cell::{Ref, RefCell}, rc::Rc};
use List::{Cons, Nil};

#[derive(Debug)]

enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(10)), Rc::new(Cons(Rc::new(RefCell::new(20)), Rc::clone(&a))));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);  
    println!("c after = {:?}", c);  
}
