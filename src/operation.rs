use crate::error::{Context, Error, ErrorKind};
use crate::function::FunctionCall;
use crate::instruction::ReturnValue;
use crate::session::{ExecSession, ParseSession};
use crate::token::Operator;
use crate::variable::Value;

#[derive(Debug)]
pub enum Operand<'a> {
    Value(Value),
    Identifier(&'a str, usize),
    FunctionCall(&'a FunctionCall, usize),
}

impl<'a> Operand<'a> {
    #[inline]
    pub fn get_value(self, args: &mut OperationArgs) -> Result<Value, Error> {
        match self {
            Operand::Value(val) => Ok(val),
            Operand::Identifier(id, pos) => match args.exec_session.get_variable(id) {
                Some(var) => Ok(var.get_value().clone()),
                None => {
                    return Err(Error::new(args.context, pos, ErrorKind::IdentifierNotFound));
                }
            },
            Operand::FunctionCall(f, pos) => {
                match f.call(
                    args.parse_session,
                    args.exec_session,
                    None,
                    args.context,
                    pos,
                ) {
                    Ok(output) => match output {
                        ReturnValue::Value(value) => Ok(value),
                        _ => unreachable!("Function calls should not return Break or Return types"),
                    },
                    Err(e) => return Err(e),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct OperationArgs<'a> {
    pub stack: &'a mut Vec<Operand<'a>>,
    pub exec_session: &'a mut ExecSession,
    pub parse_session: &'a ParseSession,
    pub private_access_typeid: Option<usize>,
    pub context: Context,
}

#[inline]
pub fn add(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap().get_value(args)?;
    let lhs = args.stack.pop().unwrap().get_value(args)?;

    let result = match lhs {
        Value::Int(l) => match rhs {
            Value::Int(r) => Ok(Value::Int(l + r)),
            Value::Float(r) => Ok(Value::Float(l as f64 + r)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Add,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Float(l) => match rhs {
            Value::Float(r) => Ok(Value::Float(l + r)),
            Value::Int(r) => Ok(Value::Float(l + r as f64)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Add,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Str(ref l) => match rhs {
            Value::Str(r) => {
                let mut result = String::with_capacity(l.borrow().len() + r.borrow().len());
                result.push_str(l.borrow().as_str());
                result.push_str(r.borrow().as_str());
                Ok(Value::new_string(result))
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Add,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            Operator::Add,
            args.parse_session.get_typename(lhs.typeid()),
            args.parse_session.get_typename(rhs.typeid()),
        )),
    };

    match result {
        Ok(val) => {
            args.stack.push(Operand::Value(val));
            Ok(())
        }
        Err(e) => Err(Error::new(args.context, pos, e)),
    }
}

#[inline]
pub fn subtract(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap().get_value(args)?;
    let lhs = args.stack.pop().unwrap().get_value(args)?;

    let result = match lhs {
        Value::Int(l) => match rhs {
            Value::Int(r) => Ok(Value::Int(l - r)),
            Value::Float(r) => Ok(Value::Float(l as f64 - r)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Add,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Float(l) => match rhs {
            Value::Float(r) => Ok(Value::Float(l - r)),
            Value::Int(r) => Ok(Value::Float(l - r as f64)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Add,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            Operator::Sub,
            args.parse_session.get_typename(lhs.typeid()),
            args.parse_session.get_typename(rhs.typeid()),
        )),
    };

    match result {
        Ok(val) => {
            args.stack.push(Operand::Value(val));
            Ok(())
        }
        Err(e) => Err(Error::new(args.context, pos, e)),
    }
}

#[inline]
pub fn multiply(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap().get_value(args)?;
    let lhs = args.stack.pop().unwrap().get_value(args)?;

    let result = match lhs {
        Value::Int(l) => match rhs {
            Value::Int(r) => Ok(Value::Int(l * r)),
            Value::Float(r) => Ok(Value::Float(l as f64 * r)),
            Value::Str(ref r) => Ok(Value::new_string(r.borrow().repeat(l.abs() as usize))),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Mult,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Float(l) => match rhs {
            Value::Float(r) => Ok(Value::Float(l * r)),
            Value::Int(r) => Ok(Value::Float(l * r as f64)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Mult,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Str(ref l) => match rhs {
            Value::Int(r) => Ok(Value::new_string(l.borrow().repeat(r.abs() as usize))),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Mult,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            Operator::Mult,
            args.parse_session.get_typename(lhs.typeid()),
            args.parse_session.get_typename(rhs.typeid()),
        )),
    };

    match result {
        Ok(val) => {
            args.stack.push(Operand::Value(val));
            Ok(())
        }
        Err(e) => Err(Error::new(args.context, pos, e)),
    }
}

#[inline]
pub fn divide(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap().get_value(args)?;
    let lhs = args.stack.pop().unwrap().get_value(args)?;

    let result = match lhs {
        Value::Int(l) => match rhs {
            Value::Int(r) => {
                if r != 0 {
                    Ok(Value::Float(l as f64 / r as f64))
                } else {
                    Err(ErrorKind::ZeroDivision)
                }
            }
            Value::Float(r) => {
                if r != 0.0_f64 {
                    Ok(Value::Float(l as f64 / r))
                } else {
                    Err(ErrorKind::ZeroDivision)
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Div,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Float(l) => match rhs {
            Value::Float(r) => {
                if r != 0.0_f64 {
                    Ok(Value::Float(l / r))
                } else {
                    Err(ErrorKind::ZeroDivision)
                }
            }
            Value::Int(r) => {
                if r != 0 {
                    Ok(Value::Float(l / r as f64))
                } else {
                    Err(ErrorKind::ZeroDivision)
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Div,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            Operator::Div,
            args.parse_session.get_typename(lhs.typeid()),
            args.parse_session.get_typename(rhs.typeid()),
        )),
    };

    match result {
        Ok(val) => {
            args.stack.push(Operand::Value(val));
            Ok(())
        }
        Err(e) => Err(Error::new(args.context, pos, e)),
    }
}

#[inline]
pub fn modulo(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap().get_value(args)?;
    let lhs = args.stack.pop().unwrap().get_value(args)?;

    let result = match lhs {
        Value::Int(l) => match rhs {
            Value::Int(r) => Ok(Value::Int(l % r)),
            Value::Float(r) => Ok(Value::Float(l as f64 % r)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Mod,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Float(l) => match rhs {
            Value::Float(r) => Ok(Value::Float(l % r)),
            Value::Int(r) => Ok(Value::Float(l % r as f64)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Mod,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            Operator::Mod,
            args.parse_session.get_typename(lhs.typeid()),
            args.parse_session.get_typename(rhs.typeid()),
        )),
    };

    match result {
        Ok(val) => {
            args.stack.push(Operand::Value(val));
            Ok(())
        }
        Err(e) => Err(Error::new(args.context, pos, e)),
    }
}

#[inline]
pub fn power(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap().get_value(args)?;
    let lhs = args.stack.pop().unwrap().get_value(args)?;

    let result = match lhs {
        Value::Int(l) => match rhs {
            Value::Int(r) => Ok(Value::Float((l as f64).powf(r as f64))),
            Value::Float(r) => Ok(Value::Float((l as f64).powf(r))),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Pow,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Float(l) => match rhs {
            Value::Float(r) => Ok(Value::Float(l.powf(r))),
            Value::Int(r) => Ok(Value::Float(l.powf(r as f64))),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Pow,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            Operator::Pow,
            args.parse_session.get_typename(lhs.typeid()),
            args.parse_session.get_typename(rhs.typeid()),
        )),
    };

    match result {
        Ok(val) => {
            args.stack.push(Operand::Value(val));
            Ok(())
        }
        Err(e) => Err(Error::new(args.context, pos, e)),
    }
}

#[inline]
pub fn and(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap().get_value(args)?;
    let lhs = args.stack.pop().unwrap().get_value(args)?;

    let result = match lhs {
        Value::Bool(l) => match rhs {
            Value::Bool(r) => {
                if l != false && r != false {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::And,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            Operator::And,
            args.parse_session.get_typename(lhs.typeid()),
            args.parse_session.get_typename(rhs.typeid()),
        )),
    };

    match result {
        Ok(val) => {
            args.stack.push(Operand::Value(val));
            Ok(())
        }
        Err(e) => Err(Error::new(args.context, pos, e)),
    }
}

#[inline]
pub fn or(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap().get_value(args)?;
    let lhs = args.stack.pop().unwrap().get_value(args)?;

    let result = match lhs {
        Value::Bool(l) => match rhs {
            Value::Bool(r) => {
                if l != false || r != false {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Or,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            Operator::Or,
            args.parse_session.get_typename(lhs.typeid()),
            args.parse_session.get_typename(rhs.typeid()),
        )),
    };

    match result {
        Ok(val) => {
            args.stack.push(Operand::Value(val));
            Ok(())
        }
        Err(e) => Err(Error::new(args.context, pos, e)),
    }
}

#[inline]
pub fn less_than(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap().get_value(args)?;
    let lhs = args.stack.pop().unwrap().get_value(args)?;

    let result = match lhs {
        Value::Int(l) => match rhs {
            Value::Int(r) => {
                if l < r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::Float(r) => {
                if (l as f64) < r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::LessThan,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Float(l) => match rhs {
            Value::Float(r) => {
                if l < r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::Int(r) => {
                if l < (r as f64) {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::LessThan,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Str(ref l) => match rhs {
            Value::Str(ref r) => Ok(Value::Bool(l.borrow().as_str() < r.borrow().as_str())),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::LessThan,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            Operator::LessThan,
            args.parse_session.get_typename(lhs.typeid()),
            args.parse_session.get_typename(rhs.typeid()),
        )),
    };

    match result {
        Ok(val) => {
            args.stack.push(Operand::Value(val));
            Ok(())
        }
        Err(e) => Err(Error::new(args.context, pos, e)),
    }
}

#[inline]
pub fn greater_than(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap().get_value(args)?;
    let lhs = args.stack.pop().unwrap().get_value(args)?;

    let result = match lhs {
        Value::Int(l) => match rhs {
            Value::Int(r) => {
                if l > r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::Float(r) => {
                if (l as f64) > r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::LessThan,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Float(l) => match rhs {
            Value::Float(r) => {
                if l > r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::Int(r) => {
                if l > (r as f64) {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::LessThan,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Str(ref l) => match rhs {
            Value::Str(ref r) => Ok(Value::Bool(l.borrow().as_str() > r.borrow().as_str())),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::LessThan,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            Operator::GreaterThan,
            args.parse_session.get_typename(lhs.typeid()),
            args.parse_session.get_typename(rhs.typeid()),
        )),
    };

    match result {
        Ok(val) => {
            args.stack.push(Operand::Value(val));
            Ok(())
        }
        Err(e) => Err(Error::new(args.context, pos, e)),
    }
}

#[inline]
pub fn equal(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap().get_value(args)?;
    let lhs = args.stack.pop().unwrap().get_value(args)?;

    let result = match lhs {
        Value::Int(l) => match rhs {
            Value::Int(r) => {
                if l == r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::Float(r) => {
                if (l as f64) == r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::None => Ok(Value::Bool(false)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Equal,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Float(l) => match rhs {
            Value::Float(r) => {
                if l == r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::Int(r) => {
                if l == (r as f64) {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::None => Ok(Value::Bool(false)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Equal,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Str(ref l) => match rhs {
            Value::Str(ref r) => {
                if *l.borrow() == *r.borrow() {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::None => Ok(Value::Bool(false)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Equal,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Bool(l) => match rhs {
            Value::Bool(r) => {
                if l == r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::None => Ok(Value::Bool(false)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Equal,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::None => match rhs {
            Value::None => Ok(Value::Bool(true)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Equal,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Class(ref l) => match rhs {
            Value::Class(ref r) => Ok(Value::Bool(std::rc::Rc::ptr_eq(l, r))),
            Value::None => Ok(Value::Bool(false)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Equal,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Vector(ref l) => match rhs {
            Value::Vector(ref r) => Ok(Value::Bool(std::rc::Rc::ptr_eq(l, r))),
            Value::None => Ok(Value::Bool(false)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::Equal,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
    };

    match result {
        Ok(val) => {
            args.stack.push(Operand::Value(val));
            Ok(())
        }
        Err(e) => Err(Error::new(args.context, pos, e)),
    }
}

#[inline]
pub fn not_equal(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap().get_value(args)?;
    let lhs = args.stack.pop().unwrap().get_value(args)?;

    let result = match lhs {
        Value::Int(l) => match rhs {
            Value::Int(r) => {
                if l != r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::Float(r) => {
                if (l as f64) != r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::None => Ok(Value::Bool(true)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::NotEqual,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Float(l) => match rhs {
            Value::Float(r) => {
                if l != r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::Int(r) => {
                if l != (r as f64) {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::None => Ok(Value::Bool(true)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::NotEqual,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Str(ref l) => match rhs {
            Value::Str(ref r) => {
                if *l.borrow() != *r.borrow() {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::None => Ok(Value::Bool(true)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::NotEqual,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Bool(l) => match rhs {
            Value::Bool(r) => {
                if l != r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            Value::None => Ok(Value::Bool(true)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::NotEqual,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::None => match rhs {
            Value::None => Ok(Value::Bool(false)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::NotEqual,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Class(ref l) => match rhs {
            Value::Class(ref r) => Ok(Value::Bool(!std::rc::Rc::ptr_eq(l, r))),
            Value::None => Ok(Value::Bool(true)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::NotEqual,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Vector(ref l) => match rhs {
            Value::Vector(ref r) => Ok(Value::Bool(!std::rc::Rc::ptr_eq(l, r))),
            Value::None => Ok(Value::Bool(true)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                Operator::NotEqual,
                args.parse_session.get_typename(lhs.typeid()),
                args.parse_session.get_typename(rhs.typeid()),
            )),
        },
    };

    match result {
        Ok(val) => {
            args.stack.push(Operand::Value(val));
            Ok(())
        }
        Err(e) => Err(Error::new(args.context, pos, e)),
    }
}

#[inline]
pub fn negate(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let operand = args.stack.pop().unwrap().get_value(args)?;

    let result = match operand {
        Value::Int(n) => Ok(Value::Int(n * (-1))),
        Value::Float(n) => Ok(Value::Float(n * (-1.0_f64))),
        _ => Err(ErrorKind::InvalidOperationForType(
            Operator::Neg,
            args.parse_session.get_typename(operand.typeid()),
        )),
    };

    match result {
        Ok(val) => {
            args.stack.push(Operand::Value(val));
            Ok(())
        }
        Err(e) => Err(Error::new(args.context, pos, e)),
    }
}

#[inline]
pub fn not(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let operand = args.stack.pop().unwrap().get_value(args)?;

    let result = match operand {
        Value::Bool(b) => {
            if b == false {
                Ok(Value::Bool(true))
            } else {
                Ok(Value::Bool(false))
            }
        }
        _ => Err(ErrorKind::InvalidOperationForType(
            Operator::Not,
            args.parse_session.get_typename(operand.typeid()),
        )),
    };

    match result {
        Ok(val) => {
            args.stack.push(Operand::Value(val));
            Ok(())
        }
        Err(e) => Err(Error::new(args.context, pos, e)),
    }
}

#[inline]
pub fn member_access(args: &mut OperationArgs, pos: usize) -> Result<(), Error> {
    let rhs_operand = args.stack.pop().unwrap();
    let lhs_operand = args.stack.pop().unwrap();

    let lhs = lhs_operand.get_value(args)?;
    let private_access = if let Some(id) = args.private_access_typeid {
        if id == lhs.typeid() {
            true
        } else {
            false
        }
    } else {
        false
    };

    let value = match rhs_operand {
        Operand::Identifier(name, idpos) => match lhs {
            Value::Class(c) => {
                match c
                    .borrow()
                    .get_property(name, private_access, args.parse_session)
                {
                    Ok(var) => var.get_value_clone(),
                    Err(errorkind) => {
                        return Err(Error::new(args.context, idpos, errorkind));
                    }
                }
            }
            _ => {
                return Err(Error::new(
                    args.context,
                    pos,
                    ErrorKind::InvalidMemberAccess,
                ));
            }
        },
        Operand::FunctionCall(f, _pos) => {
            if let Some(_) = f.associated_typeid() {
                return Err(Error::new(
                    f.context(),
                    f.scope_res_pos().unwrap(),
                    ErrorKind::InvalidScopeAccess,
                ));
            } else {
                match f.call(
                    args.parse_session,
                    args.exec_session,
                    Some(lhs.clone()),
                    args.context,
                    f.context().start - args.context.start,
                ) {
                    Ok(output) => match output {
                        ReturnValue::Value(value) => value,
                        _ => unreachable!("Function calls should not return Break or Return types"),
                    },
                    Err(e) => return Err(e),
                }
            }
        }
        Operand::Value(_) => {
            return Err(Error::new(
                args.context,
                pos,
                ErrorKind::InvalidMemberAccess,
            ));
        }
    };

    args.stack.push(Operand::Value(value));
    Ok(())
}

#[inline]
pub fn gettype(args: &mut OperationArgs) -> Result<(), Error> {
    let operand = args.stack.pop().unwrap().get_value(args)?;

    args.stack.push(Operand::Value(Value::new_string(
        args.parse_session.get_typename(operand.typeid()),
    )));

    Ok(())
}
