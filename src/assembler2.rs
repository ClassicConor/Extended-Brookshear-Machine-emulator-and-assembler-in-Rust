use once_cell::sync::Lazy;
use std::collections::HashMap;
pub static LABEL_ADDRESSES: Lazy<std::sync::RwLock<HashMap<String, u8>>> =
    Lazy::new(|| std::sync::RwLock::new(HashMap::new()));

pub static REGISTER_OPERATION_INSTRUCTIONS: Lazy<HashMap<&str, u8>> = Lazy::new(|| {
    HashMap::from([
        ("ADDI", 0x50),
        ("ADDF", 0x60),
        ("OR", 0x7),
        ("AND", 0x80),
        ("XOR", 0x90),
    ])
});

pub static CONDITIONAL_JUMP_INSTRUCTIONS: Lazy<HashMap<&str, u8>> = Lazy::new(|| {
    HashMap::from([
        ("JMPEQ", 0x00),
        ("JMPNE", 0x01),
        ("JMPGE", 0x02),
        ("JMPLE", 0x03),
        ("JMPGT", 0x04),
        ("JMPLT", 0x05),
    ])
});

// fn rot_parser(line: &str) -> [u8; 2] {
//     let binding: String = line
//         .trim()
//         .split_whitespace()
//         .collect::<Vec<&str>>()
//         .join(" ");

//     let rot_parts: Vec<&str> = binding.split(',').collect();

//     if rot_parts.len() != 2 {
//         panic!("Error: ROT instruction must have exactly two parts.");
//     }

//     if rot_parts[0].starts_with("R") && !rot_parts[1].starts_with("R") {
//         let n_reg: u8 = u8::from_str_radix(&rot_parts[0].trim().replace("R", ""), 16)
//             .unwrap_or_else(|_| panic!("Error: Invalid register number '{}'", rot_parts[0]));

//         let rot_amount: u8 = u8::from_str_radix(&rot_parts[1].trim(), 16)
//             .unwrap_or_else(|_| panic!("Error: Invalid rotation amount '{}'", rot_parts[1]));

//         if rot_amount > 15 {
//             panic!("Error: Rotation amount must be between 0 and 15.");
//         }

//         let opcode: u8 = 0xA << 4 | (n_reg & 0x0F);
//         let operand: u8 = rot_amount & 0x0F;

//         return [opcode, operand];
//     }

//     std::process::exit(1); // Exit if the format is incorrect
// }

fn compare_length(rest_length: usize, expected_length: usize) {
    if rest_length != expected_length {
        panic!(
            "Error: Expected {} arguments, but got {}.",
            expected_length, rest_length
        );
    }
}

fn confirm_if_valid_register(instruction_string: &str) -> bool {
    if instruction_string.starts_with("R") {
        let register_number: u8 = u8::from_str_radix(&instruction_string[1..], 16)
            .unwrap_or_else(|_| panic!("Error: Invalid register number '{}'", instruction_string));
        if register_number > 15 {
            panic!("Error: Register number must be between 0 and 15.");
        }
        return true;
    }
    return false;
}

fn remove_r_from_register_string(instruction_string: &str) -> u8 {
    if instruction_string.starts_with("R") {
        let instruction_string: u8 = u8::from_str_radix(&instruction_string[1..], 16)
            .unwrap_or_else(|_| panic!("Error: Invalid register number '{}'", instruction_string));
        if instruction_string > 15 {
            panic!("Error: Register number must be between 0 and 15.");
        }
        return instruction_string;
    }
    panic!(
        "Error: Invalid register '{}'. Register must start with 'R' followed by a number.",
        instruction_string
    );
}

fn confirm_valid_value(value: &str) -> bool {
    if u8::from_str_radix(value, 16).is_ok() {
        return true;
    }
    panic!(
        "Error: Invalid value '{}'. Value must be a hexadecimal number.",
        value
    );
}

fn grab_valid_value(value: &str) -> u8 {
    u8::from_str_radix(value, 16).unwrap_or_else(|_| {
        panic!(
            "Error: Invalid value '{}'. Value must be a hexadecimal number.",
            value
        )
    })
}

fn confirm_valid_memory(memory: &[String]) -> bool {
    if memory.len() == 3 && memory[0] == "[" && memory[2] == "]" {
        return true;
    }
    false
}

fn confirm_equal_strings(instruction_string: &str, expected_string: &str) {
    if instruction_string != expected_string {
        panic!(
            "Error: Expected instruction '{}', but got '{}'.",
            expected_string, instruction_string
        );
    }
}

