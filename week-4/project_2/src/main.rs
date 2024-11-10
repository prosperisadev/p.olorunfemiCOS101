//Determine annual salary according to age and experiece

use std::io;

fn main() {

    println!("Welcome to Prosperity's Salary Evaluator, input your responses below to know how much you can earn");
    //Step 1: Get input of level of experience
    println!("Enter Number of years of experience");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a real string");
    let experience:i32 = input1.trim().parse().expect("Unable to read experience");

    //Step 2: Get input of age of employee
    println!("Enter Age");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a real string");
    let age:i32 = input2.trim().parse().expect("Not valid age");

    //Step 3: Determine level of experience and age

    if age >= 40 && experience >= 5 {
        println!("Experienced employees like you earn N1,560,000. Congratulations!");
    }

    else if age >= 30  && age < 40 && experience >= 5 {
        println!("Experienced employees like you earn N1,480,000. Congratulations!");
    }

    else if age < 28 && experience >= 5 {
        println!("Experienced employees like you earn N1,300,000. Congratulations! Hey you're young");
    }

    else {
        println!("You're on your journey rookie! But for now, you earn N100,000");
    }

}
