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

fn fill_label_address(
    lines: Vec<String>,
) -> (HashMap<String, u8>, Vec<String>, HashMap<String, u8>) {
    let mut label_hashmap: HashMap<String, u8> = HashMap::new();
    let mut data_hashmap: HashMap<String, u8> = HashMap::new();
    let mut new_lines: Vec<String> = Vec::new();

    for (count, line) in lines.iter().enumerate() {
        if line.contains(':') {
            let split_line: Vec<&str> = line.split(':').collect();
            if split_line.len() == 2 {
                let variable_name: String = split_line[0].trim().to_string();
                if split_line[1].trim().starts_with("DATA") {
                    let value: u8 =
                        data_entry(split_line[1].trim().split_whitespace().nth(1).unwrap());

                    println!(
                        "Found DATA entry: variable_name = '{}', value = {:02X}",
                        variable_name, value
                    );
                    data_hashmap.insert(variable_name, value);
                } else {
                    let address: u8 = (count * 2) as u8;
                    label_hashmap.insert(variable_name, address);
                    new_lines.push(split_line[1].trim().to_string());
                }
            } else {
                panic!("Error: Invalid label format in line '{}'", line);
            }
        } else {
            new_lines.push(line.trim().to_string());
        }
    }

    println!("Size of new lines: {}", new_lines.len());

    return (label_hashmap, new_lines, data_hashmap);
}

fn data_entry(line: &str) -> u8 {
    // This function is a placeholder for handling DATA entries.
    // It can be expanded to handle specific logic related to DATA labels.
    println!("Data entry found: {}", line);

    let trimmed: String = line.trim().to_string();

    println!("Trimmed data entry: '{}'", trimmed);

    if trimmed
        .chars()
        .all(|c: char| (c == '0' || c == '1') && trimmed.len() == 8)
    {
        let binding: u8 = u8::from_str_radix(&trimmed, 2).unwrap_or_else(|_| {
            panic!("Error: Invalid binary data value '{}'", trimmed);
        });
        return binding;
    } else if trimmed.len() == 2 && trimmed.chars().all(|c| c.is_digit(16)) {
        let binding: u8 = u8::from_str_radix(&trimmed, 16).unwrap_or_else(|_| {
            panic!("Error: Invalid hexadecimal data value '{}'", trimmed);
        });
        return binding;
    } else if trimmed.len() == 1 && trimmed.chars().all(|c| c.is_digit(10)) {
        let binding: u8 = trimmed.parse().unwrap_or_else(|_| {
            panic!("Error: Invalid decimal data value '{}'", trimmed);
        });
        return binding;
    } else {
        panic!("Error: Invalid data entry '{}'", line);
    }
}

fn insert_data_labels(
    lines: &Vec<String>,
    data_hashmap: &HashMap<String, u8>,
    label_addresses: &HashMap<String, u8>,
) -> Vec<String> {
    let variable_names_list: Vec<String> = data_hashmap.keys().cloned().collect();

    let mut new_lines: Vec<String> = Vec::new();

    for line in lines.iter() {
        let mut new_line: String = line.clone();

        for variable_name in &variable_names_list {
            if new_line.contains(variable_name) {
                let value: u8 = *data_hashmap.get(variable_name).unwrap();
                let value_str: String = format!("{:02X}", value);
                new_line = new_line.replace(variable_name, &value_str);
            }
        }

        new_lines.push(new_line);
    }

    let label_names_list: Vec<String> = label_addresses.keys().cloned().collect();

    let mut new_lines_with_labels: Vec<String> = Vec::new();

    for line in new_lines.iter() {
        let mut new_line: String = line.clone();

        for label_name in &label_names_list {
            if new_line.contains(label_name) {
                let address: u8 = *label_addresses.get(label_name).unwrap();
                let address_str = format!("{:02X}", address);

                new_line = new_line.replace(label_name, &address_str);
            }
        }
        new_lines_with_labels.push(new_line);
    }

    return new_lines_with_labels;
}

pub fn assember_cleaning() -> (Vec<String>, HashMap<String, u8>) {
    let lines: Vec<String> = match read_initial_data() {
        Ok(lines) => lines,
        Err(_e) => {
            panic!("Error reading file: test1.nha");
        }
    };

    println!("Initial lines: {:?}", lines);

    let removed_comments: Vec<String> = remove_comments(lines.clone());
    let trimmed_lines: Vec<String> = remove_whitespace(removed_comments.clone());

    let label_address: HashMap<String, u8>;
    let cleaned_lines: Vec<String>;
    let data_lines: HashMap<String, u8>;

    (label_address, cleaned_lines, data_lines) = fill_label_address(trimmed_lines.clone());

    let final_lines: Vec<String> = insert_data_labels(&cleaned_lines, &data_lines, &label_address);

    (final_lines, label_address)
}
