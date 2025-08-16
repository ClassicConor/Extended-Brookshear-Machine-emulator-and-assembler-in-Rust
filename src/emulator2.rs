mod emulator_functions2;
use std::thread;

use emulator_functions2::EmulatorFunctions;

pub struct Emulator {
    assembled_code: Vec<u8>,
    register_values: [u8; 16], // Assuming 16 registers, indexed from 0 to 15
    memory: [u8; 256],         // Assuming a memory size of 256 bytes
    halted: bool,
    program_counter: usize, // To keep track of the current instruction
    cir: u16,               // Current instruction register
    ef: EmulatorFunctions,  // Instance of EmulatorFunctions for utility methods
    jump_instruction: bool, // Flag for jump instructions
}

impl Emulator {
    pub fn new(assembled_code: Vec<u8>) -> Self {
        println!("Emulator is running...");
        let mut memory: [u8; 256] = [0; 256];
        for (i, byte) in assembled_code.iter().enumerate() {
            memory[i] = *byte; // Load assembled code into memory
        }

        Emulator {
            assembled_code,
            register_values: [0; 16], // Initialize all registers to 0
            memory,                   // Initialize all memory to 0
            halted: false,
            program_counter: 0, // Start at the beginning of the assembled code
            cir: 0,             // Initialize the current instruction register
            ef: EmulatorFunctions::new(), // Create an instance of EmulatorFunctions
            jump_instruction: false, // Initialize jump instruction flag
        }
    }

    pub fn run(&mut self) {
        for (index, line) in self.assembled_code.iter().enumerate() {
            println!("Executing line {}: {:02X}", index, line);
            // Add your execution logic here
        }

        println!("List of register values: {:02X?}", self.register_values);

        println!("List of memory values: \n{:02X?}", self.memory);

        while !self.halted && self.program_counter < self.assembled_code.len() {
            println!(
                "Program Counter: {}, Assembled Code Length: {}",
                self.program_counter,
                self.assembled_code.len()
            );

            self.fetch();
            self.decode();

            if self.jump_instruction {
                println!("Jump instruction detected, skipping fetch-decode cycle");
                self.jump_instruction = false; // Reset the jump instruction flag
            } else {
                self.program_counter += 2; // Move to the next instruction
            }

            println!("");

            println!("Current Register Values: {:02X?}", self.register_values);
            // println!("Current Memory State: \n{:02X?}", self.memory);
            // thread::sleep(std::time::Duration::from_millis(1000)); // Simulate a delay for each instruction
        }

        println!("Emulator has halted.");
        println!("Final register values: {:02X?}", self.register_values);
        println!("Final memory state: \n{:02X?}", self.memory);
    }

    fn fetch(&mut self) {
        // println!("Fetching");
        let high: u16 = self.assembled_code[self.program_counter] as u16;
        let low: u16 = self.assembled_code[self.program_counter + 1] as u16;
        self.cir = (high << 8) | low;
    }
    fn decode(&mut self) {
        // let ef: ef = emulator_functions::EmulatorFunctions {};
        let nibble: u8 = self.ef.get_nibble(self.cir, 0); // Get the first 4 bits
        println!(
            "Decoded instruction: {:04X}, Nibble: {:02X}",
            self.cir, nibble
        );

        println!("Executing instruction with nibble: {:04X}", nibble);

        match nibble {
            0x00 => self.nop(),                      // NOP WORKS
            0x01 => self.load_from_memory_direct(),  // Working
            0x02 => self.load_value_into_register(), // LOAD VALUE TO REGISTER WORKS
            0x03 => self.store_to_memory(),          // Working
            0x04 => self.move_register_value(),      // Working
            0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0A => self.register_instruction(nibble), // Working
            0x0B => self.jump_equal(),               // Working
            0x0C => self.halt(),                     // HALT WORKS
            0x0D => self.load_from_memory(),         // Working
            0x0E => self.store_in_memory(),          // Working
            0x0F => self.jump_unconditional_or_with_test(), // Working
            _ => panic!(
                "Error: Invalid instruction nibble '{:01X}' in instruction {:04X}",
                nibble, self.cir
            ),
        }
    }

    fn nop(&self) {
        println!("No Operation (NOP)");
    }

    fn load_from_memory_direct(&mut self) {
        println!("Loading from memory directly");
        let register_address: u8 = self.ef.get_nibble(self.cir, 1);
        let memory_address: u8 = self.ef.get_byte(self.cir, 1);
        let memory_address_value: u8 = self.memory[memory_address as usize];

        self.register_values[register_address as usize] = memory_address_value;
    }

    fn load_value_into_register(&mut self) {
        println!("Loading value into register");
        let register_address: u8 = self.ef.get_nibble(self.cir, 1);
        let value: u8 = self.ef.get_byte(self.cir, 1); // Get the first byte (high nibble)

        self.register_values[register_address as usize] = value;
        println!(
            "Loaded value {:02X} into register {}",
            value, register_address
        );
    }

    fn store_to_memory(&mut self) {
        println!("Storing value to memory - Getting");
        let register_address: u8 = self.ef.get_nibble(self.cir, 1);
        let memory_address: u8 = self.ef.get_byte(self.cir, 1);
        let register_value: u8 = self.register_values[register_address as usize];
        self.memory[memory_address as usize] = register_value;
    }

    fn move_register_value(&mut self) {
        println!("Moving register value");
        let register_r: u8 = self.ef.get_nibble(self.cir, 2);
        let register_s: u8 = self.ef.get_nibble(self.cir, 3);
        let register_r_value: u8 = self.register_values[register_r as usize];

        self.register_values[register_s as usize] = register_r_value;
    }

