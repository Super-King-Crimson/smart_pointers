pub mod prevention;

pub fn explain() {
    println!("It is possible to create memory that never cleaned up (memory leak)");
    //These are called reference cycles: where objects refer to each other in a cycle, so rc = 0 never happens

    //Let's make one:
    elaborate();
    prevention::explain();
}

use std::cell::RefCell;
use std::rc::Rc; 

//time to make another ConsList
//this time instead of being able to modify the value, 
//you can now modify the pointer that the ConsList is referring to
#[derive(Debug)]
pub enum ConsList {
    Cons(i32, RefCell<Rc<ConsList>>),
    Nil,
}

use ConsList::*;

impl ConsList {
    fn next(&self) -> Option<&RefCell<Rc<ConsList>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn elaborate() {
    //because we can modify refcells, we can make two different cons lists
    let a_rc = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    let b_rc = Rc::new(Cons(3, RefCell::new(Rc::clone(&a_rc))));

    println!("a: {:?}", a_rc);
    println!("b: {:?}", b_rc);

    if let Some(ref_cell) = a_rc.next() {
        *ref_cell.borrow_mut() = Rc::clone(&b_rc);
    }

    println!("a_rc count: {}", Rc::strong_count(&a_rc));
    println!("b_rc count: {}", Rc::strong_count(&b_rc));

    //STACK OVERFLOW: you're trying to print something with infinite length
    //refers to itself a billion times a -> b -> a -> b -> a -> b -> a...
    // println!("a: {a_rc:?}");

} //Even after the function ends and a_rc and b_rc go out of scope, the Rc is only 1
//the ref count will never drop to 0, because the heap data of a_rc and b_rc
//will always refer to each other in a REFERENCE CYCLE