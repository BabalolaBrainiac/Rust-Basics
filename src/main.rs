// Enums

struct Product {
    name: String,
    price: f32,
    in_stock: bool,
    category: ProductCategory,
}

enum ProductCategory {
    Clothing,
    Electronics,
    Books,
}

fn main() {
    let category = ProductCategory::Electronics;

    let newProduct = Product {
        name: String::from("Book"),
        price: 300.0,
        in_stock: true,
        category,
    };
}