    fn register_instruction(&mut self, nibble: u8) {
        println!("Register operation with nibble: {:02X}", nibble);
        let reg_a: u8 = self.ef.get_nibble(self.cir, 2);
        let reg_b: u8 = self.ef.get_nibble(self.cir, 3);
        let reg_a_value: u8 = self.register_values[reg_a as usize];
        let reg_b_value: u8 = self.register_values[reg_b as usize];
        let storage_register: u8 = self.ef.get_nibble(self.cir, 1);

        match nibble {
            0x05 => {
                println!("Adding integer values: {} + {}", reg_a_value, reg_b_value);
                self.register_values[storage_register as usize] =
                    reg_a_value.wrapping_add(reg_b_value);
            }
            0x06 => {
                println!(
                    "Adding floating point values: {} + {}",
                    reg_a_value, reg_b_value
                );
            }
            0x07 => {
                println!("OR operation: {} | {}", reg_a_value, reg_b_value);
                self.register_values[storage_register as usize] = reg_a_value | reg_b_value;
            }
            0x08 => {
                println!("AND operation: {} & {}", reg_a_value, reg_b_value);
                self.register_values[storage_register as usize] = reg_a_value & reg_b_value;
            }
            0x09 => {
                println!("XOR operation: {} ^ {}", reg_a_value, reg_b_value);
                self.register_values[storage_register as usize] = reg_a_value ^ reg_b_value;
            }
            0x0A => {
                let target_reg: u8 = storage_register;
                let rot_amount: u8 = self.ef.get_nibble(self.cir, 3); // ensures 0–7
                let data: u8 = self.register_values[target_reg as usize];
                let rotated: u8 = data.rotate_right(rot_amount.into());
                self.register_values[target_reg as usize] = rotated;
                println!(
                    "Rotating register {} by {} bits: {:02X} -> {:02X}",
                    target_reg, rot_amount, data, rotated
                );
            }
            _ => panic!(
                "Error: Invalid register operation nibble '{:01X}' in instruction {:04X}",
                nibble, self.cir
            ),
        }
    }

    fn jump_equal(&mut self) {
        println!("Jump if equal operation");
        let register_r_address: u8 = self.ef.get_nibble(self.cir, 1);
        let memory_location: u8 = self.ef.get_byte(self.cir, 1);

        println!(
            "Jumping to memory address: {:02X} if register {} is equal to 0",
            memory_location, register_r_address
        );

        if register_r_address == 0 {
            println!(
                "{:05}    Jumping to memory address: {:02X}",
                1, memory_location
            );
            self.jump_instruction = true; // Set the jump instruction flag

            self.program_counter = memory_location as usize;
        } else if register_r_address > 0 && register_r_address < 16 {
            println!(
                "{:05}     Jumping to memory address: {:04X} if register {} is equal to 0",
                1, memory_location, register_r_address
            );
            let register_0_value: u8 = self.register_values[0];
            let register_r_value: u8 = self.register_values[register_r_address as usize];
            println!(
                "Register 0 value: {:02X}, Register {} value: {:02X}",
                register_0_value, register_r_address, register_r_value
            );
            if register_r_value == register_0_value {
                println!(
                    "Jump condition met, jumping to memory address: {:02X}",
                    memory_location
                );
                self.jump_instruction = true; // Set the jump instruction flag

                self.program_counter = memory_location as usize;
            }
        } else {
            panic!("Error: Invalid register address for jump operation");
        }

        return;
    }

    fn halt(&mut self) {
        self.halted = true;
        println!("Halting the emulator");
    }

    fn load_from_memory(&mut self) {
        println!("Loading from memory");
        let register_saving_address: u8 = self.ef.get_nibble(self.cir, 2);
        let memory_address_in_register: u8 = self.ef.get_nibble(self.cir, 3);
        let memory_address: u8 = self.register_values[memory_address_in_register as usize];

        let memory_value: u8 = self.memory[memory_address as usize];
        self.register_values[register_saving_address as usize] = memory_value;
    }

    fn store_in_memory(&mut self) {
        println!("Storing in memory");
        let register_address: u8 = self.ef.get_nibble(self.cir, 2);
        let register_value: u8 = self.register_values[register_address as usize];
        let memory_address_in_registry: u8 = self.ef.get_nibble(self.cir, 3);
        let memory_address: u8 = self.register_values[memory_address_in_registry as usize];
        self.memory[memory_address as usize] = register_value;
    }

    fn jump_unconditional_or_with_test(&mut self) {
        // JMPEQ - Working
        // JMPNE - Working
        // JMPGE - Working
        // JMPGT - Working
        // JMPLE - Working
        // JMPLT - Working
        println!("Jump unconditional or with test operation");
        let register_address: u8 = self.ef.get_nibble(self.cir, 1);
        let register_value: u8 = self.register_values[register_address as usize];
        let which_test: u8 = self.ef.get_nibble(self.cir, 2);
        let memory_address_stored_in_register: u8 = self.ef.get_nibble(self.cir, 3);
        let memory_address: u8 = self.register_values[memory_address_stored_in_register as usize];
        let register_value_at_0: u8 = self.register_values[0];

        println!("Register value at 0: {:02X}", register_value_at_0);
        println!(
            "Register value at {}: {:02X}",
            register_address, register_value
        );

        let do_the_jump: bool =
            self.ef
                .jump_with_test(which_test, register_value_at_0, register_value);

        if do_the_jump {
            self.jump_instruction = true; // Set the jump instruction flag
            self.program_counter = memory_address as usize;
        } else {
            println!(
                "Jump condition not met, not jumping to memory address: {:02X}",
                memory_address
            );
        }
    }
}
