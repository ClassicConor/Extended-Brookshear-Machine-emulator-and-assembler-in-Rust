mod emulator_functions;
use emulator_functions::EmulatorFunctions;

pub struct Emulator {
    assembled_code: Vec<u8>,
    register_values: [u8; 16], // Assuming 16 registers, indexed from 0 to 15
    memory: [u8; 256],         // Assuming a memory size of 256 bytes
    halted: bool,
    program_counter: usize, // To keep track of the current instruction
    cir: u16,               // Current instruction register
    ef: EmulatorFunctions,  // Instance of EmulatorFunctions for utility methods
}

impl Emulator {
    pub fn new(assembled_code: Vec<u8>) -> Self {
        println!("Emulator is running...");
        let mut memory: [u8; 256] = [0; 256];
        for (i, byte) in assembled_code.iter().enumerate() {
            memory[i] = *byte; // Load assembled code into memory
        }

        println!("Assembled code loaded into memory:");
        for (i, byte) in memory.iter().enumerate() {
            println!("Memory[{}]: {:02X}", i, byte);
        }

        Emulator {
            assembled_code,
            register_values: [0; 16], // Initialize all registers to 0
            memory,                   // Initialize all memory to 0
            halted: false,
            program_counter: 0, // Start at the beginning of the assembled code
            cir: 0,             // Initialize the current instruction register
            ef: EmulatorFunctions::new(), // Create an instance of EmulatorFunctions
        }
    }

    pub fn run(&mut self) {
        for (index, line) in self.assembled_code.iter().enumerate() {
            println!("Executing line {}: {:02X}", index, line);
            // Add your execution logic here
        }

        while !self.halted && self.program_counter < self.assembled_code.len() {
            println!(
                "Program Counter: {}, Assembled Code Length: {}",
                self.program_counter,
                self.assembled_code.len()
            );

            self.fetch();
            self.decode();
            self.program_counter += 2; // Move to the next instruction
        }

        println!("Emulator has halted.");
        println!("Final register values: {:02X?}", self.register_values);
        println!("Final memory state: {:02X?}", self.memory);
    }

