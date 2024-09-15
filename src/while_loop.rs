use crate::error::{Context, Error, ErrorKind};
use crate::expression::Expression;
use crate::instruction::{Instruction, ReturnValue};
use crate::session::{ExecSession, ParseSession};
use crate::variable::Value;

#[derive(Debug, Clone)]
pub struct WhileLoop {
    condition: Expression,
    body: Vec<Instruction>,

    context: Context,
}

impl WhileLoop {
    #[inline]
    pub fn new(condition: Expression, body: Vec<Instruction>, context: Context) -> WhileLoop {
        WhileLoop {
            condition,
            body,
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
        exec_session: &mut ExecSession,
        parse_session: &ParseSession,
    ) -> Result<bool, Error> {
        match self.condition.exec(exec_session, parse_session) {
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
    ) -> Result<ReturnValue, Error> {
        let mut condition_result = self.eval_condition(exec_session, parse_session)?;

        'main_loop: while condition_result != false {
            for item in &self.body {
                match item.exec(exec_session, parse_session) {
                    Ok(return_value) => match return_value {
                        ReturnValue::Return(value) => {
                            return Ok(ReturnValue::Return(value));
                        }
                        ReturnValue::Break => {
                            break 'main_loop;
                        }
                        ReturnValue::Value(_) => (),
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            condition_result = self.eval_condition(exec_session, parse_session)?;
        }

        Ok(ReturnValue::Value(Value::None))
    }
}
