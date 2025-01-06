use std::io::Write;

fn main() {

    let name_of_commissioners: [&str; 5] = ["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calitus Ogbonna", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry: [&str; 5] = ["Internal Affairs ", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let geopolitical_zone: [&str; 5] = ["South West", "North East", "South South", "South West", "South East"];

    let convicted_ministers = format!(
        "
         S/N,Name of Commissioner,Ministry,Geopolitical Zone,\n
         1.,{:<25},{:<18},{:<15}, \n
         2.,{:<25},{:<18},{:<15}, \n
         3.,{:<25},{:<18},{:<15}, \n
         4.,{:<25},{:<18},{:<15}, \n
         5.,{:<25},{:<18},{:<15}, \n",
         name_of_commissioners[0], ministry[0], geopolitical_zone[0],
         name_of_commissioners[1], ministry[1], geopolitical_zone[1],
         name_of_commissioners[2], ministry[2], geopolitical_zone[2],
         name_of_commissioners[3], ministry[3], geopolitical_zone[3],
         name_of_commissioners[4], ministry[4], geopolitical_zone[4],);
    let mut file = std::fs::File::create("EFCC Convicted Ministers.csv").expect("create unsuccessful");
    file.write_all("EFCC Convicted Ministers DataBase\n"
    .as_bytes()).expect("write failed");
    file.write_all(convicted_ministers.as_bytes()).expect("write failed");
    println!("\nEFCC Convicted Ministers Database Written into file");
}
