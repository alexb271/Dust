use crate::class::{ClassDefinition, ClassFunction, ClassInstance};
use crate::error::{Context, Error, ErrorKind};
use crate::function::{BuiltinFunction, Function};
use crate::session::{ExecSession, ParseSession};
use crate::variable::{AnnotatedIdentifier, Value, Variable};

use rand::Rng;
use std::collections::HashMap;
use std::io::Write;

pub const DYN_KEYWORD: &'static str = "dyn";
pub const TYPEID_DYN: usize = 0;
pub const TYPEID_NONE: usize = 1;
pub const TYPEID_INT: usize = 2;
pub const TYPEID_FLOAT: usize = 3;
pub const TYPEID_STRING: usize = 4;
pub const TYPEID_BOOL: usize = 5;
pub const TYPEID_VEC: usize = 6;
pub const TYPEID_RESULT: usize = 7;
pub const TYPEID_FILE: usize = 8;
pub const TYPEID_FILESYSTEM: usize = 9;
pub const TYPEID_MATH: usize = 10;

// Required function signature:
// fn(&mut ExecSession, &ParseSession, Context, usize) -> Result<Value, Error>

pub fn load_builtin_functions(function_store: &mut HashMap<String, Function>) {
    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_DYN)];
    function_store.insert(
        String::from("println"),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, printline)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_DYN)];
    function_store.insert(
        String::from("print"),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, print)),
    );

    #[cfg(not(test))]
    {
        let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_STRING)];
        function_store.insert(
            String::from("input"),
            Function::BuiltinFunction(BuiltinFunction::new(arguments, input)),
        );
    }

    let arguments = vec![
        AnnotatedIdentifier::new("a".to_string(), TYPEID_INT),
        AnnotatedIdentifier::new("b".to_string(), TYPEID_INT),
    ];
    function_store.insert(
        String::from("rand"),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, rand)),
    );

    let arguments = vec![
        AnnotatedIdentifier::new("a".to_string(), TYPEID_INT),
        AnnotatedIdentifier::new("b".to_string(), TYPEID_INT),
    ];
    function_store.insert(
        String::from("range"),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, range)),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_STRING)];
    function_store.insert(
        String::from("panic"),
        Function::BuiltinFunction(BuiltinFunction::new(arguments, panic)),
    );
}

pub fn load_builtin_class_definitions(class_definitions: &mut Vec<ClassDefinition>) {
    class_definitions.push(make_int_class());
    class_definitions.push(make_float_class());
    class_definitions.push(make_string_class());
    class_definitions.push(make_bool_class());
    class_definitions.push(make_vec_class());
    class_definitions.push(make_result_class());
    class_definitions.push(make_file_class());
    class_definitions.push(make_filesystem_class());
    class_definitions.push(make_math_class());
}

fn printline(
    exec_session: &mut ExecSession,
    parse_session: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    #[cfg(test)]
    {
        let operand = exec_session.get_variable("a").unwrap();
        let text = operand.get_value().to_string(parse_session) + "\n";
        exec_session.output_stream.push_str(&text);
    }
    #[cfg(not(test))]
    {
        let operand = exec_session.get_variable("a").unwrap();
        println!("{}", operand.get_value().to_string(parse_session));
    }
    Ok(Value::None)
}

fn print(
    exec_session: &mut ExecSession,
    parse_session: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    #[cfg(test)]
    {
        let operand = exec_session.get_variable("a").unwrap();
        let text = operand.get_value().to_string(parse_session);
        exec_session.output_stream.push_str(&text);
    }
    #[cfg(not(test))]
    {
        let operand = exec_session.get_variable("a").unwrap();
        print!("{}", operand.get_value().to_string(parse_session));
        std::io::stdout().flush().expect("Failed to flush stdout");
    }
    Ok(Value::None)
}

#[cfg(not(test))]
fn input(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let prompt = match operand.get_value() {
        Value::Str(prompt) => prompt,
        _ => panic!("Invalid value in built-in function"),
    };
    print!("{}", prompt.borrow());
    std::io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    if input.ends_with('\n') {
        input.pop();
    }
    Ok(Value::new_string(input))
}

