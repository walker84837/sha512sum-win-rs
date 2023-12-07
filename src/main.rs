use std::{
    fs::File,
    io::Read,
    path::PathBuf,
};
use structopt::StructOpt;
use anyhow::{Result, Error};

mod hashing;

/// A Windows port of the sha512sum command: print or check SHA512 (512-bit) checksums.
#[derive(StructOpt)]
#[structopt(name = "sha512sum-win")]
struct Args {
    /// The file's path.
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    #[structopt(short, long)]
    check: bool,
}

fn main() -> Result<()> {
    let opt = Args::from_args();

    let mut contents = Vec::new();

    match File::open(&opt.path) {
        Ok(mut file) => {
            if let Err(e) = file.read_to_end(&mut contents) {
                eprintln!("Error reading file {}: {}", opt.path.display(), e);
            }
        }
        Err(e) => {
            eprintln!("Error opening file {}: {}", opt.path.display(), e);
        }
    }

    let checking = opt.check;

    if !checking {
        let checksum: String = hashing::Sha512Sum::get_checksum(&contents);
        println!("{}  {}", checksum, opt.path.display());
        return Ok(());
    } else {
        let contents_str = match std::str::from_utf8(&contents) {
            Ok(s) => s,
            Err(e) => {
                let error_msg = format!("Error converting binary data to string: {}", e);
                return Err(Error::msg(error_msg));
            }
        };

        let arr: Vec<&str> = contents_str.trim().split_whitespace().collect();
        let checksum_init = arr[0];
        let file_name = PathBuf::from(arr[1]);

        let mut contents_input = Vec::new();

        match File::open(&file_name) {
            Ok(mut file) => {
                if let Err(e) = file.read_to_end(&mut contents_input) {
                    eprintln!("Error reading file {}: {}", file_name.display(), e);
                }
            }
            Err(e) => {
                let error_msg = format!("Error opening file {}: {}", file_name.display(), e);
                return Err(Error::msg(error_msg));
            }
        }

        let calculated_checksum = hashing::Sha512Sum::get_checksum(&contents_input);

        if checksum_init == calculated_checksum {
            println!("{}: OK", file_name.display());
            return Ok(());
        } else {
            eprintln!("{}: FAILED", file_name.display());
                let error_msg = String::from("sha512sum-win: WARNING: 1 computed checksum did NOT match");
                return Err(Error::msg(error_msg));
        }
    }
}
