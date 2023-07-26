use std::cell::RefCell;
use std::rc::Rc;

pub fn explain() {
    //If you use RefCell with Rc you can have multiple owners of mutable data
    
    //Rc only allows immutable references,
    //but RefCell allows you to take an immutable reference and make it mutable
    //alright time to make that cons list enum for the billionth (3rd) time
    
    elaborate();
}

#[derive(Debug)] 
pub enum ConsList {
    //Rc so its countable, RefCell so it's mutable
    Cons(Rc<RefCell<i32>>, Rc<ConsList>),
    Nil,
}

use ConsList::*;

fn elaborate() {
    //Rc wrapped in RefCell: multiple mutable to 5
    let value = Rc::new(RefCell::new(5));

    //throw it in a, connect it to a 6, then end
    let a = Cons(Rc::clone(&value), Rc::new(Cons(Rc::new(RefCell::new(6)), Rc::new(Nil))));
    let a_rc = Rc::new(a);

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a_rc));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a_rc));

    //Calling the fn auto derefs Rc, call another to deref RefCell
    //explicit derefs here: rust auto derefs the Rc so you don't need the parentheses or asterisk
    *(*value).borrow_mut() += 10;

    println!("a after = {:?}", a_rc);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    //Oh btw Mutex<T> is the thread-safe version of RefCell<T>
}