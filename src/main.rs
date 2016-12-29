use std::io::prelude::*;
use std::env;
use std::io::stdin;
use std::io::BufReader;
use std::fs::File;

fn print_usage()
{
    println!("Usage: ls | redirect [options]");
    println!("Options:");
    println!("    --prefix=abc          - lines matching this prefix will be");
    println!("                            redirected to the output file.");
    println!("    --filename=redirected - the filename to write redirected");
    println!("                            output to.");
}

fn main()
{
    let mut prefix = String::from(">");
    let mut filename = String::from("redirected");

    for arg in env::args().skip(1) {
        if !arg.starts_with("--") {
            print_usage();
            return;
        }

        let mut it = arg.chars().skip(2);
        let name: String = it.by_ref().take_while(|&a| a != '=').collect();
        let value: String = it.collect();

        match name.as_str() {
            "prefix" => prefix = value,
            "filename" => filename = value,
            _ => {
                print_usage();
                return;
            },
        }
    }

    let mut file = match File::create(filename) {
        Ok(f) => f,
        Err(_) => panic!("Error opening redirected file for writing."),
    };

    for line_res in BufReader::new(stdin()).lines() {
        let mut line = line_res.expect("Error reading line from stdin.");
        if line.starts_with(prefix.as_str()) {
            line.push('\n');
            file.write_all(line.into_bytes().as_slice()).
                expect("Failed to write to file.");
        } else {
            println!("{}", line);
        }
    }
}
