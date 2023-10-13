//question 1
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Hassan"),
        age: 21,
    };

    println!("Person's name: {}", person.name);
}
//question 2
enum Color {
    Red,
    Green,
    Blue,
}

fn get_rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Red => (255, 0, 0),
        Color::Green => (0, 255, 0),
        Color::Blue => (0, 0, 255),
    }
}

fn main() {
    let red_rgb = get_rgb(Color::Red);
    let green_rgb = get_rgb(Color::Green);
    let blue_rgb = get_rgb(Color::Blue);

    // Print the RGB values for Red, Green, and Blue
    println!("RGB for Red: {:?}", red_rgb);
    println!("RGB for Green: {:?}", green_rgb);
    println!("RGB for Blue: {:?}", blue_rgb);
}
//question 3
fn sum_tuple(tuple: (i32, i32)) -> i32 {
    let (first, second) = tuple;
    first + second
}

fn main() {
    let my_tuple = (5, 10);
    let result = sum_tuple(my_tuple);
    println!("Sum of the tuple elements: {}", result); // Output: Sum of the tuple elements: 15
}
// question 4
enum MyOption {
    StringValue(String),
    NumberValue(i32),
}

fn print_number(option: MyOption) {
    match option {
        MyOption::StringValue(_) => {
            println!("The provided value is not a number.");
        }
        MyOption::NumberValue(num) => {
            println!("The provided number is: {}", num);
        }
    }
}

fn main() {
    let my_string_option = MyOption::StringValue(String::from("Hello, Rust!"));
    let my_number_option = MyOption::NumberValue(42);

    print_number(my_string_option); // Output: The provided value is not a number.
    print_number(my_number_option); // Output: The provided number is: 42
}
//question 5
fn append_world(input_string: &mut String) {
    input_string.push_str(" World!");
}

fn main() {
    let mut my_string = String::from("Hello,");
    append_world(&mut my_string);
    println!("{}", my_string); // Output: Hello, World!
}
// question 6
struct Book {
    title: String,
}

impl Book {
    fn new(title: &str) -> Book {
        Book {
            title: String::from(title),
        }
    }

    fn get_title(&self) -> &String {
        &self.title
    }
}

fn main() {
    let book = Book::new("Rust Programming Guide");
    let title_reference = book.get_title();
    println!("Book Title: {}", title_reference);
}
// question 7
// Define the Book struct with fields title, author, and pages
struct Book {
    title: String,
    author: String,
    pages: u32,
}

// Define the Status enum with variants Active, Inactive, and Suspended
enum Status {
    Active,
    Inactive,
    Suspended,
}

// Function to create a tuple containing the book's title and its status
fn get_book_status(book: &Book, status: Status) -> (String, Status) {
    (book.title.clone(), status)
}

fn main() {
    // Create an instance of the Book struct
    let book = Book {
        title: String::from("Sample Book"),
        author: String::from("John Doe"),
        pages: 200,
    };

    // Print the book's title
    println!("Book Title: {}", book.title);

    // Use the get_book_status function and print the result
    let status = Status::Active;
    let (title, book_status) = get_book_status(&book, status);
    println!("Book Title: {}, Status: {:?}", title, book_status);
}
//question 8
enum Status {
    Active,
    Inactive,
    Suspended,
}

struct Book {
    title: String,
    author: String,
    pages: u32,
}

fn describe_status(status: &Status) -> &'static str {
    match status {
        Status::Active => "Active Status",
        Status::Inactive => "Inactive Status",
        Status::Suspended => "Suspended Status",
    }
}

fn describe_book_author(book: &Book) {
    println!("Author: {}", book.author);
}

fn main() {
    let my_status = Status::Active;
    let my_book = Book {
        title: String::from("Sample Book"),
        author: String::from("John Doe"),
        pages: 200,
    };

    println!("Status Description: {}", describe_status(&my_status));
    describe_book_author(&my_book);
}
// question 9
struct Book {
    title: String,
    author: String,
    pages: u32,
}

fn take_ownership(book: Book) -> String {
    book.title
}

fn main() {
    let my_book = Book {
        title: String::from("Sample Book"),
        author: String::from("John Doe"),
        pages: 200,
    };

    let book_title = take_ownership(my_book);
    println!("Title taken: {}", book_title); 

}
// question 10
// Define the Book struct and its fields as public
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
}

// Define the utils module with the display_book function
mod utils {
    use super::Book;

    pub fn display_book(book: &Book) {
        println!("Title: {}", book.title);
        println!("Author: {}", book.author);
        println!("Pages: {}", book.pages);
    }
}

fn main() {
    // Create a Book instance
    let my_book = Book {
        title: String::from("Sample Book"),
        author: String::from("John Doe"),
        pages: 200,
    };

    // Call the display_book function from the utils module to print the book's details
    utils::display_book(&my_book);
}
// question 11
