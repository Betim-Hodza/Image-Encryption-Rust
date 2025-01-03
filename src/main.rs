use std::fs::File;
use std::io::{self, Read, Write};
use std::process::exit;
use std::thread;
use std::time::Duration;
use anyhow::Result;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use aes::Aes256;
use rand::Rng;
use hex;

// create an Alias
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

// Encryption
// makes a cipher with Aes256 with the key + iv and encrypts the plaintext with it
fn encrypt_data(plaintext: &[u8], key: &[u8; 32], iv: &[u8; 16]) -> Result<Vec<u8>> 
{
    let cipher = Aes256Cbc::new_from_slices(key, iv)?;
    let ciphertext = cipher.encrypt_vec(plaintext);
    Ok(ciphertext)
}

// Decrypt func
// read the file, extract the IV, then decrypt
fn decrypt_file(filepath: &str, key_hex: &str) -> Result<Vec<u8>> 
{
    // Convert the hex key to bytes
    let key = hex::decode(key_hex)?;
    if key.len() != 32 
    {
        return Err(anyhow::anyhow!("Invalid key length. Expected 32 bytes."));
    }

    // Read the IV and ciphertext from the file
    let mut file = File::open(filepath)?;
    let mut iv = [0u8; 16];
    file.read_exact(&mut iv)?;

    let mut ciphertext = Vec::new();
    file.read_to_end(&mut ciphertext)?;

    // Create the cipher
    let cipher = Aes256Cbc::new_from_slices(&key, &iv)?;

    // Decrypt the data
    let decrypted_data = cipher.decrypt_vec(&ciphertext)?;
    
    Ok(decrypted_data)
}

fn generate_random_key() -> [u8; 32] 
{
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 32];
    rng.fill(&mut key);
    key
}

fn prompt()
{
    println!("Rust Image-Encryption: AES256");
    println!("Choose an option:");
    println!("[1] Encrypt an Image");
    println!("[2] Decrypt an Image");
    println!("[3] Exit");
    println!();

    // sleep for 1 sec
    thread::sleep(Duration::from_secs(1)); 
}

fn main() -> Result<()> 
{
    loop
    {
        let mut input = String::new();
        prompt();

        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice: i32 = input.trim().parse().expect("invalid Int");

        if choice == 1
        {
            // Grab image input from user
            let mut image_input = String::new();
            println!("\nPlease input the image name(note must be in current dir): ");
            io::stdin().read_line(&mut image_input).expect("Failed to read image");
            // use that image input to create the path for the image later
            image_input = image_input.trim().to_string();
            let input_path: &str = &image_input;

            
            // Grab bin name from user
            let mut bin_name = String::new();
            println!("Please name your encrypted .bin file(note add .bin to your fname):");
            io::stdin().read_line(&mut bin_name).expect("Failed to read bin");
            // similar to the image input but only for the bin path to be created later
            bin_name = bin_name.trim().to_string();
            let encrypted_path: &str = &bin_name;


            // Grab hardcoded key
            let key = generate_random_key();
            

            // read image into memory
            let mut input_file = File::open(input_path)?;
            let mut buffer = Vec::new();
            input_file.read_to_end(&mut buffer)?;
            

            // Generate random IV
            let mut iv = [0u8; 16];
            rand::thread_rng().fill(&mut iv);
            
            // encrypt the data
            let encrypted_data = encrypt_data(&buffer, &key, &iv)?;
            // Write IV + encrypted data to file
            {
                let mut out = File::create(encrypted_path)?;
                out.write_all(&iv)?;
                out.write_all(&encrypted_data)?;
            }
            
            println!("Encrypted data written to {}", encrypted_path);
            println!();
            println!("KEY: (keep this a secret): {}", hex::encode(key));
            println!();

            // sleep for 1 sec
            thread::sleep(Duration::from_secs(1)); 
            
            
            continue;
        }
        else if choice == 2 
        { 

            // Grab bin name from user
            let mut bin_name = String::new();
            println!("Please enter the name of the encrypted .bin file(note add .bin to your fname):");
            io::stdin().read_line(&mut bin_name).expect("Failed to read bin");
            // similar to the image input but only for the bin path to be created later
            bin_name = bin_name.trim().to_string();
            let encrypted_path: &str = &bin_name;


            // ask user for decryption name to store the image in
            let mut decyrpted_name: String = String::new();
            println!("Please name your decrypted image (note, please append the file extension):");
            io::stdin().read_line(&mut decyrpted_name).expect("Failed to read decrypted name");
            let decrypted_path: &str = &decyrpted_name;


            // accept user's key
            let mut key_input = String::new();
            println!("Please input the secret key (32 bytes in hex (no spaces)): ");
            io::stdin().read_line(&mut key_input).expect("Failed to read user_key");
            let key_input = key_input.trim();


            // Match works similarly to try catch
            match decrypt_file(encrypted_path, key_input) 
            {
                // if file was decrypted write it out
                Ok(decrypted_data) => 
                {
                    let mut out = File::create(decrypted_path)?;
                    out.write_all(&decrypted_data)?;
                    println!("Decrypted data written to {}", decrypted_path);
                },
                // throw error if decryption failed
                Err(e) => 
                {
                    println!("Decryption error: {}", e);
                }
            }

            continue;
        }
        else if choice == 3 
        {
            println!("Exiting...");
            exit(0);
        }
        else 
        {
            println!("invalid choice: {}, please choose again", choice);
            continue;
        }
    }

}
