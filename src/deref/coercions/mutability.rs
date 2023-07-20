use super::super::custom_type::SuperKingBox;

pub fn explain() {
    println!("oh uh, there's another deref btw");
    
    //Funny enough deref alone won't work if you have a mutable references, you need to implement DerefMut
    let mut s_box = SuperKingBox::new(20);
    *s_box += 10;

    println!("Wait brb gotta go impl DerefMut");

    //Ok done now I acn explain how derefs work with mutability
    //A mutable smart pointer can get coerced to mutable, and immutable to immutable
    //Additionally, a mutable smart pointer can get coerced to immutable
    //However, an immutable smart pointer cannot EVER get coerced to mutable
}