fn rand(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let min = match operand.get_value_clone() {
        Value::Int(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("b").unwrap();
    let max = match operand.get_value_clone() {
        Value::Int(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::Int(rand::thread_rng().gen_range(min..=max)))
}

fn range(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let min = match operand.get_value_clone() {
        Value::Int(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("b").unwrap();
    let max = match operand.get_value_clone() {
        Value::Int(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::new_vec_instance_from(
        (min..max).map(|i| Value::Int(i)).collect(),
    ))
}

fn panic(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    context: Context,
    pos: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let msg = match operand.get_value() {
        Value::Str(msg) => msg,
        _ => panic!("Invalid value in bult-in function"),
    };
    Err(Error::new(
        context,
        pos,
        ErrorKind::CustomError(format!("Explicit panic: {}", msg.borrow())),
    ))
}

fn self_to_string(
    exec_session: &mut ExecSession,
    parse_session: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    Ok(Value::new_string(
        operand.get_value().to_string(parse_session),
    ))
}

#[inline]
fn make_int_class() -> ClassDefinition {
    let mut functions = HashMap::new();

    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("parse"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, int_parse)),
            false,
            true,
        ),
    );
    functions.insert(
        String::from("abs"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), int_abs)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("to_float"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), int_to_float)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("to_string"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), self_to_string)),
            true,
            true,
        ),
    );

    ClassDefinition::new_without_constructor(functions, TYPEID_INT)
}

fn int_parse(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let text = match operand.get_value() {
        Value::Str(text) => text,
        _ => panic!("Invalid value in built-in function"),
    };

    match text.borrow().parse::<isize>() {
        Ok(number) => Ok(make_result(true, Value::Int(number))),
        Err(e) => Ok(make_result(false, Value::new_string(e.to_string()))),
    }
}

fn int_abs(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let number = match operand.get_value() {
        Value::Int(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::Int(number.abs()))
}

fn int_to_float(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let number = match operand.get_value() {
        Value::Int(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::Float(*number as f64))
}

#[inline]
fn make_float_class() -> ClassDefinition {
    let mut functions = HashMap::new();

    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("parse"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, float_parse)),
            false,
            true,
        ),
    );
    functions.insert(
        String::from("abs"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), float_abs)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("is_nan"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), float_is_nan)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("to_int"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), float_to_int)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("pi"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), float_pi)),
            false,
            true,
        ),
    );
    functions.insert(
        String::from("to_string"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), self_to_string)),
            true,
            true,
        ),
    );

    ClassDefinition::new_without_constructor(functions, TYPEID_FLOAT)
}

fn float_parse(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let text = match operand.get_value() {
        Value::Str(text) => text,
        _ => panic!("Invalid value in built-in function"),
    };

    match text.borrow().parse::<f64>() {
        Ok(number) => Ok(make_result(true, Value::Float(number))),
        Err(e) => Ok(make_result(false, Value::new_string(e.to_string()))),
    }
}

fn float_abs(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let number = match operand.get_value() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::Float(number.abs()))
}

fn float_is_nan(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let number = match operand.get_value() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::Bool(number.is_nan()))
}

fn float_to_int(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let number = match operand.get_value() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::Int(number.round() as isize))
}

fn float_pi(_: &mut ExecSession, _: &ParseSession, _: Context, _: usize) -> Result<Value, Error> {
    Ok(Value::Float(std::f64::consts::PI))
}

#[inline]
fn make_string_class() -> ClassDefinition {
    let mut functions = HashMap::new();

    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("join"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, string_join)),
            true,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("starts_with"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, string_starts_with)),
            true,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("ends_with"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, string_ends_with)),
            true,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("contains"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, string_contains)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("pop"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), string_pop)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("trim"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), string_trim)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("to_lowercase"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), string_to_lowercase)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("to_uppercase"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), string_to_uppercase)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("len"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), string_len)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("chars"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), string_chars)),
            true,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> = vec![
        AnnotatedIdentifier::new(String::from("a"), TYPEID_INT),
        AnnotatedIdentifier::new(String::from("b"), TYPEID_INT),
    ];
    functions.insert(
        String::from("substring"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, string_substring)),
            true,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("split"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, string_split)),
            true,
            true,
        ),
    );

    ClassDefinition::new_without_constructor(functions, TYPEID_STRING)
}

