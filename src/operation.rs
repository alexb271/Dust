use crate::error::{Context, Error, ErrorKind};
use crate::session::{ExecSession, ParsedSession};
use crate::token::Operator;
use crate::variable::Value;

pub enum Operand<'a> {
    Value(Value),
    Identifier(&'a str, usize),
}

impl<'a> Operand<'a> {
    #[inline]
    pub fn get_value(self, session: &ExecSession, context: Context) -> Result<Value, Error> {
        match self {
            Operand::Value(val) => Ok(val),
            Operand::Identifier(id, pos) => match session.get_variable(id) {
                Some(var) => Ok(var.get_value().clone()),
                None => {
                    return Err(Error::new(context, pos, ErrorKind::IdentifierNotFound));
                }
            },
        }
    }

    #[inline]
    pub fn get_value_ref(
        &'a self,
        session: &'a ExecSession,
        context: Context,
    ) -> Result<&'a Value, Error> {
        match self {
            Operand::Value(val) => Ok(val),
            Operand::Identifier(id, pos) => match session.get_variable(id) {
                Some(var) => Ok(var.get_value()),
                None => {
                    return Err(Error::new(context, *pos, ErrorKind::IdentifierNotFound));
                }
            },
        }
    }
}

pub struct OperationArgs<'a> {
    pub stack: &'a mut Vec<Operand<'a>>,
    pub exec_session: &'a mut ExecSession,
    pub parsed_session: &'a ParsedSession,
    pub context: Context,
}

