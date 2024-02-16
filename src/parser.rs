use crate::branch::{Branch, BranchBody};
use crate::error::{Context, Error, ErrorKind};
use crate::expression::Expression;
use crate::function::{Function, FunctionCall, Return, UserFunction};
use crate::instruction::{Break, Instruction, VariableAssign, VariableInit};
use crate::session::ParsedSession;
use crate::token::{Operator, Parenthesis, Token};
use crate::variable::{AnnotatedIdentifier, OptionallyAnnotatedIdentifier};
use crate::while_loop::WhileLoop;
use std::collections::HashMap;

use pest::{iterators::Pair, iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct DustParser;

pub fn parse(input: &str, session: &mut ParsedSession) -> Result<Vec<Instruction>, Error> {
    let parse_result = DustParser::parse(Rule::start_symbol, &input);
    let parsed_content;

    match parse_result {
        Ok(mut content) => {
            parsed_content = content.next().unwrap();
        }
        Err(e) => {
            let pos = match e.line_col {
                pest::error::LineColLocation::Pos((_, column)) => column,
                pest::error::LineColLocation::Span((_, _), (column, _)) => column,
            };

            let offset = session.get_source_code_offset();
            let end_pos = offset + input.len();

            let err = Error::new(
                Context {
                    start: offset,
                    end: end_pos,
                },
                pos.saturating_sub(1),
                ErrorKind::SyntaxError,
            );
            return Err(err);
        }
    }

    let offset = session.get_source_code_offset();
    let full_text = session.get_source_code().to_string() + input;
    let mut builder = Builder::new(offset, &full_text);
    builder.build(parsed_content, session)
}

#[derive(Debug, Clone)]
struct Builder<'a> {
    offset: usize,
    text: &'a str,
    type_map: HashMap<String, usize>,
}

impl<'a> Builder<'a> {
    #[inline]
    fn new(offset: usize, text: &'a str) -> Self {
        let mut type_map = HashMap::new();
        type_map.insert(crate::DYN_KEYWORD.to_string(), crate::TYPEID_DYN);
        type_map.insert("none".to_string(), crate::TYPEID_NONE);
        type_map.insert("number".to_string(), crate::TYPEID_NUMBER);
        type_map.insert("string".to_string(), crate::TYPEID_STRING);
        type_map.insert("bool".to_string(), crate::TYPEID_BOOL);
        // TODO User Defined Types

        Builder {
            offset,
            text,
            type_map,
        }
    }

    #[inline]
    fn get_typeid(&self, typename: &str) -> Option<usize> {
        self.type_map.get(typename).copied()
    }

