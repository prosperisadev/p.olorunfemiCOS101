//Calculating quadractic equation using loops

use std::io;

fn main() {

//Input a, b, c

    println!("Enter a");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Input not a string");
    let a:f32 = input1.trim().parse().expect("Input not a number");

    println!("Enter b");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Input not a string");
    let b:f32 = input2.trim().parse().expect("Input not a number");

    println!("Enter c");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Input not a string");
    let c:f32 = input3.trim().parse().expect("Input not a number");

//Calculate the discriminant

    let d:f32 = b * b - (4.0 * a * c);

    if d > 0.0 {
        let root1 = -b + d.sqrt() / (2.0 * a);
        let root2 = -b - d.sqrt() / (2.0 * a);
        println!("The roots are Root1 = {}, Root2 = {}", root1, root2);
        println!("There are two distinct roots");
    }

    else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("The root is {}", root);
        println!("There is exactly one real root");
    }

    else {
       println!("Roots does not exist");
    }

}
