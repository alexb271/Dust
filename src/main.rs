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
            println!("{}", HELP_TEXT);
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
            eprintln!("{e}");
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

const HELP_TEXT: &str = "Dust Scripting Language
User Guide

# Using variables

You can initialize variables with the \
'let' keyword, and optionally annotate \
their types. If there is no annotation \
the variable will receive the type of the \
value on the right hand side.

let x: number = 0

let x, y, z = 0

If the type on the right hand side \
contradicts the annotated type, then a \
runtime error will occur.

let x: number = true // Runtime error

Types assigned this way are static and the \
variable cannot be assigned a value of a \
different type later on. If you need a \
dynamic variable whose type can be changed, \
you must explicitly annotate it with the \
'dyn' keyword.

let x: dyn = 1

let x: number, y: dyn = 2

Once a variable is initialized, you can reassing \
its value with the assignment operator as long as \
the previously discussed type constraints are \
satisfied.

x = 42

y = x + 1

The following built-in types are available:
none
number
string
bool

# Defining functions

You can define functions using the 'fn' keyword. \
You must annotate the types of function parameters. \
Optionally you can also annotate the type of the \
function's return value.

fn power(base: number, exponent: number) -> number {
    if (exponent == 0) {
      return 1
    }
    return base * power(base, exponent - 1)
}

# Built-in Functions:

The following built-in mathematical functions are available:
sin, cos, tan, ln, log, abs

For trigonometric functions prepend 'a' for arcus
and append 'd' for degree.

Use 'pi' for an accurate value of the constant.
For example:

sin(pi)
asind(0.5)

Other built-in functions:

rand(min: number, max: number) -> number
parse_number(input: string) -> number
is_nan(input: number) -> bool
input(prompt: string) -> string
to_string(input: dyn) -> string

# Branches

You can use the 'if' keyword with a control expression \
for conditional execution. If the expression evaluates \
to false, the block will not run.

The 'else if' keyword allows you to test more conditions \
only in case the previous ones have evaluated to false.

You can also use the 'else' keyword to specify a block \
to be executed only in case all of the control expressions \
evaluated to 'false'.

if x % 2 == 0 {
    println(x)
} else if x < 0 {
    println(-x)
} else {
    println(x + 1)
}

# Loops

You can use the 'while' keyword with an expression for \
conditional loops. The loop will continue to run as long \
as the expression evaluates to true. \
Alternatively you can use the 'break' keyword to exit \
the loop at an arbitrary point.

let x = 0
while x < 10 {
    x = x + 1
    if x == 7 {
        break
    }
}

# Printing

You can use the 'print' and 'println' built-in functions \
to print to the standard output.

let x = 0
while x < 5 {
    println(x * 2)
    x = x + 1
}

# Miscellaneous

Supported operators: +, -, /, *, ^, %, <, >, ==, !=, =, and, or, not, typeof
You can use '\\' at the end of a line for multiline input.
Input 'clear' to clear the screen.
Input 'reset' to delete all functions and variables.";
