//Welcome back, cons list!
use super::r#box::recursive_types::List::{Cons, Nil};

pub mod multiple_owners;

pub fn explain() {
    println!("Oh my god we just went back to GC land");

    //Sometimes one value has multiple owners, which kinda muddles Rust's ownership system
    //Like graph data structs where multiple edges point to another node
    //You'd only want to clean up when there are no more pointers to a node

    println!("Rc<T> keeps track of references to a value: when it's 0, value is dropped");
    //Perfect for situations where you don't know when a value will be dropped @compile-time
    //Only can be used in single-threaded environments

    //Alright let's make two lists that refer to a;
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    //This doesn't work: Rust expects each value to have ONE owner
    //We COULD add reference params to Cons
        //each item in the list lives as long as the entire list
    //but that's not always what we'll want

    //Instead, let's make a new Cons list that instead uses Rc<T>
        //We'd replace but that'd kinda break the box example so...
    multiple_owners::explain();   
}


