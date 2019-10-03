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

#[derive(Debug, Hash, PartialEq, Eq, AddAssign)]
struct BookId(usize);

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
    all_books: HashMap<BookId, Book>,
    user_libraries: HashMap<UserId, Vec<Book>>,
}

impl LibraryManager {
    fn new() -> Self {
        LibraryManager {
            all_books: HashMap::new(),
            user_libraries: HashMap::new(),
        }
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
    current_book: Option<BookId>,
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
    fn new_user(&mut self, name: &str) -> Result<&Display, String> {
        match self.user_manager.new_user(name) {
            Ok(user) => {
                self.display.update_user(user.clone());
                Ok(&self.display)
            }
            Err(message) => Err(message),
        }
    }

    fn login_user(&mut self, name: &str) -> Result<&Display, String> {
        match self.user_manager.get_user(name) {
            Ok(user) => {
                self.display.update_user(user.clone());
                Ok(&self.display)
            }
            Err(message) => Err(message),
        }
    }
}

#[test]
fn test_set_active_user() {
    let mut reader = OnlineBookReader::new();
    assert_eq!(
        reader
            .new_user("Jane")
            .unwrap()
            .current_user
            .as_ref()
            .unwrap()
            .name,
        String::from("Jane")
    );

    assert!(reader.login_user("Joe").is_err());
    assert_eq!(
        reader
            .new_user("John")
            .unwrap()
            .current_user
            .as_ref()
            .unwrap()
            .name,
        String::from("John")
    );
    assert!(reader.login_user("Jane").is_ok());
}
