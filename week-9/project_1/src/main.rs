use std::io::Write;

fn main() {

    let portfolio_1 = "Lager: \n 33 Export \n Desperados \n Goldberg \n Gulder \n Heineken \n Star \n \n";
    let portfolio_2 = "Stout: \n Legend \n Turbo King \n Williams \n \n";
    let portfolio_3 = "Non-Alcoholic: \n Amstel Malta \n Maltina \n Malta Gold \n Fayrouz \n \n";

    let mut file = std::fs::File::create("Project-1.txt").expect("create unsuccessful");
    file.write_all("Welcome to Nigeria Brewery Limited Portfolio\n"
    .as_bytes()).expect("write failed");
    file.write_all(portfolio_1.as_bytes()).expect("write failed");
    file.write_all(portfolio_2.as_bytes()).expect("write failed");
    file.write_all(portfolio_3.as_bytes()).expect("write failed");
    println!("\nNigerian Breweries Portfolio Data Written into file");
}
