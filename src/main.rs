pub mod domme;
pub mod html;
pub mod css;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    // TODO: Read file from stdin for now?
    let dom = test_html_parser("sample.html");
    println!("{:?}", dom);

    let css = test_css_parser("sample.css");
    println!("{:?}", css);
}


fn test_html_parser(path: &str) -> domme::Node {
    let mut f = File::open(path).expect("File not found!");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Reading file failed!");

    return html::parse(contents);
}

fn test_css_parser(path: &str) -> css::Stylesheet {
    let mut f = File::open(path).expect("File not found!");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Reading file failed!");

    return css::parse(contents);
}