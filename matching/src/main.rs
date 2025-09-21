fn main() {
    handle_option(Some(42));
    handle_option(None);
    let vec = vec![
        vec![1, 23, 4],
        vec![11, 21, 17, 1],
        vec![2, 4, 17, 1],
        vec![8, 10, 1],
    ];
    for x in vec {
        match find_even(&x) {
            Some(v) => println!("{v}"),
            None => eprintln!("None"),
        }
    }
}
fn handle_option(v: Option<i32>) {
    match v {
        Some(v) => println!("Valeur reçue : {v}"),
        None => println!("Pas de valeur reçue"),
    }
}
fn find_even(vec: &Vec<i32>) -> Option<i32> {
    for x in vec.iter() {
        let x = *x;
        if x % 2 == 0 {
            return Some(x);
        }
    }
    None
}
