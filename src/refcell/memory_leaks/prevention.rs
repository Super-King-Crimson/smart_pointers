pub fn explain() {
    println!("Rust doesn't catch reference cycles and memory leaks");
    //You can prevent refcycles by testing your code lmaoooo
    //you can also reorganize data 
        //so it expresses ownership (or non-ownership) over a value
    //for example, our ConsList could instead borrow the next Cons/Nil variant 
    //(we shouldn't do this tho because then we'd have to specify lifetimes)
    weak_refs::explain();
}

mod weak_refs {
    use std::borrow::BorrowMut;
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    pub fn explain() {
        println!("Wait actually there's another method");

        //Remember, calling Rc::clone() increments strong count of an Rc
        //Calling Rc::downgrade() increments weak count and makes a weak reference
        //weak references are like a borrow: they don't increment the count
        //any weak ref cycle gets broken when strong count gets to 0
        
        let a: Rc<i32> = Rc::new(10);
        let b: Rc<i32> = Rc::clone(&a);
        let c: Weak<i32> = Rc::downgrade(&a);

        println!("strong count: {}", Rc::strong_count(&b));
        println!("weak count: {}", Rc::weak_count(&a));

        //Because the value weak refs refer to could be dropped,
        //you need to check that the value still exists to do anything with it
        println!("{:?}", b);
        println!("{:?}", c); //the println doesn't assume the value exists
        println!("{:?}", c.upgrade());
        drop(a);
        drop(b);
        println!("{:?}", c.upgrade());

        //Because the value may not exist, an Option is returned,
        //so the some and none case can be handled

        println!("Now, let's make a tree structure \
            so we can prove Weak<T> prevents refcycles");
        
        //See tree.rs
    }
}