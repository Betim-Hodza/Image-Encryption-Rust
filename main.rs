use std::error::Error;
use std::path::Path;
use clap::{App, load_yaml};

use image_sec::{encrypt_image, decrypt_image};

fn main() -> Result<(), Box<dyn Error>>
{
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let encrypt = matches.value_of("encrypt");
    match encrypt 
    {
        Some(dir) => match encrypt_image(Path::new(dir)) {
            Ok(_) => println!("Image was successfully encrypted!"),
            Err(_) => eprintln!("Error encrypting image")
        },
        None => ()
    }

    let decrypt = matches.value_of("decrypt");
    match decrypt 
    {
        Some(dir) => match decrypt_image(Path::new(dir)) {
            Ok(_) => println!("Image was successfully encrypted!"),
            Err(_) => eprintln!("Error decrypting image")
        },
        None => ()
    }
    Ok(())
}