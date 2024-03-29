use csv;
use std::error::Error;

fn main() {
    let path = "./content.csv";
    if let Err(e) = read_from_file(path) {
        eprintln!("{}", e);
    }
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
