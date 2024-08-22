use std::{env, fs, process};
use std::collections::HashSet;
use std::path::PathBuf;

use colored::Colorize;
use serde_json::Value;

mod tests;

pub fn extract_nested_keys(value: &Value, parent_key: &str, keys: &mut HashSet<String>) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let full_key = if parent_key.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", parent_key, key)
                };

                keys.insert(full_key.clone());
                extract_nested_keys(val, &full_key, keys);
            }
        }
        Value::Array(arr) => {
            for (index, item) in arr.iter().enumerate() {
                let full_key = format!("{}[{}]", parent_key, index);
                extract_nested_keys(item, &full_key, keys);
                keys.insert(full_key.clone());
            }
        }
        _ => {}
    }
}

pub fn compare_json_keys(main_json: &Value, other_json: &Value) -> HashSet<String> {
    let mut main_keys = HashSet::new();
    let mut other_keys = HashSet::new();

    extract_nested_keys(main_json, "", &mut main_keys);
    extract_nested_keys(other_json, "", &mut other_keys);

    main_keys
        .difference(&other_keys)
        .cloned()
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let exit_on_fail = &args.iter().any(|arg| arg == "--fail");
    let sort_lines = &args.iter().any(|arg| arg == "--sort");
    let help = &args.iter().any(|arg| arg == "--help");

    if args.len() < 3 || *help {
        eprintln!("Usage: {} <main_file> <other_file_1> <other_file_2> ... [--fail]", args[0]);
        eprintln!("  --fail: Exit with non-zero status if any files are missing keys.");
        eprintln!("  --sort: Sort the output keys alphabetically.");
        process::exit(if *help { 0 } else { 1 });
    }

    let main_file = PathBuf::from(&args[1]);
    let other_file_paths = args[2..]
        .iter()
        .filter(|x| !x.is_empty() && !x.starts_with("--") && x.to_string() != args[1])
        .map(PathBuf::from)
        .collect::<Vec<_>>()
        .into_iter();

    if other_file_paths.len() == 0 {
        eprintln!("Error: No other files provided.");
        process::exit(1);
    }

    let main_content = fs::read_to_string(&main_file)?;
    let main_json: Value = serde_json::from_str(&main_content)?;

    let mut all_files_valid = true;

    for other_file in other_file_paths.clone() {
        println!("\n{}", format!("Comparing '{}' to '{}'", main_file.display().to_string().blue(), other_file.display().to_string().blue()).green());

        let other_content = fs::read_to_string(other_file.clone())?;
        let other_json: Value = serde_json::from_str(&other_content)?;

        let mut differences: Vec<String> = compare_json_keys(&main_json, &other_json).iter().map(|key| key.to_string()).collect();

        if *sort_lines {
            differences.sort();
        }

        for difference in &differences {
            eprintln!(" - Key '{}' is present in the main file but not in the compared file '{}'", difference.red(), other_file.display().to_string().yellow());
        }

        if !differences.is_empty() {
            all_files_valid = false;
        } else {
            println!("{}", format!("All keys are present in '{}'.", other_file.display().to_string().blue()).green());
        }
    }

    if all_files_valid {
        println!("All {} files are valid.", other_file_paths.len());
        process::exit(0);
    }

    if *exit_on_fail {
        process::exit(1);
    }

    Ok(())
}
