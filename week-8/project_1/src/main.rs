fn main() {
    println!("Welcome to Public Service APS leel checker by FGN, input your information to check your level\n");

    let level = vec!["Intern", "Paralegal", "Placement", "Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher", "Senior Administrator",
    "PhD Candidate", "Associate", "Snr Teacher", "Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher", "Director", "Senior Lecturer",
    "Senior Associate 3-4", "Deputy Principal", "CEO", "Dean", "Partner", "Principal"]

    let mut input1 = String::new();
    println!("Input your level as a staff i.e intern, paralegal, administrator or CEO...");
    std::io::stdin().read_line(&mut input1).expect("Invalid String");
    let staff_level: String = input1.trim().parse().expect("Not a valid staff level");

    let mut input2 = String::new();
    println!("Input no. of years of experience i.e 1,2,3,4...");
    std::io::stdin().read_line(&mut input2).expect("Invalid String");
    let years_of_experience:usize = input2.trim().parse().expect("Not a valid number");


    let v = Vec

}
