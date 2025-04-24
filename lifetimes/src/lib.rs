#[derive(Debug, Clone)]  // Derive the Clone trait for Person
pub struct Person {
    pub name: String,  // Change `name` to be a `String` instead of `&str`
    pub age: u8,
}

impl Person {
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),  // Convert `&str` to `String`
            age: 0,
        }
    }
}
