use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str::FromStr;


pub static LABEL_ADDRESSES: Lazy<std::sync::RwLock<HashMap<String, u8>>> =
    Lazy::new(|| std::sync::RwLock::new(HashMap::new()));



/// Enum for register operation instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegisterOp {
    ADDI,
    ADDF,
    OR,
    AND,
    XOR,
}

impl RegisterOp {
    fn opcode(self) -> u8 {
        match self {
            RegisterOp::ADDI => 0x50,
            RegisterOp::ADDF => 0x60,
            RegisterOp::OR   => 0x07,
            RegisterOp::AND  => 0x80,
            RegisterOp::XOR  => 0x90,
        }
    }
}

impl FromStr for RegisterOp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADDI" => Ok(RegisterOp::ADDI),
            "ADDF" => Ok(RegisterOp::ADDF),
            "OR"   => Ok(RegisterOp::OR),
            "AND"  => Ok(RegisterOp::AND),
            "XOR"  => Ok(RegisterOp::XOR),
            _ => Err(()),
        }
    }
}



/// Enum for conditional jump instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConditionalJump {
    JMPEQ,
    JMPNE,
    JMPGE,
    JMPLE,
    JMPGT,
    JMPLT,
}

impl ConditionalJump {
    fn code(self) -> u8 {
        match self {
            ConditionalJump::JMPEQ => 0x00,
            ConditionalJump::JMPNE => 0x01,
            ConditionalJump::JMPGE => 0x02,
            ConditionalJump::JMPLE => 0x03,
            ConditionalJump::JMPGT => 0x04,
            ConditionalJump::JMPLT => 0x05,
        }
    }
}

impl FromStr for ConditionalJump {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JMPEQ" => Ok(ConditionalJump::JMPEQ),
            "JMPNE" => Ok(ConditionalJump::JMPNE),
            "JMPGE" => Ok(ConditionalJump::JMPGE),
            "JMPLE" => Ok(ConditionalJump::JMPLE),
            "JMPGT" => Ok(ConditionalJump::JMPGT),
            "JMPLT" => Ok(ConditionalJump::JMPLT),
            _ => Err(()),
        }
    }
}

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

/// Checks if a string is a valid register (e.g., "R0".."RF"). Returns Ok(register_number) or Err(error message).
fn parse_register(instruction_string: &str) -> Result<u8, String> {
    if instruction_string.starts_with("R") {
        let reg_str = &instruction_string[1..];
        match u8::from_str_radix(reg_str, 16) {
            Ok(num) if num <= 15 => Ok(num),
            Ok(_) => Err(format!("Register number must be between 0 and 15: {}", instruction_string)),
            Err(_) => Err(format!("Invalid register number: {}", instruction_string)),
        }
    } else {
        Err(format!("Register must start with 'R': {}", instruction_string))
    }
}

/// Deprecated: Use parse_register instead.
fn remove_r_from_register_string(instruction_string: &str) -> u8 {
    parse_register(instruction_string).unwrap()
}

/// Checks if a string is a valid hex value (0-255). Returns Ok(value) or Err(error message).
fn parse_hex_value(value: &str) -> Result<u8, String> {
    match u8::from_str_radix(value, 16) {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Invalid value '{}'. Value must be a hexadecimal number.", value)),
    }
}

/// Deprecated: Use parse_hex_value instead.
fn grab_valid_value(value: &str) -> u8 {
    parse_hex_value(value).unwrap()
}

