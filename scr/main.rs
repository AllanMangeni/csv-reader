use std::error::Error;
use csv::Reader;

fn main() -> Result<(), Box<dyn Error>> {
    let data = Reader::from_path("file.csv")?;

    for result in data.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
