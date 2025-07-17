use std::process::Command;
use colored::Colorize;

pub fn get_svn_status(directory: &str) -> Vec<(String, String)> {
    let path = std::path::Path::new(directory);
    let status_vector = get_svn_status_vector(path);
    let status_tuple_vector = convert_status_vector_to_tuple(&status_vector);
    return status_tuple_vector;
}

fn get_svn_status_vector(path: &std::path::Path) -> Vec<String> {
    let output = Command::new("svn")
        .args(&["status", path.to_str().unwrap()])
        .output()
        .expect("Failed to execute svn status command");
    let status_output = String::from_utf8(output.stdout).expect("Invalid UTF-8 in svn status output");
    let status_lines: Vec<String> = status_output.lines().map(|s| s.to_string()).collect();
    return status_lines;
}

fn convert_status_vector_to_tuple(status_vector: &Vec<String>) -> Vec<(String, String)> {
    let mut status_tuple_vector: Vec<(String, String)> = Vec::new();
    for line in status_vector {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let stats = parts[0];
            let file_path = parts[1..].join(" ");
            status_tuple_vector.push((stats.to_string(), file_path.to_string()));
        }
    }
    return status_tuple_vector;
}

fn colorize_status(status: &str) -> colored::ColoredString {
    match status {
        "M" => status.blue(),           // Modified
        "A" => status.green(),          // Added
        "D" => status.red(),            // Deleted
        "C" => status.bright_red(),     // Conflict
        "?" => status.yellow(),         // Untracked
        "!" => status.bright_red(),     // Missing
        "I" => status.dimmed(),         // Ignored
        "R" => status.cyan(),           // Replaced
        "X" => status.magenta(),        // External
        "~" => status.bright_magenta(), // Obstructed
        _ => status.normal(),           // Default
    }
}

pub fn print_svn_status(directory: &str) {
    let status = get_svn_status(&directory);
    for file in &status {
        let colored_status = colorize_status(&file.0);
        println!("      {} {}", colored_status, file.1);
    }
    println!("\n");
}

pub fn print_filtered_svn_status(directory: &str, filter: String) {
    println!("Filter: {}", filter);
    let status = get_svn_status(&directory);
    for file in &status {
        if file.1.contains(&filter) {
            let colored_status = colorize_status(&file.0);
            println!("      {} {}", colored_status, file.1);
        }
    }
    println!("\n");
}
