//Struct
fn main() {
    #[derive(Debug)]
    struct Book {
        title: String,
        author: String,
        year: u32,
    }
    #[derive(Debug)] // derive is a macro that allows us to print out the struct in a more readable format
    struct User {
        active: bool,
        sign_in_count: u64,
        name: String,
        email: String,
        age: u32,
    }
    let mut book1 = Book {
        title: "Shadow slave".to_string(),
        author: "Herman Melville".to_string(),
        year: 1928,
    };
    book1.year = 2020; // updating our book
    println!("{:#?}", book1);
    let book2 = Book {
        title: "John Wick".to_string(),
        ..book1 //  copies rest of the fields from book1
    };
    println!("{:#?}", book2);

    // a function that creates a user using User struct
    fn create_user(name: String, email: String, age: u32) -> User {
        // remember to add the return type as we are returning a User
        User {
            name,
            email,
            age,
            active: true,
            sign_in_count: 1,
        }
    }

    let user1 = create_user("John Wick".to_string(), "johnwick@m.com".to_string(), 20);

    println!("{:#?}", user1);

    // Tuple Struct
    // basically creating a struct without a name field
    // this is useful when we want to create a struct that has a fixed number of fields
    // and we don't want to give them a name
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);

    // Unit-like structs
    //Unit-like structs are a special kind of struct in Rust that don't have any fields. They're called "unit-like" because they're similar to the unit type () in Rust, which represents an empty value or void.
    #[derive(Debug)]
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    let subject2 = AlwaysEqual;
    println!("{:?}", subject);
}
