//I didn't even know you could do this! One value tuple structs are real!
//Unlike a real box, this one's fully stored on the stack
pub struct SuperKingBox<T>(T);

impl<T> SuperKingBox<T> {
    pub fn new(value: T) -> SuperKingBox<T> {
        SuperKingBox(value)
    }
}

pub fn create() {
    println!("Obviously derefs won't work here yet");
    //We haven't impl'd the trait

    //Once we do tho, this should work
    let x = "five";
    let skc_y = SuperKingBox::new(x);

    println!("Despite everything, five is still five, right? {}", x == *skc_y);
}

use std::ops::{Deref, DerefMut};
impl<T> Deref for SuperKingBox<T> {
    //Target type is an associated type defined for the Deref trait
    //It also exists for iterators (we'll learn more later)
    type Target = T;

    fn deref(&self) -> &Self::Target {
        //the deref function should return a reference, which will be dereferenced
        //And since this is a tuple struct, we access element 0
        &self.0
        //In the background, the code run is:
            // *(y.deref())
        //when you try to deref a SuperKingBox
    }
}

//For some reason I feel compelled to add this random thing...
impl<T> DerefMut for SuperKingBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}