#[inline]
pub fn add(args: &mut OperationArgs, operator: Operator, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap();
    let rhs = rhs.get_value_ref(args.exec_session, args.context)?;
    let lhs = args.stack.pop().unwrap();
    let lhs = lhs.get_value_ref(args.exec_session, args.context)?;

    let result = match lhs {
        Value::Number(l) => match rhs {
            Value::Number(r) => Ok(Value::Number(*l + *r)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Str(l) => match rhs {
            Value::Str(r) => {
                let mut result = Box::new(String::with_capacity(l.len() + r.len()));
                result.push_str(l);
                result.push_str(r);
                Ok(Value::Str(result))
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            operator,
            args.parsed_session.get_typename(lhs.typeid()),
            args.parsed_session.get_typename(rhs.typeid()),
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
pub fn subtract(args: &mut OperationArgs, operator: Operator, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap();
    let rhs = rhs.get_value_ref(args.exec_session, args.context)?;
    let lhs = args.stack.pop().unwrap();
    let lhs = lhs.get_value_ref(args.exec_session, args.context)?;

    let result = match lhs {
        Value::Number(l) => match rhs {
            Value::Number(r) => Ok(Value::Number(*l - *r)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            operator,
            args.parsed_session.get_typename(lhs.typeid()),
            args.parsed_session.get_typename(rhs.typeid()),
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
pub fn multiply(args: &mut OperationArgs, operator: Operator, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap();
    let rhs = rhs.get_value_ref(args.exec_session, args.context)?;
    let lhs = args.stack.pop().unwrap();
    let lhs = lhs.get_value_ref(args.exec_session, args.context)?;

    let result = match lhs {
        Value::Number(l) => match rhs {
            Value::Number(r) => Ok(Value::Number(*l * *r)),
            Value::Str(r) => Ok(Value::Str(Box::new(r.repeat(l.round() as usize)))),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Str(l) => match rhs {
            Value::Number(r) => Ok(Value::Str(Box::new(l.repeat(r.round() as usize)))),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            operator,
            args.parsed_session.get_typename(lhs.typeid()),
            args.parsed_session.get_typename(rhs.typeid()),
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
pub fn divide(args: &mut OperationArgs, operator: Operator, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap();
    let rhs = rhs.get_value_ref(args.exec_session, args.context)?;
    let lhs = args.stack.pop().unwrap();
    let lhs = lhs.get_value_ref(args.exec_session, args.context)?;

    let result = match lhs {
        Value::Number(l) => match rhs {
            Value::Number(r) => {
                if *r != 0.0_f64 {
                    Ok(Value::Number(*l / *r))
                } else {
                    Err(ErrorKind::ZeroDivision)
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            operator,
            args.parsed_session.get_typename(lhs.typeid()),
            args.parsed_session.get_typename(rhs.typeid()),
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
pub fn modulo(args: &mut OperationArgs, operator: Operator, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap();
    let rhs = rhs.get_value_ref(args.exec_session, args.context)?;
    let lhs = args.stack.pop().unwrap();
    let lhs = lhs.get_value_ref(args.exec_session, args.context)?;

    let result = match lhs {
        Value::Number(l) => match rhs {
            Value::Number(r) => Ok(Value::Number(*l % *r)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            operator,
            args.parsed_session.get_typename(lhs.typeid()),
            args.parsed_session.get_typename(rhs.typeid()),
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
pub fn power(args: &mut OperationArgs, operator: Operator, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap();
    let rhs = rhs.get_value_ref(args.exec_session, args.context)?;
    let lhs = args.stack.pop().unwrap();
    let lhs = lhs.get_value_ref(args.exec_session, args.context)?;

    let result = match lhs {
        Value::Number(l) => match rhs {
            Value::Number(r) => Ok(Value::Number(l.powf(*r))),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            operator,
            args.parsed_session.get_typename(lhs.typeid()),
            args.parsed_session.get_typename(rhs.typeid()),
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
pub fn and(args: &mut OperationArgs, operator: Operator, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap();
    let rhs = rhs.get_value_ref(args.exec_session, args.context)?;
    let lhs = args.stack.pop().unwrap();
    let lhs = lhs.get_value_ref(args.exec_session, args.context)?;

    let result = match lhs {
        Value::Bool(l) => match rhs {
            Value::Bool(r) => {
                if *l != false && *r != false {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            operator,
            args.parsed_session.get_typename(lhs.typeid()),
            args.parsed_session.get_typename(rhs.typeid()),
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
pub fn or(args: &mut OperationArgs, operator: Operator, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap();
    let rhs = rhs.get_value_ref(args.exec_session, args.context)?;
    let lhs = args.stack.pop().unwrap();
    let lhs = lhs.get_value_ref(args.exec_session, args.context)?;

    let result = match lhs {
        Value::Bool(l) => match rhs {
            Value::Bool(r) => {
                if *l != false || *r != false {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            operator,
            args.parsed_session.get_typename(lhs.typeid()),
            args.parsed_session.get_typename(rhs.typeid()),
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
pub fn less_than(args: &mut OperationArgs, operator: Operator, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap();
    let rhs = rhs.get_value_ref(args.exec_session, args.context)?;
    let lhs = args.stack.pop().unwrap();
    let lhs = lhs.get_value_ref(args.exec_session, args.context)?;

    let result = match lhs {
        Value::Number(l) => match rhs {
            Value::Number(r) => {
                if *l < *r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            operator,
            args.parsed_session.get_typename(lhs.typeid()),
            args.parsed_session.get_typename(rhs.typeid()),
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
pub fn greater_than(args: &mut OperationArgs, operator: Operator, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap();
    let rhs = rhs.get_value_ref(args.exec_session, args.context)?;
    let lhs = args.stack.pop().unwrap();
    let lhs = lhs.get_value_ref(args.exec_session, args.context)?;

    let result = match lhs {
        Value::Number(l) => match rhs {
            Value::Number(r) => {
                if *l > *r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        _ => Err(ErrorKind::InvalidOperationForTypes(
            operator,
            args.parsed_session.get_typename(lhs.typeid()),
            args.parsed_session.get_typename(rhs.typeid()),
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
pub fn equal(args: &mut OperationArgs, operator: Operator, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap();
    let rhs = rhs.get_value_ref(args.exec_session, args.context)?;
    let lhs = args.stack.pop().unwrap();
    let lhs = lhs.get_value_ref(args.exec_session, args.context)?;

    let result = match lhs {
        Value::Number(l) => match rhs {
            Value::Number(r) => {
                if *l == *r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Str(ref l) => match rhs {
            Value::Str(ref r) => {
                if *l == *r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Bool(l) => match rhs {
            Value::Bool(r) => {
                if *l == *r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        Value::None => match rhs {
            Value::None => Ok(Value::Bool(true)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
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
pub fn not_equal(args: &mut OperationArgs, operator: Operator, pos: usize) -> Result<(), Error> {
    let rhs = args.stack.pop().unwrap();
    let rhs = rhs.get_value_ref(args.exec_session, args.context)?;
    let lhs = args.stack.pop().unwrap();
    let lhs = lhs.get_value_ref(args.exec_session, args.context)?;

    let result = match lhs {
        Value::Number(l) => match rhs {
            Value::Number(r) => {
                if *l != *r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Str(ref l) => match rhs {
            Value::Str(ref r) => {
                if *l != *r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        Value::Bool(l) => match rhs {
            Value::Bool(r) => {
                if *l != *r {
                    Ok(Value::Bool(true))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
            )),
        },
        Value::None => match rhs {
            Value::None => Ok(Value::Bool(false)),
            _ => Err(ErrorKind::InvalidOperationForTypes(
                operator,
                args.parsed_session.get_typename(lhs.typeid()),
                args.parsed_session.get_typename(rhs.typeid()),
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
pub fn negate(args: &mut OperationArgs, operator: Operator, pos: usize) -> Result<(), Error> {
    let operand = args.stack.pop().unwrap();
    let operand = operand.get_value_ref(args.exec_session, args.context)?;

    let result = match operand {
        Value::Number(n) => Ok(Value::Number(n * (-1.0_f64))),
        _ => Err(ErrorKind::InvalidOperationForType(
            operator,
            args.parsed_session.get_typename(operand.typeid()),
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
pub fn not(args: &mut OperationArgs, operator: Operator, pos: usize) -> Result<(), Error> {
    let operand = args.stack.pop().unwrap();
    let operand = operand.get_value_ref(args.exec_session, args.context)?;

    let result = match operand {
        Value::Bool(b) => {
            if *b == false {
                Ok(Value::Bool(true))
            } else {
                Ok(Value::Bool(false))
            }
        }
        _ => Err(ErrorKind::InvalidOperationForType(
            operator,
            args.parsed_session.get_typename(operand.typeid()),
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
pub fn gettype(args: &mut OperationArgs) -> Result<(), Error> {
    let operand = args.stack.pop().unwrap();
    let operand = operand.get_value_ref(args.exec_session, args.context)?;

    args.stack.push(Operand::Value(Value::Str(Box::new(
        args.parsed_session.get_typename(operand.typeid()),
    ))));
    Ok(())
}
