use std::{fmt::format, panic::AssertUnwindSafe};

fn mov_r_to_r_parser(parts: Vec<&str>) -> i32 {
    let opcode: i32 = 0x40; // MOV opcode
    let reg1: i32 = i32::from_str_radix(&parts[0][1..], 16).unwrap_or(33);
    let reg2: i32 = i32::from_str_radix(&parts[1][1..], 16).unwrap_or(33);

    if reg1 > 15 || reg2 > 15 {
        eprintln!("Error: Register number out of range (0-F).");
        return 33; // Error code
    }

    let assembled_line: i32 = (opcode << 8) | (reg1 << 4) | reg2;
    println!("{:#06X}", assembled_line); // e.g., 0x1402
    return assembled_line;
}

fn mov_mem_to_r_parser(parts: Vec<&str>) -> i32 {
    let opcode: i32 = 0x1;
    let reg: i32 = i32::from_str_radix(&parts[1][1..], 16).unwrap_or(33);
    let combined_opcode: i32 = (opcode << 8) | (reg & 0x0F);

    if reg > 15 {
        eprintln!("Error: Register number out of range (0-F).");
        return 33; // Error code
    }

    let address: i32 = parts[0]
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .parse::<i32>()
        .unwrap_or(0);

    if address < 0 || address > 255 {
        eprintln!("Error: Address out of range (0-255).");
        return 33; // Error code
    }

    let assembled_line: i32 = (combined_opcode << 12) | (address & 0x0FFF);
    println!("{:#06X}", assembled_line); // e.g., 0x1001
    return assembled_line;
}

fn mov_parser_choice(line: &str) -> i32 {
    let binder: String = line
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("");

    println!("Binder: {}", binder);

    let parts: Vec<&str> = binder.split("->").collect();

    if parts.len() != 2 {
        eprintln!("Error: MOV instruction must have exactly two parts.");
        return 33; // Error code
    }

    if parts[0].starts_with("R") && parts[1].starts_with("R") {
        return mov_r_to_r_parser(parts);
    } else if parts[0].starts_with("[") && parts[1].starts_with("R") {
        return mov_mem_to_r_parser(parts);
    } else {
        eprintln!("Error: Invalid MOV instruction format.");
        return 33; // Error code
    }

    33
}

fn rot_parser(line: &str) -> String {
    let binding: String = line
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    let rot_parts: Vec<&str> = binding.split(',').collect();

    if rot_parts.len() != 2 {
        eprintln!("Error: ROT instruction must have exactly two parts.");
        std::process::exit(1);
    }

    if rot_parts[0].starts_with("R") && !rot_parts[1].starts_with("R") {
        let register: String = rot_parts[0].trim().replace("R", "");
        let shift: String = rot_parts[1].trim().to_string();
        return format! {"0xA{}{}0", register, shift};
    }

    std::process::exit(1); // Exit if the format is incorrect
}

fn register_operation(opcode: String, line: &str) -> String {
    let cleaned_line: String = line.replace("R", "").replace("->", "").replace(",", "");

    println!("Line: {}", line);
    println!("Cleaned line: {}", cleaned_line);

    let parts: Vec<&str> = cleaned_line.split_whitespace().collect();
    if parts.len() != 3 {
        eprintln!("Error: instructions must have exactly three parts");
        std::process::exit(1);
    }

    let op1: String = parts[0].to_string();
    let op2: String = parts[1].to_string();
    let op3: String = parts[2].to_string();

    let full_instruction: String = format!("{}{}{}{}", opcode, op1, op2, op3);
    return full_instruction;
}

fn add_parser_new(line: &str) -> String {
    let opcode: String;

    match line.chars().nth(3) {
        Some('I') => opcode = "5".to_string(), // ADDI
        Some('F') => opcode = "6".to_string(), // AD
        _ => {
            eprintln!("Error: Invalid ADD instruction type.");
            return "33".to_string(); // Error code
        }
    }

    let assembled_line: String = register_operation(opcode, &line[4..]).to_string();

    return assembled_line;
}

fn or_parser(line: &str) -> String {
    let opcode: String = "0x7".to_string();
    let assembled_line: String = register_operation(opcode, &line[2..]);
    return assembled_line;
}

