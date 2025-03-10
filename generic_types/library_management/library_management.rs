#[allow(dead_code)]
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    status: BookStatus,
}

#[derive(Debug)]
enum BookStatus {
    Available,
    Borrowed,
}

#[derive(Debug)]
struct Library {
    book_list: Vec<Book>,
}

impl Book {
    fn new(title: &str, author: &str) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            status: BookStatus::Available, // Un livre est dispo par défaut
        }
    }
}

impl Library {
    fn new() -> Self {
        Self {
            book_list: Vec::new(),
        }
    }

    fn add_book(&mut self, book: Book) {
        self.book_list.push(book)
    }

    fn find_book(&mut self, title: &str) -> Option<&mut Book> {
        self.book_list.iter_mut().find(|book| book.title == title)
    }

    fn borrow_book(&mut self, title: &str) -> Result<(), String> {
        let book = self.find_book(title).ok_or("Livre introuvable.")?; // 💡 On utilise ok_or("message")? pour transformer l'Option en Result, ce qui évite le premier match.
        match book.status {
            BookStatus::Available => {
                book.status = BookStatus::Borrowed;
                Ok(())
            }
            BookStatus::Borrowed => Err(String::from("Le livre est déjà emprunté.")),
        }
    }

    fn return_book(&mut self, title: &str) -> Result<(), String> {
        let book = self.find_book(title).ok_or("Livre introuvable.")?;
        match book.status {
            BookStatus::Borrowed => {
                book.status = BookStatus::Available;
                Ok(())
            }
            BookStatus::Available => Err(String::from(
                "Vous ne pouvez pas rendre un livre qui n'était pas emprunté !",
            )),
        }
    }
}

fn main() {
    let mut library = Library::new();
    library.add_book(Book::new("Rust Programming", "Steve"));

    match library.borrow_book("Rust Programming") {
        Ok(_) => println!("Livre emprunté avec succès !"),
        Err(e) => println!("Erreur: {}", e),
    }

    // Vérifier l'état du livre après l'emprunt
    println!("{:?}", library.find_book("Rust Programming"));

    // Tenter d'emprunter un livre déjà emprunté
    match library.borrow_book("Rust Programming") {
        Ok(_) => println!("Livre emprunté avec succès !"),
        Err(e) => println!("Erreur: {}", e),
    }

    // Retourner le livre
    match library.return_book("Rust Programming") {
        Ok(_) => println!("Livre retourné avec succès !"),
        Err(e) => println!("Erreur: {}", e),
    }
}
