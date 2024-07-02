# Library Management System in Rust

## Objective
Create a simple library management system using Rust that involves adding books, listing books, borrowing books, and returning books.

## Requirements
1. **Book Struct**:
   - Define a `Book` struct with the following fields:
     - `id`: `u32`
     - `title`: `u32` (using numeric IDs for simplicity)
     - `author`: `u32` (using numeric IDs for simplicity)
     - `is_available`: `bool`

2. **Library Struct**:
   - Define a `Library` struct with the following field:
     - `books`: `Vec<Book>`

3. **Book Methods**:
   - Implement methods for the `Book` struct:
     - `new(id: u32, title: u32, author: u32) -> Book`: Creates a new book.
     - `borrow(&mut self)`: Marks the book as borrowed if it is available.
     - `return_book(&mut self)`: Marks the book as returned if it is not available.

4. **Library Methods**:
   - Implement methods for the `Library` struct:
     - `new() -> Library`: Creates a new, empty library.
     - `add_book(&mut self, book: Book)`: Adds a new book to the library.
     - `list_books(&self)`: Lists all books in the library with their availability status.
     - `find_book_index(&self, id: u32) -> i32`: Finds the index of a book by its ID. Returns -1 if not found.
     - `borrow_book(&mut self, id: u32)`: Borrows a book by its ID if available.
     - `return_book(&mut self, id: u32)`: Returns a book by its ID if it is currently borrowed.

## Instructions
1. Clone this repository to your local machine.
2. Implement the required functions in `main.rs`.
3. Use the `main` function to interact with the user and demonstrate the functionality of your program.

## Hint (Optional)
When creating a new book or modifying its fields, avoid assigning any field with a value of 0. This is because using numeric IDs for simplicity may lead to confusion or errors if 0 is used as a valid value for any field. It's recommended to use non-zero values for all fields to ensure consistency and avoid potential issues in your library management system.
