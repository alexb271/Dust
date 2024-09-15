use crate::error::{Context, Error, ErrorKind};
use crate::process_to_string;
use crate::token::Operator;
use crate::Session;

#[test]
fn math() {
    let mut session = Session::new();

    let result = process_to_string("print(1 / 0.0)", &mut session);
    let error = Error::new(Context { start: 6, end: 11 }, 2, ErrorKind::ZeroDivision);
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(5/(10 - 2 * 5))", &mut session);
    let error = Error::new(Context { start: 6, end: 20 }, 1, ErrorKind::ZeroDivision);
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::sin (float::pi()))", &mut session);
    let expected = "0";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::cos (float::pi()/2))", &mut session);
    let expected = "0";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::tan (float::pi()/4))", &mut session);
    let expected = "1";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::asin (1.0))", &mut session);
    let expected = "1.570796";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::acos (0.0))", &mut session);
    let expected = "1.570796";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::atan (float::pi()))", &mut session);
    let expected = "1.262627";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::sind (90.0))", &mut session);
    let expected = "1";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::cosd (90.0))", &mut session);
    let expected = "0";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::tand (45.0))", &mut session);
    let expected = "1";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::asind (1.0))", &mut session);
    let expected = "90";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::acosd (1.0))", &mut session);
    let expected = "0";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::atand (1.0))", &mut session);
    let expected = "45";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::ln (2.0))", &mut session);
    let expected = "0.693147";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::log (100.0))", &mut session);
    let expected = "2";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::sind(90.0)^2)", &mut session);
    let expected = "1";
    assert_eq!(result, expected);

    let result = process_to_string("print(Math::sin (Math::cos (0.0)))", &mut session);
    let expected = "0.841471";
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

    let result = process_to_string("print((-5.25).abs())", &mut session);
    let expected = "5.25";
    assert_eq!(result, expected);

    let result = process_to_string("print(5.25.abs())", &mut session);
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

    let result = process_to_string("print(\"asd\" < \"qwe\")", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    let result = process_to_string("print(\"asd\" > \"qwe\")", &mut session);
    let expected = "false";
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
fn variables() {
    let mut session = Session::new();

    let result = process_to_string("print(x)", &mut session);
    let error = Error::new(
        Context { start: 6, end: 7 },
        0,
        ErrorKind::IdentifierNotFound,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    let _result = process_to_string("let x = 5", &mut session);
    session.clear();
    let result = process_to_string("print(x)", &mut session);
    let error = Error::new(
        Context { start: 6, end: 7 },
        0,
        ErrorKind::IdentifierNotFound,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
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
}

#[test]
fn looping() {
    let mut session = Session::new();

    session.clear();
    let result = process_to_string(
        "let x = 0; while x < 5 { x = x + 1; print(x); print(\"\\n\") }",
        &mut session,
    );
    let expected = "1\n2\n3\n4\n5\n";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "let x = 0; while x < 5 { x = x + 1; print(x); print(\"\\n\"); if x == 3 { break }}",
        &mut session,
    );
    let expected = "1\n2\n3\n";
    assert_eq!(result, expected);

    session.clear();
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

    session.clear();
    let result = process_to_string(
        "
let x = 10;
let v = Vec::new();
v.push(1); v.push(2); v.push(3);
for item in v {
    println(item + x);
}",
        &mut session,
    );
    let expected = "11\n12\n13\n";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "
let v = Vec::new();
v.push(1); v.push(2); v.push(3); v.push(4); v.push(5); v.push(6);
for item in v {
    if item == 4 {
        break;
    }
    println(item);
}",
        &mut session,
    );
    let expected = "1\n2\n3\n";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "
class A {
    a: Vec = Vec::new();
    b: Vec = Vec::new();

    pub fn test(self) {
        self.a.push(1); self.a.push(2); self.a.push(3);
        self.b.push(4); self.b.push(5); self.b.push(6);

        if true {
            while true {
                for a in self.a {
                    println(a)
                    for b in self.b {
                        print(b)
                    }
                    println(\"\")
                }
                break;
            }
        }
    }
}
A::new().test();",
        &mut session,
    );
    let expected = "1\n456\n2\n456\n3\n456\n";
    assert_eq!(result, expected);
}

#[test]
fn branching() {
    let mut session = Session::new();

    session.clear();
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

    let result = process_to_string(
        "let x = 3; if x > 10 {x = x + 10} else { x = x - 1 } print(x)",
        &mut session,
    );
    let expected = "2";
    assert_eq!(result, expected);
}

#[test]
fn functions() {
    let mut session = Session::new();

    session.clear();
    let result = process_to_string("fn print() {}", &mut session);
    let error = Error::new(
        Context { start: 0, end: 13 },
        3,
        ErrorKind::FunctionAlreadyDefined,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(pow(2, 6))", &mut session);
    let error = Error::new(
        Context { start: 6, end: 15 },
        0,
        ErrorKind::FunctionNotFound,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn f(x: dyn) {x}\nf()", &mut session);
    let error = Error::new(
        Context { start: 17, end: 20 },
        0,
        ErrorKind::InvalidNumberOfArguments,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn f(x: dyn) {x}\nf(1, 2)", &mut session);
    let error = Error::new(
        Context { start: 17, end: 24 },
        0,
        ErrorKind::InvalidNumberOfArguments,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn printer(x: dyn) { print(x) }\nprinter(5)", &mut session);
    let expected = "5";
    assert_eq!(result, expected);

    let result = process_to_string(
        "
fn power(base: int, exponent: int) -> int {
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
fn count_up () -> int {
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

fn add_one(lhs: int) -> int {
    return lhs + 1
}

print(add_one(count_up()))",
        &mut session,
    );
    let expected = "1\n2\n3\n4\n5\n7";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "
if (false) {
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
}

#[test]
fn type_system() {
    let mut session = Session::new();

    let result = process_to_string("print(typeof not true)", &mut session);
    let expected = "bool";
    assert_eq!(result, expected);

    let result = process_to_string("let x = 5; print(typeof x == \"int\")", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(true + true)", &mut session);
    let error = Error::new(
        Context { start: 6, end: 17 },
        5,
        ErrorKind::InvalidOperationForTypes(Operator::Add, "bool".to_string(), "bool".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(- not true)", &mut session);
    let error = Error::new(
        Context { start: 6, end: 16 },
        0,
        ErrorKind::InvalidOperationForType(Operator::Neg, "bool".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(not \"str\" ^ 1)", &mut session);
    let error = Error::new(
        Context { start: 6, end: 19 },
        10,
        ErrorKind::InvalidOperationForTypes(Operator::Pow, "string".to_string(), "int".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(not 1 + 1)", &mut session);
    let error = Error::new(
        Context { start: 6, end: 14 },
        0,
        ErrorKind::InvalidOperationForType(Operator::Not, "int".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn nothing() {}\n5 + nothing()", &mut session);
    let error = Error::new(
        Context { start: 16, end: 29 },
        2,
        ErrorKind::InvalidOperationForTypes(Operator::Add, "int".to_string(), "none".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let x: int = true", &mut session);
    let error = Error::new(
        Context { start: 0, end: 17 },
        11,
        ErrorKind::InvalidAssignment("int".to_string(), "bool".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let x = 5\nx = true\nprint(x)", &mut session);
    let error = Error::new(
        Context { start: 10, end: 18 },
        2,
        ErrorKind::InvalidAssignment("int".to_string(), "bool".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
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
        ErrorKind::InvalidAssignment("int".to_string(), "bool".to_string()),
    );
    let expected = "true".to_string()
        + &error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let x : asd = 5", &mut session);
    let error = Error::new(
        Context { start: 0, end: 15 },
        8,
        ErrorKind::UnknownType("asd".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn function(x: int, y) { return x + y }", &mut session);
    let error = Error::new(
        Context { start: 0, end: 39 },
        20,
        ErrorKind::MissingAnnotation,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn fun(x: int) {x = true; print(x)}\nfun(1)", &mut session);
    let error = Error::new(
        Context { start: 16, end: 25 },
        2,
        ErrorKind::InvalidAssignment("int".to_string(), "bool".to_string()),
    );
    let expected = "Backtrace:\n\n  fun called at 2:1\n  root\n\nIn function fun:\n".to_string()
        + &error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
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
        "fn fun(x: int) { x = x + 1; print(x) }\nlet a = none; fun(a)",
        &mut session,
    );
    let error = Error::new(
        Context { start: 53, end: 55 },
        4,
        ErrorKind::InvalidArgumentType("none".to_string(), "int".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "1\n1\nfn f() -> bool {\n  if true {\n    return 5 + 1\n  }\n}\nf()",
        &mut session,
    );
    let error = Error::new(
        Context { start: 23, end: 53 },
        21,
        ErrorKind::InvalidReturnType("int".to_string(), "bool".to_string()),
    );
    let expected = "Backtrace:\n\n  f called at 8:1\n  root\n\nIn function f:\n".to_string()
        + &error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("1\n1\nfn f() -> int {\n  1 + 1\n}\nf()", &mut session);
    let error = Error::new(
        Context { start: 4, end: 20 },
        10,
        ErrorKind::InvalidReturnType("none".to_string(), "int".to_string()),
    );
    let expected = "Backtrace:\n\n  f called at 6:1\n  root\n\nIn function f:\n".to_string()
        + &error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn f() -> bool {\n  1 + 1\n  return;\n}\nf()", &mut session);
    let error = Error::new(
        Context { start: 27, end: 34 },
        0,
        ErrorKind::InvalidReturnType("none".to_string(), "bool".to_string()),
    );
    let expected = "Backtrace:\n\n  f called at 5:1\n  root\n\nIn function f:\n".to_string()
        + &error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("for i in 123 {\n}", &mut session);
    let error = Error::new(
        Context { start: 0, end: 16 },
        9,
        ErrorKind::ForLoopNotVec("int".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn f() -> dyn { return 0 }\nprint(f())", &mut session);
    let expected = "0".to_string();
    assert_eq!(result, expected);
}

#[test]
fn class() {
    let mut session = Session::new();

    session.clear();
    let result = process_to_string("class A {a:dyn = none; a:dyn = none;}", &mut session);
    let error = Error::new(
        Context { start: 0, end: 37 },
        23,
        ErrorKind::MemberAlreadyDefined,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("class A { fn f() {} fn f() {} }", &mut session);
    let error = Error::new(
        Context { start: 20, end: 29 },
        3,
        ErrorKind::FunctionAlreadyDefined,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("class A {a: A = A::new()}", &mut session);
    let error = Error::new(Context { start: 0, end: 25 }, 9, ErrorKind::RecursiveType);
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("class A {a:dyn = A::new()}", &mut session);
    let error = Error::new(Context { start: 0, end: 26 }, 9, ErrorKind::RecursiveType);
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("5.member", &mut session);
    let error = Error::new(
        Context { start: 0, end: 8 },
        1,
        ErrorKind::InvalidMemberAccess,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    assert_eq!(result, expected);
    session.clear();
    let result = process_to_string("int::f()", &mut session);
    let error = Error::new(Context { start: 0, end: 8 }, 5, ErrorKind::FunctionNotFound);
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("class A {}\nlet a = A::new();\na.int::f()", &mut session);
    let error = Error::new(
        Context { start: 29, end: 39 },
        5,
        ErrorKind::InvalidScopeAccess,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("class A {}\nlet a = A::new();\na.member", &mut session);
    let error = Error::new(
        Context { start: 29, end: 37 },
        2,
        ErrorKind::HasNoMember("A".to_string(), "member".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn f(self) {}", &mut session);
    let error = Error::new(
        Context { start: 0, end: 13 },
        5,
        ErrorKind::SelfOutsideMethod,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "class A { fn hello() {} } let x = A::new(); x.hello()",
        &mut session,
    );
    let error = Error::new(
        Context { start: 44, end: 53 },
        2,
        ErrorKind::FunctionNotFound,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("class A { fn hello(self) {} } A::hello();", &mut session);
    let error = Error::new(
        Context { start: 30, end: 40 },
        3,
        ErrorKind::FunctionNotFound,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("class A { fn f(self) { self = none }}", &mut session);
    let error = Error::new(
        Context { start: 23, end: 34 },
        0,
        ErrorKind::IdentifierIsKeyword,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("class A(x: bool) {} let a = A::new()", &mut session);
    let error = Error::new(
        Context { start: 20, end: 37 },
        11,
        ErrorKind::InvalidNumberOfArguments,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("class A {item:bool = 0;}\nlet a = A::new();", &mut session);
    let error = Error::new(
        Context { start: 9, end: 23 },
        10,
        ErrorKind::InvalidAssignment("bool".to_string(), "int".to_string()),
    );
    let expected = "Backtrace:\n\n  new called at 2:9\n  root\n\nIn function new:\n".to_string()
        + &error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "class A {x: bool = false} let a = A::new(); print(a.x)",
        &mut session,
    );
    let error = Error::new(
        Context { start: 50, end: 54 },
        2,
        ErrorKind::MemberIsPrivate("x".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "class A {x: bool = false} let a = A::new(); a.x = true",
        &mut session,
    );
    let error = Error::new(
        Context { start: 44, end: 54 },
        2,
        ErrorKind::MemberIsPrivate("x".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("class A {fn hello(){}} A::hello();", &mut session);
    let error = Error::new(
        Context { start: 23, end: 34 },
        3,
        ErrorKind::MemberFunctionIsPrivate("hello".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "class A {fn hello(self){}} let a = A::new(); a.hello()",
        &mut session,
    );
    let error = Error::new(
        Context { start: 45, end: 54 },
        2,
        ErrorKind::MemberFunctionIsPrivate("hello".to_string()),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "class A {pub item: bool = false;} let a = A::new(); print(a.item);",
        &mut session,
    );
    let expected = "false";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "
class Num {pub num: int = 0;}
class A {pub num: Num = Num::new();}
let a = A::new();
a.num.num = 25;
print(a.num.num);",
        &mut session,
    );
    let expected = "25";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "class MyClass {pub item: dyn = none;} let a = MyClass::new(); a.item = a; print(a);",
        &mut session,
    );
    let expected = "MyClass";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "
class MyClass {
    pub item: int = 0;
    pub fn print_item(self, prefix: string) {
        self.item = self.item + 1;
        print(prefix + self.item.to_string());
    }
}
let x = MyClass::new();
x.item = 10;
x.print_item(\"Item: \");",
        &mut session,
    );
    let expected = "Item: 11";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "
class MyNum {
    pub num: int = 0;
}

class MyClass {
    my_num: MyNum = MyNum::new();

    pub fn get_my_num(self) -> MyNum {
        return self.my_num;
    }
}

let x = MyClass::new();
x.get_my_num().num = 8;
print(x.get_my_num().num.to_string());",
        &mut session,
    );
    let expected = "8";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "
class A {
    pub fn get_number(self) -> int {
        return self.get_five() + 1;
    }

    fn get_five(self) -> int {
        return 5;
    }
}

let a = A::new();
print(a.get_number());",
        &mut session,
    );
    let expected = "6";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "
class A {
    pub fn get_number() -> int {
        return A::get_five() + 1;
    }

    fn get_five() -> int {
        return 5;
    }
}

print(A::get_number());",
        &mut session,
    );
    let expected = "6";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "class MyClass(x: int) {pub item: dyn = x + 1;} let a = MyClass::new(7); print(a.item);",
        &mut session,
    );
    let expected = "8";
    assert_eq!(result, expected);
}

#[test]
fn string() {
    let mut session = Session::new();

    session.clear();
    let result = process_to_string("print(\"asd\".len())", &mut session);
    let expected = "3";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let s = \"qwe\"; print(s.pop()); print(s)", &mut session);
    let expected = "eqw";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let s = \"   qwe   \"; print(s.trim())", &mut session);
    let expected = "qwe";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let s = \"qWe\"; print(s.to_lowercase())", &mut session);
    let expected = "qwe";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let s = \"qWe\"; print(s.to_uppercase())", &mut session);
    let expected = "QWE";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(\"catpaw\".starts_with(\"cat\"))", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(\"catpaw\".starts_with(\"paw\"))", &mut session);
    let expected = "false";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(\"catpaw\".ends_with(\"cat\"))", &mut session);
    let expected = "false";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(\"catpaw\".ends_with(\"paw\"))", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let x = \"cat\"; x.join(\"paw\"); print(x)", &mut session);
    let expected = "catpaw";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(\"qwe\".chars())", &mut session);
    let expected = "(\"q\", \"w\", \"e\")";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "print(\"cat-fur-paw\".substring(4, 7).unwrap())",
        &mut session,
    );
    let expected = "fur";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "print(\"cat-fur-paw\".substring(400, 700).is_ok())",
        &mut session,
    );
    let expected = "false";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(\"cat:fur:paw\".split(\":\"))", &mut session);
    let expected = "(\"cat\", \"fur\", \"paw\")";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(\"catfurpaw\".contains(\"fur\"))", &mut session);
    let expected = "true";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(\"catfurpaw\".contains(\"ear\"))", &mut session);
    let expected = "false";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(\"cat:fur:paw\".split(\"-\"))", &mut session);
    let expected = "(\"cat:fur:paw\")";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(\"cat:fur:paw\".split(\"--\"))", &mut session);
    let error = Error::new(
        Context { start: 6, end: 31 },
        14,
        ErrorKind::CustomError(String::from("Delimiter must be a single character")),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);
}

#[test]
fn vec() {
    let mut session = Session::new();

    session.clear();
    let result = process_to_string("let v = Vec::new(); v.get(-1);", &mut session);
    let error = Error::new(
        Context { start: 20, end: 30 },
        2,
        ErrorKind::IndexOutOfRange(-1, 0),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let v = Vec::new(); v.get(0);", &mut session);
    let error = Error::new(
        Context { start: 20, end: 30 },
        2,
        ErrorKind::IndexOutOfRange(0, 0),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let v = Vec::new(); v.push(true); v.get(1);", &mut session);
    let error = Error::new(
        Context { start: 34, end: 43 },
        2,
        ErrorKind::IndexOutOfRange(1, 1),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let v = Vec::new(); print(v);", &mut session);
    let expected = "()";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let v = Vec::new(); v.push(v); print(v);", &mut session);
    let expected = "([...])";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "let v = Vec::new(); v.push(1); v.push(2); v.push(3); print(v)",
        &mut session,
    );
    let expected = "(1, 2, 3)";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "let v = Vec::new(); v.push(1); v.push(2); v.push(3); print(v.len())",
        &mut session,
    );
    let expected = "3";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "
let v = Vec::new(); v.push(1); v.push(2); v.push(3);
print(v.get(0)); print(v.get(1)); print(v.get(2))",
        &mut session,
    );
    let expected = "123";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "
let v = Vec::new(); v.push(1); v.push(2); v.push(3);
v.set(1, \"#\")
print(v.get(0)); print(v.get(1)); print(v.get(2))",
        &mut session,
    );
    let expected = "1#3";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "
let v = Vec::new(); v.push(1); v.push(2); v.push(3);
v.clear(); v.push(9);
for item in v { print(item); }",
        &mut session,
    );
    let expected = "9";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "
let v = Vec::new(); v.push(1); v.push(2); v.push(3);
print(v.remove(1))
print(v)
print(v.pop())
print(v)",
        &mut session,
    );
    let expected = "2(1, 3)3(1)";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("for i in range(0, 10) { print(i) }", &mut session);
    let expected = "0123456789";
    assert_eq!(result, expected);
}

#[test]
fn number_parsing() {
    let mut session = Session::new();

    session.clear();
    let result = process_to_string(
        "
let x = int::parse(\"42\");
if x.is_ok() {
    print(x.value() + 1)
}",
        &mut session,
    );
    let expected = "43";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "
let x = float::parse(\"64.32\");
if x.is_ok() {
    print(x.value() + 1)
}",
        &mut session,
    );
    let expected = "65.32";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "let x = float::parse(\"not a number\"); print(x.is_ok())",
        &mut session,
    );
    let expected = "false";
    assert_eq!(result, expected);
}

#[test]
fn result() {
    let mut session = Session::new();

    session.clear();
    let result = process_to_string(
        "let r = Result::new(true, none); print(r.value())",
        &mut session,
    );
    let error = Error::new(
        Context { start: 39, end: 48 },
        2,
        ErrorKind::CustomError(String::from(
            "A 'Result' must be checked with 'is_ok()' before accessing its value",
        )),
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "let r = Result::new(false, none); print(r.unwrap())",
        &mut session,
    );
    let error = Error::new(
        Context { start: 40, end: 50 },
        2,
        ErrorKind::CustomError(String::from(
            "Unwrap called on a 'Result' containing an error",
        )),
    );

    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "let r = Result::new(true, \"Hello!\"); if r.is_ok() { print(r.value()) }",
        &mut session,
    );
    let expected = "Hello!";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "let r = Result::new(true, \"Hello!\");  print(r.unwrap())",
        &mut session,
    );
    let expected = "Hello!";
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
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);
}

#[test]
fn syntax() {
    let mut session = Session::new();

    session.clear();
    let result = process_to_string("break", &mut session);
    let error = Error::new(Context { start: 0, end: 5 }, 0, ErrorKind::SyntaxError);
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn breaker() { break }", &mut session);
    let error = Error::new(Context { start: 15, end: 18 }, 0, ErrorKind::SyntaxError);
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string(
        "fn breaker(x: int) -> int { return\nx + 1 }\nprint\n(breaker(2))",
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

    session.clear();
    let result = process_to_string("let x: int = nonef();", &mut session);
    let error = Error::new(
        Context { start: 0, end: 20 },
        13,
        ErrorKind::FunctionNotFound,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("fn new() {}", &mut session);
    let error = Error::new(
        Context { start: 0, end: 11 },
        3,
        ErrorKind::IdentifierIsKeyword,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("let bool = false", &mut session);
    let error = Error::new(
        Context { start: 0, end: 16 },
        4,
        ErrorKind::IdentifierIsTypename,
    );
    let expected = error.print_to_string(session.parse_session.get_source_code(), &Vec::new());
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

Error: Function not found";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(1 +\n2 + f(12) - 3\n+ 9 * 5)", &mut session);
    let expected = "In line 2:

 1| print(1 +
 2| 2 + f(12) - 3
        ^
 3| + 9 * 5)

Error: Function not found";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(2 + f(12) - 3)", &mut session);
    let expected = "In line 1:

 1| print(2 + f(12) - 3)
              ^

Error: Function not found";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("print(2 + f(12) - 3)\n1 + 2", &mut session);
    let expected = "In line 1:

 1| print(2 + f(12) - 3)
              ^

Error: Function not found";
    assert_eq!(result, expected);

    session.clear();
    let result = process_to_string("\nprint(2 + f(12) - 3)", &mut session);
    let expected = "In line 2:

 2| print(2 + f(12) - 3)
              ^

Error: Function not found";
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
