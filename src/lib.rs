pub mod logic {

    use std::io;
    use std::process;
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Bill {
        title: String,
        amount: i32,
    }

    impl Bill {
        pub fn new(title: &str, amount: i32) -> Self {
            Self {
                title: title.to_owned(),
                amount,
            }
        }
    }

    pub type Bills = Vec<Bill>;

    pub fn get_input() -> String {
        let mut buff = String::new();

        loop {
            io::stdin().read_line(&mut buff).unwrap_or_else(|err| {
                println!("Error: {err}");
                process::exit(1);
            });
            if buff.trim().len() == 0 {
                println!("\nInput cannot be empty\n");
                continue;
            } else { break; }
        }
        buff.trim().to_owned()
    }

    pub fn add(bills: &mut Bills) {
        println!("\nEnter title:\n");

        let title = get_input();

        loop {
            println!("\nEnter amount:\n");

            let amount = get_input().parse::<i32>();
        
            match amount {
                Ok(n) => {
                    let bill = Bill::new(&title, n);

                    if find(bills, &title) {
                        println!("\nA bill with this name already exists. Would you like to replace it with a new one? (yes/no)");

                        loop {
                            let answer = get_input();

                            match answer.to_lowercase().as_str() {
                                "yes" => {
                                    replace(bills, &title, bill);
                                    println!("\nThe bill has been replaced");
                                    break;
                                },
                                "no" => { break },
                                _ => {
                                    println!("\nWrong input, expected 'yes' or 'no'");
                                },
                            }
                        }
                        break;
                    } else {
                        bills.push(bill);
                        println!("\nThe bill has been added");
                        break;
                    }
                },
                Err(_) => {
                    println!("\nWrong input, expected a number");
                    continue;
                },
            }
        }
    }

    pub fn display(bills: &Bills) {
        if bills.len() == 0 {
            println!("\nNo bills found");
            return;
        }
        
        println!("");

        for (i, b) in bills.iter().enumerate() {
            println!("{}. Title: {}, Amount: {}", i + 1, b.title, b.amount);
        }
    }

    fn replace(bills: &mut Bills, title: &str, new: Bill) {
        let position = bills.iter().position(|x| x.title == title).unwrap();
        bills.remove(position);
        bills.insert(position, new);
    }

    fn find(bills: &Bills, title: &str) -> bool {
        let result = bills.iter().find(|x| x.title == title);
        match result {
            Some(_) => true,
            None => false,
        }
    }

    pub fn remove(bills: &mut Bills) {
        println!("\nEnter a bill's title to remove:\n");

        let title = get_input();

        if find(bills, &title) {
            let position = bills.iter().position(|x| x.title == title).unwrap();

            println!("\nConfirm the operation (yes/no)\n");
            
            loop {
                let answer = get_input();

                match answer.as_str() {
                    "yes" => {
                        bills.remove(position);
                        println!("\nThe bill has been removed");
                        break;
                    },
                    "no" => break,
                    _ => {
                        println!("\nWrong input, expected 'yes' or 'no'\n");
                        continue;
                    }
                }
            }   
        } else {
            println!("\nNo bills with that title found");
        }
    }

    pub fn edit(bills: &mut Bills) {
        println!("\nEnter a title of bill to edit\n");
        let answer = get_input();

        let position = bills.iter().position(|x| x.title == answer);

        match position {
            Some(i) => {
                let clone = bills[i].clone();

                println!("\nEnter a new title:\n");
                let title = get_input();

                bills[i].title = title;

                loop {
                    println!("\nEnter a new amount:\n");
                    let amount = get_input().parse::<i32>();

                    match amount {
                        Ok(n) => {
                            bills[i].amount = n;
                            println!("\nThe bill has been edited");

                            println!("\nConfirm changes (yes/no)");
                            loop {
                                let answer = get_input().to_lowercase();

                                match answer.as_str() {
                                    "yes" => {
                                        println!("\nChanges confirmed");
                                        break;
                                    },
                                    "no" => {
                                        bills[i] = clone;
                                        println!("\nChanges declined");
                                        break;
                                    },
                                    _ => {
                                        println!("\nWrong input,expected 'yes' or 'no'");
                                        continue;
                                    },
                                }
                            }
                            break;
                        },
                        Err(_) => {
                            println!("\nWrong input, expected a number");
                            continue;
                        },
                    }
                }
            },
            None => println!("\nNo bills with that name found"),
        }
    }


}