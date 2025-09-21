fn main() {
    let mut a :u8 = 255;
    a = a.wrapping_add(1) ;
    println!("{}", a);
}
