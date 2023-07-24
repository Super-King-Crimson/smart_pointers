pub enum ConsList {
    Cons(i32, Rc<ConsList>),
    Nil
}

use std::rc::Rc; //not in prelude, unlike drop and Drop
use ConsList::*;

pub fn explain() {
    let a = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));
    let a_rc = Rc::new(a); //reference count (rc) = 1 

    //Instead of taking ownership, clone the Rc<ConsList> that holds a
    //Calling clone increments rc
    let b = Cons(3, Rc::clone(&a_rc)); //rc = 2    
    let c = Cons(4, Rc::clone(&a_rc)); //rc = 3

    //We COULD call a_rc.clone(), but rust convention says use Rc::clone(&a)
    //Rc::clone() just increments rc, which takes less time than a full clone
    //Calling Rc::clone() as a static method allows visual distinguishing from a normal deep copy clone for performance optimizations

    //We can even see how many references an Rc has with Rc::strong_count()
    println!("After creating c, we have {} references", Rc::strong_count(&a_rc));

    {
        let d = Cons(5, Rc::clone(&a_rc));    
        let e = Cons(19, Rc::clone(&a_rc));
        {
            let f = Cons(1, Rc::clone(&a_rc));
            println!("After creating f, we have {} references", Rc::strong_count(&a_rc));
        }
        println!("After exiting f's scope, we have {} references", Rc::strong_count(&a_rc));
    }
    println!("After exiting d and e's scope, we have {} references", Rc::strong_count(&a_rc));
    //Wait, strong count? Apparently that's a thing, we'll learn about it soon

    //Note that you can only share immutable references through Rc's, 
    //otherwise you may break Rust's Pointer Safety Principle (all of these are aliased)

    //To mutate data, use interior mutability pattern and RefCell<T> with Rc<T>
} //We can't display it, but here a_ref's rc drops to 0, and it's fully dropped