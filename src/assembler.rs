use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static LABEL_ADDRESSES: Lazy<std::sync::RwLock<HashMap<String, u8>>> =
    Lazy::new(|| std::sync::RwLock::new(HashMap::new()));

fn rot_parser(line: &str) -> [u8; 2] {
    let binding: String = line
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    let rot_parts: Vec<&str> = binding.split(',').collect();

    if rot_parts.len() != 2 {
        panic!("Error: ROT instruction must have exactly two parts.");
    }

    if rot_parts[0].starts_with("R") && !rot_parts[1].starts_with("R") {
        let n_reg: u8 = u8::from_str_radix(&rot_parts[0].trim().replace("R", ""), 16)
            .unwrap_or_else(|_| panic!("Error: Invalid register number '{}'", rot_parts[0]));

        let rot_amount: u8 = u8::from_str_radix(&rot_parts[1].trim(), 16)
            .unwrap_or_else(|_| panic!("Error: Invalid rotation amount '{}'", rot_parts[1]));

        if rot_amount > 15 {
            panic!("Error: Rotation amount must be between 0 and 15.");
        }

        let opcode: u8 = 0xA << 4 | (n_reg & 0x0F);
        let operand: u8 = rot_amount & 0x0F;

        return [opcode, operand];
    }

    std::process::exit(1); // Exit if the format is incorrect
}

fn register_operation(add_instruction: u8, line: &str) -> [u8; 2] {
    let cleaned_line: String = line.replace("R", "").replace("->", "").replace(",", "");

    let parts: Vec<&str> = cleaned_line.split_whitespace().collect();
    if parts.len() != 3 {
        panic!("Error: ADD instruction must have exactly three parts.");
    }

    let reg_m: u8 = u8::from_str_radix(parts[0], 16).unwrap_or_else(|_| {
        panic!("Error: Invalid register number '{}'", parts[0]);
    });

    let reg_n: u8 = u8::from_str_radix(parts[1], 16).unwrap_or_else(|_| {
        panic!("Error: Invalid register number '{}'", parts[1]);
    });

    let reg_p: u8 = u8::from_str_radix(parts[2], 16).unwrap_or_else(|_| {
        panic!("Error: Invalid register number '{}'", parts[2]);
    });

    let opcode: u8 = add_instruction | reg_p & 0x0F;
    let operand: u8 = (reg_m & 0x0F) << 4 | (reg_n & 0x0F);

    return [opcode, operand];
}

fn add_parser_new(line: &str) -> [u8; 2] {
    let add_instruction: u8;

    match line.chars().nth(3) {
        Some('I') => add_instruction = 0x05 << 4,
        Some('F') => add_instruction = 0x06 << 4,
        _ => {
            panic!("Error: Invalid ADD instruction format. Expected 'ADDI' or 'ADDF'.");
        }
    }

    return register_operation(add_instruction, &line[4..]);
}

fn or_parser(line: &str) -> [u8; 2] {
    let assembled_line: [u8; 2] = register_operation(0x7 << 4, &line[2..]);
    return assembled_line;
}

fn and_parser(line: &str) -> [u8; 2] {
    let assembled_line: [u8; 2] = register_operation(0x8 << 4, &line[2..]);
    return assembled_line;
}

fn xor_parser(line: &str) -> [u8; 2] {
    let assembled_line: [u8; 2] = register_operation(0x9 << 4, &line[2..]);
    return assembled_line;
}

fn jmp_parser_choice(line: &str) -> [u8; 2] {
    if line.starts_with("JMPEQ") {
        return jmpeq_filter(&line[5..]);
    } else if line.starts_with("JMPNE") {
        return jmp_conditional(0x01, &line[5..]);
    } else if line.starts_with("JMPGE") {
        return jmp_conditional(0x02, &line[5..]);
    } else if line.starts_with("JMPLE") {
        return jmp_conditional(0x03, &line[5..]);
    } else if line.starts_with("JMPGT") {
        return jmp_conditional(0x04, &line[5..]);
    } else if line.starts_with("JMPLT") {
        return jmp_conditional(0x05, &line[5..]);
    } else if line.starts_with("JMP") {
        return jmp_unconditional(&line[3..]);
    } else {
        panic!("Error: Invalid jump instruction '{}'", line);
    }
}