fn string_join(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let string = match operand.get_value() {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("a").unwrap().get_value();
    let argument = match operand {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    string.borrow_mut().push_str(argument.borrow().as_str());
    Ok(Value::None)
}

fn string_starts_with(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let string = match operand.get_value() {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("a").unwrap().get_value();
    let argument = match operand {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::Bool(
        string.borrow().starts_with(argument.borrow().as_str()),
    ))
}

fn string_ends_with(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let string = match operand.get_value() {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("a").unwrap().get_value();
    let argument = match operand {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::Bool(
        string.borrow().ends_with(argument.borrow().as_str()),
    ))
}

fn string_contains(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let string = match operand.get_value() {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("a").unwrap().get_value();
    let argument = match operand {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::Bool(
        string.borrow().contains(argument.borrow().as_str()),
    ))
}

fn string_pop(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let string = match operand.get_value() {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };
    if string.borrow().is_empty() {
        Ok(Value::None)
    } else {
        Ok(Value::new_string(
            string.borrow_mut().pop().unwrap().to_string(),
        ))
    }
}

fn string_trim(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let string = match operand.get_value() {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::new_string(string.borrow().trim().to_string()))
}

fn string_to_lowercase(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let string = match operand.get_value() {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::new_string(
        string.borrow().to_lowercase().to_string(),
    ))
}

fn string_to_uppercase(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let string = match operand.get_value() {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::new_string(
        string.borrow().to_uppercase().to_string(),
    ))
}

fn string_len(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let string = match operand.get_value() {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Int(string.borrow().len() as isize))
}

fn string_chars(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let string = match operand.get_value() {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    let vec: Vec<Value> = string
        .borrow()
        .chars()
        .map(|ch| Value::new_string(ch.to_string()))
        .collect();
    Ok(Value::new_vec_instance_from(vec))
}

fn string_substring(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let string = match operand.get_value() {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("a").unwrap();
    let index_1 = match operand.get_value() {
        Value::Int(i) => *i as usize,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("b").unwrap();
    let index_2 = match operand.get_value() {
        Value::Int(i) => *i as usize,
        _ => panic!("Invalid value in built-in function"),
    };

    match string.borrow().get(index_1..index_2) {
        Some(substring) => Ok(make_result(true, Value::new_string(substring.to_string()))),
        None => Ok(make_result(false, Value::None)),
    }
}

fn string_split(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    context: Context,
    pos: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let string = match operand.get_value() {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("a").unwrap();
    let delimiter = match operand.get_value() {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    let delimiter_len = delimiter.borrow().len();
    if delimiter_len != 1 {
        return Err(Error::new(
            context,
            pos,
            ErrorKind::CustomError(String::from("Delimiter must be a single character")),
        ));
    }

    let delimiter: char = delimiter.borrow().chars().next().unwrap();

    Ok(Value::new_vec_instance_from(
        string
            .borrow()
            .split(delimiter)
            .map(|s| Value::new_string(s.to_string()))
            .collect(),
    ))
}

#[inline]
fn make_bool_class() -> ClassDefinition {
    let mut functions = HashMap::new();

    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("parse"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, bool_parse)),
            false,
            true,
        ),
    );
    functions.insert(
        String::from("to_string"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), self_to_string)),
            true,
            true,
        ),
    );

    ClassDefinition::new_without_constructor(functions, TYPEID_BOOL)
}

fn bool_parse(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let string = match operand.get_value() {
        Value::Str(string) => string,
        _ => panic!("Invalid value in built-in function"),
    };

    match string.borrow().parse::<bool>() {
        Ok(boolean) => Ok(make_result(true, Value::Bool(boolean))),
        Err(e) => Ok(make_result(false, Value::new_string(e.to_string()))),
    }
}

#[inline]
fn make_vec_class() -> ClassDefinition {
    let mut functions = HashMap::new();

    functions.insert(
        String::from("new"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), vec_new)),
            false,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_DYN)];
    functions.insert(
        String::from("push"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, vec_push)),
            true,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_INT)];
    functions.insert(
        String::from("get"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, vec_get)),
            true,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> = vec![
        AnnotatedIdentifier::new(String::from("a"), TYPEID_INT),
        AnnotatedIdentifier::new(String::from("b"), TYPEID_DYN),
    ];
    functions.insert(
        String::from("set"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, vec_set)),
            true,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_INT)];
    functions.insert(
        String::from("remove"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, vec_remove)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("pop"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), vec_pop)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("clear"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), vec_clear)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("len"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), vec_len)),
            true,
            true,
        ),
    );

    ClassDefinition::new_without_constructor(functions, TYPEID_VEC)
}

fn vec_new(_: &mut ExecSession, _: &ParseSession, _: Context, _: usize) -> Result<Value, Error> {
    Ok(Value::new_vec_instance())
}

