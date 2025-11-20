use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let command = match args.next() {
        Some(c) => c,
        None => {
            println!("No command provided.");
            return;
        }
    };

    println!("you ran: {}", command);

        match command.as_str() {
            "add" => {

        println!("ADD choosen");

        let amount_str = match args.next() {
        Some(v) => v,
        None => {
            eprintln!("Missing <amount>. Usage: cargo run -- add <amount> <category> <note>");
            return;
        }   
    };


                let amount: f64 = match amount_str.parse() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Amount must be a number, got: {}", amount_str);
                        return;
                    }
                };
                println!("amount as number: {}", amount);

                let category = match args.next() {
                    Some(v) => v,

                    None => {
                        eprintln!("Missing <category>. Usage: Cargo run -- add <amount> <category> <note>");
                        return;
                    }
                };
                println!("category as String: {}", category);

                let note = match args.next() {
                    Some(v) => v,

                    None => {
                        eprintln!("Missing <note>. Usage: Cargo run -- add <amount> <category> <note>");
                        return;
                    }
                };
                println!("note as String: {}", note);

            } 
            "list" => {
                println!("LIST choosen")
            } 
            "summary" => {
                println!("SUMMARY choosen")
            }
            _ => {
                println!("Unknown command")
            } 
        }
}
