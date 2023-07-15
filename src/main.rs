use smart_pointers::*;
use r#box;
use deref;
use r#drop;

fn main() {
    topic::introduce();
    r#box::explain();
    deref::explain();
    r#drop::explain();
}