fn vec_push(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let vec = match operand.get_value() {
        Value::Vector(vec) => vec,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("a").unwrap();
    vec.borrow_mut().push(operand.get_value_clone());
    Ok(Value::None)
}

fn vec_get(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    context: Context,
    pos: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let vec = match operand.get_value() {
        Value::Vector(vec) => vec,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("a").unwrap();
    let index = match operand.get_value() {
        Value::Int(i) => *i,
        _ => panic!("Invalid value in built-in function"),
    };

    if index < 0 || index >= vec.borrow().len() as isize {
        Err(Error::new(
            context,
            pos,
            ErrorKind::IndexOutOfRange(index, vec.borrow().len()),
        ))
    } else {
        Ok(vec.borrow()[index as usize].clone())
    }
}

fn vec_set(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    context: Context,
    pos: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let vec = match operand.get_value() {
        Value::Vector(vec) => vec,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("a").unwrap();
    let index = match operand.get_value() {
        Value::Int(i) => *i,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("b").unwrap();

    if index < 0 || index >= vec.borrow().len() as isize {
        Err(Error::new(
            context,
            pos,
            ErrorKind::IndexOutOfRange(index, vec.borrow().len()),
        ))
    } else {
        vec.borrow_mut()[index as usize] = operand.get_value_clone();
        Ok(Value::None)
    }
}

fn vec_remove(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    context: Context,
    pos: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let vec = match operand.get_value() {
        Value::Vector(vec) => vec,
        _ => panic!("Invalid value in built-in function"),
    };

    let operand = exec_session.get_variable("a").unwrap();
    let index = match operand.get_value() {
        Value::Int(i) => *i,
        _ => panic!("Invalid value in built-in function"),
    };

    if index < 0 || index >= vec.borrow().len() as isize {
        Err(Error::new(
            context,
            pos,
            ErrorKind::IndexOutOfRange(index, vec.borrow().len()),
        ))
    } else {
        Ok(vec.borrow_mut().remove(index as usize))
    }
}

fn vec_pop(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let vec = match operand.get_value() {
        Value::Vector(vec) => vec,
        _ => panic!("Invalid value in built-in function"),
    };
    if vec.borrow().is_empty() {
        Ok(Value::None)
    } else {
        Ok(vec.borrow_mut().pop().unwrap())
    }
}

fn vec_clear(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let vec = match operand.get_value() {
        Value::Vector(vec) => vec,
        _ => panic!("Invalid value in built-in function"),
    };

    vec.borrow_mut().clear();
    Ok(Value::None)
}

fn vec_len(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let vec = match operand.get_value() {
        Value::Vector(vec) => vec,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::Int(vec.borrow().len() as isize))
}

#[inline]
fn make_result_class() -> ClassDefinition {
    let mut functions = HashMap::new();

    let args: Vec<AnnotatedIdentifier> = vec![
        AnnotatedIdentifier::new(String::from("a"), TYPEID_BOOL),
        AnnotatedIdentifier::new(String::from("b"), TYPEID_DYN),
    ];
    functions.insert(
        String::from("new"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, result_new)),
            false,
            true,
        ),
    );
    functions.insert(
        String::from("is_ok"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), result_is_ok)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("value"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), result_value)),
            true,
            true,
        ),
    );
    functions.insert(
        String::from("unwrap"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), result_unwrap)),
            true,
            true,
        ),
    );

    ClassDefinition::new_without_constructor(functions, TYPEID_RESULT)
}

fn make_result(is_ok: bool, value: Value) -> Value {
    let mut result = ClassInstance::new(TYPEID_RESULT);
    result.add_property("is_ok", Variable::new(Value::Bool(is_ok), false), false);
    result.add_property("value", Variable::new(value, true), false);
    result.add_property(
        "is_checked",
        Variable::new(Value::Bool(false), false),
        false,
    );
    Value::new_class_instance(result)
}

fn result_new(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let is_ok: bool = match operand.get_value() {
        Value::Bool(is_ok) => *is_ok,
        _ => panic!("Invalid value in built-in function"),
    };
    let operand = exec_session.get_variable("b").unwrap();
    let value: Value = operand.get_value_clone();

    Ok(make_result(is_ok, value))
}

