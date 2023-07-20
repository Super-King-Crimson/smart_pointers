pub mod custom_type;
pub mod early_drop;

pub fn explain() {
    println!("Second trait of smart pointers is Drop");
    //Controls what happens to the smart pointer when it goes out of scope
    //Removes risk of leakage: no need to manually call memory freeing code

    custom_type::explain();
    early_drop::explain();
}