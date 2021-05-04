#[macro_use]
extern crate magic_crypt;

use magic_crypt::MagicCryptTrait;

use std::fs::{self, File};
use std::io::Write;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
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
    let mut out_file;
    let payload;

    if args.encrypt {
        let content = fs::read_to_string(&args.path).expect("could not read file");
        payload = mc.encrypt_str_to_base64(content);
        out_file = File::create(encrypted_filename).expect("could not write file");
    } else if args.decrypt {
        let content = fs::read_to_string(encrypted_filename).expect("could not read file");
        payload = mc.decrypt_base64_to_string(content).unwrap();
        let decrypted_filename = format!("{}{}", &args.path.to_str().unwrap(), ".dec");
        out_file = File::create(decrypted_filename).expect("could not write file");
    } else {
        panic!("you didn't pass an encrypt or decrypt flag!");
    }

    write!(out_file, "{}", payload).ok();
}
