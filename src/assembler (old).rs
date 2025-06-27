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

fn jmp_parser_choice(line: &str) -> String {
    let total_operation: String;

    if line.starts_with("JMPEQ") {
        total_operation = jmpeq_filter(&line[5..].to_string());
    } else if line.starts_with("JMPNE") {
        total_operation = jmp_conditional("1".to_string(), line[5..].to_string());
    } else if line.starts_with("JMPGE") {
        total_operation = jmp_conditional("2".to_string(), line[5..].to_string())
    } else if line.starts_with("JMPLE") {
        total_operation = jmp_conditional("3".to_string(), line[5..].to_string());
    } else if line.starts_with("JMPGT") {
        total_operation = jmp_conditional("4".to_string(), line[5..].to_string());
    } else if line.starts_with("JMPLT") {
        total_operation = jmp_conditional("5".to_string(), line[5..].to_string());
    } else if line.starts_with("JMP") {
        total_operation = jmp_unconditional(&line[3..].to_string());
    } else {
        eprintln!("Error: Invalid jump instruction.");
        return "33".to_string(); // Error code
    }
    total_operation
}

fn jmp_conditional(opcode: String, line: String) -> String {
    let binding: String = line.replace("R", "");
    let jmpne_parts: Vec<&str> = binding.split(',').collect();

    if jmpne_parts.len() != 2 {
        eprintln!("Error: Conditional jump instruction must have exactly two parts.");
        return "33".to_string(); // Error code
    }

    let less_than_value: String = jmpne_parts[0].trim().to_string();
    let jump_to: String = jmpne_parts[1].trim().to_string();

    return format! {"0xF{}{}{}", opcode, less_than_value, jump_to};
}

fn jmpeq_filter(line: &str) -> String {
    let jmpeq_parts: Vec<&str> = line.split(',').collect();

    if jmpeq_parts[0].contains("R") {
        return jmp_conditional("0".to_string(), line.to_string());
    }

    if jmpeq_parts.len() != 2 {
        eprintln!("Error: JMPEQ instruction must have exactly two parts.");
        return "33".to_string(); // Error code
    }

    let equal_value = jmpeq_parts[0].trim().to_string();
    let jump_to = jmpeq_parts[1].trim().to_string();

    return format!("0xB{}0{}", equal_value, jump_to);
}

fn jmp_unconditional(line: &str) -> String {
    let binding: String = line.trim().to_string();

    if binding.starts_with("R") {
        let address: String = binding[1..].to_string();
        return format!("0xF00{}", address);
    } else {
        let address: String = binding.to_string();
        return format!("0xB0{}", address);
    }
}

// fn data_parser(line: &str) -> i32 {
//     let opcode: i32 = 0x00;
//     let cleaned_line: String = line.trim().to_string();
//     let byte_line: i32 = match cleaned_line.parse::<i32>() {
//         Ok(value) => value,
//         Err(_) => {
//             eprintln!("Error: Invalid data format in line '{}'", line);
//             return 33; // Error code
//         }
//     };

//     if byte_line < 0 || byte_line > 255 {
//         eprintln!(
//             "Error: Value out of range for 16-bit signed integer: {}",
//             byte_line
//         );
//         return 33; // Error code
//     }

//     let assembled_line: i32 = (opcode << 12) | (byte_line & 0x0FFF);
//     println!("Assembled data line: {:#X}", assembled_line);
//     return assembled_line;
// }

fn mov_parser_choice(line: &str) -> String {
    let binding: String = line
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");
    let parts: Vec<&str> = binding.split("->").collect();

    if parts.len() != 2 {
        eprintln!("Error: MOV instruction must have exactly two parts.");
        return "33".to_string(); // Error code
    }

    if parts[0].starts_with("R") && parts[1].starts_with("R") {
        let register_1_address: String = parts[0].trim().replace("R", "");
        let register_2_address: String = parts[1].trim().replace("R", "");
        return format!("0x40{}{}", register_1_address, register_2_address);
    } else if parts[0].starts_with("[R") && parts[1].starts_with("R") {
        let memory_cell_address: String = parts[0]
            .trim()
            .replace("[", "")
            .replace("]", "")
            .replace("r", "");
        let register_address: String = parts[1].trim().replace("R", "");
        return format!("0x10{}{}", memory_cell_address, register_address);
    } else if parts[0].starts_with("R") && parts[1].starts_with("[R") {
        let register_address: String = parts[0].trim().replace("R", "");
        let memory_cell_address: String = parts[1]
            .trim()
            .replace("[", "")
            .replace("]", "")
            .replace("R", "");
        return format!("0xE0{}{}", register_address, memory_cell_address);
    } else if parts[0].starts_with("[") && parts[1].starts_with("R") {
        let value: String = parts[0]
            .trim()
            .replace("[", "")
            .replace("]", "")
            .replace("R", "");
        let register_address: String = parts[1].trim().replace("R", "");
        return format!("0x2{}0{}", value, register_address);
    } else if parts[0].starts_with("R") && parts[1].starts_with("[") {
        let register_address: String = parts[0].trim().replace("R", "");
        let memory_cell_address: String = parts[1].trim().replace("[", "").replace("]", "");
        return format!("0x3{}{}", register_address, memory_cell_address);
    } else {
        eprintln!("Error: Invalid MOV instruction format.");
        return "33".to_string(); // Error code
    }
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
        } else if line.starts_with("MOV") {
            assembled_line = mov_parser_choice(&line[3..]);
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
        } else if line.starts_with("JMP") {
            assembled_line = jmp_parser_choice(&line);
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
