mod config;

use std::{fs::File, io::Read};

use toml::Table;

fn main() {
    let mut file = File::open(&format!("config.toml")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let main_table = contents.parse::<Table>().unwrap();
    if let Some(server) = main_table.get("server") {
        println!("{}", server);
    }
}
