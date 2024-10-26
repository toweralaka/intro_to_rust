use std::fmt;

enum Status {
    Available,
    CheckedOut
}

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
    status: Status,
}

impl Book {
    fn new(title: &str, author: &str) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            status: Status::Available,
        }
    }

    fn check_out(&mut self) {
        self.status = Status::CheckedOut;
    }

    fn return_book(&mut self) {
        self.status = Status::Available;
    }

    fn print_info(&self) {
        println!("{} by {} is currently {}", self.title, self.author, self.status);
    }
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    // Create a new, empty library
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    // Add a book to the library
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    // List all books in the library
    fn list_books(&self) {
        for book in &self.books {
            book.print_info();
        }
    }
    fn remove_book(&self, title:&str){

    }
    // Find a book in the library by author
    fn find_book_by_author(&self, author: &str) {
        let mut author_books = Vec::new();  // Create a vector to store books by the author
        
        // Loop through all books in the library
        for book in &self.books {
            // If the book's author matches the specified author, add it to the vector
            if book.author == author {
                author_books.push(book);
            }
        }
        
        // Check if the vector has books by the author
        if author_books.len() > 0 {
            // Print the books if found
            println!("These are the books by {}:", &author);
            for au_book in &author_books {
                au_book.print_info();
            }
        } else {
            // If no books are found, display a message
            println!("There are no books by {}.", &author);
        }
    }
    // fn find_book_by_author(&self, author: &str) {
    //     let mut author_books = Vec::new();  // Vector to store books by the author
        
    //     // Collect all books by the author
    //     for book in &self.books {
    //         if book.author == author {
    //             author_books.push(book);
    //         }
    //     }
        
    //     // Check if the author has books in the library
    // use is_empty() for efficiency
    //     if !author_books.is_empty() {
    //         println!("These are the books by {}:", author);
    // use .iter() for perfomance
    //         for au_book in author_books.iter() {
    //             au_book.print_info();
    //         }
    //     } else {
    //         println!("There are no books by {}.", author);
    //     }
    // }

    // Find a book in the library
    fn find_book(&self, title:&str){
        for book in &self.books {
            if book.title == title{
                book.print_info();
                return;  // Exit early if the book is found
            }
        }
        println!("Book not found");
    }
//     fn find_book(&self, title: &str) {
//     let mut found = false;  // Track if the book is found
//     for book in &self.books {
//         // Check if the title matches (compare strings)
//         if book.title == String::from(title) {
//             book.print_info();  // Print book details if found
//             found = true;       // Mark the book as found
//             break;              // Exit the loop early since we found the book
//         }
//     }
//     if found == false {
//         println!("Book not found");  // Message if the book is not in the library
//     }
// }

}

fn main() {
    let mut my_library = Library::new();

    let book1 = Book::new("1984", "George Orwell");
    let book2 = Book::new("The Rust Programming Language", "Steve Klabnik");

    my_library.add_book(book1);
    my_library.add_book(book2);

    my_library.list_books();
    my_library.find_book("1984");
    my_library.find_book_by_author("George Orwell");
    my_library.find_book_by_author("Georgej Orwell");
}
