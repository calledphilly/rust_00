fn main() {
    let mut value = String::new();
    let a = String::from("Like a");
    {
        let b = String::from(" jambun");
        return_reference(&a, &b, &mut value);
    }
    let str1 = "je suis".to_owned();
    let str2 = "un jambon";
    let str3 = str1.clone() + " " + str2;
    let str4 = &str3;
    let str5 = str1.clone() + " " + str4;
    println!("'{value}' '{str3}' '{str5}' !");
}

fn return_reference<'a>(
    first: &'a String,
    second: &String,
    value: &'a mut String,
) -> &'a mut String {
    *value = format!("{}{}", first, second);
    value
}
