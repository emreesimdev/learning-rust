#![allow(dead_code)]

// Custom data structure for our books
struct Book {
    title: String,
    author: String,
    year: i32,
}

use std::io;

fn main() {
    // Dynamic lists (Vector) to store books in the Heap
    let mut library:  Vec<Book> = Vec::new();

    // Pre-filling the library with some data
    let book1 = Book {
        title: String::from("The Stranger"),
        author: String::from("Albert Camus"),
        year: 1942,
    };

    let book2 = Book {
        title: String::from("Crime and Punishment"),
        author: String::from("Fyodor Dostoyevski"),
        year: 1866,
    };

    let book3 = Book {
        title: String::from("Animal Farm"),
        author: String::from("George Orwell"),
        year: 1945,
    };

    let book4 = Book {
        title: String::from("Of Mice and Men"),
        author: String::from("John Steinbeck"),
        year: 1937,
    };

    let book5 = Book {
        title: String::from("The Lord of the Rings"),
        author: String::from("J. R. R. Tolkien"),
        year: 1954,
    };

    // Adding books to the vector
    library.push(book1);
    library.push(book2);
    library.push(book3);
    library.push(book4);
    library.push(book5);

    // Feature 1: Add new books
    println!("--- ADD A NEW BOOK ---"); 
    loop {
    println!("Enter the title of the book (or type 'q' to exit):");
    // Reading title
    let mut book_title_input = String::new();
    io::stdin()
        .read_line(&mut book_title_input)
        .expect("Failed to read line");
    
    // Exit condition
    if book_title_input.trim() == "q" {
        break;
    }

    // Reading author
    println!("Enter the author of the book");
    let mut book_author_input = String::new();
    io::stdin()
        .read_line(&mut book_author_input)
        .expect("Failed to read line");

    // Reading year
    println!("Enter the year of the book");
    let mut book_year_input = String::new();
    io::stdin()
        .read_line(&mut book_year_input)
        .expect("Failed to read line");

    // Parsing String to i32 with error handling
    let year_int: i32 = match book_year_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            continue; // Restart the loop if input is invalid
        }
    };

    // Creating and adding the new book
    let new_book = Book {
        title: book_title_input.trim().to_string(),
        author: book_author_input.trim().to_string(),
        year: year_int,
    };

    library.push(new_book);
    println!("Book added succesfully!");

    }

    // Feature 2: List all books
    println!("---MY LIBRARY---");

    // Iterating over the vector reference using & to keep ownership
    for book in &library {
        println!("Title: {}", book.title);
        println!("Author: {}", book.author);
        println!("Year: {}", book.year);
        println!("------------------------");
    }

    // Feature 3: Search by Author
    let search_author = "Albert Camus";
    println!("\nSearching for books by: {}", search_author);

    for book in &library {
        if book.author == search_author {
            println!("{}", book.title);
        }
    }

    // Feature 4: Remove a book
    println!("---REMOVE A BOOK---");
    loop {
    println!("Enter the number of the book to delete (or type 'e' to exit):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed_input = input.trim();

    // Exit check    
    if trimmed_input == "e" {
        break;
    }

    // Parsing to usize 
    let index_to_delete: usize = match trimmed_input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            continue;
        }
    };    

    // Safety check 
    if index_to_delete < library.len() {
        let removed_book = library.remove(index_to_delete);
        println!("Deleted: {}", removed_book.title);
        break; // Exit after succesful deletion
    } else {
        println!("Book not found at index {}! Try again.", index_to_delete);
    }
        
    } 
}