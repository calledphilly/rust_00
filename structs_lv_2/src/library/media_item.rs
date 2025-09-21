use std::fmt;

use crate::library::{book::Book, cd::Cd};

pub enum MediaItem {
    Book(Book),
    Cd(Cd),
}

impl MediaItem {
    pub fn new_book(title: &str, pages: u32) -> Self {
        Self::Book(Book::new(title, pages))
    }
    pub fn new_cd(title: &str, quantities: u32) -> Self {
        Self::Cd(Cd::new(title, quantities))
    }
}

impl fmt::Display for MediaItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Book(book) => write!(f, "{}", book),
            Self::Cd(cd) => write!(f, "{}", cd),
        }
    }
}

pub trait Printable {
    fn print(&mut self) {}
}

impl<I, T> Printable for T /* Vec<MediaItem> */
where
    T: Iterator<Item = I>,
    I: fmt::Display,
{
    fn print(&mut self) {
        println!("[");
        for x in self {
            println!("  {},", x)
        }
        println!("]");
    }
}

#[macro_export]
macro_rules! vec_mediaitem {
    [ $($title: expr => $kind: ident ($qty: expr)),* ] => {
        {
            let mut items: Vec<MediaItem> = Vec::new();
            $(
                items.push(match stringify!($kind){
                    "Book" => MediaItem::new_book($title,$qty),
                    "Cd" => MediaItem::new_cd($title,$qty),
                    _ => panic!("Kind not recognized.")
                });
            )*
            items
        }
    };
}