fn and_parser(line: &str) -> String {
    let opcode: String = "0x8".to_string();
    let assembled_line: String = register_operation(opcode, &line[2..]);
    return assembled_line;
}

fn xor_parser(line: &str) -> String {
    let opcode: String = "0x9".to_string();
    let assembled_line: String = register_operation(opcode, &line[2..]);
    return assembled_line;
}

fn jmp_eq_parser(line: &str) -> i32 {
    let jmpeq_parts: Vec<&str> = line.split(",").collect();
    let opcode: i32;
    let operand: i32;

    if jmpeq_parts[0].trim().starts_with("R") {
        opcode = 0xF0;
        operand = jmpeq_parts[0]
            .trim()
            .replace("R", "")
            .parse::<i32>()
            .unwrap_or(0);
    } else {
        opcode = 0xB0;
        operand = jmpeq_parts[0].trim().parse::<i32>().unwrap_or(0);
    }

    return (opcode << 12) | (operand & 0x0FFF);
}

fn jmp_ne_parser(line: &str) -> i32 {
    let binding = line.replace("R", "");
    let jmpne_parts: Vec<&str> = binding.split(',').collect();

    let n_reg: i32 = i32::from_str_radix(jmpne_parts[0].trim(), 16).unwrap_or(0);
    let m_reg: i32 = i32::from_str_radix(jmpne_parts[1].trim(), 16).unwrap_or(0);

    let byte1: i32 = (0xF << 4) | n_reg;
    let byte2: i32 = (0x2 << 4) | m_reg;

    let assembled_line: i32 = (byte1 << 8) | byte2;

    println!("Assembled JMPNE line: {:#X}", assembled_line);

    return assembled_line;
}

fn jmp_ge_parser(line: &str) -> i32 {
    let binding = line.replace("R", "");
    let jmpne_parts: Vec<&str> = binding.split(',').collect();

    let n_reg: i32 = i32::from_str_radix(jmpne_parts[0].trim(), 16).unwrap_or(0);
    let m_reg: i32 = i32::from_str_radix(jmpne_parts[1].trim(), 16).unwrap_or(0);

    let byte1: i32 = (0xF << 4) | n_reg;
    let byte2: i32 = (0x3 << 4) | m_reg;

    let assembled_line: i32 = (byte1 << 8) | byte2;

    println!("Assembled JMPNE line: {:#X}", assembled_line);

    return assembled_line;
}

fn jmp_le_parser(line: &str) -> i32 {
    let binding = line.replace("R", "");
    let jmpne_parts: Vec<&str> = binding.split(',').collect();

    let n_reg: i32 = i32::from_str_radix(jmpne_parts[0].trim(), 16).unwrap_or(0);
    let m_reg: i32 = i32::from_str_radix(jmpne_parts[1].trim(), 16).unwrap_or(0);

    let byte1: i32 = (0xF << 4) | n_reg;
    let byte2: i32 = (0x4 << 4) | m_reg;

    let assembled_line: i32 = (byte1 << 8) | byte2;

    println!("Assembled JMPNE line: {:#X}", assembled_line);

    return assembled_line;
}

fn jmp_gt_parser(line: &str) -> i32 {
    let binding = line.replace("R", "");
    let jmpne_parts: Vec<&str> = binding.split(',').collect();

    let n_reg: i32 = i32::from_str_radix(jmpne_parts[0].trim(), 16).unwrap_or(0);
    let m_reg: i32 = i32::from_str_radix(jmpne_parts[1].trim(), 16).unwrap_or(0);

    let byte1: i32 = (0xF << 4) | n_reg;
    let byte2: i32 = (0x5 << 4) | m_reg;

    let assembled_line: i32 = (byte1 << 8) | byte2;

    println!("Assembled JMPNE line: {:#X}", assembled_line);

    return assembled_line;
}

fn jmp_lt_parser(line: &str) -> i32 {
    let binding: String = line.replace("R", "");
    let jmpne_parts: Vec<&str> = binding.split(',').collect();

    let n_reg: i32 = i32::from_str_radix(jmpne_parts[0].trim(), 16).unwrap_or(0);
    let m_reg: i32 = i32::from_str_radix(jmpne_parts[1].trim(), 16).unwrap_or(0);

    let byte1: i32 = (0xF << 4) | n_reg;
    let byte2: i32 = (0x6 << 4) | m_reg;

    let assembled_line: i32 = (byte1 << 8) | byte2;

    println!("Assembled JMPNE line: {:#X}", assembled_line);

    return assembled_line;
}

