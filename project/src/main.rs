use project::Person;

fn main() {
    let person_new = Person::new("Alice".to_string(), 99);
    let person_builder = Person::builder()
        .name("Alice".to_string())
        .age(99)
        .build()
        .unwrap();

    println!("Person created with new: {:?}", person_new);
    println!("Person created with builder: {:?}", person_builder);
}