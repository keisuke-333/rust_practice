use std::cell::Cell;

struct Person {
    name: String,
    age: Cell<u32>,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: Cell::new(30),
    };

    person.age.set(31);
    println!("Name:{}, Age:{}", person.name, person.age.get());
}