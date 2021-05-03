#[macro_use]
extern crate magic_crypt;

use magic_crypt::MagicCryptTrait;

use std::fs::File;
use std::io::Write;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    password: String,
    #[structopt(short, long)]
    encrypt: bool,
    #[structopt(short, long)]
    decrypt: bool,
}

fn main() {
    let args = Cli::from_args();

    let mc = new_magic_crypt!(&args.password, 256);

    let encrypted_filename = format!("{}{}", &args.path.to_str().unwrap(), ".enc");

    if args.encrypt {
        let content = std::fs::read_to_string(&args.path).expect("could not read file");

        let base64 = mc.encrypt_str_to_base64(content);

        let mut output = File::create(encrypted_filename).expect("could not write file");

        write!(output, "{}", base64).ok();
    } else if args.decrypt {
        let content = std::fs::read_to_string(encrypted_filename).expect("could not read file");

        let decrypted = mc.decrypt_base64_to_string(content).unwrap();

        let encrypted_filename = format!("{}{}", &args.path.to_str().unwrap(), ".dec");

        let mut output = File::create(encrypted_filename).expect("could not write file");

        write!(output, "{}", decrypted).ok();
    } else {
        panic!("you didn't pass an encrypt or decrypt flag!");
    }
}
