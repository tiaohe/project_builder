#[cfg(test)]
mod tests {
    use crate::{Company, Person};

    #[test]
    fn test_person_new_vs_builder() {
        let person_new = Person::new("Alice".to_string(), 30);
        let person_builder = Person::builder()
            .name("Alice".to_string())
            .age(30)
            .build()
            .unwrap();

        assert_eq!(person_new, person_builder, "Person created with new should be equal to person created with builder");
    }

    #[test]
    fn test_company_new_vs_builder() {
        let founder = Person::new("Alice".to_string(), 30);
        let company_new = Company::new(
            "Tech Corp".to_string(),
            2000,
            "Bob".to_string(),
            100,
            founder.clone(),
        );

        let company_builder = Company::builder()
            .name("Tech Corp".to_string())
            .founded(2000)
            .ceo("Bob".to_string())
            .employees(100)
            .founder(founder)
            .build()
            .unwrap();

        assert_eq!(company_new, company_builder, "Company created with new should be equal to company created with builder");
    }

    use aspect_macro::elapsed;
    #[elapsed("Pre")]
    #[test]
    fn test_function_pre() {
        println!("Inside test function pre");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    #[elapsed("Post")]
    #[test]
    fn test_function_post() {
        println!("Inside test function post");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    #[elapsed("Around")]
    #[test]
    fn test_function_around() {
        println!("Inside test function around");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    #[elapsed("All")]
    #[test]
    fn test_function_all() {
        println!("Inside test function all");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }




}