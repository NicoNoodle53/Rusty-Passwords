use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const PASSWORD_FILE: &str = "passwords.txt";

fn load_passwords() -> Vec<String> {
    // If the file doesn't exist, create it with a default password "admin"
    if !Path::new(PASSWORD_FILE).exists() {
        let mut file = File::create(PASSWORD_FILE).expect("Failed to create password file");
        writeln!(file, "admin").expect("Failed to write to password file");
    }

    let file = File::open(PASSWORD_FILE).expect("Failed to open password file");
    let reader = BufReader::new(file);

    let mut passwords = Vec::new();
    for line in reader.lines() {
        if let Ok(pw) = line {
            if !pw.trim().is_empty() {
                passwords.push(pw.trim().to_string());
            }
        }
    }
    passwords
}

fn save_passwords(passwords: &[String]) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(PASSWORD_FILE)
        .expect("Failed to open password file for writing");

    for pw in passwords {
        writeln!(file, "{}", pw).expect("Failed to write to password file");
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let mut valid_passwords = load_passwords();

    println!("Please enter a password:");
    let input = get_input();

    if valid_passwords.contains(&input) {
        println!("Access granted!\n");

        loop {
            println!("--- Password Manager Menu ---");
            println!("1. Add password");
            println!("2. Remove password");
            println!("3. Exit");
            println!("Choose an option (1-3):");

            let choice = get_input();

            match choice.as_str() {
                "1" => {
                    println!("Enter new password to add:");
                    let new_pw = get_input();
                    if !new_pw.is_empty() && !valid_passwords.contains(&new_pw) {
                        valid_passwords.push(new_pw);
                        save_passwords(&valid_passwords);
                        println!("Password added successfully!\n");
                    } else {
                        println!("Invalid or duplicate password.\n");
                    }
                }
                "2" => {
                    println!("Enter password to remove:");
                    let rem_pw = get_input();
                    if let Some(pos) = valid_passwords.iter().position(|p| p == &rem_pw) {
                        valid_passwords.remove(pos);
                        save_passwords(&valid_passwords);
                        println!("Password removed successfully!\n");
                    } else {
                        println!("Password not found in the list.\n");
                    }
                }
                "3" => {
                    println!("Exiting program. Goodbye!");
                    break;
                }
                _ => {
                    println!("Invalid option. Please enter 1, 2, or 3.\n");
                }
            }
        }
    } else {
        println!("Access denied! Incorrect password.");
    }
}