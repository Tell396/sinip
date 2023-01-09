use std::collections::HashMap;
use std::env;

fn parse_ini(input: &str) -> Result<HashMap<String, HashMap<String, String>>, String> {
    let mut result = HashMap::new();
    let mut current_section: Option<String> = None;

    for line in input.lines() {
        // Find lines
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') {
            continue;
        }

        // Find the comment
        if line.starts_with('#') || line.starts_with(';') {
            let comment = &line[1..line.len()];
            current_section = Some(comment.to_string());
            continue;
        }

        // Find a section
        if line.starts_with('[') && line.ends_with(']') {
            let section = &line[1..line.len() - 1];
            current_section = Some(section.to_string());
            continue;
        }

        let mut parts = line.splitn(2, '=');
        let key = parts.next().ok_or("Invalid line")?;
        let value = parts.next().ok_or("Invalid line")?;

        if let Some(ref section) = current_section {
            result
                .entry(section.to_string())
                .or_insert_with(HashMap::new)
                .insert(key.to_string(), value.to_string());
        } else {
            return Err("Key found outside of section".to_string());
        }
    }

    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <ini file>", args[0]);
        return;
    }

    let input = std::fs::read_to_string(&args[1]).expect("Error reading file");
    let result = parse_ini(&input);
    println!("{:?}", result);
}
