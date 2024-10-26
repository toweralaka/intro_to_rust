use std::fmt;

// the possible states of the book
enum Status {
    Available,
    CheckedOut
}

// implement human-readable format of each variant of status
// implement the fmt::Display trait to make the Status enum
// printable in a human-readable format.
// When printing, Rust automatically calls this implementation
// to display the string defined in each match arm.
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::Available => write!(f, "available"),
            Status::CheckedOut => write!(f, "checked out"),
        }
    }
}

struct Book {
    title: String,
    author: String,
    status: Status
}

impl Book {
    fn new(name: &str, author: &str)->Book{
        let the_book = Book {
            title: String::from(name),
            author: String::from(author),
            status: Status::Available
        };
        return the_book
    }
    fn check_out(&mut self) {
        self.status = Status::CheckedOut;
    }
    fn return_book(&mut self) {
        self.status = Status::Available;
    }
    fn print_info(&self) {
        println!("The book title {} by {} is {}", self.title, self.author, self.status);
    }
}

fn main() {
    // Create a new book and print its info
    let mut my_book = Book::new("The Rust Book", "Jane Doe");
    my_book.print_info();

    // Check out the book and print its status
    my_book.check_out();
    my_book.print_info();

    // Return the book and print its status
    my_book.return_book();
    my_book.print_info();
}
