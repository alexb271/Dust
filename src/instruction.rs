use crate::branch::Branch;
use crate::error::{Context, Error, ErrorKind};
use crate::expression::Expression;
use crate::function::Return;
use crate::session::{ExecSession, ParsedSession};
use crate::variable::{OptionallyAnnotatedIdentifier, Value, Variable};
use crate::while_loop::WhileLoop;

#[derive(Debug, Clone)]
pub enum Instruction {
    Expression(Expression),
    Branch(Branch),
    WhileLoop(WhileLoop),
    Return(Return),
    Break(Break),
    VariableInit(VariableInit),
    VariableAssign(VariableAssign),
}

impl Instruction {
    #[inline]
    pub fn exec(&self, exec_session: &mut ExecSession, parsed_session: &ParsedSession) -> Result {
        match self {
            Instruction::Expression(e) => e.exec(exec_session, parsed_session),
            Instruction::Branch(b) => b.exec(exec_session, parsed_session),
            Instruction::WhileLoop(l) => l.exec(exec_session, parsed_session),
            Instruction::Return(r) => r.exec(exec_session, parsed_session),
            Instruction::Break(_) => Ok(ReturnValue::Break),
            Instruction::VariableInit(vi) => vi.exec(exec_session, parsed_session),
            Instruction::VariableAssign(va) => va.exec(exec_session, parsed_session),
        }
    }

    #[inline]
    pub fn context(&self) -> Context {
        match self {
            Instruction::Expression(e) => e.context(),
            Instruction::Branch(b) => b.context(),
            Instruction::WhileLoop(l) => l.context(),
            Instruction::Return(r) => r.context(),
            Instruction::Break(br) => br.context(),
            Instruction::VariableInit(vi) => vi.context(),
            Instruction::VariableAssign(va) => va.context(),
        }
    }
}

pub type Result = std::result::Result<ReturnValue, Error>;

#[derive(Debug, Clone)]
pub enum ReturnValue {
    Value(Value),
    Return(
        (
            Value,
            usize, // global position of the token that produced tha value
        ),
    ),
    Break,
}

impl ReturnValue {
    #[inline]
    pub fn expect(self, msg: &str) -> Value {
        match self {
            ReturnValue::Value(v) | ReturnValue::Return((v, _)) => v,
            _ => panic!("{msg}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Break {
    context: Context,
}

impl Break {
    #[inline]
    pub fn new(context: Context) -> Break {
        Break { context }
    }

    #[inline]
    pub fn context(&self) -> Context {
        self.context
    }
}

#[derive(Debug, Clone)]
pub struct VariableInit {
    context: Context,
    identifiers: Vec<OptionallyAnnotatedIdentifier>,
    assign_pos: usize,
    expr: Expression,
}

impl VariableInit {
    #[inline]
    pub fn new(
        context: Context,
        identifiers: Vec<OptionallyAnnotatedIdentifier>,
        assign_pos: usize,
        expr: Expression,
    ) -> Self {
        VariableInit {
            context,
            identifiers,
            assign_pos,
            expr,
        }
    }

    #[inline]
    pub fn context(&self) -> Context {
        self.context
    }

    pub fn exec(&self, exec_session: &mut ExecSession, parsed_session: &ParsedSession) -> Result {
        match self.expr.exec(exec_session, parsed_session) {
            Ok(output) => {
                let value = output.expect("Expressions should always return a value on success");

                for item in &self.identifiers {
                    if let Some(typeid) = item.typeid() {
                        if typeid == 0 {
                            let var = Variable::new(value.clone(), true);
                            exec_session.add_variable(&item.name(), var);
                        } else if typeid == value.typeid() {
                            let var = Variable::new(value.clone(), false);
                            exec_session.add_variable(&item.name(), var);
                        } else {
                            return Err(Error::new(
                                self.context,
                                self.assign_pos,
                                ErrorKind::InvalidAssignment(
                                    parsed_session.get_typename(typeid),
                                    parsed_session.get_typename(value.typeid()),
                                ),
                            ));
                        }
                    } else {
                        let var = Variable::new(value.clone(), false);
                        exec_session.add_variable(&item.name(), var);
                    }
                }

                Ok(ReturnValue::Value(Value::None))
            }
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableAssign {
    context: Context,
    identifier: String,
    assign_pos: usize,
    expr: Expression,
}

impl VariableAssign {
    #[inline]
    pub fn new(context: Context, identifier: String, assign_pos: usize, expr: Expression) -> Self {
        VariableAssign {
            context,
            identifier,
            assign_pos,
            expr,
        }
    }

    #[inline]
    pub fn context(&self) -> Context {
        self.context
    }

    pub fn exec(&self, exec_session: &mut ExecSession, parsed_session: &ParsedSession) -> Result {
        let rhs = self
            .expr
            .exec(exec_session, parsed_session)?
            .expect("Expressions should always return a value on success");

        if let Some(lhs_var) = exec_session.get_variable_mut(&self.identifier) {
            if lhs_var.is_dynamic() || lhs_var.typeid() == rhs.typeid() {
                lhs_var.set_value(rhs.clone());
            } else {
                return Err(Error::new(
                    self.context,
                    self.assign_pos,
                    ErrorKind::InvalidAssignment(
                        parsed_session.get_typename(lhs_var.typeid()),
                        parsed_session.get_typename(rhs.typeid()),
                    ),
                ));
            }
        } else {
            return Err(Error::new(self.context, 0, ErrorKind::IdentifierNotFound));
        }

        Ok(ReturnValue::Value(Value::None))
    }
}
