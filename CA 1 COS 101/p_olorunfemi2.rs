//Staff Publication Incentive System

use std::io;

fn main() {

	for _ in 0..99 {
	println!("Welcome to the Staff Publication Incentive System
		      Enter your details below to see if you qualify");

	let mut input2 = String::new();
	let mut input3 = String::new();

//Step 1, get staff information to determine their incentive
	println!("Enter Staff Name");
	io::stdin().read_line(&mut input2).expect("Not valid string");
	let staff_name:String = input2.trim().parse().expect("Not a valid string");


	println!("Enter Number of Publications");
	io::stdin().read_line(&mut input3).expect("Not valid string");
	let no_of_publications:i32 = input3.trim().parse().expect("Not a valid number");

//Step 2, use staff information to determine incentive and print response
	if no_of_publications >= 3 && no_of_publications == 5
{
//Step 2, print no. of publications
	println!("Your name is {}", staff_name);
	println!("Your qualified incentive is N500,000");
}

else if no_of_publications >= 5 && no_of_publications <= 10
{
//Step 3, print no. of publications
	println!("Your name is {}", staff_name);
	println!("Your qualified incentive is N800,000");
}

else if no_of_publications >= 10
{
//Step 3, print no. of publications
	println!("Your name is {}", staff_name);
	println!("Your qualified incentive is N1,000,000");

}
}
}