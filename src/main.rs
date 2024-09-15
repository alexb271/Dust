use atty::Stream;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::{env, fs, io::Write};

use dust::Session;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("Usage: {} [file_path]", args[0]);
        std::process::exit(1);
    }
    if args.len() == 2 {
        if args[1] == "--help" {
            println!(
                "Dust Scripting Language Interpreter v0.1\n\nUsage: {} [file_path]\n\n{}",
                args[0], HELP_TEXT
            );
        } else {
            process_file(&args[1]);
        }
    } else {
        interpreter();
    }
}

fn process_file(file_path: &str) {
    let input = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file '{file_path}': {e}");
            std::process::exit(1);
        }
    };

    let mut session = Session::new();
    session.process(&input);
}

fn readline(rl: &mut DefaultEditor, prompt: &str) -> Result<String, ReadlineError> {
    let readline = rl.readline(prompt);
    match readline {
        Ok(line) => {
            _ = rl.add_history_entry(line.as_str());
            Ok(line)
        }
        Err(err) => Err(err),
    }
}

fn interpreter() {
    let mut rl = DefaultEditor::new().expect("Failed to initialize rustyline");
    let stdin_is_terminal = atty::is(Stream::Stdin);
    let mut session = Session::new();
    let mut print_newline = false; // workaround for rustyline deleting the last line

    if stdin_is_terminal {
        println!("[Dust v0.1]");
    }

    loop {
        let mut input = String::new();

        while input.is_empty() || input.ends_with('\\') {
            if input.ends_with('\\') {
                input.pop();
                input.push('\n');
            } else if stdin_is_terminal && print_newline {
                // rustyline deletes the last line and replaces it with the prompt
                println!();
            }
            let mut line = readline(&mut rl, ">>> ").unwrap_or_else(|e| {
                match e {
                    ReadlineError::Eof => (),
                    _ => eprintln!("{e}"),
                }
                std::process::exit(1);
            });

            if line.ends_with('\n') {
                line.pop();
            }

            input.push_str(&line);
        }

        match input.as_str() {
            "q" | "Q" | "exit" => {
                break;
            }
            "clear" => {
                print!("\x1Bc");
                std::io::stdout()
                    .flush()
                    .expect("Failed to flush standard output");
                print_newline = false;
            }
            "reset" => {
                session.clear();
                print_newline = false;
            }
            _ => {
                session.process(&input);
                print_newline = true;
            }
        }
    }
}

const HELP_TEXT: &str = "While in interpreter mode:
Use '\\' at the end of a line for multiline input.
Input 'clear' to clear the screen.
Input 'reset' to delete all classes, functions and variables.";
