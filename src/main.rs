use std::fs;
use std::fs::OpenOptions;
use std::collections::HashMap;
use std::io::Write;
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
                println!("--- Your budget entries ---");

                match fs::read_to_string("budget.csv") {
                    Ok(content) => {
                        if content.trim().is_empty() {
                            println!("(no entries yet)");
                        } else {
                            print!("{}", content);
                        }
                    }
                    Err(_) => {
                        println!("No budget.csv found (no entries yet).");
                    }
                }
            }

             
            "summary" => {
                println!("--- SUMMARY ---");
                //this part is not finished, working on 
                //read the whole file into a String
                let content = match fs::read_to_string("budget.csv") {
                    Ok(c) => c,
                    Err(_) => {
                        println!("No budget.csv found or could not read it.");
                        return;
                    }
                };
                // later for the total
                let mut total_spent: f64 = 0.0;
                // for now, only prints the raw content
                println!("content:\n{}", content);
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