fn grab_memory_parts(source: &str) -> Vec<String> {
    let source_parts: Vec<String> = source
        .replace("[", " [ ")
        .replace("]", " ] ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    source_parts
}

fn process_rot_instruction(rest: &[String]) -> [u8; 2] {
    compare_length(rest.len(), 3);
    confirm_equal_strings(&rest[1], ",");

    println!("Processing ROT instruction: R{}, {}", rest[0], rest[2]);

    let n_reg: u8 = remove_r_from_register_string(&rest[0]);
    let rot_amount: u8 = u8::from_str_radix(&rest[2], 16)
        .unwrap_or_else(|_| panic!("Error: Invalid rotation amount '{}'", rest[2]));

    if rot_amount > 15 {
        panic!("Error: Rotation amount must be between 0 and 15.");
    }

    let opcode: u8 = 0xA << 4 | (n_reg & 0x0F);
    let operand: u8 = rot_amount & 0x0F;
    [opcode, operand]
}

fn process_register_operation_instructions(instruction_string: &str, rest: &[String]) -> [u8; 2] {
    let instruction_code: u8 = match REGISTER_OPERATION_INSTRUCTIONS.get(instruction_string) {
        Some(&code) => code,
        None => panic!("Error: Invalid instruction '{}'", instruction_string),
    };

    compare_length(rest.len(), 5);

    let reg_n: u8 = remove_r_from_register_string(&rest[0]);
    let reg_m: u8 = remove_r_from_register_string(&rest[2]);
    let reg_p: u8 = remove_r_from_register_string(&rest[4]);

    let opcode: u8 = instruction_code | (reg_p & 0x0F);
    let operand: u8 = (reg_n & 0x0F) << 4 | (reg_m & 0x0F);

    println!(
        "Processing instruction: {} R{}, R{}, R{} -> {:02X}, {:02X}",
        instruction_string, reg_n, reg_m, reg_p, opcode, operand
    );

    return [opcode, operand];
}

fn process_mov_instruction(rest: &[String]) -> [u8; 2] {
    compare_length(rest.len(), 3);
    confirm_equal_strings(&rest[1], "->");

    let part_1_source_parts: Vec<String> = grab_memory_parts(&rest[0]);
    let part_2_source_parts: Vec<String> = grab_memory_parts(&rest[2]);

    if part_1_source_parts.len() == 1 && part_2_source_parts.len() == 1 {
        let binding: [u8; 2] = mov_one_to_one_part(
            part_1_source_parts[0].as_str(),
            part_2_source_parts[0].as_str(),
        );
        return binding;
    } else if part_1_source_parts.len() == 1 && part_2_source_parts.len() == 3 {
        let binding: [u8; 2] =
            mov_one_to_three_parts(part_1_source_parts[0].as_str(), &part_2_source_parts);
        return binding;
    } else if part_1_source_parts.len() == 3 && part_2_source_parts.len() == 1 {
        let bindings: [u8; 2] =
            mov_three_to_one_parts(&part_1_source_parts, part_2_source_parts[0].as_str());
        return bindings;
    } else {
        panic!(
            "1. Error: Invalid MOV instruction format. Expected one part in the first or second position, but got '{}', '{}'.",
            part_1_source_parts[0], part_2_source_parts[0]
        );
    }
}

fn mov_one_to_one_part(part_1: &str, part_2: &str) -> [u8; 2] {
    if confirm_if_valid_register(part_1) && confirm_if_valid_register(part_2) {
        let reg_m: u8 = remove_r_from_register_string(part_1);
        let reg_n: u8 = remove_r_from_register_string(part_2);
        if reg_m > 15 || reg_n > 15 {
            panic!("Error: Register numbers must be between 0 and 15.");
        }
        [0x40, (reg_m << 4) | reg_n]
    } else if confirm_valid_value(part_1) && confirm_if_valid_register(part_2) {
        let value: u8 = grab_valid_value(part_1);
        let reg_n: u8 = remove_r_from_register_string(part_2);
        [0x20 | (reg_n & 0x0F), value]
    } else {
        panic!(
            "2. Error: Invalid MOV instruction format. Expected one part in the first or second position, but got '{}', '{}'.",
            part_1, part_2
        );
    }
}

fn mov_one_to_three_parts(part_1: &str, part_2: &[String]) -> [u8; 2] {
    if confirm_if_valid_register(part_1) && confirm_valid_memory(part_2) {
        if confirm_if_valid_register(part_2[1].as_str()) {
            let reg_m: u8 = remove_r_from_register_string(part_2[1].as_str());
            let reg_n: u8 = remove_r_from_register_string(part_1);
            [0xE0, (reg_m << 4) | reg_n]
        } else if confirm_valid_value(part_2[1].as_str()) {
            let value: u8 = grab_valid_value(part_2[1].as_str());
            let reg_n: u8 = remove_r_from_register_string(part_1);
            [0x30 | (reg_n & 0x0F), value]
        } else {
            panic!(
                "3. Error: Invalid MOV instruction format. Expected one part in the first or second position, but got '{}', '{}'.",
                part_1, part_2[1]
            );
        }
    } else {
        panic!(
            "4. Error: Invalid MOV instruction format. Expected one part in the first or second position, but got '{}', '{:?}'.",
            part_1, part_2
        );
    }
}

fn mov_three_to_one_parts(part_1: &[String], part_2: &str) -> [u8; 2] {
    if confirm_valid_memory(part_1) && confirm_if_valid_register(part_2) {
        if confirm_if_valid_register(part_1[1].as_str()) {
            let reg_n: u8 = remove_r_from_register_string(part_1[1].as_str());
            let memory_address: u8 = remove_r_from_register_string(part_2);
            [0xD0, (reg_n << 4) | memory_address]
        } else if confirm_valid_value(part_1[1].as_str()) {
            let value: u8 = grab_valid_value(part_1[1].as_str());
            let memory_address: u8 = remove_r_from_register_string(part_2);
            [0x10 | (memory_address & 0x0F), value]
        } else {
            panic!(
                "5. Error: Invalid MOV instruction format. Expected one part in the first or second position, but got '{:?}', '{}'.",
                part_1, part_2
            );
        }
    } else {
        panic!(
            "6. Error: Invalid MOV instruction format. Expected one part in the first or second position, but got '{:?}', '{}'.",
            part_1, part_2
        );
    }
}

fn process_jmp_instruction(rest: &[String]) -> [u8; 2] {
    compare_length(rest.len(), 1);

    if confirm_if_valid_register(&rest[0]) {
        let reg_n: u8 = remove_r_from_register_string(&rest[0]);
        return [0xF0, 0x00 | (reg_n & 0x0F)];
    } else if confirm_valid_value(&rest[0]) {
        let value: u8 = grab_valid_value(&rest[0]);
        return [0xB0, value];
    } else {
        panic!(
            "Error: Invalid JMP instruction format. Expected a register or a value, but got '{}'.",
            rest[0]
        );
    }
}

fn process_conditional_jump_instruction(instruction_string: &str, rest: &[String]) -> [u8; 2] {
    compare_length(rest.len(), 3);
    confirm_equal_strings(rest[1].as_str(), ",");

    if instruction_string == "JMPEQ" && confirm_valid_value(&rest[0]) {
        let value: u8 = grab_valid_value(&rest[0]);
        let reg_n: u8 = remove_r_from_register_string(&rest[2]);
        return [0xB << 4 | (reg_n & 0x0F), value];
    }

    let instruction_code: u8 = match CONDITIONAL_JUMP_INSTRUCTIONS.get(instruction_string) {
        Some(&code) => code,
        None => panic!(
            "Error: Invalid conditional jump instruction '{}'",
            instruction_string
        ),
    };

    let reg_n: u8 = remove_r_from_register_string(&rest[0]);
    let reg_m: u8 = remove_r_from_register_string(&rest[2]);
    return [
        0xF << 4 | (reg_m & 0x0F),
        instruction_code << 4 | (reg_n & 0x0F),
    ];
}

fn parse_instructions(lines: Vec<String>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    for line in lines {
        let split_line: Vec<String> = line
            .replace("->", " -> ")
            .replace(",", " , ")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let mut code: Vec<u8> = Vec::new();

        match split_line.get(0).map(|s| s.as_str()) {
            Some("HALT") => code.extend_from_slice(&[0xC0, 0x00]),
            Some("NOP") => code.extend_from_slice(&[0x0F, 0xFF]),
            Some("ROT") => code.extend_from_slice(&process_rot_instruction(&split_line[1..])),
            Some("MOV") => code.extend_from_slice(&process_mov_instruction(&split_line[1..])),
            Some("ADDI") | Some("ADDF") | Some("OR") | Some("AND") | Some("XOR") => {
                let returned_code: [u8; 2] =
                    process_register_operation_instructions(&split_line[0], &split_line[1..]);
                code.extend_from_slice(&returned_code);
            }
            Some("JMP") => code.extend_from_slice(&process_jmp_instruction(&split_line[1..])),
            Some("JMPEQ") | Some("JMPNE") | Some("JMPGE") | Some("JMPLE") | Some("JMPGT")
            | Some("JMPLT") => {
                let returned_code: [u8; 2] =
                    process_conditional_jump_instruction(&split_line[0], &split_line[1..]);
                code.extend_from_slice(&returned_code);
            }
            _ => println!("Error: Invalid instruction '{}'", line),
        }

        bytes.extend(code);
    }

    return bytes;
}

pub fn assembler(cleaned_lines: Vec<String>, label_addresses: HashMap<String, u8>) -> Vec<u8> {
    // Store the label addresses in the static variable using write lock
    {
        let mut map: std::sync::RwLockWriteGuard<'_, HashMap<String, u8>> =
            LABEL_ADDRESSES.write().unwrap();
        *map = label_addresses;
    }

    let bytes: Vec<u8> = parse_instructions(cleaned_lines);

    println!("Final assembled code:");

    for line in &bytes {
        println!("{:02X}", line);
    }

    return bytes;
}
