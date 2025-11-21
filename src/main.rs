use std::env;
use std::fs::OpenOptions;
use std::io::Write;


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

                save_entry(amount, &category, &note);
                println!("Saved entry to budget.csv");

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
fn save_entry(amount: f64, category: &str, note: &str) {
    let mut file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open("budget.csv")
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Could not open budget.csv: {}", e);
            return;
        }
    };

    // very simple CSV line: amount,category,note\n
    let line = format!("{},{},{}\n", amount, category, note);

    if let Err(e) = file.write_all(line.as_bytes()) {
        eprintln!("Failed to write to file: {}", e);
    }
}
