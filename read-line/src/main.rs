use std::io;
fn main() {
    let mut buffer = String::new();
    println!("Enter your name");
    println!("Name:");
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error occured during lecture of input of user");

    if let Some('\n') = buffer.chars().next_back() {
        buffer.pop();
    }
    println!("Hello '{buffer}'!");
}
