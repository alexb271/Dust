use crate::parser;
use crate::session::BacktraceItem;
use crate::token::Operator;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    CustomError(String),

    // General errors
    SyntaxError,
    ZeroDivision,
    FunctionNotFound,
    FunctionAlreadyDefined,
    IdentifierNotFound,
    IdentifierIsKeyword,
    IdentifierIsTypename,
    InvalidNumberOfArguments,
    IterationLimitReached,
    IndexOutOfRange(isize, usize),

    // Type related errors
    UnknownType(String),
    InvalidOperationForType(Operator, String),
    InvalidOperationForTypes(Operator, String, String),
    ConditionalExpressionNotBool(String),
    InvalidAssignment(String, String),
    InvalidArgumentType(String, String),
    InvalidReturnType(String, String),
    ForLoopNotVec(String),
    MissingAnnotation,

    // Class related errors
    HasNoMember(String, String),
    InvalidMemberAccess,
    InvalidScopeAccess,
    SelfOutsideMethod,
    MemberAlreadyDefined,
    RecursiveType,
    MemberIsPrivate(String),
    MemberFunctionIsPrivate(String),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::CustomError(message) => write!(f, "{message}"),

            // General errors
            ErrorKind::SyntaxError => write!(f, "Syntax error"),
            ErrorKind::ZeroDivision => write!(f, "Division by zero"),
            ErrorKind::FunctionNotFound => write!(f, "Function not found"),
            ErrorKind::FunctionAlreadyDefined => write!(f, "Function is already defined"),
            ErrorKind::IdentifierNotFound => write!(f, "Identifier not found"),
            ErrorKind::IdentifierIsKeyword => write!(f, "Expected identifier, found keyword"),
            ErrorKind::IdentifierIsTypename => write!(f, "Expected identifier, found type"),
            ErrorKind::InvalidNumberOfArguments => {
                write!(f, "Invalid number of arguments passed to function")
            }
            ErrorKind::IterationLimitReached => write!(f, "Maximum iteration count reached"),
            ErrorKind::IndexOutOfRange(i, s) => {
                write!(f, "Index '{}' is out of range for size '{}'", i, s)
            }

            // Type related errors
            ErrorKind::UnknownType(t) => {
                write!(f, "Unknown type '{}'", t)
            }
            ErrorKind::InvalidOperationForType(op, t) => {
                write!(f, "Invalid operation '{}' for type '{}'", op, t)
            }
            ErrorKind::InvalidOperationForTypes(op, t1, t2) => {
                write!(
                    f,
                    "Invalid operation '{}' for types '{}' and '{}'",
                    op, t1, t2
                )
            }
            ErrorKind::ConditionalExpressionNotBool(t) => {
                write!(
                    f,
                    "Conditional statement must receive a 'bool', found '{}'",
                    t
                )
            }
            ErrorKind::InvalidAssignment(t1, t2) => {
                write!(
                    f,
                    "Cannot assign to variable with type '{}' a value of type '{}'",
                    t1, t2
                )
            }
            ErrorKind::InvalidArgumentType(t1, t2) => {
                write!(f, "Invalid argument type '{}', expected '{}'", t1, t2)
            }
            ErrorKind::InvalidReturnType(t1, t2) => {
                write!(
                    f,
                    "Function returned type '{}', but its signature expects '{}'",
                    t1, t2
                )
            }
            ErrorKind::ForLoopNotVec(t) => {
                write!(
                    f,
                    "For loop operand is of type '{}' but it must be of type 'Vec'",
                    t
                )
            }
            ErrorKind::MissingAnnotation => write!(f, "Missing type annotation"),

            // Class related errors
            ErrorKind::HasNoMember(t, name) => {
                write!(f, "Object of type '{}' has no member called '{}'", t, name)
            }
            ErrorKind::InvalidMemberAccess => write!(f, "Invalid member access"),
            ErrorKind::InvalidScopeAccess => write!(f, "Invalid scope access"),
            ErrorKind::SelfOutsideMethod => {
                write!(f, "The 'self' parameter is only allowed in methods")
            }
            ErrorKind::MemberAlreadyDefined => {
                write!(f, "A member with this name is already defined")
            }
            ErrorKind::RecursiveType => {
                write!(f, "Recursive types are not allowed during initialization")
            }
            ErrorKind::MemberIsPrivate(name) => {
                write!(f, "Member '{}' is private", name)
            }
            ErrorKind::MemberFunctionIsPrivate(name) => {
                write!(f, "Member function '{}' is private", name)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Error {
    context: Context,
    pos: usize,
    kind: ErrorKind,
}

impl Error {
    pub fn new(context: Context, pos: usize, kind: ErrorKind) -> Error {
        Error { context, pos, kind }
    }

    pub fn print_to_string(&self, source_code: &str, backtrace: &Vec<BacktraceItem>) -> String {
        match self.kind {
            ErrorKind::IterationLimitReached => {
                format!("Error: {}", self.kind)
            }
            _ => {
                let mut result = String::new();
                let padding = source_code.lines().count().to_string().len();

                if !backtrace.is_empty() {
                    result += print_backtrace_to_string(backtrace, padding).as_str();
                }

                result += print_to_string_with_marker(source_code, self.context, self.pos, padding)
                    .as_ref();
                result += format!("\nError: {}", self.kind).as_ref();
                result
            }
        }
    }
}

fn print_to_string_with_marker(
    text: &str,
    mut context: Context,
    mut pos: usize,
    padding: usize,
) -> String {
    let mut result = String::new();
    let padding = padding + 1;

    if text.chars().nth(context.end - 1) == Some('\n') {
        context.end -= 1;
    }

    // offset the start of the context so that
    // it starts at the beginning of the line it's on

    let (line_count_to_context_start, distance_from_line_beginning) =
        parser::get_line_column(context.start, text);

    context.start = context.start - (distance_from_line_beginning - 1);
    pos += distance_from_line_beginning - 1;

    // offset the end of the context so that
    // it ends at the end of the line it's on

    let mut count = 0;
    for ch in text[context.end..text.len() - 1].chars() {
        if ch == '\n' {
            break;
        }
        count += 1;
    }
    context.end += count;

    // restrict the text to the adjusted context

    let relevant_text = &text[context.start..context.end];

    // there could still be newlines between the start of the context
    // and the marker we want to draw

    let (line_count_to_line_with_marker, distance_from_last_newline) =
        parser::get_line_column(pos, relevant_text);
    let line_count_to_line_with_marker = line_count_to_line_with_marker - 1;

    let line_count_total = line_count_to_context_start + line_count_to_line_with_marker;
    result.push_str(format!("In line {}:\n\n", line_count_total).as_str());

    // print all the lines that are between the start of the context
    // and the beginning of the line that contains the position of the marker
    // this could be zero so make sure the first line has its indicator printed

    let mut newline_count = 0;
    let mut iter = relevant_text.chars();
    let mut current_line = line_count_to_context_start;

    if line_count_to_line_with_marker > 0 {
        result.push_str(format!("{: >padding$}| ", current_line).as_str());
        current_line += 1;

        while let Some(ch) = iter.next() {
            result.push(ch);

            if ch == '\n' {
                result.push_str(format!("{: >padding$}| ", current_line).as_str());
                current_line += 1;

                newline_count += 1;
                if newline_count == line_count_to_line_with_marker {
                    break;
                }
            }
        }
    } else {
        result.push_str(format!("{: >padding$}| ", current_line).as_str());
        current_line += 1;
    }

    // print the actual line that contains the position of the marker
    // there may or may not be more lines after this one in the context

    while let Some(ch) = iter.next() {
        if ch == '\n' {
            break;
        }
        result.push(ch);
    }

    result.push('\n');

    // print the appropriate amount of spaces on the line below
    // including padding and the pipe and space characters
    // then print the marker itself

    for _ in 1..(distance_from_last_newline + padding + 2) {
        result.push(' ');
    }

    result.push('^');
    result.push('\n');

    // print the rest of the context, for example lines below the line
    // that contained the position of the marker

    if let Some(ch) = iter.next() {
        result.push_str(format!("{: >padding$}| ", current_line).as_str());
        current_line += 1;
        result.push(ch);
    }

    while let Some(ch) = iter.next() {
        result.push(ch);
        if ch == '\n' {
            result.push_str(format!("{: >padding$}| ", current_line).as_str());
            current_line += 1;
        }
    }

    // make sure the string ends with a newline

    if !result.ends_with('\n') {
        result.push('\n');
    }

    result
}

fn print_backtrace_to_string(backtrace: &Vec<BacktraceItem>, padding: usize) -> String {
    let mut result = String::from("Backtrace:\n\n");

    for item in backtrace {
        result += format!(
            "  {} called at {: >padding$}:{}\n",
            item.name, item.line, item.col
        )
        .as_ref();
    }

    result += "  root\n\n";
    result += format!("In function {}:\n", backtrace[0].name).as_ref();

    result
}
