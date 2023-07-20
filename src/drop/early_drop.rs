use super::custom_type::SuperKingPointer;

pub fn explain() {
    println!("You can't disable drop, as it's entire purpose is to be automatic");
    //You CAN, however, call drop early with std::mem::drop (in prelude)

    let skc_ptr1 = SuperKingPointer{
        data: String::from("GUYS")
    };
    
    let skc_ptr2 = SuperKingPointer{
        data: String::from("GUYS THEY TURNED ME INTO A SMART POINTER")
    };

    drop(skc_ptr1);
    println!("OH GOD THEY'RE ABOUT TO DROP ME");
    
    // skc_ptr2.drop();
    //Nope, skc2 is safe for now.
    //You can't explicity call a destructor (fn that cleans up an instance)

    //Remember, Rust automatically frees all variables at the end of scope
    //If we called drop it wouldn't remove the variable, so it would be double-freed    
} //skc_ptr2 dropped.

//drop takes ownership then drops so double free doesn't happen