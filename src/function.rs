use crate::error::{Context, Error, ErrorKind};
use crate::expression::Expression;
use crate::instruction::{self, Instruction, ReturnValue};
use crate::session::{ExecSession, ParsedSession};
use crate::variable::{AnnotatedIdentifier, Value};

#[derive(Debug, Clone)]
pub enum Function {
    UserFunction(UserFunction),
    BuiltinFunction(BuiltinFunction),
}

impl Function {
    #[inline]
    pub fn arguments(&self) -> &Vec<AnnotatedIdentifier> {
        match self {
            Function::UserFunction(f) => f.arguments(),
            Function::BuiltinFunction(f) => f.arguments(),
        }
    }

    #[inline]
    pub fn exec(
        &self,
        exec_session: &mut ExecSession,
        parsed_session: &ParsedSession,
    ) -> instruction::Result {
        match self {
            Function::UserFunction(f) => f.exec(exec_session, parsed_session),
            Function::BuiltinFunction(f) => f.exec(exec_session),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuiltinFunction {
    arguments: Vec<AnnotatedIdentifier>,
    body: fn(&mut ExecSession) -> Value,
}

impl BuiltinFunction {
    #[inline]
    pub fn new(arguments: Vec<AnnotatedIdentifier>, body: fn(&mut ExecSession) -> Value) -> Self {
        BuiltinFunction { arguments, body }
    }

    #[inline]
    pub fn arguments(&self) -> &Vec<AnnotatedIdentifier> {
        &self.arguments
    }

    pub fn exec(&self, exec_session: &mut ExecSession) -> instruction::Result {
        Ok(ReturnValue::Value((self.body)(exec_session)))
    }
}

#[derive(Debug, Clone)]
pub struct UserFunction {
    context: Context,
    arguments: Vec<AnnotatedIdentifier>,
    return_typeid: Option<(/*typeid*/ usize, /*global token pos*/ usize)>,
    body: Vec<Instruction>,
}

impl UserFunction {
    #[inline]
    pub fn new(
        context: Context,
        arguments: Vec<AnnotatedIdentifier>,
        return_typeid: Option<(/*typeid*/ usize, /*global token pos*/ usize)>,
        body: Vec<Instruction>,
    ) -> Self {
        UserFunction {
            context,
            arguments,
            return_typeid,
            body,
        }
    }

    #[inline]
    pub fn context(&self) -> Context {
        self.context
    }

    #[inline]
    pub fn arguments(&self) -> &Vec<AnnotatedIdentifier> {
        &self.arguments
    }

    pub fn exec(
        &self,
        exec_session: &mut ExecSession,
        parsed_session: &ParsedSession,
    ) -> instruction::Result {
        for item in &self.body {
            match item.exec(exec_session, parsed_session) {
                Ok(return_value) => match return_value {
                    ReturnValue::Return((value, pos)) => {
                        if let Some((expected_typeid, _)) = self.return_typeid {
                            if expected_typeid == value.typeid() {
                                return Ok(ReturnValue::Value(value));
                            } else {
                                return Err(Error::new(
                                    item.context(),
                                    pos.saturating_sub(item.context().start),
                                    ErrorKind::InvalidReturnType(
                                        parsed_session.get_typename(value.typeid()),
                                        parsed_session.get_typename(expected_typeid),
                                    ),
                                ));
                            }
                        } else {
                            return Ok(ReturnValue::Value(value));
                        }
                    }
                    ReturnValue::Value(_) => (),
                    ReturnValue::Break => {
                        return Err(Error::new(item.context(), 0, ErrorKind::SyntaxError));
                    }
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }

        if let Some((expected_typeid, pos)) = self.return_typeid {
            if expected_typeid == crate::TYPEID_NONE {
                return Ok(ReturnValue::Value(Value::None));
            } else {
                return Err(Error::new(
                    self.context,
                    pos - self.context.start,
                    ErrorKind::InvalidReturnType(
                        parsed_session.get_typename(crate::TYPEID_NONE),
                        parsed_session.get_typename(expected_typeid),
                    ),
                ));
            }
        } else {
            return Ok(ReturnValue::Value(Value::None));
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    name: String,
    context: Context,
    line_col: (usize, usize),
    arguments: Vec<Expression>,
}

impl FunctionCall {
    #[inline]
    pub fn new(
        name: String,
        context: Context,
        line_col: (usize, usize),
        arguments: Vec<Expression>,
    ) -> FunctionCall {
        FunctionCall {
            name,
            context,
            line_col,
            arguments,
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn context(&self) -> Context {
        self.context
    }

    #[inline]
    pub fn get_line_col(&self) -> (usize, usize) {
        self.line_col
    }

    #[inline]
    pub fn arguments(&self) -> &Vec<Expression> {
        &self.arguments
    }
}

#[derive(Debug, Clone)]
pub struct Return {
    context: Context,
    expr: Option<Expression>,
}

impl Return {
    #[inline]
    pub fn new(context: Context, expr: Option<Expression>) -> Return {
        Return { context, expr }
    }

    #[inline]
    pub fn context(&self) -> Context {
        self.context
    }

    #[inline]
    fn expr_pos(&self) -> usize {
        match &self.expr {
            Some(expr) => expr.context().start,
            None => 0,
        }
    }

    pub fn exec(
        &self,
        exec_session: &mut ExecSession,
        parsed_session: &ParsedSession,
    ) -> instruction::Result {
        if let Some(expr) = &self.expr {
            match expr.exec(exec_session, parsed_session) {
                Ok(output) => {
                    let value = output.expect("");
                    Ok(ReturnValue::Return((value, self.expr_pos())))
                }
                Err(e) => Err(e),
            }
        } else {
            Ok(ReturnValue::Return((Value::None, self.expr_pos())))
        }
    }
}
