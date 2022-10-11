use simple_cli::logic::*;

fn main() {
    let mut bills: Bills = vec!();

    loop {
        println!("
        type 'add' to add a new bill
        type 'get' to display existing bills
        type 'remove' to remove a bill
        ");

        let input = get_input();

        match input.to_lowercase().as_str() {
            "add" => add(&mut bills),
            "get" => display(&bills),
            "remove" => remove(&mut bills),
            _ => println!("Wrong input!"),
        }
    }
}