fn jmp_parser(line: &str) -> i32 {
    let binding: String = line.trim().to_string();

    let opcode: i32;
    let operand: i32;

    if binding.starts_with("R") {
        opcode = 0xF0; // JMP opcode for register
        operand = binding[1..].parse::<i32>().unwrap_or(0);
        let assembled_line: i32 = (opcode << 8) | (operand & 0x0FFF);
        println!("Assembled JMP line: {:#X}", assembled_line);

        return (opcode << 12) | (operand & 0x0FFF);
    } else {
        opcode = 0xB0; // JMP opcode
        operand = binding.parse::<i32>().unwrap_or(0);
        let assembled_line: i32 = (opcode << 8) | (operand & 0x0FFF);
        println!("Assembled JMP line: {:#X}", assembled_line);
        return (opcode << 12) | (operand & 0x0FFF);
    }
}

fn jmp_parser_choice(line: &str) -> i32 {
    let total_operation: i32;

    if line.starts_with("JMPEQ") {
        total_operation = jmp_eq_parser(&line[5..].to_string());
    } else if line.starts_with("JMPNE") {
        total_operation = jmp_ne_parser(&line[5..].to_string());
    } else if line.starts_with("JMPGE") {
        total_operation = jmp_ge_parser(&line[5..].to_string())
    } else if line.starts_with("JMPLE") {
        total_operation = jmp_le_parser(&line[5..].to_string());
    } else if line.starts_with("JMPGT") {
        total_operation = jmp_gt_parser(&line[5..].to_string());
    } else if line.starts_with("JMPLT") {
        total_operation = jmp_lt_parser(&line[5..].to_string());
    } else if line.starts_with("JMP") {
        total_operation = jmp_parser(&line[3..].to_string());
    } else {
        eprintln!("Error: Invalid jump instruction.");
        return 33; // Error code
    }

    total_operation
}

fn data_parser(line: &str) -> i32 {
    let opcode: i32 = 0x00;
    let cleaned_line: String = line.trim().to_string();
    let byte_line: i32 = match cleaned_line.parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: Invalid data format in line '{}'", line);
            return 33; // Error code
        }
    };

    if byte_line < 0 || byte_line > 255 {
        eprintln!(
            "Error: Value out of range for 16-bit signed integer: {}",
            byte_line
        );
        return 33; // Error code
    }

    let assembled_line: i32 = (opcode << 12) | (byte_line & 0x0FFF);
    println!("Assembled data line: {:#X}", assembled_line);
    return assembled_line;
}

fn parse_instructions(lines: Vec<String>) -> Vec<String> {
    let mut assembly_instructions: Vec<String> = Vec::new();

    for line in lines {
        println!("Processing line: {}", line);
        let assembled_line: String;

        if line == "HALT" {
            assembled_line = "0xC000".to_string();
        } else if line == "NOP" {
            assembled_line = "0x0000".to_string()
        // } else if line.starts_with("MOV") {
        //     assembled_line = mov_parser_choice(&line[3..]);
        } else if line.starts_with("ROT") {
            assembled_line = rot_parser(&line[3..]);
        } else if line.starts_with("ADDI") || line.starts_with("ADDF") {
            assembled_line = add_parser_new(&line);
        } else if line.starts_with("OR") {
            assembled_line = or_parser(&line);
        } else if line.starts_with("AND") {
            assembled_line = and_parser(&line[3..]);
        } else if line.starts_with("XOR") {
            assembled_line = xor_parser(&line[3..]);
        // } else if line.starts_with("JMP") {
        //     assembled_line = jmp_parser_choice(&line);
        // } else if line.starts_with("DATA") {
        //     println!("Data line: {}", line);
        //     assembled_line = data_parser(&line[4..]);
        } else {
            assembled_line = "33".to_string();
        }

        println!("Assembled line: {}", assembled_line);

        assembly_instructions.push(assembled_line);
    }

    return assembly_instructions;
}

pub fn assembler(lines: Vec<String>) -> Vec<String> {
    let final_assembler_code: Vec<String> = parse_instructions(lines);
    return final_assembler_code;
}
