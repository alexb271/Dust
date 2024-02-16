use crate::error::Context;
use crate::instruction::{self, ReturnValue};
use crate::operation::{self, Operand, OperationArgs};
use crate::session::{self, ExecSession, ParsedSession};
use crate::token::{Operator, Parenthesis, Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Expression {
    context: Context,
    tokens: Vec<Token>,
}

impl Expression {
    pub fn context(&self) -> Context {
        self.context
    }

    pub fn exec(
        &self,
        exec_session: &mut ExecSession,
        parsed_session: &ParsedSession,
    ) -> instruction::Result {
        let mut stack: Vec<Operand> = Vec::new();

        let mut args = OperationArgs {
            stack: &mut stack,
            exec_session,
            parsed_session,
            context: self.context,
        };

        for token in &self.tokens {
            match token.kind() {
                TokenKind::Value(val) => args.stack.push(Operand::Value(val.clone())),
                TokenKind::Operator(op) => match op {
                    Operator::Add => operation::add(&mut args, *op, token.pos())?,
                    Operator::Sub => operation::subtract(&mut args, *op, token.pos())?,
                    Operator::Mult => operation::multiply(&mut args, *op, token.pos())?,
                    Operator::Div => operation::divide(&mut args, *op, token.pos())?,
                    Operator::Mod => operation::modulo(&mut args, *op, token.pos())?,
                    Operator::Pow => operation::power(&mut args, *op, token.pos())?,
                    Operator::And => operation::and(&mut args, *op, token.pos())?,
                    Operator::Or => operation::or(&mut args, *op, token.pos())?,
                    Operator::LessThan => operation::less_than(&mut args, *op, token.pos())?,
                    Operator::GreaterThan => operation::greater_than(&mut args, *op, token.pos())?,
                    Operator::Equal => operation::equal(&mut args, *op, token.pos())?,
                    Operator::NotEqual => operation::not_equal(&mut args, *op, token.pos())?,
                    Operator::Neg => operation::negate(&mut args, *op, token.pos())?,
                    Operator::Not => operation::not(&mut args, *op, token.pos())?,
                    Operator::Typeof => operation::gettype(&mut args)?,
                },
                TokenKind::Identifier(id) => args.stack.push(Operand::Identifier(&id, token.pos())),
                TokenKind::FunctionCall(f) => {
                    match session::call_function(
                        args.parsed_session,
                        args.exec_session,
                        f,
                        self.context,
                        token.pos(),
                    ) {
                        Ok(output) => match output {
                            ReturnValue::Value(value) => args.stack.push(Operand::Value(value)),
                            _ => unreachable!(
                                "Function calls should not return Break or Return types"
                            ),
                        },
                        Err(e) => return Err(e),
                    }
                }
                _ => unreachable!(),
            }
        }

        match args
            .stack
            .pop()
            .unwrap()
            .get_value(args.exec_session, self.context)
        {
            Ok(value) => Ok(ReturnValue::Value(value)),
            Err(e) => Err(e),
        }
    }

    // Postfix Conversion Functions
    pub fn compile(input: Vec<Token>, context: Context) -> Expression {
        let mut output = Vec::new();
        let mut stack = Vec::new();

        for token in input {
            match token.kind() {
                TokenKind::Value(_) | TokenKind::Identifier(_) | TokenKind::FunctionCall(_) => {
                    output.push(token);
                }
                TokenKind::Operator(_) => {
                    Expression::process_operator(token, &mut stack, &mut output);
                }
                TokenKind::Parenthesis(_) => {
                    Expression::process_parenthesis(token, &mut stack, &mut output);
                }
            }
        }

        while stack.len() > 0 {
            output.push(stack.pop().unwrap());
        }

        Expression {
            context,
            tokens: output,
        }
    }

    fn process_operator(token: Token, stack: &mut Vec<Token>, output: &mut Vec<Token>) {
        let (precedence, left_assoc, is_unary) = match token.kind() {
            TokenKind::Operator(op) => (op.precedence(), op.is_left_associative(), op.is_unary()),
            _ => panic!("Token is not an operator"),
        };

        if let Some(top_of_stack) = stack.last() {
            match top_of_stack.kind() {
                TokenKind::Parenthesis(Parenthesis::Left) => stack.push(token),
                TokenKind::Operator(op) => {
                    if op.precedence() < precedence {
                        stack.push(token);
                    } else if op.precedence() > precedence {
                        if is_unary && op.is_unary() {
                            stack.push(token);
                        } else {
                            output.push(stack.pop().unwrap());
                            Expression::process_operator(token, stack, output);
                        }
                    } else {
                        if left_assoc {
                            output.push(stack.pop().unwrap());
                        }
                        stack.push(token);
                    }
                }
                _ => unreachable!(),
            }
        } else {
            stack.push(token);
        }
    }

    fn process_parenthesis(token: Token, stack: &mut Vec<Token>, output: &mut Vec<Token>) {
        let par = match token.kind() {
            TokenKind::Parenthesis(p) => p,
            _ => panic!("Token is not a parenthesis"),
        };

        match par {
            Parenthesis::Left => stack.push(token),
            Parenthesis::Right => loop {
                let temp = stack.pop().unwrap();
                if let TokenKind::Parenthesis(Parenthesis::Left) = temp.kind() {
                    break;
                } else {
                    output.push(temp);
                }
            },
        }
    }
}
