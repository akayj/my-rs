use std::collections::HashMap;

mod raii;
mod refer;

pub use raii::*;
pub use refer::*;

pub fn new_hashmap() {
    // Creating a new Hash Map.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    // For types that implement the *Copy* trait, like *i32*,
    // the value are copied into the hash map.
    // For owned values like *String*, the values will be moved and the
    // hash map will be the owner of those values.
    map.insert(field_name, field_value);

    println!("map is {:?}", map);

    //     println!("field_name: {}", field_name);
    //     println!("field_value: {}", field_value);
}
