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
        let price_str = match args.next() {
        Some(v) => v,
        None => {
            eprintln!("Missing <category>. Usage: Cargo run -- add <price> <category>");
            return;
        }   
    };


                let price: f64 = match price_str.parse() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("price must be a number, got: {}", price_str);
                        return;
                    }
                };
                println!("price as number: {} $", price);

                let category = match args.next() {
                    Some(v) => v,

                    None => {
                        eprintln!("Missing <category>. Usage: Cargo run -- add <price> <category>");
                        return;
                    }
                };
                println!("category as String: {}", category);

                save_entry(price, &category);
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
fn save_entry(price: f64, category: &str) {
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

    // very simple CSV line: price,category,note\n
    let line = format!("{}: {} $\n", category, price);

    if let Err(e) = file.write_all(line.as_bytes()) {
        eprintln!("Failed to write to file: {}", e);
    }
}
