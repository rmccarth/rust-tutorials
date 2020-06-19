#!/usr/bin/env run-cargo-script
// goal is to make a method that allows us to create and store a contact with some contact details

use std::io::{self, Read};
extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

struct User {
    username: String,
    email: String,
    phone: String,
}

fn main() {
    
    let mut verbose = false;
    let email = "slixperi@gmail.com";
    let phone = "4129770305";
    let mut username = "World".to_string();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Create a Contact");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Be verbose");
        ap.refer(&mut username)
            .add_option(&["--username"], Store,
            "Username");
        ap.parse_args_or_exit();
    }

    if verbose {
        println!("name is {}", &username);
    }
    println!("Hello {}!", &username);
}

fn get_input() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(())
}

fn print_info(info: &str) {
    println!("{}", info);
}
