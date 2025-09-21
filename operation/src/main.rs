fn main() {
    enum Operation {
        Add(i32, i32),
        Subtract(i32, i32),
        Multiply(i32, i32),
        Divide(i32, i32)
    }
    impl Operation {
        fn run(&self) -> Result<i32, String> {
            match self {
                Self::Add(x, y) => Ok(x + y),
                Self::Subtract(x, y) => Ok(x - y),
                Self::Multiply(x, y) => Ok(x * y),
                Self::Divide(_, 0) => Err("Division by zero".to_string()),
                Self::Divide(x, y) => Ok(x / y),
            }
        }
    }
    let op1 = Operation::Add(2,2);
    match op1.run() {
        Ok(v) => println!("{v}"),
        Err(v) => eprint!("{v}")
    };
    let op2 = Operation::Divide(2,2);
    match op2.run() {
        Ok(v) => println!("{v}"),
        Err(v) => eprint!("{v}")
    };
    let op2_1 = Operation::Divide(2,0);
    match op2_1.run() {
        Ok(v) => println!("{v}"),
        Err(v) => eprintln!("{v}")
    };
    let op3 = Operation::Multiply(2,2);
    match op3.run() {
        Ok(v) => println!("{v}"),
        Err(v) => eprintln!("{v}")
    };
    let op4 = Operation::Subtract(2,2);
    match op4.run() {
        Ok(v) => println!("{v}"),
        Err(v) => eprint!("{v}")
    };
}