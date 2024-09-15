use crate::builtin::{TYPEID_DYN, TYPEID_NONE};
use crate::error::{Context, Error, ErrorKind};
use crate::expression::Expression;
use crate::instruction::{self, Instruction, ReturnValue};
use crate::session::{BacktraceItem, ExecSession, FnQueryOptions, ParseSession};
use crate::variable::{AnnotatedIdentifier, Value, Variable};
use std::collections::HashMap;

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
    pub fn is_builtin(&self) -> bool {
        match self {
            Function::UserFunction(_) => false,
            Function::BuiltinFunction(_) => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuiltinFunction {
    arguments: Vec<AnnotatedIdentifier>,
    body: fn(&mut ExecSession, &ParseSession, Context, usize) -> Result<Value, Error>,
}

impl BuiltinFunction {
    #[inline]
    pub fn new(
        arguments: Vec<AnnotatedIdentifier>,
        body: fn(&mut ExecSession, &ParseSession, Context, usize) -> Result<Value, Error>,
    ) -> Self {
        BuiltinFunction { arguments, body }
    }

    #[inline]
    pub fn arguments(&self) -> &Vec<AnnotatedIdentifier> {
        &self.arguments
    }

    pub fn exec(
        &self,
        exec_session: &mut ExecSession,
        parse_session: &ParseSession,
        context: Context,
        pos: usize,
    ) -> instruction::Result {
        match (self.body)(exec_session, parse_session, context, pos) {
            Ok(v) => Ok(ReturnValue::Value(v)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserFunction {
    arguments: Vec<AnnotatedIdentifier>,
    return_typeid: Option<(/*typeid*/ usize, /*global token pos*/ usize)>,
    body: Vec<Instruction>,

    context: Context,
}

impl UserFunction {
    #[inline]
    pub fn new(
        arguments: Vec<AnnotatedIdentifier>,
        return_typeid: Option<(/*typeid*/ usize, /*global token pos*/ usize)>,
        body: Vec<Instruction>,
        context: Context,
    ) -> Self {
        UserFunction {
            arguments,
            return_typeid,
            body,
            context,
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
        parse_session: &ParseSession,
    ) -> instruction::Result {
        for item in &self.body {
            match item.exec(exec_session, parse_session) {
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
                                        parse_session.get_typename(value.typeid()),
                                        parse_session.get_typename(expected_typeid),
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
            if expected_typeid == TYPEID_NONE {
                return Ok(ReturnValue::Value(Value::None));
            } else {
                return Err(Error::new(
                    self.context,
                    pos - self.context.start,
                    ErrorKind::InvalidReturnType(
                        parse_session.get_typename(TYPEID_NONE),
                        parse_session.get_typename(expected_typeid),
                    ),
                ));
            }
        } else {
            return Ok(ReturnValue::Value(Value::None));
        }
    }
}

#[derive(Debug, Clone)]
pub struct Return {
    expr: Option<Expression>,

    context: Context,
}

impl Return {
    #[inline]
    pub fn new(expr: Option<Expression>, context: Context) -> Return {
        Return { expr, context }
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
        parse_session: &ParseSession,
    ) -> instruction::Result {
        if let Some(expr) = &self.expr {
            match expr.exec(exec_session, parse_session) {
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

#[derive(Debug, Clone)]
pub struct FunctionCall {
    associated_type: Option<(
        /*typeid*/ usize,
        /*scope resulolution operator position*/ usize,
    )>,
    name: String,
    arguments: Vec<Expression>,
    private_access_typeid: Option<usize>,

    context: Context,
    name_pos: usize,
    line_col: (usize, usize),
}

impl FunctionCall {
    #[inline]
    pub fn new(
        associated_type: Option<(usize, usize)>,
        name: String,
        arguments: Vec<Expression>,
        private_access_typeid: Option<usize>,

        context: Context,
        name_pos: usize,
        line_col: (usize, usize),
    ) -> FunctionCall {
        FunctionCall {
            associated_type,
            name,
            arguments,
            private_access_typeid,
            context,
            name_pos,
            line_col,
        }
    }

    #[inline]
    pub fn associated_typeid(&self) -> Option<usize> {
        if let Some((id, _)) = self.associated_type {
            Some(id)
        } else {
            None
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn arguments(&self) -> &Vec<Expression> {
        &self.arguments
    }

    #[inline]
    pub fn context(&self) -> Context {
        self.context
    }

    #[inline]
    pub fn scope_res_pos(&self) -> Option<usize> {
        if let Some((_, pos)) = self.associated_type {
            Some(pos)
        } else {
            None
        }
    }

    #[inline]
    pub fn name_pos(&self) -> usize {
        self.name_pos
    }

    #[inline]
    pub fn line_col(&self) -> (usize, usize) {
        self.line_col
    }

    pub fn call(
        &self,
        parse_session: &ParseSession,
        exec_session: &mut ExecSession,
        caller_object: Option<Value>,
        context: Context,
        pos: usize,
    ) -> instruction::Result {
        let fn_query_options: Option<FnQueryOptions> = match caller_object {
            Some(ref c) => {
                let typeid = c.typeid();
                if let Some(id) = self.private_access_typeid {
                    if typeid == id {
                        Some(FnQueryOptions::new(typeid, true, true))
                    } else {
                        Some(FnQueryOptions::new(typeid, true, false))
                    }
                } else {
                    Some(FnQueryOptions::new(typeid, true, false))
                }
            }
            None => match self.associated_typeid() {
                Some(typeid) => {
                    if let Some(id) = self.private_access_typeid {
                        if typeid == id {
                            Some(FnQueryOptions::new(typeid, false, true))
                        } else {
                            Some(FnQueryOptions::new(typeid, false, false))
                        }
                    } else {
                        Some(FnQueryOptions::new(typeid, false, false))
                    }
                }
                None => None,
            },
        };

        let function = match parse_session.get_function(self.name(), fn_query_options) {
            Ok(f) => f,
            Err(errorkind) => {
                return Err(Error::new(context, pos + self.name_pos, errorkind));
            }
        };

        if function.arguments().len() != self.arguments().len() {
            return Err(Error::new(
                context,
                pos + self.name_pos,
                ErrorKind::InvalidNumberOfArguments,
            ));
        }

        let mut fn_scope: HashMap<String, Variable> = HashMap::new();
        for i in 0..function.arguments().len() {
            let expr = &self.arguments()[i];
            let value = expr
                .exec(exec_session, parse_session)?
                .expect("Expressions should always return a value on success");

            let result;
            let expected_type_id = function.arguments()[i].typeid();

            if expected_type_id != TYPEID_DYN {
                if value.typeid() != expected_type_id {
                    let pos = expr.context().start - self.context().start;
                    let typename = parse_session.get_typename(value.typeid());
                    let expected_type_name = parse_session.get_typename(expected_type_id);
                    return Err(Error::new(
                        context,
                        pos,
                        ErrorKind::InvalidArgumentType(typename, expected_type_name),
                    ));
                }
                result = Variable::new(value, false);
            } else {
                result = Variable::new(value, true);
            }

            fn_scope.insert(function.arguments()[i].name().to_string(), result);
        }

        if let Some(c) = caller_object {
            fn_scope.insert("self".to_string(), Variable::new(c, true));
        } else if let Some(id) = self.associated_typeid() {
            fn_scope.insert(
                "#".to_string(),
                Variable::new(Value::Int(id as isize), false),
            );
        }

        exec_session.increment_call_count()?;
        exec_session.add_scope(fn_scope);

        let result = match function {
            Function::UserFunction(f) => f.exec(exec_session, parse_session),
            Function::BuiltinFunction(f) => f.exec(exec_session, parse_session, context, pos),
        };

        if let Err(_) = result {
            if !function.is_builtin() || self.name() == "new" {
                let backtrace_item = BacktraceItem::new(self.name(), self.line_col());
                exec_session.add_backtrace(backtrace_item)
            }
        }

        exec_session.pop_scope();
        exec_session.decrement_call_count();
        return result;
    }
}
