use ::project::{Company, Person};

#[cfg(all(test, not(target_arch = "skip_tests")))]
mod tests {
    use crate::{Company, Person};

    #[rustfmt::skip]
    #[test]
    fn test_person_new_vs_builder() {
        let person_new = Person::new("Alice".to_string(), 30);
        let person_builder = Person::builder()
            .name("Alice".to_string())
            .age(30)
            .build()
            .unwrap();

        assert_eq!(
            person_new, person_builder,
            "Person created with new should be equal to person created with builder"
        );
    }

    #[test]
    fn test_company_new_vs_builder() {
        let founder = Person::new("Alice".to_string(), 30);
        let company_new =
            Company::new("Tech Corp".to_string(), 2000, "Bob".to_string(), 100, founder.clone());

        let company_builder = Company::builder()
            .name("Tech Corp".to_string())
            .founded(2000)
            .ceo("Bob".to_string())
            .employees(100)
            .founder(founder)
            .build()
            .unwrap();

        assert_eq!(
            company_new, company_builder,
            "Company created with new should be equal to company created with builder"
        );
    }
}
