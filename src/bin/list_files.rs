extern crate clap;
extern crate pac;

use clap::{Arg, App};
use pac::Pac;
use pac::direct::DPac;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let matches = App::new("PAC File Lister")
        .version("0.1")
        .author("Marime Gui")
        .about("Lists all files inside of a .pac file")
        .arg(
            Arg::with_name("INPUT")
                .help("File to list")
                .required(true)
                .index(1),
        )
.get_matches();
    let reader = &mut BufReader::new(File::open(matches.value_of("INPUT").unwrap()).unwrap());
    let direct_pac = DPac::import(reader).unwrap();
    let pac = Pac::from_direct(direct_pac).unwrap();
    for file in pac.files {
        println!("{}", file.path);
    }
}