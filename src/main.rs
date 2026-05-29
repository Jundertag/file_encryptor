use enc_file::{AeadAlg, EncryptOptions, KdfAlg, KdfParams, decrypt_file, encrypt_file};
use secrecy::SecretString;
use std::path::Path;
use std::io;

fn main() {
    let manifest: Vec<String> = vec![
        String::from("yay.txt"),
        String::from("help me.txt"),
        String::from("tom's obvious language.txt"),
        String::from("SOMETHING.txt")
    ];

    let password = SecretString::new("mypassword".into());
    let opts = EncryptOptions {
        alg: AeadAlg::XChaCha20Poly1305,
        kdf: KdfAlg::Argon2id,
        kdf_params: KdfParams { 
            t_cost: 4,
            mem_kib: 64 * 1024,
            parallelism: 12,
        },
        force: true,
        ..Default::default()
    };
    
    let mut input = String::new();
    println!("1: encrypt files\n2: decrypt files");
    io::stdin().read_line(&mut input).expect("failed to read input");

    if input.trim() == String::from("1") {
        println!("Encrypting selected files...");
        for filename in &manifest {
            let path_string = String::from("C:\\Users\\jayde\\Desktop\\") + filename;
            let output: Option<&Path> = Some(Path::new(&path_string));
            encrypt_file(
                Path::new(&path_string),
                output,
                password.clone(),
                opts.clone()
            ).unwrap();

            println!("Encrypted file {filename}")
        }

        println!("All files encrypted")
    } else if input.trim() == String::from("2") {
        println!("Decrypting selected files...");
        for filename in &manifest {
            let path_string = String::from("C:\\Users\\jayde\\Desktop\\") + filename;
            let output: Option<&Path> = Some(Path::new(&path_string));
            match decrypt_file(
                Path::new(&path_string), 
                output, 
                password.clone()
            ) {
                Ok(_) => {},
                Err(error) => {
                    panic!("Problem opening the file: {error:?}")
                }
            }

            println!("Decrypted file {filename}")
        }

        println!("All files decrypted!")
    } else {
        println!("Unknown input, closing");
        return;
    }
}
