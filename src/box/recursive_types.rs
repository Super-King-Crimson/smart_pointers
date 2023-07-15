pub fn explain() {
    println!("Boxes aren't really useful on their own, 
as single values are usually better left on the stack instead of the heap");

    //One place where boxes are useful is in recursive types (like a cons list)
    elaborate();
}

//Each item contains two elements: the value of the item and the value of the next item
//It's basically a linked list but for Lisp users (lisp linked list? woah im a poet)
//You make it by calling a cons function that makes a pair from the two args provided
//The last value in a cons list is Nil (not nil or null)
    //Example: (1, (2, (3, Nil)))
pub enum List {
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}
//You probaly won't use cons lists in Rust, just use Vec<T>
//But recursive types can be useful, so let's go over it

use List::*;

fn elaborate() {
    println!("If you don't include boxes, Rust won't let you make this typ");
    print!("Rust needs to know how much space a type takes up at compile-time, ");
    println!("and this structure could take up infinite space");
    //And rust will tell you that: 
        //has infinite size...insert some indirection (Box, Rc, or &)
    //Rust computes the size of a type by finding the largest amount of data it could store
    //So to find how much data a List can store, it needs enough data to store a Cons with an i32 and a List, which, if we're going for maximum size, would need to store the Cons variant that stores an i32 and another List, which would store an i32 and a List, this List, of course, being the Cons variant, would store another i32 and another List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, which stores an i32 and a List, and so on until your computer blows up. Thanks for reading until the end! -superkc
    //Instead of blowing up your computer, we can insert some indirection:
        //store the value indirectly in a pointer: a pointer always has a known size
    //Now the items will be next to each other, pointing to their next item, instead of inside each other

    #[allow(unused)]
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))));
    //Yay!
    super::return_to_main();
}