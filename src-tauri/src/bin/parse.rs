use std::env;
use std::fs::File;
use std::path::Path;

use youfinance_lib::banking::parsers::csv::parse_csv;
use youfinance_lib::banking::parsers::mta::parse_mta;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin parse -- <path.csv | path.mta>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let path = Path::new(file_path);
    
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            std::process::exit(1);
        }
    };

    if let Some(ext) = path.extension() {
        if ext == "csv" {
            println!("Parsing CSV file: {}", file_path);
            match parse_csv(file, 0) {
                Ok(transactions) => {
                    println!("Successfully parsed {} transactions:", transactions.len());
                    for (i, tx) in transactions.iter().enumerate() {
                        println!("Transaction {}:\n{:#?}\n", i + 1, tx);
                    }
                }
                Err(e) => eprintln!("Failed to parse CSV: {}", e),
            }
        } else if ext == "mta" {
            println!("Parsing MTA file: {}", file_path);
            match parse_mta(file, 0) {
                Ok(transactions) => {
                    println!("Successfully parsed {} transactions:", transactions.len());
                    for (i, tx) in transactions.iter().enumerate() {
                        println!("Transaction {}:\n{:#?}\n", i + 1, tx);
                    }
                }
                Err(e) => eprintln!("Failed to parse MTA: {}", e),
            }
        } else {
            eprintln!("Unsupported file extension: {:?}", ext);
        }
    } else {
        eprintln!("File has no extension. Please provide a .csv or .mta file.");
    }
}
