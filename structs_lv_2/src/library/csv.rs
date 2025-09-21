use std::{
    fs,
    io::{self, Write},
};

use crate::library::media_item::MediaItem;

pub fn collect_mediaitem_from_csv(path: &str) -> io::Result<Vec<MediaItem>> {
    let mut vec: Vec<MediaItem> = Vec::new();
    let content = fs::read_to_string(path)?;
    for line in content.lines().skip(1) {
        if line.contains("book") {
            let mut line = line.to_string();
            // collect title
            let mut colon_offset = line.find(',').unwrap();
            let title = line.drain(..colon_offset).collect::<String>();
            let title = title.as_str();
            line.remove(0);
            // collect pages
            colon_offset = line.find(',').unwrap();
            let pages = line.drain(..colon_offset).collect::<String>();
            let pages = pages.parse::<u32>().unwrap();
            line.remove(0);
            // append book to vec
            let book = MediaItem::new_book(title, pages);
            vec.push(book);
        } else if line.contains("cd") {
            let mut line = line.to_string();
            // collect title
            let mut colon_offset = line.find(',').unwrap();
            let title = line.drain(..colon_offset).collect::<String>();
            let title = title.as_str();
            line.remove(0);
            // collect quantities
            colon_offset = line.find(',').unwrap();
            let quantities = line.drain(..colon_offset).collect::<String>();
            let quantities = quantities.parse::<u32>().unwrap();
            line.remove(0);
            // append cd to vec
            let cd = MediaItem::new_cd(title, quantities);
            vec.push(cd);
        }
    }
    Ok(vec)
}

pub fn write_mediaitem_in_csv(f: &mut fs::File, media_item: &MediaItem) -> io::Result<()> {
    match media_item {
        MediaItem::Book(book) => writeln!(f, "{},{},book", book.title, book.pages)?,
        MediaItem::Cd(cd) => writeln!(f, "{},{},cd", cd.title, cd.quantities)?,
    };
    Ok(())
}

pub fn create_csv(path: &str, headers: &str) -> io::Result<fs::File> {
    let mut f = fs::File::create(path)?;
    writeln!(f, "{}", headers)?;
    Ok(f)
}
pub fn open_csv(path: &str) -> io::Result<fs::File> {
    let f = fs::File::options().append(true).create(true).open(path)?;
    Ok(f)
}

pub fn print_csv(path: &str) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    println!("{}", content);
    Ok(())
}
