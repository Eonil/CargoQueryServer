extern crate toml;
use std::env;
use std::fs::File;
use std::string::String;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let qpath = args[1].clone();
    let qpath1: &str = &qpath; 

    let mut file1 = File::open("Cargo.toml").unwrap();
    let mut s = String::new();	
    file1.read_to_string(&mut s).unwrap();

    let v: toml::Value = s.parse().unwrap();
    let v1: &toml::Value = v.lookup(qpath1).unwrap();
    println!("{}", v1.as_str().unwrap());
}
