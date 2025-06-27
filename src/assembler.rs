fn exit_program() {
    eprintln!("Exiting program due to an error.");
    std::process::exit(1);
}

fn rot_parser(line: &str) -> [u8; 2] {
    let binding: String = line
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    println!("Processing ROT line: {}", binding);

    let rot_parts: Vec<&str> = binding.split(',').collect();

    if rot_parts.len() != 2 {
        eprintln!("Error: ROT instruction must have exactly two parts.");
        std::process::exit(1);
    }

    if rot_parts[0].starts_with("R") && !rot_parts[1].starts_with("R") {
        let n_reg: u8 = u8::from_str_radix(&rot_parts[0].trim().replace("R", ""), 16)
            .unwrap_or_else(|_| {
                eprintln!("Error: Invalid register number '{}'", rot_parts[0]);
                std::process::exit(1);
            });

        let rot_amount: u8 = u8::from_str_radix(&rot_parts[1].trim(), 16).unwrap_or_else(|_| {
            eprintln!("Error: Invalid rotation amount '{}'", rot_parts[1]);
            std::process::exit(1);
        });

        if rot_amount > 15 {
            eprintln!("Error: Rotation amount must be between 0 and 15.");
            std::process::exit(1);
        }

        let opcode: u8 = 0xA << 4 | (n_reg & 0x0F);
        let operand: u8 = rot_amount & 0x0F;

        return [opcode, operand];
    }

    std::process::exit(1); // Exit if the format is incorrect
}

fn register_operation(add_instruction: u8, line: &str) -> [u8; 2] {
    let cleaned_line: String = line.replace("R", "").replace("->", "").replace(",", "");

    println!("Line: {}", line);
    println!("Cleaned line: {}", cleaned_line.to_string());

    let parts: Vec<&str> = cleaned_line.split_whitespace().collect();
    if parts.len() != 3 {
        eprintln!("Error: instructions must have exactly three parts");
        std::process::exit(1);
    }

    let reg_m: u8 = u8::from_str_radix(parts[0], 16).unwrap_or_else(|_| {
        eprintln!("Error: Invalid register number '{}'", parts[0]);
        std::process::exit(1);
    });

    let reg_n: u8 = u8::from_str_radix(parts[1], 16).unwrap_or_else(|_| {
        eprintln!("Error: Invalid register number '{}'", parts[1]);
        std::process::exit(1);
    });

    let reg_p: u8 = u8::from_str_radix(parts[2], 16).unwrap_or_else(|_| {
        eprintln!("Error: Invalid register number '{}'", parts[2]);
        std::process::exit(1);
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
            eprintln!("Error: Invalid ADD instruction type.");
            std::process::exit(1);
        }
    }

    let assembled_line: [u8; 2] = register_operation(add_instruction, &line[4..]);

    return assembled_line;
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
        eprintln!("Error: Invalid jump instruction.");
        std::process::exit(1);
    }
}

fn jmp_unconditional(line: &str) -> [u8; 2] {
    let binding: String = line.trim().to_string();
    println!("Processing JMP line: {}", binding);

    if binding.starts_with("R") {
        let address: u8 =
            u8::from_str_radix(&binding.replace("R", "").trim(), 16).unwrap_or_else(|_| {
                eprintln!("Error: Invalid register number '{}'", binding);
                std::process::exit(1);
            });
        return [0xF0, 0x00 | address & 0x0F];
    } else {
        let address: String = binding.to_string();
        return [
            0xB0,
            u8::from_str_radix(&address.trim(), 16).unwrap_or_else(|_| {
                eprintln!("Error: Invalid address '{}'", address);
                std::process::exit(1);
            }),
        ];
    }
}

