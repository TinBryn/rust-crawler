use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    let _uri = uri::Uri::from_str("");
}


pub mod uri;
pub mod html;
pub mod graph;
