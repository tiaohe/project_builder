use builder_macro::Builder;

#[derive(Debug, PartialEq, Builder, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

#[derive(Debug, PartialEq, Builder)]
pub struct Company {
    pub name: String,
    pub founded: u32,
    pub ceo: String,
    pub employees: usize,
    pub founder: Person,
}

impl Company {
    pub fn new(name: String, founded: u32, ceo: String, employees: usize, founder: Person) -> Self {
        Company {
            name,
            founded,
            ceo,
            employees,
            founder,
        }
    }
}
