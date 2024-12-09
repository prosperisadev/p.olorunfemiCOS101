fn main() {

    //initialization of tuple with data type
    let datatype_tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("Tuple content = {:?}", datatype_tuple);

    //initialization of tuple without data type
    let no_datatype_tuple = ("Rust", "fun", 100);
    println!("Tuple content = {:?}", no_datatype_tuple);

    //accessing tuple element at index 0
    println!("Value at index 0 = {}", datatype_tuple.0);

    //accessing tuple element at index 0
    println!("Value at index 0 = {}", datatype_tuple.1);

    //accessing tuple element at index 0
    println!("Value at index 0 = {}", datatype_tuple.2);
}
