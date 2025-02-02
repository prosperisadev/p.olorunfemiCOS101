use std::io;
use std::io::Read;

fn main() {

    let mut input1 = String::new();
    println!("Welcome to Globacom Ltd Database Access App \nInput your role from the below to access your authorized database
    1. Administrator
    2. Project Manager
    3. Employee
    4. Customer
    5. Vendor");
    io::stdin().read_line(&mut input1).expect("Not valid input");
    let role:String = input1.to_lowercase().trim().parse().expect("Not a valid string");
    
    
    if role == "administrator" {
     administrator()
    }

    else if role == "project manager" {
        project_manager()
    }

    else if role == "employee" {
        employee()
    }

    else if role == "customer" {
        customer()
    }

    else if role == "vendor" {
        vendor()
    }

    else {
        println!("Unauthorized attempt to access, accessing our data illegally makes you liable to being sued by Globacom Ltd.");
    }
}


fn administrator() {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    println!("\nThank you for using the Globacom Lts Database\n Kindly sign out using CTRL + C when you're done to ensure data security.\n Globacom IT Team");
}

fn project_manager() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    println!("\nThank you for using the Globacom Lts Database\n Kindly sign out using CTRL + C when you're done to ensure data security.\n Globacom IT Team");
}

fn employee() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    println!("\nThank you for using the Globacom Lts Database\n Kindly sign out using CTRL + C when you're done to ensure data security.\n Globacom IT Team");
}

fn customer() {
    let mut file = std::fs::File::open("customer_table_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    println!("\nThank you for using the Globacom Lts Database\n Kindly sign out using CTRL + C when you're done to ensure data security.\n Globacom IT Team");
}

fn vendor() {
    let mut file = std::fs::File::open("dataplan_table_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    println!("\nThank you for using the Globacom Lts Database\n Kindly sign out using CTRL + C when you're done to ensure data security.\n Globacom IT Team");
}

