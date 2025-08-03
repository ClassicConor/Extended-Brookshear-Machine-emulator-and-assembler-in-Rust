pub struct EmulatorFunctions {}

impl EmulatorFunctions {
    pub fn new() -> Self {
        EmulatorFunctions {}
    }

    pub fn get_nibble(&self, cir: u16, index: usize) -> u8 {
        if index > 3 {
            panic!("Error: Index out of bounds for nibble extraction");
        }
        let nibble: u8 = ((cir >> (12 - (index * 4))) & 0x0F) as u8; // Extract the nibble
        nibble
    }

    pub fn get_byte(&self, cir: u16, index: usize) -> u8 {
        if index > 1 {
            panic!("Error: Index out of bounds for byte extraction");
        }

        if index == 0 {
            return (cir >> 8) as u8; // Extract the high byte
        } else {
            return (cir & 0xFF) as u8; // Extract the low byte
        }
    }

    pub fn jump_with_test(
        &self,
        jump_command: u8,
        register_value_at_0: u8,
        register_value_at_r: u8,
    ) -> bool {
        match jump_command {
            0 => register_value_at_r == register_value_at_0, // EQ
            1 => register_value_at_r != register_value_at_0, // NE
            2 => register_value_at_r >= register_value_at_0, // LT
            3 => register_value_at_r <= register_value_at_0, // GT
            4 => register_value_at_r > register_value_at_0,  // LE
            5 => register_value_at_r < register_value_at_0,  // GE
            _ => {
                panic!("Error: Invalid jump command");
            }
        }
    }
}
