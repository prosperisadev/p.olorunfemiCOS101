//Create Struct (Fundamental detail Organizer)for laptop detaisl
struct Laptops{
    brand:String,
    cost:u32, 
    _quantity_available:u32,
    quantity_requested:u32
}

//Create logic & function for calculating cost of laptop depending on quantity
impl Laptops {
fn laptop_price(&self)->u32 {
    self.cost * self.quantity_requested
}
}

fn main() {
    //Create the structure for each laptop brand
    let laptop_1 = Laptops{
        brand:String::from("HP"),
        cost:650_000, 
        _quantity_available:10,
        quantity_requested:3
    };

    let laptop_2 = Laptops{
        brand:String::from("IBM"),
        cost:755_000, 
        _quantity_available:6,
        quantity_requested:3
    };

    let laptop_3 = Laptops{
        brand:String::from("Toshiba"),
        cost:550_000, 
        _quantity_available:10,
        quantity_requested:3
    };

    let laptop_4 = Laptops{
        brand:String::from("Dell"),
        cost:850_000, 
        _quantity_available:4,
        quantity_requested:3
    };

    //Create logic for getting total prices of all laptops by calling function of laptop prices
    let total_laptop_price = laptop_1.laptop_price() + laptop_2.laptop_price() + laptop_3.laptop_price() + laptop_4.laptop_price();
    
    //Print Total laptop prices leveraging structs and functions
    println!("Total cost of purchasing 3 of {}, {}, {}, and {} is {} naira", laptop_1.brand, laptop_2.brand, laptop_3.brand, laptop_4.brand, total_laptop_price);
}
