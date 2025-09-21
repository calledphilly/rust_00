fn main() {
    struct Person {
        name: String,
        age: i8,
    }
    impl Person {
        fn new(name: &str, age:i8) -> Self {
            Self {name: name.to_string(), age}
        }
        fn display (&self) -> () {
            println!("L'utilisateur s'appelle {} et a {} ans.", self.name, self.age);
        }
        fn is_major(&self) -> bool {
            let value = match self.age {
                age if age >= 18 => true,
                age if age < 18 => false,
                _ => panic!("Invalid age")
            };
            value
        }
    }
    let person = Person::new("John", 18);
    person.display();
    if person.is_major() {
        println!("{} est majeur.",person.name)
    } else {
        println!("{} n'est pas majeur.",person.name)
    }

}
