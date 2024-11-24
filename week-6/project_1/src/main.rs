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
    println!("Select your preferred meal (p,f,a,e,w, p&f or p&f&w...");
    io::stdin().read_line(&mut input1).expect("Not valid string");
    let binding = input1.to_string();
    let selected_meal:&str = binding.trim();

    //Process to define what meal and provide output
    if selected_meal == "p" {
        println!("Your selected meal is Poundo Yam & Edinkaiko Soup and costs #3,200");
    }

    else if selected_meal == "f" {
        println!("Your selected meal is Fried rice & chicken and costs #3,000");
    }

    else if selected_meal == "a" {
        println!("Your selected meal is Amala & Ewedu Soup and costs #2,500");
    }

    else if selected_meal == "e" {
        println!("Your selected meal is Eba & Egusi Soup and costs #2,000");
    }

    else if selected_meal == "w" {
        println!("Your selected meal is White Rice & stew and costs #2,500");
    }

    else if selected_meal == "p&f" {
        println!("Your selected meal is Poundo Yam pack  & Fried Rice packand costs #6,200");
    }

    else if selected_meal == "p&a" {
        println!("Your selected meal is Poundo Yam pack  & Amala Pack and costs #5,700");
    }

    else if selected_meal == "p&e" {
        println!("Your selected meal is Poundo Yam pack  & Eba Pack and costs #5,200");
    }

    else if selected_meal == "p&w" {
        println!("Your selected meal is Poundo Yam pack  & White Rice Pack and costs #5,700");
    }

    else if selected_meal == "f&a" {
        println!("Your selected meal is Fried Rice pack  & Amala Pack and costs #5,500");
    }

    else if selected_meal == "f&e" {
        println!("Your selected meal is Fried Rice pack  & Eba Pack and costs #5,200");
    }

    else if selected_meal == "f&w" {
        println!("Your selected meal is Fried Rice pack  & White Rice Pack and costs #5,500");
    }

    else if selected_meal == "a&e" {
        println!("Your selected meal is Amala pack  & Eba Pack and costs #4,500");
    }

    else if selected_meal == "a&w" {
        println!("Your selected meal is Amala pack  & White Rice Pack and costs #5,000");
    }

    else if selected_meal == "p&f&a" {
        println!("Your selected meal is Poundo Yam pack, Fried Rice Pack & Amala Pack and costs #8,700");
    }

     else if selected_meal == "p&f&e" {
        println!("Your selected meal is Poundo Yam pack, Fried Rice Pack & Eba Pack and costs #8,200");
    }

     else if selected_meal == "p&f&w" {
        println!("Your selected meal is Poundo Yam pack, Fried Rice Pack & White Rice Pack and costs #8,700");
    }

     else if selected_meal == "f&a&e" {
        println!("Your selected meal is Fried Rice pack, Amala Pack & Eba Pack and costs #8,500");
    }

    else if selected_meal == "f&a&w" {
        println!("Your selected meal is Fried Rice pack, Amala Pack & Eba Pack and costs #8,500");
    }
}
