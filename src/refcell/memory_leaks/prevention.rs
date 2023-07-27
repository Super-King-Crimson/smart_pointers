pub fn explain() {
    println!("Rust doesn't catch reference cycles and memory leaks");
    //You can prevent refcycles by testing your code lmaoooo
    //you can also reorganize data 
        //so it expresses ownership (or non-ownership) over a value
    //for example, our ConsList could instead borrow the next Cons/Nil variant 
    //(we shouldn't do this tho because then we'd have to specify lifetimes)
    weak_refs::explain();
}

mod weak_refs;