use std::collections::HashMap;
use std::collections::BTreeSet;
use std::io::{stdin, stdout, Write};

const ADD: &str = "add";
const TO: &str = "to";
const PRINT: &str = "print";
const EXIT: &str = "exit";

struct HrRecord {
    departnemt: String,
    employee: String,
}

enum Command {
    AddRecord(HrRecord),
    PrintAll,
    PrintDepartment(String),
    Continue,
    Exit,
}

fn main() {
    let mut depts: HashMap<String, BTreeSet<String>> = HashMap::new();

    // Input loop
    loop {
        print!("COMMAND> ");
        stdout().flush().unwrap();

        let mut line: String = String::new();
        match stdin().read_line(&mut line) {
            Ok(_) => (),
            Err(error) => panic!("Error: {}", error),
        }

        // Parse command after having read the whole line
        match parse_command(&line) {
            Command::AddRecord(record) => {
                let names = depts.entry(record.departnemt).or_insert(BTreeSet::<String>::new());
                names.insert(record.employee);
            },
            Command::PrintDepartment(dept) => summarize_department(&depts, &dept),
            Command::PrintAll => summarize(&depts),
            Command::Continue => continue,
            Command::Exit => {
                println!("");
                break;
            },
        }
    }
}

fn summarize(depts: &HashMap<String, BTreeSet<String>>) {
    for k in depts.keys() {
        summarize_department(&depts, k);
    }
}

fn summarize_department(depts: &HashMap<String, BTreeSet<String>>, departnemt: &String) {
    match depts.get(departnemt) {
        Some(names) => {
            println!("Department: {}", departnemt);
            for e in names {
                println!("\t{}", e);
            }
        },
        None => eprintln!("Department {} does not exist", departnemt),
    }
}

fn parse_command(line: &String) -> Command {
    let line = line.trim();
    if line.is_empty() {
        return Command::Continue;
    }

    let mut employee: String = String::new();
    let mut departnemt: String = String::new();
    let mut print: bool = false;
    
    // Parse command
    for (i, word) in line.split_whitespace().enumerate() {
        match i {
            0 => match word {
                ADD => continue,
                PRINT => print = true,
                EXIT => return Command::Exit,
                _ => {
                    eprintln!("Invalid command");
                    return Command::Continue;
                }
            },
            1 => {
                if print {
                    departnemt += word
                } else {
                    employee += word
                }
            }
            2 => {
                if print {
                    eprintln!("Invalid syntax - expected command to end after dept name");
                    return Command::Continue
                } else if word != TO {
                    eprintln!("Invalid syntax - expected \"{}\"", TO);
                    return Command::Continue;
                }
            },
            3 => departnemt += word,
            _ => {
                eprintln!("Invalid syntax - expected command to end after dept name");
                return Command::Continue;
            }
        }
    }

    if print {
        if departnemt == "" {
            Command::PrintAll
        } else {
            Command::PrintDepartment(departnemt)
        }
    } else {
        Command::AddRecord(
            HrRecord {
                departnemt,
                employee,
            }
        )
    }  
}