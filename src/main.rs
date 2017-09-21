pub mod domme;
pub mod html;

use std::iter::FromIterator;

fn main() {
    let vec = Vec::from_iter((0..100).into_iter());

    for a in vec {
        println!("Something: {:?}", a);
    }
}
