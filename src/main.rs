
#[derive(Debug)]
struct Book {
    id: u32,        // Unique identifier for the book
    title: u32,     // Title of the book
    author: u32,    // Author of the book
    is_available: bool, // Availability status of the book
}

impl Book {
    fn new(id: u32, title: u32, author: u32) -> Book {
        todo!()
    }

    fn borrow(&mut self) {
        todo!()
    }

    fn return_book(&mut self) {
        todo!()
    }
}

struct Library {
    books: Vec<Book>, // Collection of books in the library
}

impl Library {
    fn new() -> Library {
        todo!()
    }

    fn add_book(&mut self, book: Book) {
        todo!()
    }

    fn list_books(&self) {
        todo!()
    }

    fn find_book_index(&self, id: u32) -> i32 {
        todo!()
    }

    fn borrow_book(&mut self, id: u32) {
        todo!()
    }

    fn return_book(&mut self, id: u32) {
        todo!()
    }
}

fn main() {
    let mut library = Library::new();

    // Adding books to the library
    library.add_book(Book::new(1, 101, 201));
    library.add_book(Book::new(2, 102, 202));
    library.add_book(Book::new(3, 103, 203));

    println!("Library books:");
    library.list_books();

    println!("\nBorrowing book with ID 2:");
    library.borrow_book(2);

    println!("\nLibrary books after borrowing:");
    library.list_books();

    println!("\nReturning book with ID 2:");
    library.return_book(2);

    println!("\nLibrary books after returning:");
    library.list_books();
}
