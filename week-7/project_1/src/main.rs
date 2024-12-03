use std::io;

fn area_of_trapezium () {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    println!("Welcome to Area of Trapezium Calculator, input your height");
    io::stdin().read_line(&mut input1).expect("Not valid string");
    let height:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Input base 1");
    io::stdin().read_line(&mut input2).expect("Not valid string");
    let base1:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Input base 2");
    io::stdin().read_line(&mut input3).expect("Not valid string");
    let base2:f32 = input3.trim().parse().expect("Not a valid number");

    let aot = height / 2.0 * (base1 + base2);
    println!("Your Area of Trapezium = {}", aot);
}

fn area_of_rhombus (){

    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("Welcome to Area of Rhombus Calculator, input your first diagonal");
    io::stdin().read_line(&mut input1).expect("Not valid string");
    let diagonal1:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Input base 1");
    io::stdin().read_line(&mut input2).expect("Not valid string");
    let diagonal2:f32 = input2.trim().parse().expect("Not a valid number");

    let aor = 0.5 * diagonal1 * diagonal2;
    println!("Your Area of Trapezium = {}", aor);
    
}

fn area_of_parallelogram () {

     let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Welcome to Area of Parallelogram Calculator, input your base");
    io::stdin().read_line(&mut input1).expect("Not valid string");
    let base:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Input base 1");
    io::stdin().read_line(&mut input2).expect("Not valid string");
    let altitude:f32 = input2.trim().parse().expect("Not a valid number");

    let aop = base * altitude;
    println!("Your Area of Parallelogram = {}", aop);
    
}

fn area_of_cube (){

    let mut input1 = String::new();

    println!("Welcome to Area of Parallelogram Calculator, input the length of your side");
    io::stdin().read_line(&mut input1).expect("Not valid string");
    let length_of_side:f32 = input1.trim().parse().expect("Not a valid number");

    let aoc = 6.0 * length_of_side.powf(2.0);
    println!("Your Area of Cube = {}", aoc);
    
}


fn volume_of_cylinder (){

    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("Welcome to Volume of Cylinder Calculator, input your radius");
    io::stdin().read_line(&mut input1).expect("Not valid string");
    let radius:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Input base 1");
    io::stdin().read_line(&mut input2).expect("Not valid string");
    let height:f32 = input2.trim().parse().expect("Not a valid number");

    let voc = 3.142 * radius.powf(2.0) * height;
    println!("Your Area of Trapezium = {}", voc);
    
}



fn main() {
    println!("Welcome to Prosperity Calculator, what would you like to calculate
        For Area of Trapezium, type T
        For Area of Rhombus, type R
        For Area of Parallelogram, type P
        For Area of Cube, type C
        For Volume of Cylinder, type V");

    let mut input1 = String::new();
    println!("What would you like to calculate (T,R,P,C or V)");
    io::stdin().read_line(&mut input1).expect("Not valid string");
    let chosen_calculation:char = input1.trim().parse().expect("Not a valid character");

    if chosen_calculation == 'T'{
        area_of_trapezium()
    }

    else if chosen_calculation == 'R'{
        area_of_rhombus()
    }

    else if chosen_calculation == 'P'{
        area_of_parallelogram()
    }

    else if chosen_calculation == 'C'{
        area_of_cube()
    }

    else if chosen_calculation == 'V'{
        volume_of_cylinder()
    }

    else {
        println!("You inputed the wrong value");
    }

}
