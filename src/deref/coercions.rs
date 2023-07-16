pub mod mutability;

pub fn explain() {
    println!("Remember how things get automatically dereferenced in some cases?");

    let arr: [u8; 8] = [1, 3, 7, 15, 31, 63, 127, 255];
    let arr_ref = &arr;

    println!("Watch me do a magic trick: {}", arr_ref[3]); //NO DEREF REQUIRED???

    //Also &String can be coerced into a &str because String deref returns a &str
    //Oh yeah also this
    let val = &&*&&****&&&&*&**&&*&String::from("Bro Why Does This Work");
    this_is_cool(val); //rust automatically derefs my &&&String into a &str
    //if rust didn't do this then uhh
    this_is_cool(&(***val)[..]);
    //what in GOD's name

    //ok finally fr
    mutability::explain();
}

fn this_is_cool(val: &str) {
    println!("{val}");
}