mod Company {

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Person {
        pub name: String,
        pub age: u8,
        pub mail: String,
    }

    impl Person {
        pub fn new(name: &str, age: u8, mail: &str) -> Self {
            Person {
                name: name.to_string(),
                age,
                mail: mail.to_string(),
            }
        }

        pub fn with_name(mut self, name: &str) -> Self {
            self.name = name.to_string();
            self
        }
    }

    impl Default for Person {
        fn default() -> Self {
            Person {
                name: "Unknown".to_string(),
                age: 0,
                mail: "Unknown".to_string(),
            }
        }
    }

    #[repr(C)] 
    struct CStruct { 
        a: i32, b: f64, c: i8, 
    }
}

mod Shapes {
    #[derive(Debug, Copy, Clone)]
    pub struct Rectangle(pub i32, pub i32);
}

fn person_demo() {
    use Company::Person;

    let mut p1 = Person::default(); 
    p1.name = "John".to_string();
    println!("{:?}", p1);

    let p2 = Person::new("Jane", 25, "jane@mail.com");
    println!("{:?}", p2);

    let p3 = Person::default().with_name("Alice");
    println!("{:?}", p3);
}

fn shapes_demo() {
    use Shapes::Rectangle;

    let rect_1 = Shapes::Rectangle(10, 20);
    println!("{:?} - width: {}, height: {}", rect_1, rect_1.0, rect_1.1);

    let rect_2 = rect_1;
    println!("{:?} - width: {}, height: {}", rect_2, rect_2.0, rect_2.1);
    
    println!("{:?} - width: {}, height: {}", rect_1, rect_1.0, rect_1.1);
}

fn main() {
    person_demo();
}
