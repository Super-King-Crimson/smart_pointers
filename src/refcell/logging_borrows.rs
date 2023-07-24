pub use std::cell::RefCell;

pub fn explain() {
    println!("Instead of & and &mut, \
    RefCells use borrow() and borrow_mut()");

    //borrow returns a Ref<T>, borrow_mut returns a RefMut<T>, both implement Deref
    //only reason these exist is so rust can borrow check during run-time
    
    //A RefCell keeps track of how many Ref and RefMut's are currently alive
    //Just like compiler checks, if there is a RefMut and any other Ref, panics!
    let a = RefCell::new(10);
    let mut a_ref1 = a.borrow_mut();

    //this PANICS, but lets the program compile
    // let mut a_ref2 = a.borrow_mut();  //BorrowMutError
    //and since it's a panic you can't handle the Err

    //Using a RefCell means you:
        //catch borrowing errors later
        //must pay performance cost for keeping track of borrows
    //But mock objects are really cool so i'd say its worth it probably
}