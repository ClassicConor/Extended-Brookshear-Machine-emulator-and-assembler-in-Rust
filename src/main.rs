use std::collections::HashMap;
mod assembler;
mod assembler_cleaner;
mod emulator;

fn main() {
    let (cleaned_lines, label_addresses): (Vec<String>, HashMap<String, u8>) =
        assembler_cleaner::assember_cleaning();

    let assembled_code: Vec<u8> = assembler::assembler(cleaned_lines, label_addresses);

    emulator::Emulator::new(assembled_code).run();
    println!("Emulator finished running.");
}
