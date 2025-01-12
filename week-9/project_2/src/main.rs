use std::io::Write;

fn main() {

    let student_name: [&str; 5] = ["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric_number: [&str; 5] = ["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department: [&str; 5] = ["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let level: [i32; 5] = [300, 100, 200, 200, 100];


    let pau_sims = format!(
        "
         S/N,  Student Name,     Matric Number,    Department,      Level, \n
         1.,   {:<17}, {:<18}, {:<15}, {:<6},\n
         2.,   {:<17}, {:<18}, {:<15}, {:<6},\n
         3.,   {:<17}, {:<18}, {:<15}, {:<6},\n
         4.,   {:<17}, {:<18}, {:<15}, {:<6},\n
         5.,   {:<17}, {:<18}, {:<15}, {:<6},\n",
        student_name[0], matric_number[0], department[0], level[0],
        student_name[1], matric_number[1], department[1], level[1],
        student_name[2], matric_number[2], department[2], level[2],
        student_name[3], matric_number[3], department[3], level[3],
        student_name[4], matric_number[4], department[4], level[4],);
    let mut file = std::fs::File::create("PAU SIMS.csv").expect("create unsuccessful");
    file.write_all("Welcome to PAU Students Management Information System DataBase\n"
    .as_bytes()).expect("write failed");
    file.write_all(pau_sims.as_bytes()).expect("write failed");
    println!("\nPAU Students Management Information System Database Written into file");
}
