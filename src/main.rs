extern crate clap;

use clap::{Arg, App};

const NAME: &str = "Rust IPS Patcher";
const VERSION: &str = "0.0.1";

fn main() {
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

    let debug = matches.is_present("Verbose");
    let input_rom = matches.value_of("Input ROM").unwrap();
    let output_rom = matches.value_of("Output ROM").unwrap();
    let patch_file = matches.value_of("Patch file").unwrap();

    if debug {
        println!("----------------------\n{} {}\n----------------------", NAME, VERSION);
        println!("Parameters:\n - Input ROM:\t{}\n - Output ROM:\t{}\n - Patch file:\t{}",
                 input_rom, output_rom, patch_file);
    }
}
