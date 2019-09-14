extern crate multi_reader;
extern crate xdrgen;

use std::env;
use std::fs::{read_dir, File};
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("Stellar.rs");

    let output = File::create(&dest_path).unwrap();

    let files = read_dir("xdr").unwrap();

    let mr = multi_reader::MultiReader::new(
        files
            .filter(|p| p.as_ref().unwrap().path().extension().unwrap_or_default() == "x")
            .map(|p| File::open(p.unwrap().path()).unwrap()),
    );

    xdrgen::generate("all the files in the xdr folder", mr, output).unwrap();
}