/// Checks if a slice represents a valid memory reference: [ "[", <value>, "]" ]
fn is_valid_memory(memory: &[String]) -> bool {
    memory.len() == 3 && memory[0] == "[" && memory[2] == "]"
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

fn process_register_operation_instructions(instruction_string: &str, rest: &[String]) -> Result<[u8; 2], String> {
    let op = RegisterOp::from_str(instruction_string)
        .map_err(|_| format!("Error: Invalid instruction '{}'.", instruction_string))?;

    compare_length(rest.len(), 5);

    let reg_n = parse_register(&rest[0])?;
    let reg_m = parse_register(&rest[2])?;
    let reg_p = parse_register(&rest[4])?;

    let opcode: u8 = op.opcode() | (reg_p & 0x0F);
    let operand: u8 = (reg_n & 0x0F) << 4 | (reg_m & 0x0F);

    println!(
        "Processing instruction: {:?} R{}, R{}, R{} -> {:02X}, {:02X}",
        op, reg_n, reg_m, reg_p, opcode, operand
    );

    Ok([opcode, operand])
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
    if parse_register(part_1).is_ok() && parse_register(part_2).is_ok() {
        let reg_m: u8 = parse_register(part_1).unwrap();
        let reg_n: u8 = parse_register(part_2).unwrap();
        [0x40, (reg_m << 4) | reg_n]
    } else if parse_hex_value(part_1).is_ok() && parse_register(part_2).is_ok() {
        let value: u8 = parse_hex_value(part_1).unwrap();
        let reg_n: u8 = parse_register(part_2).unwrap();
        [0x20 | (reg_n & 0x0F), value]
    } else {
        panic!(
            "2. Error: Invalid MOV instruction format. Expected one part in the first or second position, but got '{}', '{}'.",
            part_1, part_2
        );
    }
}

fn mov_one_to_three_parts(part_1: &str, part_2: &[String]) -> [u8; 2] {
    if parse_register(part_1).is_ok() && is_valid_memory(part_2) {
        if parse_register(part_2[1].as_str()).is_ok() {
            let reg_m: u8 = parse_register(part_2[1].as_str()).unwrap();
            let reg_n: u8 = parse_register(part_1).unwrap();
            [0xE0, (reg_m << 4) | reg_n]
        } else if parse_hex_value(part_2[1].as_str()).is_ok() {
            let value: u8 = parse_hex_value(part_2[1].as_str()).unwrap();
            let reg_n: u8 = parse_register(part_1).unwrap();
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
    if is_valid_memory(part_1) && parse_register(part_2).is_ok() {
        if parse_register(part_1[1].as_str()).is_ok() {
            let reg_n: u8 = parse_register(part_1[1].as_str()).unwrap();
            let memory_address: u8 = parse_register(part_2).unwrap();
            [0xD0, (reg_n << 4) | memory_address]
        } else if parse_hex_value(part_1[1].as_str()).is_ok() {
            let value: u8 = parse_hex_value(part_1[1].as_str()).unwrap();
            let memory_address: u8 = parse_register(part_2).unwrap();
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

    if parse_register(&rest[0]).is_ok() {
        let reg_n: u8 = parse_register(&rest[0]).unwrap();
        return [0xF0, 0x00 | (reg_n & 0x0F)];
    } else if parse_hex_value(&rest[0]).is_ok() {
        let value: u8 = parse_hex_value(&rest[0]).unwrap();
        return [0xB0, value];
    } else {
        panic!(
            "Error: Invalid JMP instruction format. Expected a register or a value, but got '{}'.",
            rest[0]
        );
    }
}

fn process_conditional_jump_instruction(instruction_string: &str, rest: &[String]) -> Result<[u8; 2], String> {
    compare_length(rest.len(), 3);
    confirm_equal_strings(rest[1].as_str(), ",");

    // Special case for JMPEQ with value
    if instruction_string == "JMPEQ" {
        if let Ok(value) = parse_hex_value(&rest[0]) {
            let reg_n = parse_register(&rest[2])?;
            return Ok([0xB << 4 | (reg_n & 0x0F), value]);
        }
    }

    let jump = ConditionalJump::from_str(instruction_string)
        .map_err(|_| format!("Error: Invalid conditional jump instruction '{}'.", instruction_string))?;

    let reg_n = parse_register(&rest[0])?;
    let reg_m = parse_register(&rest[2])?;
    Ok([
        0xF << 4 | (reg_m & 0x0F),
        jump.code() << 4 | (reg_n & 0x0F),
    ])
}

fn parse_instructions(lines: Vec<String>) -> Result<Vec<u8>, String> {
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
                let returned_code = process_register_operation_instructions(&split_line[0], &split_line[1..])?;
                code.extend_from_slice(&returned_code);
            }
            Some("JMP") => code.extend_from_slice(&process_jmp_instruction(&split_line[1..])),
            Some("JMPEQ") | Some("JMPNE") | Some("JMPGE") | Some("JMPLE") | Some("JMPGT")
            | Some("JMPLT") => {
                let returned_code = process_conditional_jump_instruction(&split_line[0], &split_line[1..])?;
                code.extend_from_slice(&returned_code);
            }
            _ => return Err(format!("Error: Invalid instruction '{}'.", line)),
        }

        bytes.extend(code);
    }

    Ok(bytes)
}

pub fn assembler(cleaned_lines: Vec<String>, label_addresses: HashMap<String, u8>) -> Result<Vec<u8>, String> {
    // Store the label addresses in the static variable using write lock
    {
        let mut map: std::sync::RwLockWriteGuard<'_, HashMap<String, u8>> =
            LABEL_ADDRESSES.write().unwrap();
        *map = label_addresses;
    }

    let bytes = parse_instructions(cleaned_lines)?;

    println!("Final assembled code:");
    for line in &bytes {
        println!("{:02X}", line);
    }

    Ok(bytes)
}


#[cfg(test)]mod tests {
    use super::*;

    #[test]
    fn test_mov_reg_to_reg() {
        let cleaned_lines = vec!["MOV R1 -> R2".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0x40, 0x12]);
    }

    #[test]
    fn test_mov_value_to_reg() {
        let cleaned_lines = vec!["MOV 4A -> R3".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0x23, 0x4A]);
    }

    #[test]
    fn test_mov_mem_to_reg() {
        let cleaned_lines = vec!["MOV [3A] -> R2".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0x12, 0x3A]);
    }

    #[test]
    fn test_mov_reg_to_mem() {
        let cleaned_lines = vec!["MOV R4 -> [2C]".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0x34, 0x2C]);
    }

    #[test]
    fn test_mov_reg_to_indirect_mem() {
        let cleaned_lines = vec!["MOV R3 -> [R5]".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0xE0, 0x35]);
    }

    #[test]
    fn test_mov_indirect_mem_to_reg() {
        let cleaned_lines = vec!["MOV [R2] -> R4".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0xD0, 0x42]);
    }

    #[test]
    fn test_addi() {
        let cleaned_lines = vec!["ADDI R1, R3 -> RC".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0x5C, 0x13]);
    }

    #[test]
    fn test_addf() {
        let cleaned_lines = vec!["ADDF R2, R4 -> R6".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0x66, 0x24]);
    }

    #[test]
    fn test_or() {
        let cleaned_lines = vec!["OR R1, R2 -> R3".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0x73, 0x12]);
    }

    #[test]
    fn test_and() {
        let cleaned_lines = vec!["AND R1, R2 -> R3".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0x83, 0x12]);
    }

    #[test]
    fn test_xor() {
        let cleaned_lines = vec!["XOR R1, R2 -> R3".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0x93, 0x12]);
    }

    #[test]
    fn test_rot() {
        let cleaned_lines = vec!["ROT R3, 2".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0xA3, 0x02]);
    }

    #[test]
    fn test_jmp_addr() {
        let cleaned_lines = vec!["JMP 4A".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0xB0, 0x4A]);
    }

    #[test]
    fn test_jmp_reg() {
        let cleaned_lines = vec!["JMP R3".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0xF0, 0x03]);
    }

    #[test]
    fn test_jmpeq_addr_reg() {
        let cleaned_lines = vec!["JMPEQ 20, R3".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0xB3, 0x20]);
    }

    #[test]
    fn test_jmpeq_reg_reg() {
        let cleaned_lines = vec!["JMPEQ R2, R5".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        // The output format for this case depends on your assembler's implementation
        // Adjust the expected value if needed
        assert_eq!(result, vec![0xF5, 0x02]);
    }

    #[test]
    fn test_jmpne() {
        let cleaned_lines = vec!["JMPNE R3, R4".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0xF4, 0x13]);
    }

    #[test]
    fn test_jmpge() {
        let cleaned_lines = vec!["JMPGE R1, R2".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0xF2, 0x01]);
    }

    #[test]
    fn test_jmple() {
        let cleaned_lines = vec!["JMPLE R2, R5".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0xF5, 0x32]);
    }

    #[test]
    fn test_jmpgt() {
        let cleaned_lines = vec!["JMPGT R1, R2".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0xF2, 0x41]);
    }

    #[test]
    fn test_jmplt() {
        let cleaned_lines = vec!["JMPLT R2, R3".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0xF3, 0x52]);
    }

    #[test]
    fn test_nop() {
        let cleaned_lines = vec!["NOP".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0x0F, 0xFF]);
    }

    #[test]
    fn test_halt() {
        let cleaned_lines = vec!["HALT".to_string()];
        let result = assembler(cleaned_lines, HashMap::new()).unwrap();
        assert_eq!(result, vec![0xC0, 0x00]);
    }
}
