use crate::branch::Branch;
use crate::builtin::TYPEID_DYN;
use crate::error::{Context, Error, ErrorKind};
use crate::expression::Expression;
use crate::for_loop::ForLoop;
use crate::function::Return;
use crate::session::{ExecSession, ParseSession};
use crate::variable::{OptionallyAnnotatedIdentifier, Value, Variable};
use crate::while_loop::WhileLoop;

#[derive(Debug, Clone)]
pub enum Instruction {
    Expression(Expression),
    Branch(Branch),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
    Return(Return),
    Break(Break),
    VariableInit(VariableInit),
    VariableAssign(VariableAssign),
}

impl Instruction {
    #[inline]
    pub fn exec(&self, exec_session: &mut ExecSession, parse_session: &ParseSession) -> Result {
        match self {
            Instruction::Expression(e) => e.exec(exec_session, parse_session),
            Instruction::Branch(b) => b.exec(exec_session, parse_session),
            Instruction::WhileLoop(wl) => wl.exec(exec_session, parse_session),
            Instruction::ForLoop(fl) => fl.exec(exec_session, parse_session),
            Instruction::Return(r) => r.exec(exec_session, parse_session),
            Instruction::Break(_) => Ok(ReturnValue::Break),
            Instruction::VariableInit(vi) => vi.exec(exec_session, parse_session),
            Instruction::VariableAssign(va) => va.exec(exec_session, parse_session),
        }
    }

    #[inline]
    pub fn context(&self) -> Context {
        match self {
            Instruction::Expression(e) => e.context(),
            Instruction::Branch(b) => b.context(),
            Instruction::WhileLoop(wl) => wl.context(),
            Instruction::ForLoop(fl) => fl.context(),
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
            usize, // global position of the token that produced tha value, for error reporting
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
    identifiers: Vec<OptionallyAnnotatedIdentifier>,
    assign_pos: usize,
    expr: Expression,

    context: Context,
}

impl VariableInit {
    #[inline]
    pub fn new(
        identifiers: Vec<OptionallyAnnotatedIdentifier>,
        assign_pos: usize,
        expr: Expression,
        context: Context,
    ) -> Self {
        VariableInit {
            identifiers,
            assign_pos,
            expr,
            context,
        }
    }

    #[inline]
    pub fn context(&self) -> Context {
        self.context
    }

    pub fn exec(&self, exec_session: &mut ExecSession, parse_session: &ParseSession) -> Result {
        match self.expr.exec(exec_session, parse_session) {
            Ok(output) => {
                let value = output.expect("Expressions should always return a value on success");

                for item in &self.identifiers {
                    if let Some(typeid) = item.typeid() {
                        if typeid == TYPEID_DYN {
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
                                    parse_session.get_typename(typeid),
                                    parse_session.get_typename(value.typeid()),
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
    source_expr: Option<(Expression, /*access operator position*/ usize)>,
    identifier: String,
    private_access_typeid: Option<usize>,
    expr: Expression,

    context: Context,
    id_pos: usize,
    assign_pos: usize,
}

impl VariableAssign {
    #[inline]
    pub fn new(
        source_expr: Option<(Expression, usize)>,
        identifier: String,
        private_access_typeid: Option<usize>,
        expr: Expression,
        context: Context,
        id_pos: usize,
        assign_pos: usize,
    ) -> Self {
        VariableAssign {
            source_expr,
            identifier,
            private_access_typeid,
            expr,
            context,
            id_pos,
            assign_pos,
        }
    }

    #[inline]
    pub fn context(&self) -> Context {
        self.context
    }

    pub fn exec(&self, exec_session: &mut ExecSession, parse_session: &ParseSession) -> Result {
        let rhs = self
            .expr
            .exec(exec_session, parse_session)?
            .expect("Expressions should always return a value on success");

        if let Some((ref e, pos)) = self.source_expr {
            let source = e
                .exec(exec_session, parse_session)?
                .expect("Expressions should always return a value on success");

            match source {
                Value::Class(c) => {
                    let private_access: bool = if let Some(id) = self.private_access_typeid {
                        if id == c.borrow().typeid() {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    match c.borrow_mut().set_property(
                        self.identifier.as_str(),
                        private_access,
                        rhs,
                        parse_session,
                    ) {
                        Ok(()) => (),
                        Err(e) => match e {
                            ErrorKind::HasNoMember(_, _) => {
                                return Err(Error::new(self.context, pos, e));
                            }
                            ErrorKind::MemberIsPrivate(_) => {
                                return Err(Error::new(self.context, self.id_pos, e));
                            }
                            ErrorKind::InvalidAssignment(_, _) => {
                                return Err(Error::new(self.context, self.assign_pos, e));
                            }
                            _ => unreachable!(),
                        },
                    }
                }
                _ => {
                    return Err(Error::new(self.context, 0, ErrorKind::InvalidMemberAccess));
                }
            }
        } else {
            if let Some(lhs_var) = exec_session.get_variable_mut(&self.identifier) {
                if lhs_var.is_dynamic() || lhs_var.typeid() == rhs.typeid() {
                    lhs_var.set_value(rhs.clone());
                } else {
                    return Err(Error::new(
                        self.context,
                        self.assign_pos,
                        ErrorKind::InvalidAssignment(
                            parse_session.get_typename(lhs_var.typeid()),
                            parse_session.get_typename(rhs.typeid()),
                        ),
                    ));
                }
            } else {
                return Err(Error::new(self.context, 0, ErrorKind::IdentifierNotFound));
            }
        }

        Ok(ReturnValue::Value(Value::None))
    }
}
