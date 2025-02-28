struct Rectangle {
    width:u32, height:u32
}

//logic to calculate area of  a rectangle

impl Rectangle {
    fn area(&self)->u32 {
        //use the . operator to fetch the value of the field via the self keyword
        self.width * self.height
    }
}


fn main() {
    //instantiate the structure
    let small = Rectangle {
        width:10,
        height:20
    };
    //print the rectangle's area
    println!("Width is {} \nHeight is {} \nArea of Rectangle is {}", small.width,small.height,small.area());
}