fn jmp_unconditional(line: &str) -> [u8; 2] {
    let binding: String = line.trim().to_string();

    if binding.starts_with("R") {
        let address: u8 =
            u8::from_str_radix(&binding.replace("R", "").trim(), 16).unwrap_or_else(|_| {
                panic!("Error: Invalid jump address '{}'", binding);
            });
        return [0xF0, 0x00 | address & 0x0F];
    } else {
        let address: String = binding.to_string();
        return [
            0xB0,
            u8::from_str_radix(&address.trim(), 16).unwrap_or_else(|_| {
                panic!("Error: Invalid jump address '{}'", address);
            }),
        ];
    }
}

fn jmpeq_filter(line: &str) -> [u8; 2] {
    let jmpeq_parts: Vec<&str> = line.split(',').map(str::trim).collect();

    if jmpeq_parts.len() != 2 {
        panic!("Error: JMPEQ instruction must have exactly two parts.");
    }

    if jmpeq_parts[0].starts_with("R") {
        return jmp_conditional(0x00, &line);
    }

    let jmp_value: u8 = u8::from_str_radix(&jmpeq_parts[0].trim(), 16).unwrap_or_else(|_| {
        panic!("Error: Invalid jump value '{}'", jmpeq_parts[0]);
    });

    let register: u8 = u8::from_str_radix(&jmpeq_parts[1].trim().replace("R", ""), 16)
        .unwrap_or_else(|_| {
            panic!("Error: Invalid register number '{}'", jmpeq_parts[1]);
        });

    return [0xB << 4 | register & 0x0F, jmp_value];
}

fn jmp_conditional(instruction: u8, line: &str) -> [u8; 2] {
    let binding: String = line.replace("R", "");
    let jmpne_parts: Vec<&str> = binding.split(',').map(str::trim).collect();

    if jmpne_parts.len() != 2 {
        panic!("Error: Jump instruction must have exactly two parts.");
    }

    let reg_n: u8 = u8::from_str_radix(&jmpne_parts[0], 16).unwrap_or_else(|_| {
        panic!("Error: Invalid register number '{}'", jmpne_parts[0]);
    });
    let reg_m: u8 = u8::from_str_radix(&jmpne_parts[1], 16).unwrap_or_else(|_| {
        panic!("Error: Invalid register number '{}'", jmpne_parts[1]);
    });

    let opcode: u8 = 0xF << 4 | (reg_m & 0x0F);
    let operand: u8 = instruction << 4 | (reg_n & 0x0F);
    return [opcode, operand];
}

fn mov_grab_register(register: &str) -> u8 {
    let register_address: u8 = u8::from_str_radix(&register.trim().replace("R", ""), 16)
        .unwrap_or_else(|_| {
            panic!("Error: Invalid register number '{}'", register);
        });
    return register_address;
}

fn mov_grab_memory(memory: &str) -> u8 {
    let memory_address: u8 = u8::from_str_radix(
        &memory
            .trim()
            .replace("[", "")
            .replace("]", "")
            .replace("R", ""),
        16,
    )
    .unwrap_or_else(|_| {
        panic!("Error: Invalid memory address '{}'", memory);
    });
    return memory_address;
}

fn mov_parser_choice(line: &str) -> [u8; 2] {
    let binding: String = line
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    let parts: Vec<&str> = binding.split("->").map(str::trim).collect();

    if parts.len() != 2 {
        panic!("Error: MOV instruction must have exactly two parts.");
    }

    if parts[0].starts_with("R") && parts[1].starts_with("R") {
        // Register to Register
        let register_m: u8 = mov_grab_register(parts[0]);
        let register_n: u8 = mov_grab_register(parts[1]);
        return [0x40, (register_m << 4) | register_n];
    } else if parts[0].starts_with("R") && parts[1].starts_with("[R") {
        // Register to indirect Memory
        let register_m: u8 = mov_grab_register(parts[0]);
        let memory_address: u8 = mov_grab_memory(parts[1]);
        return [0xE0, (register_m << 4) | memory_address];
    } else if parts[0].starts_with("[R") && parts[1].starts_with("R") {
        // Memory (from Register) to Register
        let memory_address: u8 = mov_grab_memory(parts[0]);
        let register_n: u8 = mov_grab_register(parts[1]);
        return [0xD0, (register_n << 4) | memory_address];
    } else if parts[0].starts_with("[") && parts[1].starts_with("R") {
        // Memory to Register
        let memory_address: u8 = mov_grab_memory(parts[0]);
        let register_n: u8 = mov_grab_register(parts[1]);
        return [(0x1 << 4) | (register_n & 0x0F), memory_address];
    } else if parts[0].starts_with("R") && parts[1].starts_with("[") {
        // Register to Memory
        let register_m: u8 = mov_grab_register(parts[0]);
        let memory_address: u8 = mov_grab_memory(parts[1]);
        return [(0x3 << 4) | (register_m & 0x0F), memory_address];
        // return [0x30, (register_m << 4) | memory_address];
    } else if (!parts[0].starts_with("[") || !parts[0].starts_with("R"))
        && parts[1].starts_with("R")
    {
        // Immediate value to Register
        let memory_address: u8 = mov_grab_memory(parts[0]);
        let register_n: u8 = mov_grab_register(parts[1]);
        return [(0x2 << 4) | (register_n & 0x0F), memory_address];
        // return [0x20, (register_n << 4)
    } else {
        panic!("Error: Invalid MOV instruction format '{}'", binding);
    }
}

