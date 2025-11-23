use std::collections::HashMap;
use std::env;
use std::fs;
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

    match command.as_str() {
        "add" => {
            let price_str = match args.next() {
                Some(v) => v,
                None => {
                    eprintln!("--- ERROR ---");
                    eprintln!(
                        "Missing <category> and <price>. Usage: Cargo run -- add <price> <category>"
                    );
                    eprintln!("if you don't know the commands use: Cargo run -- help");
                    eprintln!("-------------");
                    return;
                }
            };

            let price: f64 = match price_str.parse() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("--- ERROR ---");
                    eprintln!("price must be a number, got: {}", price_str);
                    eprintln!("-------------");
                    return;
                }
            };
            println!("price as number: {} $", price);

            let category = match args.next() {
                Some(v) => v,

                None => {
                    eprintln!("--- ERROR ---");
                    eprintln!("Missing <category>. Usage: Cargo run -- add <price> <category>");
                    eprintln!("if you don't know the commands use: Cargo run -- help");
                    eprintln!("-------------");
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
                    eprintln!("--- ERROR ---");
                    eprintln!("No budget.csv found (no entries yet).");
                    eprintln!("-------------");
                }
            }
        }
        "help" => {
            help();
        }

        "summary" => {
            println!("--- SUMMARY ---");

            let content = match fs::read_to_string("budget.csv") {
                Ok(c) => c,
                Err(_) => {
                    eprintln!("--- ERROR ---");
                    eprintln!("No budget.csv found or could not read it.");
                    eprintln!("-------------");
                    return;
                }
            };

            // category -> total price
            let mut totals: HashMap<String, f64> = HashMap::new();
            let mut grand_total: f64 = 0.0;

            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                if let Some((category_part, rest)) = line.split_once(':') {
                    let category = category_part.trim();

                    // remove '$' and spaces from the right part
                    let price_str = rest.replace('$', "").trim().to_string();

                    if let Ok(value) = price_str.parse::<f64>() {
                        // add to that category’s total
                        let entry = totals.entry(category.to_string()).or_insert(0.0);
                        *entry += value;

                        // also add to overall total
                        grand_total += value;
                    } else {
                        eprintln!("--- NOTICE ---");
                        eprintln!("Skipping line with invalid number: {}", line);
                        eprintln!("-------------");
                    }
                } else {
                    eprintln!("--- NOTICE ---");
                    eprintln!("Skipping malformed line (no ':'): {}", line);
                    eprintln!("-------------");
                }
            }

            if totals.is_empty() {
                eprintln!("--- NOTICE ---");
                println!("No valid entries found in budget.csv");
                eprintln!("-------------");
                return;
            }

            println!();
            for (category, sum) in &totals {
                println!("{:<12} {:>8.2} $", category, sum);
            }
            println!("------------------------");
            println!("TOTAL:       {:>8.2} $", grand_total);
        }

        _ => {
            eprintln!("--- ERROR ---");
            eprintln!("Unknown command");
            eprintln!("-------------");
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
            eprintln!("--- ERROR ---");
            eprintln!("Could not open budget.csv: {}", e);
            eprintln!("-------------");
            return;
        }
    };

    // very simple CSV line: price,category,note\n
    let line = format!("{}: {} $\n", category, price);

    if let Err(e) = file.write_all(line.as_bytes()) {
        eprintln!("--- ERROR ---");
        eprintln!("Failed to write to file: {}", e);
        eprintln!("-------------");
    }
}
    fn help() {
        eprintln!("--- HELP ---");
        println!("Welcome. please use one of the following commands:");
        println!("cargo run -- add <amount> <category>");
        println!("cargo run -- list");
        println!("cargo run -- summary");
        eprintln!("------------");
    }