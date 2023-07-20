#![allow(unused)]
pub mod topic {
    pub fn introduce() {
        println!(
"
-----------------CHAPTER 15: SMART POINTERS-----------------
- A pointer is a variable that holds the memory address of some data
- That memory address is the location where some other data is stored
- The most common Rust pointer is a reference (&)
    - References only borrow the value pointed to, so no overhead

- However, smart pointers are pointers with metadata and other stuff
- We're going to learn about some smart pointers, which own the data they point to.
- btw String and Vec<T> are smart pointers, they own their data and it can be manipulated

- Smart pointers are usually implemented with structs, and implement Deref and Drop
    - Deref allows an instance of that smart pointer struct to be like a reference
        - This allows it to work with references or smart pointers
    - Drop allows you to control what happens when the smart pointer instance goes out of scope

- There are a lot of smart pointers (many libs even have their own smart pointers!),
- so we're only going over a few:
    - Box<T>
    - Rc<T>
    - Ref<T> and RefMut<T>, accessed through RefCell<T>

- Finally, we'll go over interior mutability and reference cycles: memory leaks.
- As always, check the docs.
");
    }
}

pub mod r#box;

pub mod deref;

pub mod r#drop;

pub mod reference_counter;