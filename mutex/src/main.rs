use std::sync::{Mutex, Arc};
use std::thread;
use std::rc::Rc;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        println!("num = {}", num);
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles =vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            
            //println!("number at every iteration: {}", num);

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}