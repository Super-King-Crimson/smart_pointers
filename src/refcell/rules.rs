pub fn explain() {
    println!("Here's how RefCells and interior mutability work:");

    //Quick review of borrowing rules (Pointer Safety Principle)
        //at any point, you may not have data be both aliased and mutable
            //in other words you can't have a mutable reference and any other references
        //references must always be valid
            //primary reason why you can't return a pointer to a function-scoped variable

    //A refcell represents singular ownership, like a Box
    //Except a Box enforces the rules at compile-time,
    //while a RefCell enforces them at run-time, panicking if they're broken

    //While there are benefits to checks at compile time,
    //checking borrow rules at run-time allows Rust to detect more safe scenarios
    //that the compile-time would have been uncertain about

    //Static analysis is concsrvative, and some properties of code vary too much
    //for the compiler to guarantee that 100% of situations are safe,
    //so if it finds one unsafe situation at compile-time, it doesn't let it through
    //(see the Halting Problem for an example of unsure safety)

    println!("RefCell just exists so you can tell the compiler to chill");

    //Oh btw RefCell also can only be used in single thread stuff ok
    conclude();
}

fn conclude() {
    println!("\
Ok let's go over the rules:
    - Rc<T> allows multiple owners of data by counting its references,
    - while a Box<T> and RefCell<T> can only have one owner.
    - Because of Rust's pointer safety rules,
    - Rc<T> can only be borrowed immutably,
    - while Box<T> and RefCell<T> can be borrowed mutably.
    - However, RefCell<T> safety is checked at run-time,
    - allowing mutation when rustc can't confirm the program is safe.
Oh btw since the references are checked at compile time,
you could declare an immutable RefCell and make mutable pointers to it

Mutating the value inside an immutable value \
*is* the interior mutability pattern.");
}