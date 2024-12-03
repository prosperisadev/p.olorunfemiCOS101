use std::io;
fn main() {
    println!("Welcome to Amala World, make your order and we'll let you know the price, \n note that for orders above #10,000 we are offering you a 5% discount
        p is Poundo Yam/Edinkaiko Soup = #3,200
        f is Fried Rice & Chicken = #3,000
        a is Amala & Ewedu Soup = #2,500
        e is Eba & Egusi Soup = #2,000
        w is white rice & stew = #2,500");

    //Receive Input on Type of food
    let mut input1 = String::new();
    println!("Select your preferred meal (Pack,F,A,E,W");
    io::stdin().read_line(&mut input1).expect("Not valid string");
    let selected_meal = input1.trim().to_uppercase();

    //Receive input on quantity
    let mut input2 = String::new();
    println!("How many portions would you like (1,2,3,4,5..");
    io::stdin().read_line(&mut input2).expect("Not valid string");
    let quantity:u32 = input2.trim().parse().expect("Not a valid number");

    let mut price:u32 = ;
    if selected_meal == "P" {
     price = 3200 * quantity;
    }

    else if selected_meal == "F" {
        let mut price:u32 = 3000 * quantity;
    }

    else if selected_meal == "A" {
        let mut price:u32 = 2500 * quantity;
    }

    else if selected_meal == "E" {
        let mut price:u32 = 2000 * quantity;
    }

    else if selected_meal == "W" {
       let price:u32 = 2500 * quantity;
    }


    let discount:u32 = price:u32 * 0.05;
    if price:u32 > 10000 {
        println!("Your total order is {}, for ordering above 10,000, you've won a 5% discount and your discounted price is {}", price, discount);
    }

    else if price:u32 < 10000 {
        println!("Your total order is {}, thanks for patronizing us", price); 
    }
}