    fn fetch(&mut self) {
        // println!("Fetching");
        let high: u16 = self.assembled_code[self.program_counter] as u16;
        let low: u16 = self.assembled_code[self.program_counter + 1] as u16;
        self.cir = (high << 8) | low;
        println!("Fetched instruction: {:04X}", self.cir);
    }
    fn decode(&mut self) {
        // let ef: ef = emulator_functions::EmulatorFunctions {};
        let nibble: u8 = self.ef.get_nibble(self.cir, 1); // Get the first 4 bits
        println!(
            "Decoded instruction: {:04X}, Nibble: {:01X}",
            self.cir, nibble
        );

        match nibble {
            0x00 => self.nop(),                             // Completed
            0x01 => self.load_from_memory_direct(),         // Completed
            0x02 => self.load_value_into_register(),        // Completed
            0x03 => self.store_to_memory(),                 // Completed
            0x04 => self.move_register_value(),             // Completed
            0x05 => self.add_integer(),                     // Completed
            0x06 => self.add_floating_point(),              // Completed
            0x07 => self.or(),                              // Completed
            0x08 => self.and(),                             // Completed
            0x09 => self.xor(),                             // Completed
            0x0A => self.rotate(),                          // Completed
            0x0B => self.jump_equal(),                      // Completed
            0x0C => self.halt(),                            // Completed
            0x0D => self.load_from_memory(),                // Completed
            0x0E => self.store_in_memory(),                 // Completed
            0x0F => self.jump_unconditional_or_with_test(), // Completed
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
        let register_address: u8 = self.ef.get_nibble(self.cir, 1);
        let memory_address: u8 = self.ef.get_byte(self.cir, 1);

        self.register_values[register_address as usize] = memory_address;
        println!(
            "Loaded value {:02X} into register {}",
            memory_address, register_address
        );
    }

    fn load_value_into_register(&mut self) {
        let register_address: u8 = self.ef.get_nibble(self.cir, 1);
        let value: u8 = self.ef.get_byte(self.cir, 1); // Get the first byte (high nibble)

        self.register_values[register_address as usize] = value;
        println!(
            "Loaded value {:02X} into register {}",
            value, register_address
        );
    }

    fn store_to_memory(&mut self) {
        let register_address: u8 = self.ef.get_nibble(self.cir, 1);
        let memory_address: u8 = self.ef.get_byte(self.cir, 1);

        let register_value: u8 = self.register_values[register_address as usize];
        self.memory[memory_address as usize] = register_value;
    }

    fn move_register_value(&mut self) {
        let register_r: u8 = self.ef.get_nibble(self.cir, 2);
        let register_s: u8 = self.ef.get_nibble(self.cir, 3);
        let register_r_value: u8 = self.register_values[register_r as usize];

        self.register_values[register_s as usize] = register_r_value;
    }

    fn add_integer(&mut self) {
        println!("Adding integer values");
        let int_a: u8 = self.ef.get_nibble(self.cir, 2);
        let int_b: u8 = self.ef.get_nibble(self.cir, 3);
        let storage_register: u8 = self.ef.get_nibble(self.cir, 1);
        println!(
            "Result of addition: {} + {} = {}",
            int_a,
            int_b,
            int_a + int_b
        );
        self.memory[storage_register as usize] = int_a + int_b;
    }

    fn add_floating_point(&self) {
        println!("Adding floating point values");
    }

    fn or(&mut self) {
        let var_a: u8 = self.ef.get_nibble(self.cir, 2);
        let var_b: u8 = self.ef.get_nibble(self.cir, 3);
        let storage_register: u8 = self.ef.get_nibble(self.cir, 1);
        self.memory[storage_register as usize] = var_a | var_b;
    }

    fn and(&mut self) {
        let var_a: u8 = self.ef.get_nibble(self.cir, 2);
        let var_b: u8 = self.ef.get_nibble(self.cir, 3);
        let storage_register: u8 = self.ef.get_nibble(self.cir, 1);
        self.memory[storage_register as usize] = var_a & var_b;
    }

    fn xor(&mut self) {
        let var_a: u8 = self.ef.get_nibble(self.cir, 2);
        let var_b: u8 = self.ef.get_nibble(self.cir, 3);
        let storage_register: u8 = self.ef.get_nibble(self.cir, 1);
        self.memory[storage_register as usize] = var_a ^ var_b;
    }

    fn rotate(&mut self) {
        let target_reg: u8 = self.ef.get_nibble(self.cir, 1);
        let rot_amount: u8 = self.ef.get_nibble(self.cir, 3); // ensures 0–7

        let data: u8 = self.register_values[target_reg as usize];
        let rotated: u8 = data.rotate_right(rot_amount.into());
        self.register_values[target_reg as usize] = rotated;
    }

    fn jump_equal(&mut self) {
        let register_address_at_r: u8 = self.ef.get_nibble(self.cir, 1);
        let memory_location: u8 = self.ef.get_nibble(self.cir, 1);

        if self
            .ef
            .jump_equal_logic(register_address_at_r, self.register_values.clone())
            == true
        {
            self.program_counter = memory_location as usize;
        }
    }

    fn halt(&mut self) {
        self.halted = true;
        println!("Halting the emulator");
    }

    fn load_from_memory(&mut self) {
        let register_r_address: u8 = self.ef.get_nibble(self.cir, 2);
        let register_s_address_for_memory: u8 = self.ef.get_nibble(self.cir, 3);
        let memory_value: u8 = self.memory[register_s_address_for_memory as usize];
        self.register_values[register_r_address as usize] = memory_value;
    }

    fn store_in_memory(&mut self) {
        let register_r_address: u8 = self.ef.get_nibble(self.cir, 2);
        let register_s_address: u8 = self.ef.get_nibble(self.cir, 3);

        let register_r_value: u8 = self.register_values[register_r_address as usize];
        self.register_values[register_s_address as usize] = register_r_value;
    }

    // Final method to do
    fn jump_unconditional_or_with_test(&mut self) {
        let register_address_at_r: u8 = self.ef.get_nibble(self.cir, 1);
        let which_test: u8 = self.ef.get_nibble(self.cir, 2);
        let memory_address_stored_in_register: u8 = self.ef.get_nibble(self.cir, 3);

        if register_address_at_r == 0 {
            self.program_counter = self.memory[memory_address_stored_in_register as usize] as usize;
        } else {
            let register_value_at_0: u8 = self.register_values[0];
            let register_value_at_r: u8 = self.register_values[register_address_at_r as usize];
            let do_the_jump: bool =
                self.ef
                    .jump_with_test(which_test, register_value_at_0, register_value_at_r);

            if do_the_jump {
                self.program_counter =
                    self.memory[memory_address_stored_in_register as usize] as usize;
            }
        }
    }
}
