
#[derive(Debug)]
pub struct Products {
    name: String,
    price: f64,
    quantity: u32,
}

impl Products {
    pub fn new(name: String, price: f64, quantity: u32) -> Products {
        Products {
            name,
            price,
            quantity,
        }
    }

    pub fn info(&self) {
        println!("{}: ${} x {}", self.name, self.price, self.quantity);
        println!("Super Manager: {}", super::MANAGER)
    }
}
