pub mod rules;
pub mod use_case;
pub mod logging_borrows;
pub mod multiple_mutable;
pub mod memory_leaks;

pub fn explain() {
    println!("Interior mutability is a design pattern that allows you \
    to mutate data even when there are immutable references to that data");

    //this uses unsafe code that bends rust's usual pointer safety rules (patience.)
    //we can only use interior mutability when we can ensure borrowing rules
    //will be followed at runtime (100% sure bc compiler can't guarantee)
    rules::explain();
    use_case::explain();
    logging_borrows::explain();
    multiple_mutable::explain();
    memory_leaks::explain();
}