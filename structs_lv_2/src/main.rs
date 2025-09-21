#![feature(box_patterns)]
pub mod library;
use structs_lv_2::library::list::{cons, nil, List::Cons};

fn main() {
    let list = Cons(1, cons(2, cons(3, nil())));
    let vec = list.to_vec();
    println!("{:?}", vec);
    println!("{}", list);
}
