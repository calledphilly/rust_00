use core::fmt;


pub struct Cd {
    pub title: String,
    pub quantities: u32,
}

impl Cd {
    pub fn new(title: &str, quantities: u32) -> Self {
        Self {
            title: title.to_string(),
            quantities,
        }
    }
}

impl fmt::Display for Cd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.quantities <= 1 {
            write!(
                f,
                "Cd{{title:\"{}\",quantity:{}}}",
                self.title, self.quantities
            )
        } else {
            write!(
                f,
                "Cd{{title:\"{}\",quantities:{}}}",
                self.title, self.quantities
            )
        }
    }
}