fn jmpeq_filter(line: &str) -> [u8; 2] {
    let jmpeq_parts: Vec<&str> = line.split(',').map(str::trim).collect();

    println!("Processing JMPEQ line: {}", line);
    println!("JMPEQ parts: {:?}", jmpeq_parts);

    if jmpeq_parts.len() != 2 {
        eprintln!("Error: JMPEQ instruction must have exactly two parts.");
        std::process::exit(1);
    }

    if jmpeq_parts[0].starts_with("R") {
        return jmp_conditional(0x00, &line);
    }

    println!("JMPEQ parts: {:?}", jmpeq_parts);

    let jmp_value: u8 = u8::from_str_radix(&jmpeq_parts[0].trim(), 16).unwrap_or_else(|_| {
        eprintln!("Error: Invalid jump address '{}'", jmpeq_parts[0]);
        std::process::exit(1);
    });

    let register: u8 = u8::from_str_radix(&jmpeq_parts[1].trim().replace("R", ""), 16)
        .unwrap_or_else(|_| {
            eprintln!("Error: Invalid register number '{}'", jmpeq_parts[1]);
            std::process::exit(1);
        });

    println!("Jump value: {}, Register: {}", jmp_value, register);

    return [0xB << 4 | register & 0x0F, jmp_value];
}

fn jmp_conditional(instruction: u8, line: &str) -> [u8; 2] {
    let binding: String = line.replace("R", "");
    let jmpne_parts: Vec<&str> = binding.split(',').map(str::trim).collect();

    println!("Processing conditional jump line: {}", binding);
    println!("Conditional jump parts: {:?}", jmpne_parts);
    println!("Instruction: {:02X}", instruction);

    if jmpne_parts.len() != 2 {
        eprintln!("Error: Conditional jump instruction must have exactly two parts.");
        std::process::exit(1);
    }

    let reg_n: u8 = u8::from_str_radix(&jmpne_parts[0], 16).unwrap_or_else(|_| {
        eprintln!("Error: Invalid register number '{}'", jmpne_parts[0]);
        std::process::exit(1);
    });
    let reg_m: u8 = u8::from_str_radix(&jmpne_parts[1], 16).unwrap_or_else(|_| {
        eprintln!("Error: Invalid register number '{}'", jmpne_parts[1]);
        std::process::exit(1);
    });

    println!("Register N: {}, Register M: {}", reg_n, reg_m);
    println!("Register N: {:02X}, Register M: {:02X}", reg_n, reg_m);
    println!("Instruction: {:02X}", instruction);

    let opcode: u8 = 0xF << 4 | (reg_m & 0x0F);
    let operand: u8 = instruction << 4 | (reg_n & 0x0F);
    return [opcode, operand];
}

fn mov_grab_register(register: &str) -> u8 {
    let register_address: u8 = u8::from_str_radix(&register.trim().replace("R", ""), 16)
        .unwrap_or_else(|_| {
            eprintln!("Error: Invalid register number '{}'", register);
            std::process::exit(1);
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
        eprintln!("Error: Invalid memory address '{}'", memory);
        std::process::exit(1);
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

    println!("Processing MOV line: {}", binding);
    println!("Parts: {:?}", parts);

    if parts.len() != 2 {
        eprintln!("Error: MOV instruction must have exactly two parts.");
        std::process::exit(1);
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
        eprintln!("Error: Invalid MOV instruction format.");
        std::process::exit(1);
    }
}

fn parse_instructions(lines: Vec<String>) -> Vec<String> {
    let assembly_instructions: Vec<String> = Vec::new();
    // let mut assembly_instructions_code: Vec<Vec<u8>> = Vec::new();

    for line in lines {
        println!("Processing line: {}", line);
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
        // } else if line.starts_with("DATA") {
        //     println!("Data line: {}", line);
        //     assembled_line = data_parser(&line[4..]);
        } else {
            eprintln!("Error: Invalid instruction '{}'", line);
            std::process::exit(1);
        }

        for byte in code {
            let hex_byte: String = format!("{:02X}", byte);
            println!("Hex byte: {}", hex_byte);
        }

        // assembly_instructions_code.extend(code);
    }

    return assembly_instructions;
}

pub fn assembler(lines: Vec<String>) -> Vec<String> {
    let final_assembler_code: Vec<String> = parse_instructions(lines);
    return final_assembler_code;
}
