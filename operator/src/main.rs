use operator::utils::process_user_data;

fn main() {
    let vec1 = vec!["yannis", "toto"];
    run(&vec1);
}

fn run(vec: &Vec<&'static str>) {
    for el in vec.iter() {
        match process_user_data(el) {
            Ok(data) => println!("{data}"),
            Err(e) => eprintln!("❌ {e}"),
        }
    }
}