fn result_is_ok(
    exec_session: &mut ExecSession,
    parse_session: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let result = match operand.get_value() {
        Value::Class(c) => c,
        _ => panic!("Invalid value in built-in function"),
    };

    let is_ok: bool = match result
        .borrow()
        .get_property("is_ok", true, parse_session)
        .unwrap()
        .get_value()
    {
        Value::Bool(b) => *b,
        _ => unreachable!(),
    };

    result
        .borrow_mut()
        .set_property("is_checked", true, Value::Bool(true), parse_session)
        .unwrap();

    Ok(Value::Bool(is_ok))
}

fn result_value(
    exec_session: &mut ExecSession,
    parse_session: &ParseSession,
    context: Context,
    pos: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let result = match operand.get_value() {
        Value::Class(c) => c,
        _ => panic!("Invalid value in built-in function"),
    };

    let is_checked: bool = match result
        .borrow()
        .get_property("is_checked", true, parse_session)
        .unwrap()
        .get_value()
    {
        Value::Bool(b) => *b,
        _ => unreachable!(),
    };

    if !is_checked {
        return Err(Error::new(
            context,
            pos,
            ErrorKind::CustomError(String::from(
                "A 'Result' must be checked with 'is_ok()' before accessing its value",
            )),
        ));
    }

    let value: Value = result
        .borrow()
        .get_property("value", true, parse_session)
        .unwrap()
        .get_value_clone();

    Ok(value)
}

fn result_unwrap(
    exec_session: &mut ExecSession,
    parse_session: &ParseSession,
    context: Context,
    pos: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("self").unwrap();
    let result = match operand.get_value() {
        Value::Class(c) => c,
        _ => panic!("Invalid value in built-in function"),
    };

    let is_ok: bool = match result
        .borrow()
        .get_property("is_ok", true, parse_session)
        .unwrap()
        .get_value()
    {
        Value::Bool(b) => *b,
        _ => unreachable!(),
    };

    if !is_ok {
        return Err(Error::new(
            context,
            pos,
            ErrorKind::CustomError(String::from(
                "Unwrap called on a 'Result' containing an error",
            )),
        ));
    }

    let value: Value = result
        .borrow()
        .get_property("value", true, parse_session)
        .unwrap()
        .get_value_clone();

    Ok(value)
}

#[inline]
fn make_file_class() -> ClassDefinition {
    let mut functions = HashMap::new();

    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("read"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, file_read)),
            false,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> = vec![
        AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING),
        AnnotatedIdentifier::new(String::from("b"), TYPEID_STRING),
    ];
    functions.insert(
        String::from("write"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, file_write)),
            false,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> = vec![
        AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING),
        AnnotatedIdentifier::new(String::from("b"), TYPEID_STRING),
    ];
    functions.insert(
        String::from("append"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, file_append)),
            false,
            true,
        ),
    );

    ClassDefinition::new_without_constructor(functions, TYPEID_FILE)
}

fn file_read(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let path = match operand.get_value() {
        Value::Str(path) => path,
        _ => panic!("Invalid value in built-in function"),
    };

    match std::fs::read_to_string(path.borrow().as_str()) {
        Ok(file_contents) => Ok(make_result(true, Value::new_string(file_contents))),
        Err(e) => Ok(make_result(false, Value::new_string(e.to_string()))),
    }
}

fn file_write(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let path = match operand.get_value() {
        Value::Str(path) => path,
        _ => panic!("Invalid value in built-in function"),
    };
    let operand = exec_session.get_variable("b").unwrap();
    let contents = match operand.get_value() {
        Value::Str(file_contents) => file_contents,
        _ => panic!("Invalid value in built-in function"),
    };

    match std::fs::write(path.borrow().as_str(), contents.borrow().as_str()) {
        Ok(()) => Ok(make_result(true, Value::None)),
        Err(e) => Ok(make_result(false, Value::new_string(e.to_string()))),
    }
}

fn file_append(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let path = match operand.get_value() {
        Value::Str(path) => path,
        _ => panic!("Invalid value in built-in function"),
    };
    let operand = exec_session.get_variable("b").unwrap();
    let contents = match operand.get_value() {
        Value::Str(contents) => contents,
        _ => panic!("Invalid value in built-in function"),
    };

    let mut file = match std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path.borrow().as_str())
    {
        Ok(file) => file,
        Err(e) => {
            return Ok(make_result(false, Value::new_string(e.to_string())));
        }
    };

    match file.write_all(contents.borrow().as_str().as_bytes()) {
        Ok(()) => Ok(make_result(true, Value::None)),
        Err(e) => Ok(make_result(false, Value::new_string(e.to_string()))),
    }
}

