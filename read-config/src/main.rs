
use serde::{Serialize, Deserialize};
use serde_yaml::{self};

#[derive(Debug,Serialize,Deserialize)]
struct Detail {
    item: String,
    names: Vec<String>
}

#[derive(Debug,Serialize,Deserialize)]
struct Metadata {
    item: String,
    names: Vec<String>
}

#[derive(Debug,Serialize,Deserialize)]
struct Config {
    environment: String,
    monitor: Vec<String>,
    metadata: Vec<Metadata>,
}



fn main() {
    let f = std::fs::File::open("config.yaml").expect("Could not open the file");
    let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read the file");

    println!("{:?}",scrape_config);
}
