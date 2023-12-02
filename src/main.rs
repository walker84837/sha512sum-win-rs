use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};
use structopt::StructOpt;

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

fn main() {
    let opt = Args::from_args();

    let mut contents = Vec::new();
    let mut contents_input = Vec::new();

    match read_binary_file(&opt.path, &mut contents) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    if !opt.check {
        let checksum: String = hashing::Sha512Sum::get_checksum(&contents);
        println!("{}  {}", checksum, opt.path.display());
    } else {
        let arr: Vec<&str> = std::str::from_utf8(&contents)
            .unwrap_or_default()
            .split_whitespace()
            .collect();
        let checksum_init = arr[0];
        let file_name = PathBuf::from(arr[1]);

        match read_binary_file(&file_name, &mut contents_input) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error while reading checksum file: {}", e);
                return;
            }
        };

        let calculated_checksum = hashing::Sha512Sum::get_checksum(&contents_input);

        if checksum_init == calculated_checksum {
            println!("{}: OK", file_name.display());
        } else {
            eprintln!("{}: FAILED", file_name.display());
            eprintln!("sha512sum-win: WARNING: 1 computed checksum did NOT match");
        }
    }
}

fn read_binary_file(path: &PathBuf, buffer: &mut Vec<u8>) -> io::Result<()> {
    let mut file = File::open(path)?;
    file.read_to_end(buffer)?;
    Ok(())
}

