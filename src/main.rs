use std::env;
use std::fs::File;
use std::io::{self, Error, BufReader, BufRead};
use std::io::prelude::*;



fn assemble_line(line: String, out_file: &mut File) {
    println!("{}", line);
    // Using a match statement for clarity
    let count = match &line[..2] {
        "db" => 1,
        "dw" => 2,
        "dd" => 4,
        "dq" => 8,
        _ => 0,
    };

    if count > 0 {
        // Use string slice instead of modifying the original string
        let value_str = line[2..].trim();
    
        // Parse once instead of in each loop iteration
        let value = match value_str.parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                // Handle parse error appropriately
                eprintln!("Failed to parse value: {}", value_str);
                return; // or handle error as needed
            }
        };
    
        // Write all bytes at once
        for i in 0..count {
            let byte = (value >> (i * 8)) & 0xFF;
            // Format to buffer and write
            write!(out_file, "{:02x}", byte).expect("Failed to write to file");
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Error::new(io::ErrorKind::InvalidInput, "[ERROR] Not enough arguments! \nPlease provide an input file!()"));
    }
    let in_file: File = match File::open(&args[1]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[ERROR] Couldn't open file '{}'!\n{e}!", &args[1]);
            return Err(e);
        }
    };

    let mut output_specified: bool = false;
    let mut output_file_name = String::new();
    let mut skip: bool = false;
    for i in 2..args.len() {
        if !skip {
            match args[i].as_str() {
                "-o" => {
                    output_specified = true;
                    output_file_name = args[i+1].clone();
                    skip = true;
                },
                &_ => todo!(),
            }
        } else {
            skip = false;
        }
    }

    
    
    let mut out_file: File = {
        if output_specified {
            File::create(output_file_name)?
        } else {
            File::create("out.hex")?
        }
    };

    let reader = BufReader::new(in_file);
    for line in reader.lines() {
        assemble_line(line?.trim().to_string() , &mut out_file);
    }


    Ok(())
}
