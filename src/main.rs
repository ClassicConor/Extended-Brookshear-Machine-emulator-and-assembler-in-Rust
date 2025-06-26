mod assembler;
mod assembler_cleaner;

fn main() {
    let cleaned_lines: Vec<String> = assembler_cleaner::assember_cleaning();
    let assembled_code: Vec<String> = assembler::assembler(cleaned_lines);
}
