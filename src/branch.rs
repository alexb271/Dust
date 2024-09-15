use crate::error::{Context, Error, ErrorKind};
use crate::expression::Expression;
use crate::instruction::{self, Instruction, ReturnValue};
use crate::session::{ExecSession, ParseSession};
use crate::variable::Value;

#[derive(Debug, Clone)]
pub struct BranchBody {
    condition: Expression,
    body: Vec<Instruction>,
}

impl BranchBody {
    #[inline]
    pub fn new(condition: Expression, body: Vec<Instruction>) -> Self {
        BranchBody { condition, body }
    }
}

#[derive(Debug, Clone)]
pub struct Branch {
    main_branch: BranchBody,
    else_if_branches: Vec<BranchBody>,
    else_branch: Vec<Instruction>,

    context: Context,
}

impl Branch {
    #[inline]
    pub fn new(
        main_branch: BranchBody,
        else_if_branches: Vec<BranchBody>,
        else_branch: Vec<Instruction>,
        context: Context,
    ) -> Branch {
        Branch {
            main_branch,
            else_if_branches,
            else_branch,
            context,
        }
    }

    #[inline]
    pub fn context(&self) -> Context {
        self.context
    }

    #[inline]
    fn eval_condition(
        &self,
        condition: &Expression,
        exec_session: &mut ExecSession,
        parse_session: &ParseSession,
    ) -> Result<bool, Error> {
        match condition.exec(exec_session, parse_session) {
            Ok(return_value) => {
                let value =
                    return_value.expect("Expressions should always return a value on success");

                match value {
                    Value::Bool(b) => Ok(b),
                    _ => {
                        return Err(Error::new(
                            self.context,
                            0,
                            ErrorKind::ConditionalExpressionNotBool(
                                parse_session.get_typename(value.typeid()),
                            ),
                        ));
                    }
                }
            }
            Err(e) => return Err(e),
        }
    }

    pub fn exec(
        &self,
        exec_session: &mut ExecSession,
        parse_session: &ParseSession,
    ) -> instruction::Result {
        let condition_result =
            self.eval_condition(&self.main_branch.condition, exec_session, parse_session)?;

        if condition_result == true {
            return exec_body(&self.main_branch.body, exec_session, parse_session);
        }

        if !self.else_if_branches.is_empty() {
            for body in &self.else_if_branches {
                let condition_result =
                    self.eval_condition(&body.condition, exec_session, parse_session)?;
                if condition_result == true {
                    return exec_body(&body.body, exec_session, parse_session);
                }
            }
        }

        if !self.else_branch.is_empty() {
            return exec_body(&self.else_branch, exec_session, parse_session);
        }

        return Ok(ReturnValue::Value(Value::None));
    }
}

#[inline]
fn exec_body(
    body: &Vec<Instruction>,
    exec_session: &mut ExecSession,
    parse_session: &ParseSession,
) -> instruction::Result {
    for item in body {
        match item.exec(exec_session, parse_session) {
            Ok(return_value) => match return_value {
                ReturnValue::Return(value) => {
                    return Ok(ReturnValue::Return(value));
                }
                ReturnValue::Break => {
                    return Ok(ReturnValue::Break);
                }
                ReturnValue::Value(_) => (),
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(ReturnValue::Value(Value::None))
}