    fn build(
        &mut self,
        parsed_content: Pair<Rule>,
        session: &mut ParsedSession,
    ) -> Result<Vec<Instruction>, Error> {
        let mut result: Vec<Instruction> = Vec::new();

        for pair in parsed_content.into_inner() {
            match pair.as_rule() {
                Rule::expression => {
                    result.push(Instruction::Expression(self.build_expression(pair)?))
                }
                Rule::branch => result.push(Instruction::Branch(self.build_branch(pair)?)),
                Rule::while_loop => result.push(Instruction::WhileLoop(self.build_loop(pair)?)),
                Rule::var_init => {
                    result.push(Instruction::VariableInit(self.build_variable_init(pair)?))
                }
                Rule::var_assign => result.push(Instruction::VariableAssign(
                    self.build_variable_assign(pair)?,
                )),
                Rule::function_definition => {
                    let (name, function) = self.build_function_definition(pair)?;
                    session.add_function(name, function);
                }
                Rule::loop_break | Rule::function_return => {
                    let context = self.get_context(&pair);
                    return Err(Error::new(context, 0, ErrorKind::SyntaxError));
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        Ok(result)
    }

    fn build_function_definition(&self, function: Pair<Rule>) -> Result<(String, Function), Error> {
        let mut context = self.get_context(&function);

        let mut function = function.into_inner();
        let name = function.next().unwrap().as_str().to_string();

        let mut arguments: Vec<AnnotatedIdentifier> = Vec::new();
        let mut return_typeid: Option<(usize, usize)> = None;
        let mut body: Vec<Instruction> = Vec::new();

        for pair in function {
            match pair.as_rule() {
                Rule::var_id => {
                    let pos = self.offset + pair.as_span().start() - context.start;
                    let optional_argument = self.build_annotated_identifier(pair, context)?;

                    let argument = match AnnotatedIdentifier::from_optional(optional_argument) {
                        Some(argument) => argument,
                        None => {
                            return Err(Error::new(context, pos, ErrorKind::MissingAnnotation));
                        }
                    };

                    arguments.push(argument);
                }
                Rule::return_type_annotation => {
                    let token = pair.into_inner().next().unwrap();
                    let typename = token.as_str();
                    return_typeid = match self.get_typeid(typename) {
                        Some(id) => {
                            if id == crate::TYPEID_DYN {
                                None
                            } else {
                                Some((id, self.offset + token.as_span().start()))
                            }
                        }
                        None => {
                            return Err(Error::new(
                                context,
                                self.offset + token.as_span().start() - context.start,
                                ErrorKind::UnknownType(typename.to_string()),
                            ));
                        }
                    };

                    // restrict relevant context to signature only
                    context.end = token.as_span().end() + self.offset;
                }
                Rule::expression => {
                    body.push(Instruction::Expression(self.build_expression(pair)?))
                }
                Rule::branch => body.push(Instruction::Branch(self.build_branch(pair)?)),
                Rule::while_loop => body.push(Instruction::WhileLoop(self.build_loop(pair)?)),
                Rule::function_return => {
                    body.push(Instruction::Return(self.build_function_return(pair)?))
                }
                Rule::var_init => {
                    body.push(Instruction::VariableInit(self.build_variable_init(pair)?))
                }
                Rule::var_assign => body.push(Instruction::VariableAssign(
                    self.build_variable_assign(pair)?,
                )),
                Rule::loop_break => {
                    let context = self.get_context(&pair);
                    return Err(Error::new(context, 0, ErrorKind::SyntaxError));
                }
                _ => {
                    dbg!(pair);
                    unreachable!();
                }
            }
        }

        Ok((
            name,
            Function::UserFunction(UserFunction::new(context, arguments, return_typeid, body)),
        ))
    }

    fn build_function_return(&self, function_return: Pair<Rule>) -> Result<Return, Error> {
        let context = self.get_context(&function_return);

        let expr = match function_return.into_inner().nth(1) {
            Some(pair) => Some(self.build_expression(pair)?),
            None => None,
        };

        Ok(Return::new(context, expr))
    }

    fn build_variable_init(&self, var_init: Pair<Rule>) -> Result<VariableInit, Error> {
        let context = self.get_context(&var_init);

        let var_init = var_init.into_inner();
        let mut expr = None;
        let mut identifiers: Vec<OptionallyAnnotatedIdentifier> = Vec::new();
        let mut assign_pos: usize = 0;

        for pair in var_init {
            match pair.as_rule() {
                Rule::var_id => identifiers.push(self.build_annotated_identifier(pair, context)?),
                Rule::assign => assign_pos = self.offset + pair.as_span().start() - context.start,
                Rule::expression => expr = Some(pair),
                _ => unreachable!(),
            }
        }

        let expr = expr.expect("A var_init rule should always contain an expression");
        let expr = self.build_expression(expr)?;
        Ok(VariableInit::new(context, identifiers, assign_pos, expr))
    }

    fn build_annotated_identifier(
        &self,
        var_id: Pair<Rule>,
        context: Context,
    ) -> Result<OptionallyAnnotatedIdentifier, Error> {
        let mut name = String::new();
        let mut optional_typeid: Option<usize> = None;

        for pair in var_id.into_inner() {
            match pair.as_rule() {
                Rule::identifier => name = pair.as_str().to_string(),
                Rule::type_annotation => {
                    let token = pair.into_inner().next().unwrap();
                    let typename = token.as_str();
                    let typeid = match self.get_typeid(typename) {
                        Some(id) => id,
                        None => {
                            return Err(Error::new(
                                context,
                                self.offset + token.as_span().start() - context.start,
                                ErrorKind::UnknownType(typename.to_string()),
                            ));
                        }
                    };
                    optional_typeid = Some(typeid);
                }
                _ => unreachable!(),
            }
        }

        Ok(OptionallyAnnotatedIdentifier::new(name, optional_typeid))
    }

    fn build_variable_assign(&self, var_assign: Pair<Rule>) -> Result<VariableAssign, Error> {
        let context = self.get_context(&var_assign);

        let var_assign = var_assign.into_inner();
        let mut expr = None;
        let mut identifier = String::new();
        let mut assign_pos: usize = 0;

        for pair in var_assign {
            match pair.as_rule() {
                Rule::identifier => identifier = pair.as_str().to_string(),
                Rule::assign => assign_pos = self.offset + pair.as_span().start() - context.start,
                Rule::expression => expr = Some(pair),
                _ => unreachable!(),
            }
        }

        let expr = expr.expect("A var_assign rule should always contain an expression");
        let expr = self.build_expression(expr)?;
        Ok(VariableAssign::new(context, identifier, assign_pos, expr))
    }

    fn build_loop_break(&self, loop_break: Pair<Rule>) -> Break {
        let context = self.get_context(&loop_break);
        Break::new(context)
    }

    fn build_branch(&self, branch: Pair<Rule>) -> Result<Branch, Error> {
        let context = self.get_context(&branch);

        let mut branch = branch.into_inner();

        let main_condition = self.build_expression(branch.next().unwrap())?;
        let mut main_body: Vec<Instruction> = Vec::new();
        let mut else_if_branches: Vec<BranchBody> = Vec::new();
        let mut else_body: Vec<Instruction> = Vec::new();

        for pair in branch {
            match pair.as_rule() {
                Rule::expression => {
                    main_body.push(Instruction::Expression(self.build_expression(pair)?))
                }
                Rule::branch => main_body.push(Instruction::Branch(self.build_branch(pair)?)),
                Rule::while_loop => main_body.push(Instruction::WhileLoop(self.build_loop(pair)?)),
                Rule::loop_break => main_body.push(Instruction::Break(Break::new(context))),
                Rule::function_return => {
                    main_body.push(Instruction::Return(self.build_function_return(pair)?))
                }
                Rule::var_init => {
                    main_body.push(Instruction::VariableInit(self.build_variable_init(pair)?))
                }
                Rule::var_assign => main_body.push(Instruction::VariableAssign(
                    self.build_variable_assign(pair)?,
                )),
                Rule::branch_else_if => {
                    let mut else_if_branch = pair.into_inner();
                    let condition = self.build_expression(else_if_branch.next().unwrap())?;
                    let mut body: Vec<Instruction> = Vec::new();
                    self.build_body(else_if_branch, &mut body)?;
                    else_if_branches.push(BranchBody::new(condition, body));
                }
                Rule::branch_else => {
                    self.build_body(pair.into_inner(), &mut else_body)?;
                }
                _ => unreachable!(),
            }
        }

        let main_branch = BranchBody::new(main_condition, main_body);
        Ok(Branch::new(
            context,
            main_branch,
            else_if_branches,
            else_body,
        ))
    }

    fn build_loop(&self, control_flow: Pair<Rule>) -> Result<WhileLoop, Error> {
        let context = self.get_context(&control_flow);

        let mut control_flow = control_flow.into_inner();
        let condition = self.build_expression(control_flow.next().unwrap())?;

        let mut body: Vec<Instruction> = Vec::new();
        self.build_body(control_flow, &mut body)?;

        Ok(WhileLoop::new(context, condition, body))
    }

    fn build_body(&self, pairs: Pairs<Rule>, output: &mut Vec<Instruction>) -> Result<(), Error> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::expression => {
                    output.push(Instruction::Expression(self.build_expression(pair)?))
                }
                Rule::branch => output.push(Instruction::Branch(self.build_branch(pair)?)),
                Rule::while_loop => output.push(Instruction::WhileLoop(self.build_loop(pair)?)),
                Rule::loop_break => output.push(Instruction::Break(self.build_loop_break(pair))),
                Rule::function_return => {
                    output.push(Instruction::Return(self.build_function_return(pair)?))
                }
                Rule::var_init => {
                    output.push(Instruction::VariableInit(self.build_variable_init(pair)?))
                }
                Rule::var_assign => output.push(Instruction::VariableAssign(
                    self.build_variable_assign(pair)?,
                )),
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    fn build_expression(&self, expression: Pair<Rule>) -> Result<Expression, Error> {
        let context = self.get_context(&expression);

        let local_offset = expression.as_span().start();
        let mut output = Vec::new();

        self.tokenize_expression(expression, context, local_offset, &mut output)?;
        Ok(Expression::compile(output, context))
    }

    fn tokenize_expression(
        &self,
        expression: Pair<Rule>,
        context: Context,
        local_offset: usize,
        output: &mut Vec<Token>,
    ) -> Result<(), Error> {
        for pair in expression.into_inner() {
            let pos = pair.as_span().start() - local_offset;
            match pair.as_rule() {
                Rule::number => output.push(Token::new_number(
                    pos,
                    pair.as_str().parse::<f64>().unwrap(),
                )),
                Rule::bool_true => output.push(Token::new_bool(pos, true)),
                Rule::bool_false => output.push(Token::new_bool(pos, false)),
                Rule::text => {
                    let result = pair.as_str()[1..pair.as_str().len() - 1].to_string();
                    let result = result.replace("\\\"", "\"");
                    let result = result.replace("\\n", "\n");
                    output.push(Token::new_str(pos, result));
                }
                Rule::none => output.push(Token::new_none(pos)),
                Rule::pi => output.push(Token::new_number(pos, std::f64::consts::PI)),
                Rule::identifier => {
                    output.push(create_checked_identifier(context, pos, pair.as_str())?)
                }
                Rule::add => output.push(Token::new_operator(pos, Operator::Add)),
                Rule::sub => output.push(Token::new_operator(pos, Operator::Sub)),
                Rule::mul => output.push(Token::new_operator(pos, Operator::Mult)),
                Rule::div => output.push(Token::new_operator(pos, Operator::Div)),
                Rule::modulo => output.push(Token::new_operator(pos, Operator::Mod)),
                Rule::pow => output.push(Token::new_operator(pos, Operator::Pow)),
                Rule::and => output.push(Token::new_operator(pos, Operator::And)),
                Rule::or => output.push(Token::new_operator(pos, Operator::Or)),
                Rule::less_than => output.push(Token::new_operator(pos, Operator::LessThan)),
                Rule::greater_than => output.push(Token::new_operator(pos, Operator::GreaterThan)),
                Rule::equal => output.push(Token::new_operator(pos, Operator::Equal)),
                Rule::not_equal => output.push(Token::new_operator(pos, Operator::NotEqual)),
                Rule::neg => output.push(Token::new_operator(pos, Operator::Neg)),
                Rule::not => output.push(Token::new_operator(pos, Operator::Not)),
                Rule::gettype => output.push(Token::new_operator(pos, Operator::Typeof)),

                Rule::left_par => output.push(Token::new_parenthesis(pos, Parenthesis::Left)),
                Rule::right_par => output.push(Token::new_parenthesis(pos, Parenthesis::Right)),
                Rule::expression => {
                    self.tokenize_expression(pair, context, local_offset, output)?
                }
                Rule::function_call => output.push(self.build_function_call(pair, pos)?),
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    #[inline]
    fn build_function_call(&self, function_call: Pair<Rule>, pos: usize) -> Result<Token, Error> {
        let context = self.get_context(&function_call);
        let global_pos = function_call.as_span().start() + self.offset;

        let mut function_call = function_call.into_inner();
        let name = function_call.next().unwrap().as_str().to_string();

        let mut arguments: Vec<Expression> = Vec::new();
        for pair in function_call {
            match pair.as_rule() {
                Rule::expression => arguments.push(self.build_expression(pair)?),
                _ => unreachable!(),
            }
        }

        Ok(Token::new_function_call(
            pos,
            FunctionCall::new(
                name,
                context,
                get_line_column(global_pos, self.text),
                arguments,
            ),
        ))
    }

    #[inline]
    fn get_context(&self, pair: &Pair<Rule>) -> Context {
        Context {
            start: pair.as_span().start() + self.offset,
            end: pair.as_span().end() + self.offset,
        }
    }
}

#[inline]
pub fn get_line_column(pos: usize, text: &str) -> (usize, usize) {
    let mut newline_count = 0;
    let mut last_newline_pos = 0;
    let mut char_count = 0;
    let mut iter = text.chars();

    while let Some(ch) = iter.next() {
        char_count += 1;
        if ch == '\n' {
            if char_count <= pos {
                newline_count += 1;
                last_newline_pos = char_count;
            } else {
                break;
            }
        }
    }

    (newline_count + 1, pos - last_newline_pos + 1)
}

#[inline]
fn create_checked_identifier(
    context: Context,
    pos: usize,
    identifier: &str,
) -> Result<Token, Error> {
    for item in KEYWORDS {
        if identifier == item {
            return Err(Error::new(context, pos, ErrorKind::IdentifierIsKeyword));
        }
    }

    Ok(Token::new_identifier(pos, identifier.to_string()))
}

const KEYWORDS: [&'static str; 14] = [
    crate::DYN_KEYWORD,
    "let",
    "if",
    "else",
    "while",
    "break",
    "fn",
    "return",
    "not",
    "and",
    "or",
    "none",
    "typeof",
    "pi",
];
