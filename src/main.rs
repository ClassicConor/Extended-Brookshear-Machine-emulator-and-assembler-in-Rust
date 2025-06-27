use std::collections::HashMap;
mod assembler;
mod assembler_cleaner;

fn main() {
    let (cleaned_lines, label_addresses): (Vec<String>, HashMap<String, u8>) =
        assembler_cleaner::assember_cleaning();
    let assembled_code: Vec<String> = assembler::assembler(cleaned_lines, label_addresses);
}
