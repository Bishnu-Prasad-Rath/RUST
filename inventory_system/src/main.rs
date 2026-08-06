use std::num::FpCategory;

use crate::{Category::Electronics, Status::{InStock, OutOfStock}};

enum Category {
    Electronics,
    Clothing,
    Books,
}

enum Status{
    InStock(u32),
    OutOfStock,
    Discontinued,
}

struct Product{
    id : u32,
    name : String,
    price : f64,
    category : Category,
    status : Status,
}

impl Product{
    fn new(id : u32,name : String, price : f64, category : Category, quantity :u32) -> Product{
        if (quantity > 0) {
            Product { id : id, name : name, price : price, category : category, status:Status::InStock(quantity) }
        }else{
            Product { id : id, name : name, price : price, category : category, status:Status::OutOfStock }
        }
    }

    fn calculate_tax(&mut self) -> f64{


        let rate  =  if let Category::Electronics = self.category{
              0.15
         }else if let Category::Clothing = self.category{
               0.05
         }else{
               0.01
         };

         let price = self.price * rate;
         price

    }

    fn update_stock(&mut self,new_qty : u32){
        if new_qty > 0{
            self.status = Status::InStock(new_qty);
        }else if new_qty == 0 {
            self.status = Status::OutOfStock;
        }
    }
}



fn main() {
    println!("Phase-1 is completed successfully.");
}
