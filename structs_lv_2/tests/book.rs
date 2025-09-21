use structs_lv_2::library::book::Book;

#[test]
fn test_new() {
    let book = Book::new("test", 4);
    assert_eq!(book.title, "test");
    assert_eq!(book.pages, 4);
}

#[test]
fn test_display() {
    let mut book = Book::new("test", 23);
    let mut result = format!("{}", book);
    assert_eq!(result, "Book{title:\"test\",pages:23}");

    book = Book::new("test", 11);
    result = format!("{}", book);
    assert_eq!(result, "Book{title:\"test\",pages:11}");

    book = Book::new("test", 1);
    result = format!("{}", book);
    assert_eq!(result, "Book{title:\"test\",page:1}");

    book = Book::new("test", 0);
    result = format!("{}", book);
    assert_eq!(result, "Book{title:\"test\",page:0}");
}
