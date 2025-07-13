use std::collections::HashMap;
mod assembler2;
mod assembler_cleaner;
// mod emulator;

fn main() {
    let (cleaned_lines, label_addresses): (Vec<String>, HashMap<String, u8>) =
        assembler_cleaner::assember_cleaning();

    for line in &cleaned_lines {
        println!("{}", line);
    }

    for (label, address) in &label_addresses {
        println!("Label: {}, Address: {:02X}", label, address);
    }

    let assembled_code: Vec<u8> = assembler2::assembler(cleaned_lines, label_addresses);

    // emulator::Emulator::new(assembled_code).run();
    // println!("Emulator finished running.");
}
