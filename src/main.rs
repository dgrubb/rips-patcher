#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate clap;

// Standard library
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

// Crates
use clap::{Arg, App};

const NAME: &str = "Rust IPS Patcher";
const VERSION: &str = "0.0.1";

fn main() {
    let mut log_level = log::Level::Error;
    let matches = App::new(NAME)
        .version(VERSION)
        .author("David Grubb <davidanthonygrubb@gmail.com>")
        .about(
            "Creates a patched ROM file by combining an input ROM file with \
            an IPS (International Patch System) file."
        )
        .arg(
            Arg::with_name("Input ROM")
                .short("r")
                .long("input-rom")
                .required(true)
                .index(1)
                .help("ROM file which patch shall be applied to.")
        )
        .arg(
            Arg::with_name("Patch file")
                .short("p")
                .long("patch-file")
                .required(true)
                .index(2)
                .help("Patch file in .ips format to be applied to ROM file.")
        )
        .arg(
            Arg::with_name("Output ROM")
                .short("o")
                .long("output-rom")
                .required(true)
                .index(3)
                .help("Name of new ROM file to be generated.")
        )
        .arg(
            Arg::with_name("Verbose")
                .short("v")
                .long("verbose")
                .help("Include to enable debug printing.")
        )
        .get_matches();

    // Setup some simple logging
    if matches.is_present("Verbose") {
        log_level = log::Level::Info;
    }
    simple_logger::init_with_level(log_level).unwrap();

    let input_rom_path = matches.value_of("Input ROM").unwrap();
    let output_rom_path = matches.value_of("Output ROM").unwrap();
    let patch_file_path = matches.value_of("Patch file").unwrap();

    info!("{}, version: {}", NAME, VERSION);
    info!("Parameters:\n - Input ROM:\t{}\n - Output ROM:\t{}\n - Patch file:\t{}",
          input_rom_path, output_rom_path, patch_file_path);

    if !validate_file(input_rom_path) {
        error!("Could not validate input ROM file: {}", input_rom_path);
        exit(-1);
    }

    if !validate_file(patch_file_path) {
        error!("Could not validate input path file: {}", patch_file_path);
        exit(-1);
    }

    let input_rom_data = load_binary_file(input_rom_path).expect("Error reading file");
    let path_file_data = load_binary_file(patch_file_path).expect("Error reading file");


}

fn validate_file(file: &str) -> bool {
    let fp = Path::new(file);
    (fp.exists() && fp.is_file())
}

fn load_binary_file(file_path: &str) -> std::io::Result<Vec<u8>> {
    let mut data = Vec::new();
    File::open(file_path)?.read_to_end(&mut data)?;
    Ok(data)
}

