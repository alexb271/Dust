use crate::error::{Context, Error, ErrorKind};
use crate::expression::Expression;
use crate::instruction::{Instruction, ReturnValue};
use crate::session::{ExecSession, ParseSession};
use crate::variable::{Value, Variable};

#[derive(Debug, Clone)]
pub struct ForLoop {
    alias: String,
    operand: Expression,
    body: Vec<Instruction>,

    context: Context,
    operand_pos: usize,
}

impl ForLoop {
    #[inline]
    pub fn new(
        alias: String,
        operand: Expression,
        body: Vec<Instruction>,
        context: Context,
        operand_pos: usize,
    ) -> ForLoop {
        ForLoop {
            alias,
            operand,
            body,
            context,
            operand_pos,
        }
    }

    #[inline]
    pub fn context(&self) -> Context {
        self.context
    }

    pub fn exec(
        &self,
        exec_session: &mut ExecSession,
        parse_session: &ParseSession,
    ) -> Result<ReturnValue, Error> {
        let operand = match self.operand.exec(exec_session, parse_session) {
            Ok(return_value) => {
                let value =
                    return_value.expect("Expressions should always return a value on success");
                match value {
                    Value::Vector(v) => v,
                    _ => {
                        return Err(Error::new(
                            self.context,
                            self.operand_pos,
                            ErrorKind::ForLoopNotVec(parse_session.get_typename(value.typeid())),
                        ));
                    }
                }
            }
            Err(e) => return Err(e),
        };

        'main_loop: for item in operand.borrow().iter() {
            exec_session.add_variable(&self.alias, Variable::new(item.clone(), false));

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
        }

        Ok(ReturnValue::Value(Value::None))
    }
}
