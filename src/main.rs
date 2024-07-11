mod config;
mod request;

use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "config.json";
    let config = Config::load_from_file(config_path)?;

    // Print the parsed configuration to verify
    println!("{:#?}", config);

    // Your server initialization code here

    Ok(())
}
