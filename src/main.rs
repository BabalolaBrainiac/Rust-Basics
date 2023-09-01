// Enums
//Example 01
struct Product {
    name: String,
    price: f32,
    in_stock: bool,
    category: ProductCategory,
}


// Example 02
enum ProductCategory {
    Clothing,
    Electronics,
    Books,
}

enum Commands {
    AddText(String),
    Undo,
    Replace { from: String, to: String },
    moveCursor(i32, i32),
}

impl Commands {

    fn serialize_enum($self) -> String {
        String::from("Returning serialized json");
    }
}

fn main() {
    let cmd = Commands::AddText(String::from("Testing"));
    let cmd = Commands::moveCursor(332, 333);
    let cmd = Commands::Replace {
        from: String::from("Welcome"),
        to: String::from("Goodbye"),
    };

    let category = ProductCategory::Electronics;

    let newProduct = Product {
        name: String::from("Book"),
        price: 300.0,
        in_stock: true,
        category,
    };
}
