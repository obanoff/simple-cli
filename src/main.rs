use simple_cli::logic::*;
use serde_json;
use std::fs::File;
use std::io::{Write, BufReader};

fn main() {

    let path = "data.txt";

    let input = File::open(path).expect("Error openning file");
    let buffered = BufReader::new(input);

    let mut bills: Bills = serde_json::from_reader(buffered).unwrap_or(vec![]);

    loop {
        println!("
        type 'add' to add a new bill
        type 'get' to display existing bills
        type 'remove' to remove a bill
        type 'edit' to edit a bill
        ");

        let input = get_input();

        match input.to_lowercase().as_str() {
            "add" => add(&mut bills),
            "get" => display(&bills),
            "remove" => remove(&mut bills),
            "edit" => edit(&mut bills),
            _ => println!("Wrong input!"),
        }

        let mut output = File::create(path).expect("Error openning file for writing");
        let json = serde_json::to_string(&bills).unwrap();
        write!(output, "{json}").expect("Error writing to the file");
    }
}












