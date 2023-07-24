use smart_pointers::*;

fn main() {
    topic::introduce();
    r#box::explain();
    deref::explain();
    r#drop::explain();
    reference_counter::explain();
    refcell::explain();
}