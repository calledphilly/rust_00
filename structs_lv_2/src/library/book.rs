use std::{fmt};


pub struct Book {
    pub title: String,
    pub pages: u32,
}

impl Book {
    pub fn new(title: &str, pages: u32) -> Self {
        Self {
            title: String::from(title),
            pages,
        }
    }
}
impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.pages <= 1 {
            write!(f, "Book{{title:\"{}\",page:{}}}", self.title, self.pages)
        } else {
            write!(f, "Book{{title:\"{}\",pages:{}}}", self.title, self.pages)
        }
    }
}
