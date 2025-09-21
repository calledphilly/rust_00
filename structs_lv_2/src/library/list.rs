use std::{fmt};
use List::{Cons, Nil};

#[derive(Clone)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn cons(head: i32, box_tail: Box<List>) -> Box<List> {
    Box::new(Cons(head, box_tail))
}
pub fn nil() -> Box<List> {
    Box::new(Nil)
}
impl List {
    // pub fn len(&self, mut len: i32 ) -> i32 {
    //     println!("Begging len: {}",len);
    //     if len == 0 {
    //         len = 1;
    //         println!("As begging equal 0 len equal now: {}",len);
    //     }
    //     match self {
    //         Self::Cons(_, tail) => {
    //             len = len + 1;
    //             println!("Now len + 1: {}",len);
    //             tail.len(len)
    //         },
    //         Self::Nil => {
    //             println!("At the end len: {}",len);
    //             len = len - 1;
    //             len
    //         },
    //     }
    // }
    pub fn len(&self) -> i32 {
        match self {
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    pub fn push_front(self, x: i32) -> Self {
        Cons(x, Box::new(self))
    }
    pub fn push_back(self, x: i32) -> Self {
        match self {
            Cons(head, tail) => Cons(head, Box::new(tail.push_back(x))),
            Nil => Cons(x, Box::new(Nil)),
        }
    }
    pub fn pop_back(self) -> Self {
        match self {
            Nil => Nil,
            // Cons(_, tail) if matches!(&*tail,Nil) => Nil,
            Cons(_, box Nil) => Nil,
            Cons(head, tail) => Cons(head, Box::new(tail.pop_back())),
        }
    }
    pub fn pop_front(self) -> Self {
        match self {
            Cons(_, tail) => *tail,
            Nil => Nil,
        }
    }
    pub fn from_integer(int: i32) -> Self {
        Cons(int, Box::new(Nil))
    }
    pub fn from_vec(mut vec: Vec<i32>) -> Self {
        if vec.is_empty() {
            return Nil;
        }
        vec.reverse();
        let mut list = Self::from_integer(vec[0]);
        for item in vec.iter().skip(1) {
            list = list.push_front(*item);
        }
        list
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut vec = Vec::new();
        let list = self.clone();
        list.collect_in_vec(&mut vec);
        vec
    }
    pub fn collect_in_vec(self, vec: &mut Vec<i32>) {
        match self {
            Nil => (),
            Cons(head, box tail) => {
                vec.push(head);
                tail.collect_in_vec(vec);
            },
        };
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cons(head, tail) => write!(f, "({},{})", head, tail),
            Nil => write!(f, "Nil"),
        }
    }
}
