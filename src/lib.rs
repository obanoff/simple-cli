pub mod logic {

    use std::io;
    use std::process;

    #[derive(Debug)]
    pub struct Bill {
        title: String,
        amount: i32,
    }

    impl Bill {

        fn new(title: &str, amount: i32) -> Self {
            Self {
                title: title.to_owned(),
                amount,
            }
        }
    }

    pub type Bills = Vec<Bill>;

    pub fn get_input() -> String {
        let mut buff = String::new();

        io::stdin().read_line(&mut buff).unwrap_or_else(|err| {
            println!("Error: {err}");
            process::exit(1);
        });
        buff.trim().to_owned()
    }

    pub fn add(bills: &mut Bills) {
        println!("\nEnter title\n");

        let title = get_input();

        loop {
            println!("\nEnter amount\n");

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
        println!("\nEnter a bill's title to remove");

        let title = get_input();

        if find(bills, &title) {
            let position = bills.iter().position(|x| x.title == title).unwrap();
            bills.remove(position);
            println!("\nThe bill has been removed");
        } else {
            println!("\nNo bills with that title found");
        }
    }


}