use crate::error::{Context, Error, ErrorKind};
use crate::process_to_string;
use crate::token::Operator;
use crate::Session;

#[test]
fn invalid_math() {
    let mut session = Session::new();

    let result = process_to_string("print(1 / 0)", &mut session);
    let error = Error::new(Context { start: 6, end: 11 }, 2, ErrorKind::ZeroDivision);
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(5/(10 - 2 * 5))", &mut session);
    let error = Error::new(Context { start: 6, end: 20 }, 1, ErrorKind::ZeroDivision);
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);
}

#[test]
fn expression() {
    let mut session = Session::new();

    let result = process_to_string("", &mut session);
    let expected = "";
    assert_eq!(result, expected);

    let result = process_to_string("print((1 / 3) * 3)", &mut session);
    let expected = "1";
    assert_eq!(result, expected);

    let result = process_to_string("print(5+(2.0-3)*4.9)", &mut session);
    let expected = "0.1";
    assert_eq!(result, expected);

    let result = process_to_string("print(5+2.0-3*4.9)", &mut session);
    let expected = "-7.7";
    assert_eq!(result, expected);

    let result = process_to_string("print(5 + -2)", &mut session);
    let expected = "3";
    assert_eq!(result, expected);

    let result = process_to_string("print(2 + -2 ^ 8)", &mut session);
    let expected = "258";
    assert_eq!(result, expected);

    let result = process_to_string("print(10/5)", &mut session);
    let expected = "2";
    assert_eq!(result, expected);

    let result = process_to_string("print(2 + -2 ^ 8)", &mut session);
    let expected = "258";
    assert_eq!(result, expected);

    let result = process_to_string("print(1 / 5)", &mut session);
    let expected = "0.2";
    assert_eq!(result, expected);

    let result = process_to_string("print(-(2+3))", &mut session);
    let expected = "-5";
    assert_eq!(result, expected);

    let result = process_to_string("print(-(2+3) + 1)", &mut session);
    let expected = "-4";
    assert_eq!(result, expected);

    let result = process_to_string("print(-(2^3))", &mut session);
    let expected = "-8";
    assert_eq!(result, expected);

    let result = process_to_string("print(-(2^3) + 1)", &mut session);
    let expected = "-7";
    assert_eq!(result, expected);

    let result = process_to_string("print(2^-(1)*3)", &mut session);
    let expected = "1.5";
    assert_eq!(result, expected);

    let result = process_to_string("print(-(3/6))", &mut session);
    let expected = "-0.5";
    assert_eq!(result, expected);

    let result = process_to_string("print(-(3/6) + 1)", &mut session);
    let expected = "0.5";
    assert_eq!(result, expected);

    let result = process_to_string("print(456-41-675*2^3-15)", &mut session);
    let expected = "-5000";
    assert_eq!(result, expected);

    let result = process_to_string("print((456-41-675)*2^3-15)", &mut session);
    let expected = "-2095";
    assert_eq!(result, expected);

    let result = process_to_string("print(3+4*50/5^2%5-1)", &mut session);
    let expected = "5";
    assert_eq!(result, expected);

    let result = process_to_string("print((1+-4/2.5)*16-(7%2)^3/5)", &mut session);
    let expected = "-9.8";
    assert_eq!(result, expected);

    let result = process_to_string("print(((1+-4)/2.5)*16-(7%2)^3/5)", &mut session);
    let expected = "-19.4";
    assert_eq!(result, expected);

    let result = process_to_string("print(2^4*(10%4+17.5-5)/2.5)", &mut session);
    let expected = "92.8";
    assert_eq!(result, expected);

    let result = process_to_string("print(sin (pi))", &mut session);
    let expected = "0";
    assert_eq!(result, expected);

    let result = process_to_string("print(cos (pi/2))", &mut session);
    let expected = "0";
    assert_eq!(result, expected);

    let result = process_to_string("print(tan (pi/4))", &mut session);
    let expected = "1";
    assert_eq!(result, expected);

    let result = process_to_string("print(asin (1))", &mut session);
    let expected = "1.570796";
    assert_eq!(result, expected);

    let result = process_to_string("print(acos (0))", &mut session);
    let expected = "1.570796";
    assert_eq!(result, expected);

    let result = process_to_string("print(atan (pi))", &mut session);
    let expected = "1.262627";
    assert_eq!(result, expected);

    let result = process_to_string("print(sind (90))", &mut session);
    let expected = "1";
    assert_eq!(result, expected);

    let result = process_to_string("print(cosd (90))", &mut session);
    let expected = "0";
    assert_eq!(result, expected);

    let result = process_to_string("print(tand (45))", &mut session);
    let expected = "1";
    assert_eq!(result, expected);

    let result = process_to_string("print(asind (1))", &mut session);
    let expected = "90";
    assert_eq!(result, expected);

    let result = process_to_string("print(acosd (1))", &mut session);
    let expected = "0";
    assert_eq!(result, expected);

    let result = process_to_string("print(atand (1))", &mut session);
    let expected = "45";
    assert_eq!(result, expected);

    let result = process_to_string("print(ln (2))", &mut session);
    let expected = "0.693147";
    assert_eq!(result, expected);

    let result = process_to_string("print(log (100))", &mut session);
    let expected = "2";
    assert_eq!(result, expected);

    let result = process_to_string("print(sind(90)^2)", &mut session);
    let expected = "1";
    assert_eq!(result, expected);

    let result = process_to_string("print(sin (cos (0)))", &mut session);
    let expected = "0.841471";
    assert_eq!(result, expected);

    let result = process_to_string("print(abs (-5.25))", &mut session);
    let expected = "5.25";
    assert_eq!(result, expected);

    let result = process_to_string("print(abs(5.25))", &mut session);
    let expected = "5.25";
    assert_eq!(result, expected);

    let result = process_to_string("print(true and true)", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    let result = process_to_string("print(true and false)", &mut session);
    let expected = "false";
    assert_eq!(result, expected);

    let result = process_to_string("print(false and true)", &mut session);
    let expected = "false";
    assert_eq!(result, expected);

    let result = process_to_string("print(false and false)", &mut session);
    let expected = "false";
    assert_eq!(result, expected);

    let result = process_to_string("print(true or true)", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    let result = process_to_string("print(true or false)", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    let result = process_to_string("print(false or true)", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    let result = process_to_string("print(false or false)", &mut session);
    let expected = "false";
    assert_eq!(result, expected);

    let result = process_to_string("print(not false)", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    let result = process_to_string("print(not true)", &mut session);
    let expected = "false";
    assert_eq!(result, expected);

    let result = process_to_string("print(1 < 2)", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    let result = process_to_string("print(2 < 1)", &mut session);
    let expected = "false";
    assert_eq!(result, expected);

    let result = process_to_string("print(1 > 2)", &mut session);
    let expected = "false";
    assert_eq!(result, expected);

    let result = process_to_string("print(2 > 1)", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    let result = process_to_string("print(1 == 1)", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    let result = process_to_string("print(1 == 2)", &mut session);
    let expected = "false";
    assert_eq!(result, expected);

    let result = process_to_string("print(2 == 1)", &mut session);
    let expected = "false";
    assert_eq!(result, expected);

    let result = process_to_string("print(1 != 1)", &mut session);
    let expected = "false";
    assert_eq!(result, expected);

    let result = process_to_string("print(1 != 2)", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    let result = process_to_string("print(2 != 1)", &mut session);
    let expected = "true";
    assert_eq!(result, expected);
}

#[test]
fn scripting() {
    let mut session = Session::new();

    // variables
    let result = process_to_string("print(x)", &mut session);
    let error = Error::new(
        Context { start: 6, end: 7 },
        0,
        ErrorKind::IdentifierNotFound,
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    let _result = process_to_string("let x = 5", &mut session);
    session.clear();
    let result = process_to_string("print(x)", &mut session);
    let error = Error::new(
        Context { start: 6, end: 7 },
        0,
        ErrorKind::IdentifierNotFound,
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    let result = process_to_string("let x = 3; print(x)", &mut session);
    let expected = "3";
    assert_eq!(result, expected);

    let result = process_to_string("let x = 3\nprint(x + 1)", &mut session);
    let expected = "4";
    assert_eq!(result, expected);

    let result = process_to_string("let x, y = 1\nprint(x + y)", &mut session);
    let expected = "2";
    assert_eq!(result, expected);

    let result = process_to_string("let x = 1\nx = x + 10\nprint(x)", &mut session);
    let expected = "11";
    assert_eq!(result, expected);

    let result = process_to_string("let x = (5 + 2) ^ 3\nx = x + 1\nprint(x)", &mut session);
    let expected = "344";
    assert_eq!(result, expected);

    // while_loop
    let result = process_to_string(
        "let x = 0; while x < 5 { x = x + 1; print(x); print(\"\\n\") }",
        &mut session,
    );
    let expected = "1\n2\n3\n4\n5\n";
    assert_eq!(result, expected);

    let result = process_to_string(
        "x = 0; while x < 5 { x = x + 1; print(x); print(\"\\n\"); if x == 3 { break }}",
        &mut session,
    );
    let expected = "1\n2\n3\n";
    assert_eq!(result, expected);

    let result = process_to_string(
        "
let x, y, z = 0
while x < 12 {
  while y < 4 {
    y = y + 1
  }
  x = x + 1
  if x + y > 9 {
    z = z + 1
    if x == 8 {
      if true and true and not false {
        break
      }
    }
  }
}
print(x + y + z)",
        &mut session,
    );
    let expected = "15";
    assert_eq!(result, expected);

    // branching
    let result = process_to_string(
        "
let x = 0
if (not false) == true {
  if (5 == 3) or not true { x = x + 1 }
  if 0 < 11 - (2 * 5) and 5 / 5 == 1 { x = x + 5 }
}
print(x)",
        &mut session,
    );
    let expected = "5";
    assert_eq!(result, expected);

    let result = process_to_string(
        "let x = 3; if x > 10 {x = x + 10} else { x = x - 1 } print(x)",
        &mut session,
    );
    let expected = "2";
    assert_eq!(result, expected);

    // functions
    session.clear();
    let result = process_to_string("print(pow(2, 6))", &mut session);
    let error = Error::new(
        Context { start: 6, end: 15 },
        0,
        ErrorKind::IdentifierNotFound,
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    let result = process_to_string(
        "
fn power(base: number, exponent: number) {
    if exponent == 0 {
        return 1
    }
    return base * power(base, exponent - 1)
}

print(power(2, 8))",
        &mut session,
    );
    let expected = "256";
    assert_eq!(result, expected);

    let result = process_to_string(
        "
fn count_up () {
    let x = 0
    while true {
        let three = 3
        if x == 5 {
            break
        }
        else {
            x = x + 1
            print(x)
            print(\"\\n\")
        }
    }
    if x == 5 {
        let four = 4
        return add_one(x)
    }
    x = x + 10
    return x
}

fn add_one(lhs: number) {
    return lhs + 1
}

print(add_one(count_up()))",
        &mut session,
    );
    let expected = "1\n2\n3\n4\n5\n7";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "if (false) {
    print(1)
} else if (false) {
    print(2)
}
else if (true) {
    print(3)
} else {
     print(4)
}",
        &mut session,
    );
    let expected = "3";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn printer(x: dyn) { print(x) }\nprinter(5)", &mut session);
    let expected = "5";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn f(x: dyn) {x}\nf()", &mut session);
    let error = Error::new(
        Context { start: 17, end: 20 },
        0,
        ErrorKind::InvalidNumberOfArguments,
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn f(x: dyn) {x}\nf(1, 2)", &mut session);
    let error = Error::new(
        Context { start: 17, end: 24 },
        0,
        ErrorKind::InvalidNumberOfArguments,
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);
}

#[test]
fn syntax() {
    let mut session = Session::new();

    let result = process_to_string("break", &mut session);
    let error = Error::new(Context { start: 0, end: 5 }, 0, ErrorKind::SyntaxError);
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn breaker() { break }", &mut session);
    let error = Error::new(Context { start: 15, end: 18 }, 0, ErrorKind::SyntaxError);
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "fn breaker(x: number) { return\nx + 1 }\nprint\n(breaker(2))",
        &mut session,
    );
    let expected = "3";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("while (true) {break}", &mut session);
    let expected = "";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "let break1 = 1\nlet breaker = 2\nlet break_puller = 3\nprint(break1 + breaker + break_puller)",
        &mut session,
    );
    let expected = "6";
    assert_eq!(result, expected);
}

#[test]
fn iteration_limit() {
    let mut session = Session::new();

    let result = process_to_string("fn inf_rec() { inf_rec() }\ninf_rec()", &mut session);
    let error = Error::new(
        Context { start: 0, end: 0 },
        0,
        ErrorKind::IterationLimitReached,
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);
}

#[test]
fn type_system() {
    let mut session = Session::new();

    let result = process_to_string("print(typeof not true)", &mut session);
    let expected = "bool";
    assert_eq!(result, expected);

    let result = process_to_string("let x = 5; print(typeof x == \"number\")", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(true + true)", &mut session);
    let error = Error::new(
        Context { start: 6, end: 17 },
        5,
        ErrorKind::InvalidOperationForTypes(Operator::Add, "bool".to_string(), "bool".to_string()),
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(- not true)", &mut session);
    let error = Error::new(
        Context { start: 6, end: 16 },
        0,
        ErrorKind::InvalidOperationForType(Operator::Neg, "bool".to_string()),
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(not \"str\" ^ 1)", &mut session);
    let error = Error::new(
        Context { start: 6, end: 19 },
        10,
        ErrorKind::InvalidOperationForTypes(
            Operator::Pow,
            "string".to_string(),
            "number".to_string(),
        ),
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(not 1 + 1)", &mut session);
    let error = Error::new(
        Context { start: 6, end: 14 },
        0,
        ErrorKind::InvalidOperationForType(Operator::Not, "number".to_string()),
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn nothing() {}\n5 + nothing()", &mut session);
    let error = Error::new(
        Context { start: 16, end: 29 },
        2,
        ErrorKind::InvalidOperationForTypes(
            Operator::Add,
            "number".to_string(),
            "none".to_string(),
        ),
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let x: number = true", &mut session);
    let error = Error::new(
        Context { start: 0, end: 20 },
        14,
        ErrorKind::InvalidAssignment("number".to_string(), "bool".to_string()),
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let x = 5\nx = true\nprint(x)", &mut session);
    let error = Error::new(
        Context { start: 10, end: 18 },
        2,
        ErrorKind::InvalidAssignment("number".to_string(), "bool".to_string()),
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let x: dyn = 5\nx = true; print(x)", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "let x, y: dyn = 0\ny = true; print(y)\nx = true; print(x)",
        &mut session,
    );
    let error = Error::new(
        Context { start: 37, end: 45 },
        2,
        ErrorKind::InvalidAssignment("number".to_string(), "bool".to_string()),
    );
    let expected = "true".to_string()
        + &error.print_to_string(
            session.parsed_session.get_source_code(),
            session.exec_session.get_backtrace(),
        );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let x : asd = 5", &mut session);
    let error = Error::new(
        Context { start: 0, end: 15 },
        8,
        ErrorKind::UnknownType("asd".to_string()),
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn function(x: number, y) { return x + y }", &mut session);
    let error = Error::new(
        Context { start: 0, end: 42 },
        23,
        ErrorKind::MissingAnnotation,
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "fn fun(x: number) {x = true; print(x)}\nfun(1)",
        &mut session,
    );
    let error = Error::new(
        Context { start: 19, end: 28 },
        2,
        ErrorKind::InvalidAssignment("number".to_string(), "bool".to_string()),
    );
    let expected = "Backtrace:\n\n  fun called at 2:1\n  root\n\nIn function fun:\n".to_string()
        + &error.print_to_string(
            session.parsed_session.get_source_code(),
            session.exec_session.get_backtrace(),
        );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "fn fun(x: dyn) { x = true; print(x) }\nfun(1)",
        &mut session,
    );
    let expected = "true";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "fn fun(x: number) { x = x + 1; print(x) }\nlet a = none; fun(a)",
        &mut session,
    );
    let error = Error::new(
        Context { start: 56, end: 58 },
        4,
        ErrorKind::InvalidArgumentType("none".to_string(), "number".to_string()),
    );
    let expected = error.print_to_string(
        session.parsed_session.get_source_code(),
        session.exec_session.get_backtrace(),
    );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "1\n1\nfn f() -> bool {\n  if true {\n    return 5 + 1\n  }\n}\nf()",
        &mut session,
    );
    let error = Error::new(
        Context { start: 23, end: 53 },
        21,
        ErrorKind::InvalidReturnType("number".to_string(), "bool".to_string()),
    );
    let expected = "Backtrace:\n\n  f called at 8:1\n  root\n\nIn function f:\n".to_string()
        + &error.print_to_string(
            session.parsed_session.get_source_code(),
            session.exec_session.get_backtrace(),
        );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("1\n1\nfn f() -> number {\n  1 + 1\n}\nf()", &mut session);
    let error = Error::new(
        Context { start: 4, end: 20 },
        10,
        ErrorKind::InvalidReturnType("none".to_string(), "number".to_string()),
    );
    let expected = "Backtrace:\n\n  f called at 6:1\n  root\n\nIn function f:\n".to_string()
        + &error.print_to_string(
            session.parsed_session.get_source_code(),
            session.exec_session.get_backtrace(),
        );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn f() -> bool {\n  1 + 1\n  return;\n}\nf()", &mut session);
    let error = Error::new(
        Context { start: 27, end: 34 },
        0,
        ErrorKind::InvalidReturnType("none".to_string(), "bool".to_string()),
    );
    let expected = "Backtrace:\n\n  f called at 5:1\n  root\n\nIn function f:\n".to_string()
        + &error.print_to_string(
            session.parsed_session.get_source_code(),
            session.exec_session.get_backtrace(),
        );
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn f() -> dyn { return 0 }\nprint(f())", &mut session);
    let expected = "0".to_string();
    assert_eq!(result, expected);
}

#[test]
fn number_parsing() {
    let mut session = Session::new();

    let result = process_to_string(
        "let x = parse_number(\"64.32\"); print(x + 1)",
        &mut session,
    );
    let expected = "65.32";
    assert_eq!(result, expected);

    let result = process_to_string(
        "let x = parse_number(\"not a number\"); print(is_nan(x))",
        &mut session,
    );
    let expected = "true";
    assert_eq!(result, expected);
}

#[test]
fn error_format() {
    let mut session = Session::new();

    let result = process_to_string(
        "7 + 2\n0 + 0\nprint(1 +\n2 + f(12) - 3\n+ 9 * 5)",
        &mut session,
    );
    let expected = "In line 4:

 3| print(1 +
 4| 2 + f(12) - 3
        ^
 5| + 9 * 5)

Error: Identifier not found";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(1 +\n2 + f(12) - 3\n+ 9 * 5)", &mut session);
    let expected = "In line 2:

 1| print(1 +
 2| 2 + f(12) - 3
        ^
 3| + 9 * 5)

Error: Identifier not found";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(2 + f(12) - 3)", &mut session);
    let expected = "In line 1:

 1| print(2 + f(12) - 3)
              ^

Error: Identifier not found";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(2 + f(12) - 3)\n1 + 2", &mut session);
    let expected = "In line 1:

 1| print(2 + f(12) - 3)
              ^

Error: Identifier not found";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("\nprint(2 + f(12) - 3)", &mut session);
    let expected = "In line 2:

 2| print(2 + f(12) - 3)
              ^

Error: Identifier not found";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "fn f1() {
    print(\"f1\\n\")
    f2()
}

fn f2() {
    print(\"f2\\n\")
    f3()
}

fn f3() {
    print(\"f3\\n\")
    0 / 0
}

f1()",
        &mut session,
    );
    let expected = "f1
f2
f3
Backtrace:

  f3 called at  8:5
  f2 called at  3:5
  f1 called at 16:1
  root

In function f3:
In line 13:

 13|     0 / 0
           ^

Error: Division by zero";
    assert_eq!(result, expected);
}
