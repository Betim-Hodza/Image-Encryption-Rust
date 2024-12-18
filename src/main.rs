use std::fs::File;
use std::io::{self, Read, Write};
use std::process::exit;
use anyhow::Result;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use aes::Aes256;
use rand::Rng;

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
fn decrypt_file(filepath: &str, key: &[u8; 32]) -> Result<Vec<u8>> 
{
    // read in the iv
    let mut file = File::open(filepath)?;
    let mut iv = [0u8; 16];
    file.read_exact(&mut iv)?;

    // read ciphertext
    let mut ciphertext = Vec::new();
    file.read_to_end(&mut ciphertext)?;

    // create cipher w/ key + iv, and decrypt the ciphertext w/ it 
    let cipher = Aes256Cbc::new_from_slices(key, &iv)?;
    let decrypted_data = cipher.decrypt_vec(&ciphertext)?;
    Ok(decrypted_data)
}

fn prompt()
{
    println!("Rust Image-Encryption: AES256");
    println!("Choose an option:");
    println!("[1] Encrypt an Image");
    println!("[2] Decrypt an Image");
    println!("[3] Exit");
    println!();
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
            println!("Encrypt an image");
            continue;
        }
        else if choice == 2 
        {
            println!("Decrypt an image");
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
        }

        let input_path = "HackerCat.jpg";
        let encrypted_path = "encrypted.bin";
        let decrypted_path = "decrypted.jpg";
        
        // read image into memory
        let mut input_file = File::open(input_path)?;
        let mut buffer = Vec::new();
        input_file.read_to_end(&mut buffer)?;
    
        // Hardcoded key (for demonstration)
        let key: [u8; 32] = 
        [
            0x00, 0x01, 0x02, 0x03,
            0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b,
            0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13,
            0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f
        ];
    
        // Generate IV
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
    
        // Now let's decrypt
        let decrypted_data = decrypt_file(encrypted_path, &key)?;
    
        // write the decrypted data back as an image
        {
            let mut out = File::create(decrypted_path)?;
            out.write_all(&decrypted_data)?;
        }
    
        println!("Decrypted data written to {}", decrypted_path);
    
        return Ok(());
    }

}
