pub mod recursive_types;

pub fn explain() {
    println!("I don't even know if this counts as a smart pointer");

    //A box just points to data on the heap, so they don't have many special features
    let my_box = Box::new(20u8);
    println!("{my_box}");

//Use a box when you:
    //Have a type with an unknown size and want to use it where you need to know exact size
        //like a recursive type
    //Have a lot of data but don't want it to be copied when you transfer its ownership
    //Want to own a value and you only care if it implements a specific trait
        //make a trait object

//This probably makes no sense you'll see examples of these later down the line
    //By later down the line I mean right now lmaoooooooo
    recursive_types::explain();
}

fn return_to_main() {
    println!("The only reason a Box<T> is a smart pointer is because it impls Deref & Drop");
    //That means it can be treated like a reference, and has its data dropped when it leaves scope
}