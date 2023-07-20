pub struct SuperKingPointer {
    pub data: String,
}

//for some reason Drop is in the prelude but Deref/DerefMut isnt (racis?)
impl Drop for SuperKingPointer {
    //Accepts mutable reference to self
    fn drop(&mut self) {
        //here we just print it, 
        println!("Dropped '{}'", self.data);
        //but here we can put and code we want run when value goes out of scope
    }
}

pub fn explain() {
    //Variables are dropped in stack order (first assigned, last dealloc'd)
    {
        SuperKingPointer { data: String::from("Hey") };
        SuperKingPointer { data: String::from("Hi") };
    }   //Dropped 'Hi'
        //Dropped 'Hey'

    let s = SuperKingPointer { data: String::from("hello") };
    println!("Just made a pointer to '{}'", s.data);
} //Dropped 'hello'