#[inline]
fn make_filesystem_class() -> ClassDefinition {
    let mut functions = HashMap::new();

    functions.insert(
        String::from("current_directory"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(Vec::new(), fs_current_directory)),
            false,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("change_directory"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, fs_change_directory)),
            false,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("exists"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, fs_exists)),
            false,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("list"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, fs_list)),
            false,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("remove_file"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, fs_remove_file)),
            false,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("remove_directory"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, fs_remove_directory)),
            false,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> =
        vec![AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING)];
    functions.insert(
        String::from("create_directory"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, fs_create_directory)),
            false,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> = vec![
        AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING),
        AnnotatedIdentifier::new(String::from("b"), TYPEID_STRING),
    ];
    functions.insert(
        String::from("copy"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, fs_copy)),
            false,
            true,
        ),
    );
    let args: Vec<AnnotatedIdentifier> = vec![
        AnnotatedIdentifier::new(String::from("a"), TYPEID_STRING),
        AnnotatedIdentifier::new(String::from("b"), TYPEID_STRING),
    ];
    functions.insert(
        String::from("move"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(args, fs_move)),
            false,
            true,
        ),
    );

    ClassDefinition::new_without_constructor(functions, TYPEID_FILESYSTEM)
}

fn fs_current_directory(
    _: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    match std::env::current_dir() {
        Ok(path) => Ok(make_result(
            true,
            Value::new_string(path.to_string_lossy().to_string()),
        )),
        Err(e) => Ok(make_result(false, Value::new_string(e.to_string()))),
    }
}

fn fs_change_directory(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let path = match operand.get_value() {
        Value::Str(path) => path,
        _ => panic!("Invalid value in built-in function"),
    };

    match std::env::set_current_dir(path.borrow().as_str()) {
        Ok(()) => Ok(make_result(true, Value::None)),
        Err(e) => Ok(make_result(false, Value::new_string(e.to_string()))),
    }
}

fn fs_exists(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let path = match operand.get_value() {
        Value::Str(path) => path,
        _ => panic!("Invalid value in built-in function"),
    };

    Ok(Value::Bool(
        std::path::Path::new(path.borrow().as_str()).exists(),
    ))
}

fn fs_list(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let path = match operand.get_value() {
        Value::Str(path) => path,
        _ => panic!("Invalid value in built-in function"),
    };

    let result = Value::new_vec_instance();
    let vec = match result {
        Value::Vector(ref vec) => vec,
        _ => unreachable!(),
    };

    match std::fs::read_dir(path.borrow().as_str()) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => vec.borrow_mut().push(Value::new_string(
                        entry.file_name().to_string_lossy().to_string(),
                    )),
                    Err(e) => {
                        return Ok(make_result(false, Value::new_string(e.to_string())));
                    }
                }
            }
        }
        Err(e) => {
            return Ok(make_result(false, Value::new_string(e.to_string())));
        }
    }

    Ok(make_result(true, result))
}

fn fs_remove_file(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let path = match operand.get_value() {
        Value::Str(path) => path,
        _ => panic!("Invalid value in built-in function"),
    };

    match std::fs::remove_file(path.borrow().as_str()) {
        Ok(()) => Ok(make_result(true, Value::None)),
        Err(e) => Ok(make_result(false, Value::new_string(e.to_string()))),
    }
}

fn fs_remove_directory(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let path = match operand.get_value() {
        Value::Str(path) => path,
        _ => panic!("Invalid value in built-in function"),
    };

    match std::fs::remove_dir_all(path.borrow().as_str()) {
        Ok(()) => Ok(make_result(true, Value::None)),
        Err(e) => Ok(make_result(false, Value::new_string(e.to_string()))),
    }
}

fn fs_create_directory(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let path = match operand.get_value() {
        Value::Str(path) => path,
        _ => panic!("Invalid value in built-in function"),
    };

    match std::fs::create_dir_all(path.borrow().as_str()) {
        Ok(()) => Ok(make_result(true, Value::None)),
        Err(e) => Ok(make_result(false, Value::new_string(e.to_string()))),
    }
}

