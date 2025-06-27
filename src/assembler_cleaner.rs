use std::collections::HashMap;
use std::fs;
use std::io::Error;

fn read_initial_data() -> Result<Vec<String>, Error> {
    let contents: String = fs::read_to_string("./test1.nha")?;
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    Ok(lines)
}

fn remove_comments(lines: Vec<String>) -> Vec<String> {
    let mut new_lines: Vec<String> = Vec::new();

    for line in lines {
        match line.find("//") {
            Some(index) => {
                new_lines.push(line[..index].trim().to_string());
            }
            None => {
                new_lines.push(line.trim().to_string());
            }
        };
    }

    new_lines
}

fn remove_whitespace(lines: Vec<String>) -> Vec<String> {
    let mut new_lines: Vec<String> = Vec::new();

    for line in lines.into_iter() {
        if !line.is_empty() {
            let cleaned_lines: String = line.trim().to_string();
            new_lines.push(cleaned_lines);
        }
    }

    new_lines
}

fn fill_label_address(lines: Vec<String>) -> (HashMap<String, u8>, Vec<String>) {
    let mut label_hashmap: HashMap<String, u8> = HashMap::new();
    let mut new_lines: Vec<String> = Vec::new();

    for (count, line) in lines.iter().enumerate() {
        match line.find(":") {
            Some(index) => {
                label_hashmap.insert(line[0..index].trim().to_string(), (count * 2) as u8);
                new_lines.push(line[index + 1..].trim().to_string());
            }
            None => {
                new_lines.push(line.to_string());
            }
        }
    }

    return (label_hashmap, new_lines);
}

pub fn assember_cleaning() -> Vec<String> {
    let lines: Vec<String> = match read_initial_data() {
        Ok(lines) => lines,
        Err(_e) => {
            eprint!("Error reading the file");
            return Vec::new();
        }
    };

    let removed_comments: Vec<String> = remove_comments(lines.clone());
    let trimmed_lines: Vec<String> = remove_whitespace(removed_comments.clone());

    let label_address: HashMap<String, u8>;
    let final_lines: Vec<String>;

    (label_address, final_lines) = fill_label_address(trimmed_lines.clone());

    final_lines
}
