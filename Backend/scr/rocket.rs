use std::error::Error;
use csv::Reader;
use rocket::{get, routes};

#[get("/data")]
fn get_data() -> Result<String, Box<dyn Error>> {
    let data = Reader::from_path("file.csv")?;

    let mut result = String::new();
    for record in data.records() {
        let row = record?;
        result += &format!("{:?}\n", row);
    }

    Ok(result)
}

fn main() {
    rocket::ignite().mount("/", routes![get_data]).launch();
}
