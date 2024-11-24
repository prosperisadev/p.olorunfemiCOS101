//Question 1

//Student Council Voter System for 50 Eligible Candidares

use std::io;

fn main() {

	for _ in 0..49 {

	println!("Welcome to the Student Council Voter System
		      Enter your details below to see if you qualify");

	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();

//Step 1, get candidate information to confirm qualification
	println!("Are you a Class Rep (True of False)");
	io::stdin().read_line(&mut input2).expect("Not valid string");
	let class_position:bool = input2.trim().parse().expect("Not a valid class position");


	println!("Enter Level (i.e 100,200,300, or 400)");
	io::stdin().read_line(&mut input3).expect("Not valid string");
	let level:i32 = input3.trim().parse().expect("Not a valid class level");

	println!("Enter CGPA (i.e 2.4 or 5.0)");
	io::stdin().read_line(&mut input4).expect("Not valid string");
	let cgpa:f32 = input4.trim().parse().expect("Not a valid CGPA");

//Step 2, use candidate information to determine whether they are qualified and print response to them.
	if class_position == true && level == 200 && cgpa >= 4.0
{
//Step 3, print results if qualified
	println!("Welcome Candidate,
		Name: John Doe
		Email: johndoe@pau.edu.ng
		Department: Computer Science
		State of Origin: Ibadan, Main City, Oja Igbo

		YOU CAN VOTE!",);
}

else if class_position == true && level == 300 && cgpa >= 4.0
{
//Step 4, print results if qualified
	println!("Welcome Candidate,
		Name: John Doe
		Email: johndoe@pau.edu.ng
		Department: Computer Science
		State of Origin: Ibadan, Main City, Oja Igbo

		YOU CAN VOTE!",);
}

else if class_position == true && level == 400 && cgpa >= 4.0
{
//Step 4, print results if qualified
	println!("Welcome Candidate,
		Name: John Doe
		Email: johndoe@pau.edu.ng
		Department: Computer Science
		State of Origin: Ibadan, Main City, Oja Igbo

		YOU CAN VOTE!",);
}


//Step 6, print results if unqualified
else
{
	println!("Sorry you are not eligible to vote");
}
}
}











//Question 2

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