fn fs_copy(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let source = match operand.get_value() {
        Value::Str(path) => path,
        _ => panic!("Invalid value in built-in function"),
    };
    let operand = exec_session.get_variable("b").unwrap();
    let destination = match operand.get_value() {
        Value::Str(path) => path,
        _ => panic!("Invalid value in built-in function"),
    };

    match std::fs::copy(source.borrow().as_str(), destination.borrow().as_str()) {
        Ok(_) => Ok(make_result(true, Value::None)),
        Err(e) => Ok(make_result(false, Value::new_string(e.to_string()))),
    }
}

fn fs_move(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let source = match operand.get_value() {
        Value::Str(path) => path,
        _ => panic!("Invalid value in built-in function"),
    };
    let operand = exec_session.get_variable("b").unwrap();
    let destination = match operand.get_value() {
        Value::Str(path) => path,
        _ => panic!("Invalid value in built-in function"),
    };

    match std::fs::rename(source.borrow().as_str(), destination.borrow().as_str()) {
        Ok(_) => Ok(make_result(true, Value::None)),
        Err(e) => Ok(make_result(false, Value::new_string(e.to_string()))),
    }
}

#[inline]
fn make_math_class() -> ClassDefinition {
    let mut functions: HashMap<String, ClassFunction> = HashMap::new();

    // Sine functions
    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_FLOAT)];
    functions.insert(
        String::from("sin"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_sin)),
            false,
            true,
        ),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_FLOAT)];
    functions.insert(
        String::from("sind"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_sind)),
            false,
            true,
        ),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_FLOAT)];
    functions.insert(
        String::from("asin"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_asin)),
            false,
            true,
        ),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_FLOAT)];
    functions.insert(
        String::from("asind"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_asind)),
            false,
            true,
        ),
    );

    // Cosine functions
    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_FLOAT)];
    functions.insert(
        String::from("cos"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_cos)),
            false,
            true,
        ),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_FLOAT)];
    functions.insert(
        String::from("cosd"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_cosd)),
            false,
            true,
        ),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_FLOAT)];
    functions.insert(
        String::from("acos"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_acos)),
            false,
            true,
        ),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_FLOAT)];
    functions.insert(
        String::from("acosd"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_acosd)),
            false,
            true,
        ),
    );

    // Tan functions
    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_FLOAT)];
    functions.insert(
        String::from("tan"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_tan)),
            false,
            true,
        ),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_FLOAT)];
    functions.insert(
        String::from("tand"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_tand)),
            false,
            true,
        ),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_FLOAT)];
    functions.insert(
        String::from("atan"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_atan)),
            false,
            true,
        ),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_FLOAT)];
    functions.insert(
        String::from("atand"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_atand)),
            false,
            true,
        ),
    );

    // Other math functions
    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_FLOAT)];
    functions.insert(
        String::from("ln"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_ln)),
            false,
            true,
        ),
    );

    let arguments = vec![AnnotatedIdentifier::new("a".to_string(), TYPEID_FLOAT)];
    functions.insert(
        String::from("log"),
        ClassFunction::new(
            Function::BuiltinFunction(BuiltinFunction::new(arguments, builtin_log)),
            false,
            true,
        ),
    );

    ClassDefinition::new_without_constructor(functions, TYPEID_MATH)
}

// Sine functions
fn builtin_sin(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value_clone() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Float(number.sin()))
}

fn builtin_sind(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value_clone() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Float(number.to_radians().sin()))
}

fn builtin_asin(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value_clone() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Float(number.asin()))
}

fn builtin_asind(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value_clone() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Float(number.asin().to_degrees()))
}

// Cosine functions
fn builtin_cos(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value_clone() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Float(number.cos()))
}

fn builtin_cosd(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value_clone() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Float(number.to_radians().cos()))
}

fn builtin_acos(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value_clone() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Float(number.acos()))
}

fn builtin_acosd(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value_clone() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Float(number.acos().to_degrees()))
}

// Tan functions
fn builtin_tan(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value_clone() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Float(number.tan()))
}

fn builtin_tand(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value_clone() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Float(number.to_radians().tan()))
}

fn builtin_atan(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value_clone() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Float(number.atan()))
}

fn builtin_atand(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value_clone() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Float(number.atan().to_degrees()))
}

// Other math functions
fn builtin_ln(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value_clone() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Float(number.ln()))
}

fn builtin_log(
    exec_session: &mut ExecSession,
    _: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let operand = exec_session.get_variable("a").unwrap();
    let number = match operand.get_value_clone() {
        Value::Float(number) => number,
        _ => panic!("Invalid value in built-in function"),
    };
    Ok(Value::Float(number.log(10.0_f64)))
}
