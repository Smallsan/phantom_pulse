use std::{fs::{File, self}, io::Read};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Keys {
    discord_api_key: String,
}


pub fn fetch_key() -> String {
    create_directory("config/keys.json");

    let mut key_file = File::open("config/keys.json").expect("Unable to find keys.json");

    let mut contents = String::new();
    key_file
        .read_to_string(&mut contents)
        .expect("Unable to read keys.json");

    let keys: Keys = serde_json::from_str(&contents).expect("Unable to parse keys.json");

    return keys.discord_api_key;
}


pub fn create_directory(directory_name: &str) {
    if fs::metadata(directory_name).is_ok() {
        println!("Directory {} Already Exists", directory_name);
        return;
    }

    fs::create_dir(directory_name).expect("Error creating directory");

    println!("Directory {} Has Been Created", directory_name);
}
