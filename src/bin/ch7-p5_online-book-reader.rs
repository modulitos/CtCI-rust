// Online Book Reader: Design the data structures for an online book
// reader system.

// Requirements:
// - User membership creation and extension
// - Searching the database of books
// - Reading a book
// - Only one active user at a time
// - Only one active book by this user

// Inspired by this implementation:
// https://github.com/careercup/CtCI-6th-Edition/tree/master/Java/Ch%2007.%20Object-Oriented%20Design/Q7_05_Online_Book_Reader

use std::collections::HashMap;
#[macro_use]
extern crate derive_more;

#[derive(Debug, Hash, PartialEq, Eq, AddAssign, Clone)]
struct BookId(usize);

#[derive(Clone)]
struct Book {
    id: BookId,
    title: String,
    author: String,
}

#[derive(Debug, Hash, PartialEq, Eq, AddAssign, Clone, Copy)]
struct UserId(usize);
#[derive(Clone)]
struct User {
    id: UserId,
    name: String,
}

struct LibraryManager {
    books: HashMap<BookId, Book>,
    user_libraries: HashMap<UserId, Vec<BookId>>,
}

impl LibraryManager {
    fn new() -> Self {
        let mut books = HashMap::new();
        books.insert(
            BookId(0),
            Book {
                id: BookId(0),
                title: String::from("Pride and Prejudice"),
                author: String::from("Jane Austen"),
            },
        );
        books.insert(
            BookId(1),
            Book {
                id: BookId(1),
                title: String::from("Free Will"),
                author: String::from("Sam Harris"),
            },
        );
        books.insert(
            BookId(2),
            Book {
                id: BookId(2),
                title: String::from("Oh the places you'll go"),
                author: String::from("Dr. Seuss"),
            },
        );
        books.insert(
            BookId(3),
            Book {
                id: BookId(3),
                title: String::from("Infinite Jest"),
                author: String::from("David Foster Wallace"),
            },
        );
        LibraryManager {
            books,
            user_libraries: HashMap::new(),
        }
    }

    fn add_book(&mut self, user_id: UserId, book_id: BookId) {
        match self.books.get(&book_id) {
            Some(_) => {
                if let Some(user_books) = self.user_libraries.get_mut(&user_id) {
                    user_books.push(book_id);
                } else {
                    self.user_libraries.insert(user_id, vec![book_id]);
                }
            }
            None => return, // invalid book_id
        }
    }

    fn get_book(&mut self, user_id: UserId, book_id: BookId) -> Option<Book> {
        // Check for valid book id
        if let Some(book) = self.books.get(&book_id) {
            // Check if the user has a book library:
            if let Some(user_books) = self.user_libraries.get(&user_id) {
                // Check if the book is part of the user's libarary:
                if user_books.contains(&book_id) {
                    return Some(book.clone());
                }
            }
        }
        None
    }
}

struct UserManager {
    users: HashMap<UserId, User>,
    user_names: HashMap<String, UserId>,
    user_id_seq: UserId,
}

impl UserManager {
    fn new() -> Self {
        UserManager {
            users: HashMap::new(),
            user_id_seq: UserId(0),
            user_names: HashMap::new(),
        }
    }
    fn new_user(&mut self, name: &str) -> Result<User, String> {
        if self.user_names.contains_key(name) {
            Err(String::from("User with that id already exists!"))
        } else {
            let id = self.user_id_seq;
            self.user_id_seq += UserId(1);
            self.user_names.insert(name.to_string(), id);
            let user = User {
                id,
                name: name.to_string(),
            };
            self.users.insert(id, user.clone());
            Ok(user)
        }
    }

    fn get_user(&mut self, name: &str) -> Result<&User, String> {
        if let Some(id) = self.user_names.get(name) {
            Ok(self.users.get(id).unwrap())
        } else {
            Err(String::from("User with that id already exists!"))
        }
    }
}

struct Display {
    current_page: Option<usize>,
    current_book: Option<Book>,
    current_user: Option<User>,
}

impl Display {
    fn new() -> Self {
        Display {
            current_page: None,
            current_book: None,
            current_user: None,
        }
    }

    fn update_user(&mut self, user: User) {
        self.current_user = Some(user);
    }
}

struct OnlineBookReader {
    library_manager: LibraryManager,
    user_manager: UserManager,
    display: Display,
}

impl OnlineBookReader {
    fn new() -> Self {
        OnlineBookReader {
            user_manager: UserManager::new(),
            library_manager: LibraryManager::new(),
            display: Display::new(),
        }
    }
    fn new_user(&mut self, name: &str) -> &Display {
        match self.user_manager.new_user(name) {
            Ok(user) => {
                self.display.update_user(user.clone());
                &self.display
            }
            Err(message) => panic!(message),
        }
    }

    fn login_user(&mut self, name: &str) -> &Display {
        match self.user_manager.get_user(name) {
            Ok(user) => {
                self.display.update_user(user.clone());
                &self.display
            }
            Err(message) => panic!(message),
        }
    }

    fn add_book(&mut self, book_id: BookId) {
        let user_id = self.display.current_user.as_ref().unwrap().id;
        self.library_manager.add_book(user_id, book_id);
    }

    fn open_book(&mut self, book_id: BookId) -> &Display {
        let user_id = self.display.current_user.as_ref().unwrap().id;
        self.display.current_book = self.library_manager.get_book(user_id, book_id);
        self.display.current_page = Some(0);
        &self.display
    }

    fn turn_page(&mut self) -> &Display {
        if let Some(current_page) = self.display.current_page {
            self.display.current_page = Some(current_page + 1);
            &self.display
        } else {
            panic!("can't turn page - no book is open!")
        }
    }
}

#[test]
fn test_users() {
    let mut reader = OnlineBookReader::new();
    let display = reader.new_user("Jane");
    assert_eq!(
        display.current_user.as_ref().unwrap().name,
        String::from("Jane")
    );
    assert_eq!(display.current_user.as_ref().unwrap().id, UserId(0));

    let display = reader.new_user("John");
    assert_eq!(
        display.current_user.as_ref().unwrap().name,
        String::from("John")
    );
    assert_eq!(display.current_user.as_ref().unwrap().id, UserId(1));
    reader.login_user("Jane");
}

#[test]
fn test_books() {
    let mut reader = OnlineBookReader::new();
    reader.new_user("Jane");
    reader.add_book(BookId(1));
    assert_eq!(
        reader
            .open_book(BookId(1))
            .current_book
            .as_ref()
            .unwrap()
            .title,
        String::from("Free Will")
    );
}

#[test]
fn test_page_turning() {
    let mut reader = OnlineBookReader::new();
    reader.new_user("Jane");
    reader.add_book(BookId(1));
    reader.open_book(BookId(1));
    reader.turn_page();
    reader.turn_page();
    assert_eq!(reader.turn_page().current_page, Some(3));
}
