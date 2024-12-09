fn main() {

    //Using Vec::new()
    let v : Vec<i64> = Vec::new();

    // printing the size of vector
    println!("\nThe lenghth of Vec::new is: {}", v.len());

    // Using macro
    let v = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];

    //printing the size of vector
    println!("\nThe lenghth of vec macro is: {}", v.len());
}
