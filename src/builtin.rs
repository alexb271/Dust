#[allow(unused_imports)]
use crate::{TYPEID_BOOL, TYPEID_DYN, TYPEID_NONE, TYPEID_NUMBER, TYPEID_STRING};

use crate::function::{BuiltinFunction, Function};
use crate::session::ExecSession;
use crate::variable::{AnnotatedIdentifier, Value};
use rand::Rng;
use std::collections::HashMap;

#[cfg(not(test))]
use std::io::Write;

// Required function signature: fn(&mut ExecSession) -> Value
//
// For arguments, use lowercase letters of the alphabet in order,
// so the first argument is a, the second is b, and so on
//
// Built-in functions should be simple, they should not be able
// to fail as long as they get the correct type of argument, which
// is guaranteed by the runtime

pub fn load_builtin(function_store: &mut HashMap<String, Function>) {
    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_DYN)];
    function_store.insert(
        "println".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, printline)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_DYN)];
    function_store.insert(
        "print".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, print)),
    );

    #[cfg(not(test))]
    {
        let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_STRING)];
        function_store.insert(
            "input".to_string(),
            Function::BuiltinFunction(BuiltinFunction::new(arguments, input)),
        );
    }

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_STRING)];
    function_store.insert(
        "parse_number".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, parse_number)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_DYN)];
    function_store.insert(
        "to_string".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, to_string)),
    );

    let arguments = vec![
        AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER),
        AnnotatedIdentifier::new("b".to_string(), TYPEID_NUMBER),
    ];
    function_store.insert(
        "rand".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, rand)),
    );

    // Sine functions
    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "sin".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_sin)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "sind".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_sind)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "asin".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_asin)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "asind".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_asind)),
    );

    // Cosine functions
    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "cos".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_cos)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "cosd".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_cosd)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "acos".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_acos)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "acosd".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_acosd)),
    );

    // Tangent functions
    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "tan".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_tan)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "tand".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_tand)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "atan".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_atan)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "atand".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_atand)),
    );

    // Other math functions
    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "ln".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_ln)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "log".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_log)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "abs".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_abs)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_NUMBER)];
    function_store.insert(
        "is_nan".to_string(),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, is_nan)),
    );
}

fn printline(exec_session: &mut ExecSession) -> Value {
    #[cfg(test)]
    {
        let operand = exec_session.get_variable("a").unwrap();
        let text = operand.get_value().to_string() + "\n";
        exec_session.output_stream.push_str(&text);
    }
    #[cfg(not(test))]
    {
        let operand = exec_session.get_variable("a").unwrap();
        println!("{}", operand.get_value());
    }
    Value::None
}

fn print(exec_session: &mut ExecSession) -> Value {
    #[cfg(test)]
    {
        let operand = exec_session.get_variable("a").unwrap();
        let text = operand.get_value().to_string();
        exec_session.output_stream.push_str(&text);
    }
    #[cfg(not(test))]
    {
        let operand = exec_session.get_variable("a").unwrap();
        print!("{}", operand.get_value());
        std::io::stdout().flush().expect("Failed to flush stdout");
    }
    Value::None
}

#[cfg(not(test))]
fn input(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let prompt = match operand.get_value() {
        Value::Str(prompt) => prompt,
        _ => panic!("Invalid value in built-in function"),
    };
    print!("{prompt}");
    std::io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    if input.ends_with('\n') {
        input.pop();
    }
    Value::Str(Box::new(input))
}

fn parse_number(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let text = match operand.get_value() {
        Value::Str(text) => text,
        _ => panic!("Invalid value in built-in function"),
    };

    match text.parse::<f64>() {
        Ok(number) => Value::Number(number),
        Err(_) => Value::Number(f64::NAN),
    }
}

fn to_string(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    Value::Str(Box::new(operand.get_value().to_string()))
}

fn rand(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let min = match operand.get_value() {
        Value::Number(number) => number.round() as isize,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("b").unwrap();
    let max = match operand.get_value() {
        Value::Number(number) => number.round() as isize,
        _ => panic!("Invalid value in built-in function"),
    };

    Value::Number(rand::thread_rng().gen_range(min..=max) as f64)
}

// Sine functions
fn builtin_sin(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.sin())
}

fn builtin_sind(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.to_radians().sin())
}

fn builtin_asin(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.asin())
}

fn builtin_asind(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.asin().to_degrees())
}

// Cosine functions
fn builtin_cos(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.cos())
}

fn builtin_cosd(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.to_radians().cos())
}

fn builtin_acos(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.acos())
}

fn builtin_acosd(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.acos().to_degrees())
}

// Tan
fn builtin_tan(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.tan())
}

fn builtin_tand(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.to_radians().tan())
}

fn builtin_atan(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.atan())
}

fn builtin_atand(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.atan().to_degrees())
}

// Other math functions

fn builtin_ln(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.ln())
}

fn builtin_log(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.log(10.0_f64))
}

fn builtin_abs(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Number(number.abs())
}

fn is_nan(exec_session: &mut ExecSession) -> Value {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value() {
        Value::Number(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Value::Bool(number.is_nan())
}
