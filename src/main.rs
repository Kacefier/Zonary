//! Zonary Interpreter - Made by Kacefier - Version 2.1.1 - Main
//!
//! Copyright (C) 2026 Kacefier
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod interpreter;
mod preprocessor;
mod vm;

use std::env;
use std::fs;
use std::io::{self, Write};

use anyhow::Result;
use interpreter::Interpreter;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "-h" | "--help" => {
            print_help();
            Ok(())
        }
        "-v" | "--version" => {
            println!("Zonary Interpreter v2.1.1 - Made by Kacefier");
            println!("GitHub: github.com/Kacefier/Zonary\nEmail: kacefier@zohomail.com");
            Ok(())
        }
        "-r" | "--run" => {
            if args.len() < 3 {
                eprintln!("Error: missing file argument for {}", args[1]);
                std::process::exit(1);
            }
            run_file(&args[2])
        }
        "--repl" => run_repl(),
        _ => run_file(&args[1]),
    }
}

fn print_help() {
    println!("Zonary Interpreter v2.1.1 - Made by Kacefier");
    println!("GitHub: github.com/Kacefier/Zonary\nEmail: kacefier@zohomail.com");
    println!("Usage: zonary [OPTION]");
    println!("Options:");
    println!("  -h, --help            Show this help message and exit");
    println!("  -v, --version         Show version information and exit");
    println!("  -r, --run <file>      Run the specified Zonary source file");
    println!("  --repl                Start interactive REPL mode");
    println!();
    println!("Examples:");
    println!("  zonary example.zonary      Run the program");
    println!("  zonary -r example.zonary   Equivalent to above");
    println!("  zonary --repl              Start REPL");
}

fn run_file(filename: &str) -> Result<()> {
    let content = fs::read_to_string(filename)?;
    let mut interpreter = Interpreter::new();

    match interpreter.run(&content) {
        Ok(code) => {
            if code != 0 {
                eprintln!("Program exited with code: {}", code);
                std::process::exit(code);
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_repl() -> Result<()> {
    println!("Zonary REPL v2.1.1 - Made by Kacefier");
    println!("Enter code line by line. Empty line executes.");
    println!("Type 'exit' to quit.");
    println!();

    let mut interpreter = Interpreter::new();
    let mut lines = Vec::new();

    loop {
        print!(">>> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "exit" {
            break;
        }

        if input.is_empty() {
            if lines.is_empty() {
                continue;
            }

            let code = lines.join("\n");
            match interpreter.run(&code) {
                Ok(code) => {
                    if code != 0 {
                        println!("Program exited with code: {}", code);
                    }
                    lines.clear();
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    lines.clear();
                }
            }
        } else {
            lines.push(input.to_string());
        }
    }

    Ok(())
}