fn data_parser(line: &str) -> [u8; 2] {
    let trimmed: String = line.trim().to_string();

    if trimmed.chars().all(|c| c == '0' || c == '1') && trimmed.len() == 8 {
        // Binary data (e.g. "DATA 11111111")
        let binding: u8 = u8::from_str_radix(&trimmed, 2).unwrap_or_else(|_| {
            panic!("Error: Invalid binary data value '{}'", trimmed);
        });
        //println!("Processing DATA line (binary): {}", binding);
        return [binding, 0x00];
    } else {
        // Hexadecimal data (e.g. "DATA FF")
        let binding: u8 = u8::from_str_radix(line.trim(), 16).unwrap_or_else(|_| {
            panic!("Error: Invalid hexadecimal data value '{}'", line);
        });
        return [binding, 0x00];
    }
}

fn parse_instructions(lines: Vec<String>) -> Vec<String> {
    let assembly_instructions: Vec<String> = Vec::new();

    for line in lines {
        //println!("Processing line: {}", line);
        let mut code: Vec<u8> = Vec::new();

        if line == "HALT" {
            code.extend_from_slice(&[0xC0, 0x00]);
        } else if line == "NOP" {
            code.extend_from_slice(&[0x0F, 0xFF]);
        } else if line.starts_with("MOV") {
            code.extend_from_slice(&mov_parser_choice(&line[3..]));
        } else if line.starts_with("ROT") {
            code.extend_from_slice(&rot_parser(&line[3..]));
        } else if line.starts_with("ADDI") || line.starts_with("ADDF") {
            code.extend_from_slice(&add_parser_new(&line));
        } else if line.starts_with("OR") {
            code.extend_from_slice(&or_parser(&line));
        } else if line.starts_with("AND") {
            code.extend_from_slice(&and_parser(&line[3..]));
        } else if line.starts_with("XOR") {
            code.extend_from_slice(&xor_parser(&line[3..]));
        } else if line.starts_with("JMP") {
            code.extend_from_slice(&jmp_parser_choice(&line));
        } else if line.starts_with("DATA") {
            code.extend_from_slice(&data_parser(&line[4..]));
        } else {
            panic!("Error: Invalid instruction '{}'", line);
        }

        for byte in code {
            let hex_byte: String = format!("{:02X}", byte);
            println!("Hex byte: {}", hex_byte);
        }
    }
    println!("Label addresses: {:?}", *LABEL_ADDRESSES.read().unwrap());
    // Convert the label addrress values to hex format
    let label_addresses: Vec<String> = LABEL_ADDRESSES
        .read()
        .unwrap()
        .iter()
        .map(|(label, &address)| format!("{}: {:02X}", label, address))
        .collect();

    println!("Label addresses in hex: {:?}", label_addresses.join(", "));

    return assembly_instructions;
}

pub fn assembler(cleaned_lines: Vec<String>, label_addresses: HashMap<String, u8>) -> Vec<String> {
    // Store the label addresses in the static variable using write lock
    {
        let mut map = LABEL_ADDRESSES.write().unwrap();
        *map = label_addresses;
    }

    let final_assembler_code: Vec<String> = parse_instructions(cleaned_lines);
    return final_assembler_code;
}
