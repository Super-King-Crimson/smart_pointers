pub mod for_references;
pub mod custom_type;
pub mod coercions;

pub fn explain() {
    println!("Smart pointers can be treated like regular ref with Deref");
    //To do this, we'll have to learn how normal references are dereferenced

    for_references::explain();
    
    println!("Now that we know that, we can treat a Box like a ref");

    let x = 5;
    let y_box = Box::new(x);

    println!("Even though a box, y is just x dereferenced, right? {}", x == *y_box);
    
    //Ok now let's make our OWN smart pointer that implements Deref itself
    custom_type::create();

    //Finally let's talk about deref coercions and how it works with mutability
    coercions::explain();   
}