fn main() {
    let array = vec![(10,2),(5,0)];
    test(&array);
}
fn test(array: &Vec<(i32,i32)>) {
    for x in array {
        match safe_divide(x.0, x.1) {
            Ok(v) => println!("{v}"),
            Err(e) => eprintln!("{e}")
        }
    }
}
fn safe_divide(a: i32, b: i32) -> Result<i32, &'static str> {
    match b {
        0 => Err("Division by 0"),
        _ => Ok(a / b),
    }
}
