pub mod domme;
pub mod html;
pub mod css;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "../simple.html";
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("File not found!");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "Reading file failed!",
    );

    println!("Trying to parse file contents\n{}", contents);

    let root: domme::Node = html::parse(contents);
    println!("{:#?}", root);
}
