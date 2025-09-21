// #[derive(Debug, Clone)]
// pub enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use core::fmt;
// use std::rc::Rc;

// use List::{Cons, Nil};
// pub fn cons(head: i32, tail: Rc<List>) -> Rc<List> {
//     Rc::new(Cons(head, tail))
// }

// pub fn nil() -> Rc<List> {
//     Rc::new(Nil)
// }
// impl List {
//     pub fn push_front(self: &Rc<Self>, head: i32) -> Rc<Self> {
//         cons(head, Rc::clone(self))
//     }

//     pub fn head(self: &Rc<Self>) -> Result<i32, Error> {
//         match self.as_ref() {
//             Cons(head, _) => Ok(*head),
//             Nil => Err(Error::NotFoundHead("Not Found head cause List is Nil")),
//         }
//     }
//     pub fn tail(self: &Rc<Self>) -> Result<Rc<Self>, Error> {
//         match self.as_ref() {
//             Cons(_, tail) => Ok(Rc::clone(tail)),
//             Nil => Err(Error::NotFoundTail("Not Found tail cause List is Nil")),
//         }
//     }

//     pub fn shared_tail(self: &Rc<Self>, tail: &Rc<Self>) -> Rc<List> {
//         match self.as_ref() {
//             Cons(h, t) => cons(*h, t.shared_tail(tail)),
//             Nil => Rc::clone(&tail),
//         }
//     }

//     pub fn len(self: &Rc<Self>) -> i32 {
//         match self.as_ref() {
//             Cons(_, t) => 1 + t.len(),
//             Nil => 0,
//         }
//     }
// }

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Cons(head, tail) => write!(f, "({}, {})", head, tail),
//             Nil => write!(f, "Nil"),
//         }
//     }
// }

// // use thiserror::Error;

// #[derive(Debug, /* Error */)]
// pub enum Error {
//     // #[error("Not found tail: list is Nil")]
//     NotFoundTail(&'static str),
    
//     // #[error("Not found head: list is Nil")]
//     NotFoundHead(&'static str),
// }

// // impl fmt::Display for Error {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         match self {
// //             Error::NotFoundHead => write!(f,"Not Found head cause List is Nil"),
// //             Error::NotFoundTail => write!(f,"Not found tail cause List is Nil"),
// //         }
// //     }
// // }

// // impl std::error::Error for Error {}
