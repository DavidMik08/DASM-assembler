use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};

struct Definition {
    name: String,
    addr: u32,
}

// Convert instruction to base value + leading flag offsets
fn asm_inst(inst: &str) -> i32 {
    let modifiers: Vec<&str> = inst.split('_').collect();
    
    let mut base = match modifiers[0] {
        "add" => 0, "sub" => 1, "or" => 2, "nor" => 3,
        "and" => 4, "nand" => 5, "xor" => 6, "xnor" => 7,
        "biz" => 8, "bnz" => 9, "bic" => 10, "bnc" => 11,
        "brk" => 15,
        _ => -1,
    };

    if base == -1 {
        return -1;
    }
    
    let mut shift_flag = false;
    let mut ignore_flag = false;
    
    for part in modifiers {
        if part == "shr" {
            shift_flag = true;
        } else if part == "igf" {
            ignore_flag = true;
        }
    }
    
    if shift_flag {
        base += 16;
    }
    if ignore_flag {
        base += 32;
    }

    base
}

// Convert register/memory or numeric placeholder
fn asm_io(arg: &str) -> Option<i32> {
    match arg {
        "r0" => Some(0), "r1" => Some(1), "r2" => Some(2), "r3" => Some(3),
        "r4" => Some(4), "r5" => Some(5), "ram" => Some(6), "stk" => Some(7),
        _ => {
            None
        }
    }
}

fn handle_labels(line: &str, labels: &mut Vec<Definition>, len: &mut u32) {
    let line = line.trim();
    if line.is_empty() || line.starts_with(';') {
        return; // skip empty or comment lines
    }

    // Handle data directives: db/dw/dd/dq
    let count = match &line[..2] {
        "db" => 1,
        "dw" => 2,
        "dd" => 4,
        "dq" => 8,
        _ => 0,
    };
    
    if count > 0 {
        *len += count;
        return;
    }
    

    let arg1 = line.split_whitespace().collect::<Vec<_>>()[0];
    
    if arg1.ends_with(':') {
        let label = Definition {
            name: arg1[..arg1.len() - 1].to_string(), // Remove colon
            addr: *len,
        };
        
        // Print using a reference
        println!("label: {}, address: {}", label.name, label.addr);
    
        // Then move into vector
        labels.push(label);

        return
    }
    
    // Instruction line
    let mut parts = line.split_whitespace();

    if let Some(inst) = parts.next() {
        let inst_index = asm_inst(inst);
        if inst_index != -1 {
            *len += 4;
        }
    }
    return;
}

fn assemble_line(line: &str, program: &mut String, labels: &mut Vec<Definition>) {
    let line = line.trim();
    if line.is_empty() || line.starts_with(';') {
        return; // skip empty or comment lines
    }

    // Handle data directives: db/dw/dd/dq
    let count = match &line[..2] {
        "db" => 1,
        "dw" => 2,
        "dd" => 4,
        "dq" => 8,
        _ => 0,
    };

    if count > 0 {
        let value_str = line[2..].trim();
        let value = match value_str.parse::<i64>() {
            Ok(v) => v, 
            Err(_) => {
                eprintln!("Failed to parse value: '{}'", value_str);
                return;}
        };

        for i in 0..count {
            let byte = ((value >> (i * 8)) & 0xFF) as u8;
            program.push_str(&format!("{:02x}", byte).to_string());
        }
        
        return;
    }
    
    

    let arg1 = line.split_whitespace().collect::<Vec<_>>()[0];
    
    if arg1.ends_with(':') {
        return
    }

    // Instruction line
    let mut parts = line.split_whitespace();


    if let Some(inst) = parts.next() {
        let mut inst_index = asm_inst(inst);
        if inst_index == -1 {
            eprintln!("Skipping unknown instruction: '{}'", inst);
            return;
        }
        let args: Vec<&str> = parts.collect();
        /*
        // Numeric argument flags
        if args.get(0).map(|s| s.parse::<i32>().is_ok()).unwrap_or(false) || labels.iter().find(|def| def.name == args.get(0).map(|s| s.to_string()).unwrap_or_default()).is_some() {
            inst_index += 128; // im1
        }
        if args.get(1).map(|s| s.parse::<i32>().is_ok()).unwrap_or(false) || labels.iter().find(|def| def.name == args.get(1).map(|s| s.to_string()).unwrap_or_default()).is_some() {
            inst_index += 64; // im2
        }
        */
        // Numeric argument flags
        if args.get(0).map(|s| {
            let cleaned = s.trim_end_matches(".Low").trim_end_matches(".Mid").trim_end_matches(".High");
            cleaned.parse::<i32>().is_ok() || labels.iter().any(|def| def.name == cleaned)
        }).unwrap_or(false) { inst_index += 128; }
        
        if args.get(1).map(|s| {
            let cleaned = s.trim_end_matches(".Low").trim_end_matches(".Mid").trim_end_matches(".High");
            cleaned.parse::<i32>().is_ok() || labels.iter().any(|def| def.name == cleaned)
        }).unwrap_or(false) { inst_index += 64; }  
        program.push_str(&format!("{:02x}", inst_index).to_string());


        
        // Write arguments
        for part in args {
            let mut new_part = part;
            let mut byte_select: u8 = 0;
            if part.ends_with(".Low") {
                new_part = &part[..part.len() - 4];
            } else if part.ends_with(".Mid") {
                new_part = &part[..part.len() - 4];
                byte_select = 8;
            } else if part.ends_with(".High") {
                new_part = &part[..part.len() - 5];
                byte_select = 16;
            }
            if let Some(val) = asm_io(new_part) {
                program.push_str(&format!("{:02x}", val).to_string());
            } else if new_part.parse::<i32>().is_ok() {
                let val = (new_part.parse::<i32>().unwrap() >> byte_select) & 0xFF;
                program.push_str(&format!("{:02x}", val).to_string());
            } else if let Some(def) = labels.iter().find(|def| def.name == new_part) {
                let val = (def.addr >> byte_select) & 0xFF;
                program.push_str(&format!("{:02x}", val).to_string())
            } else {
                eprintln!("Skipping unknown argument: '{}'", new_part);
                return;
            }
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("[ERROR] Not enough arguments! Provide input file.");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "No input file"));
    }

    let in_file = File::open(&args[1])?;
    let output_file_name = if args.len() > 3 && args[2] == "-o" {
        args[3].clone()
    } else {
        "out.hex".to_string()
    };

    let mut out_file = File::create(output_file_name)?;

    let mut len: u32 = 0;

    let reader = BufReader::new(in_file);
    let mut program: String = String::new();
    let mut labels: Vec<Definition> = Vec::new();
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    for line in &lines {
        handle_labels(&line, &mut labels, &mut len)
    }

    for line in &lines {
        assemble_line(&line, &mut program, &mut labels);
    }
    println!("LEN: {}", len);

    let _ = write!(out_file, "{}", &format!("{:06x}", len));
    let _ = write!(out_file, "{}", program);

    Ok(())
}
