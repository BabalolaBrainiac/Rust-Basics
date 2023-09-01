// Enums
//Example 01
// struct Product {
//     name: String,
//     price: f32,
//     in_stock: bool,
//     category: ProductCategory,
// }
//

//Example 02
// enum ProductCategory {
//     Clothing,
//     Electronics,
//     Books,
// }

enum Commands {
    AddText,
    Undo,
    Replace {
        from: String,
        to: String,
    },
    moveCursor ( i32, i32)
}

fn main() {




    //Example 01
    // let category = ProductCategory::Electronics;
    //
    // let newProduct = Product {
    //     name: String::from("Book"),
    //     price: 300.0,
    //     in_stock: true,
    //     category,
    // };
}
