struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

impl Product {
    fn new(name: String, price: f32) -> Product {
        Product {
            name: (name),
            price: (price),
            in_stock: (false),
        }
    }

    fn get_default_sales_tax() -> f32 {
        0.1
    }

    fn calculate_sales_tax(&self) -> f32 {
        self.price * 0.1
    }

    fn set_price(&mut self, price: f32) {
        self.price = price
    }

    fn buy(self) -> i32 {
        let name = self.name;
        println!("{name} was bought");
        234
    }
}

fn main() {
    let mut book = Product::new(String::from("Book"), 25.0);

    // let price = book.price;
    book.in_stock = false;

    let sales_tax = book.calculate_sales_tax();

    book.set_price(25.0);
    book.buy();
    // book.set_price(23.3);

    println!("Sales tax = {}", sales_tax)
}
