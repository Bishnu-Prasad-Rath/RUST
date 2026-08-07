#[derive(Debug)]
enum Category {
    Electronics,
    Clothing,
    Books,
}

#[derive(Debug)]
enum Status {
    InStock(u32),
    OutOfStock,
    Discontinued,
}

struct Product {
    id: u32,
    name: String,
    price: f64,
    category: Category,
    status: Status,
}

impl Product {
    fn new(id: u32, name: String, price: f64, category: Category, quantity: u32) -> Product {
        let status = if quantity > 0 {
            Status::InStock(quantity)
        } else {
            Status::OutOfStock
        };

        Product {
            id,
            name,
            price,
            category,
            status,
        }
    }

    fn calculate_tax(&self) -> f64 {
        let rate = match self.category {
            Category::Electronics => 0.15,
            Category::Clothing => 0.05,
            Category::Books => 0.00,
        };

        self.price * rate
    }

    fn update_stock(&mut self, new_qty: u32) {
        if new_qty > 0 {
            self.status = Status::InStock(new_qty);
        } else {
            self.status = Status::OutOfStock;
        }
    }
}

// Function 1: Print all items borrowed as a slice (&[Product])
fn print_all_products(products: &[Product]) {
    for product in products {
        print!("ID: {}, Name: {}, Price: ${:.2}", product.id, product.name, product.price);

        // Pattern match on the status using 'if let'
        if let Status::InStock(qty) = product.status {
            println!(" | Status: In Stock ({})", qty);
        } else {
            println!(" | Status: {:?}", product.status);
        }
    }
}

// Function 2: Search product using string slice (&str) and slice (&[Product])
fn find_by_name<'a>(products: &'a [Product], name_query: &str) -> Option<&'a Product> {
    for product in products {
        if product.name == name_query {
            return Some(product); // Returns a reference to the found product
        }
    }
    None // Return None if not found
}

fn main() {
    // Vector storing products
    let mut inventory: Vec<Product> = Vec::new();

    inventory.push(Product::new(1, String::from("Wireless Mouse"), 25.0, Category::Electronics, 10));
    inventory.push(Product::new(2, String::from("Rust Programming Book"), 45.0, Category::Books, 0));

    println!("--- ALL PRODUCTS ---");
    // Passing &inventory converts Vec<Product> into a slice &[Product] automatically!
    print_all_products(&inventory);

    println!("\n--- SEARCH RESULT ---");
    let query = "Wireless Mouse";
    
    // Using 'if let' to handle the Option returned by find_by_name
    if let Some(found) = find_by_name(&inventory, query) {
        println!("Found item: {} with tax: ${:.2}", found.name, found.calculate_tax());
    } else {
        println!("Product '{}' not found.", query);
    }
} 