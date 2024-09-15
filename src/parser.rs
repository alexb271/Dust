use crate::branch::{Branch, BranchBody};
use crate::builtin::{DYN_KEYWORD, TYPEID_DYN, TYPEID_NONE};
use crate::class::{ClassDefinition, ClassFunction, PropertyDefinition};
use crate::error::{Context, Error, ErrorKind};
use crate::expression::Expression;
use crate::for_loop::ForLoop;
use crate::function::{Function, FunctionCall, Return, UserFunction};
use crate::instruction::{Break, Instruction, VariableAssign, VariableInit};
use crate::session::ParseSession;
use crate::token::{Operator, Parenthesis, Token, TokenKind};
use crate::variable::{AnnotatedIdentifier, OptionallyAnnotatedIdentifier};
use crate::while_loop::WhileLoop;

use std::collections::{HashMap, HashSet};

use pest::{iterators::Pair, iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DustParser;

pub fn parse(input: &str, session: &mut ParseSession) -> Result<Vec<Instruction>, Error> {
    let input = input.to_string();
    session.append_source_code(&input);
    let parse_result = DustParser::parse(Rule::start_symbol, &input);
    let parse_content;

    match parse_result {
        Ok(mut content) => {
            parse_content = content.next().unwrap();
        }
        Err(e) => {
            let (lines, column) = match e.line_col {
                pest::error::LineColLocation::Pos((line, column)) => (line, column),
                pest::error::LineColLocation::Span((line, _), (column, _)) => (line, column),
            };

            let mut chars = input.chars();
            let mut line_count = 1;
            let mut char_count = 0;

            while line_count < lines {
                if chars.next().unwrap() == '\n' {
                    line_count += 1;
                }
                char_count += 1;
            }

            let start = char_count;

            while let Some(ch) = chars.next() {
                if ch == '\n' {
                    break;
                }
                char_count += 1;
            }

            let end = char_count;

            let offset = session.get_source_code_offset();
            let err = Error::new(
                Context {
                    start: start + offset,
                    end: end + offset,
                },
                column.saturating_sub(1),
                ErrorKind::SyntaxError,
            );
            return Err(err);
        }
    }

    let offset = session.get_source_code_offset();
    let full_text = session.get_source_code().to_string() + &input;
    let mut builder = Builder::new(offset, &full_text, session.create_typemap());
    builder.build(parse_content, session)
}

#[derive(Debug, Clone)]
struct Builder<'a> {
    offset: usize,
    text: &'a str,
    type_map: HashMap<String, usize>,
}

impl<'a> Builder<'a> {
    #[inline]
    fn new(offset: usize, text: &'a str, type_map: HashMap<String, usize>) -> Self {
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

    #[inline]
    fn register_type(&mut self, name: String, typeid: usize) {
        self.type_map.insert(name, typeid);
    }

    fn build(
        &mut self,
        parse_result: Pair<Rule>,
        session: &mut ParseSession,
    ) -> Result<Vec<Instruction>, Error> {
        let mut result: Vec<Instruction> = Vec::new();

        for pair in parse_result.into_inner() {
            match pair.as_rule() {
                Rule::class_definition => {
                    let typeid = session.get_next_typeid();
                    let (name, class_definition) = self.build_class_definition(pair, typeid)?;
                    session.add_class_definition(name.clone(), class_definition);
                }
                Rule::function_definition => {
                    let (name, name_pos, _uses_self, function) =
                        self.build_function_definition(pair, false, None)?;
                    let context = function.context();
                    if !session.add_function(name, Function::UserFunction(function)) {
                        return Err(Error::new(
                            context,
                            name_pos,
                            ErrorKind::FunctionAlreadyDefined,
                        ));
                    }
                }
                Rule::branch => result.push(Instruction::Branch(self.build_branch(pair, None)?)),
                Rule::while_loop => {
                    result.push(Instruction::WhileLoop(self.build_while_loop(pair, None)?))
                }
                Rule::for_loop => {
                    result.push(Instruction::ForLoop(self.build_for_loop(pair, None)?))
                }
                Rule::loop_break | Rule::function_return => {
                    let context = self.get_context(&pair);
                    return Err(Error::new(context, 0, ErrorKind::SyntaxError));
                }
                Rule::var_init => result.push(Instruction::VariableInit(
                    self.build_variable_init(pair, None)?,
                )),
                Rule::var_assign => result.push(Instruction::VariableAssign(
                    self.build_variable_assign(pair, None)?,
                )),
                Rule::expression => {
                    result.push(Instruction::Expression(self.build_expression(pair, None)?))
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        Ok(result)
    }

    fn build_class_definition(
        &mut self,
        class: Pair<Rule>,
        typeid: usize,
    ) -> Result<(String, ClassDefinition), Error> {
        let context = self.get_context(&class);
        let mut class = class.into_inner();
        let name_pair = class.next().unwrap();
        self.validate_identifier(
            name_pair.as_str(),
            context,
            self.offset + name_pair.as_span().start() - context.start,
        )?;
        let name = name_pair.as_str().to_string();
        self.register_type(name.clone(), typeid);
        let mut constructor_parameters: Vec<AnnotatedIdentifier> = Vec::new();
        let mut property_definitions: Vec<PropertyDefinition> = Vec::new();
        let mut property_names: HashSet<String> = HashSet::new();
        let mut function_definitions: HashMap<String, ClassFunction> = HashMap::new();

        for pair in class {
            match pair.as_rule() {
                Rule::constructor_parameters => {
                    let params = pair.into_inner();
                    for pair in params {
                        match pair.as_rule() {
                            Rule::var_id => {
                                let pos = self.offset + pair.as_span().start() - context.start;
                                let parameter =
                                    self.build_annotated_identifier(pair, context, pos)?;
                                constructor_parameters.push(parameter);
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                Rule::property_definition => {
                    let context = self.get_context(&pair);
                    let mut prop_def_pair = pair.into_inner();

                    let next_pair = prop_def_pair.next().unwrap();
                    let (is_public, var_id_pair) = match next_pair.as_rule() {
                        Rule::pub_keyword => (true, prop_def_pair.next().unwrap()),
                        Rule::var_id => (false, next_pair),
                        _ => unreachable!(),
                    };

                    let assign_pair = prop_def_pair.next().unwrap();
                    let init_expr_pair = prop_def_pair.next().unwrap();

                    let pos = self.offset + var_id_pair.as_span().start() - context.start;
                    let annotated_id =
                        self.build_annotated_identifier(var_id_pair, context, pos)?;

                    if annotated_id.typeid() == typeid {
                        return Err(Error::new(context, pos, ErrorKind::RecursiveType));
                    }
                    if property_names.contains(annotated_id.name()) {
                        return Err(Error::new(context, pos, ErrorKind::MemberAlreadyDefined));
                    }

                    property_names.insert(annotated_id.name().to_string());

                    let assign_pos = self.offset + assign_pair.as_span().start() - context.start;
                    let init_expr = self.build_expression(init_expr_pair, Some(typeid))?;
                    for token in init_expr.tokens() {
                        match token.kind() {
                            TokenKind::FunctionCall(f) => {
                                if let Some(id) = f.associated_typeid() {
                                    if id == typeid && f.name() == "new" {
                                        return Err(Error::new(
                                            context,
                                            pos,
                                            ErrorKind::RecursiveType,
                                        ));
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    property_definitions.push(PropertyDefinition::new(
                        is_public,
                        annotated_id,
                        init_expr,
                        context,
                        assign_pos,
                    ));
                }
                Rule::class_function_definition => {
                    let mut fn_def_pair = pair.into_inner();
                    let next_pair = fn_def_pair.next().unwrap();
                    let (is_public, fn_pair) = match next_pair.as_rule() {
                        Rule::pub_keyword => (true, fn_def_pair.next().unwrap()),
                        Rule::function_definition => (false, next_pair),
                        _ => unreachable!(),
                    };
                    let (name, name_pos, uses_self, function) =
                        self.build_function_definition(fn_pair, true, Some(typeid))?;
                    if function_definitions.contains_key(&name) {
                        return Err(Error::new(
                            function.context(),
                            name_pos,
                            ErrorKind::FunctionAlreadyDefined,
                        ));
                    }
                    function_definitions.insert(
                        name,
                        ClassFunction::new(Function::UserFunction(function), uses_self, is_public),
                    );
                }
                _ => unreachable!(),
            }
        }

        Ok((
            name,
            ClassDefinition::new(
                constructor_parameters,
                property_definitions,
                function_definitions,
                typeid,
                context,
            ),
        ))
    }

    fn build_function_definition(
        &self,
        function: Pair<Rule>,
        is_method: bool,
        private_access_typeid: Option<usize>,
    ) -> Result<(String, usize, bool, UserFunction), Error> {
        let mut function = function.into_inner();
        let signature = function.next().unwrap();
        let context = self.get_context(&signature);
        let mut signature = signature.into_inner();
        let name_pair = signature.next().unwrap();
        self.validate_identifier(
            name_pair.as_str(),
            context,
            self.offset + name_pair.as_span().start() - context.start,
        )?;
        let name = name_pair.as_str().to_string();
        let name_pos = self.offset + name_pair.as_span().start() - context.start;

        let mut arguments: Vec<AnnotatedIdentifier> = Vec::new();
        let mut return_typeid: Option<(usize, usize)> = Some((TYPEID_NONE, 0));
        let mut uses_self: bool = false;

        for pair in signature {
            match pair.as_rule() {
                Rule::self_keyword => {
                    if !is_method {
                        let pos = self.offset + pair.as_span().start() - context.start;
                        return Err(Error::new(context, pos, ErrorKind::SelfOutsideMethod));
                    }
                    uses_self = true;
                }
                Rule::var_id => {
                    let pos = self.offset + pair.as_span().start() - context.start;
                    let argument = self.build_annotated_identifier(pair, context, pos)?;
                    arguments.push(argument);
                }
                Rule::return_type_annotation => {
                    let token = pair.into_inner().next().unwrap();
                    let typename = token.as_str();
                    return_typeid = match self.get_typeid(typename) {
                        Some(id) => {
                            if id == TYPEID_DYN {
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
                }
                _ => {
                    unreachable!()
                }
            }
        }

        let mut body: Vec<Instruction> = Vec::new();

        for pair in function {
            match pair.as_rule() {
                Rule::expression => body.push(Instruction::Expression(
                    self.build_expression(pair, private_access_typeid)?,
                )),
                Rule::branch => body.push(Instruction::Branch(
                    self.build_branch(pair, private_access_typeid)?,
                )),
                Rule::while_loop => body.push(Instruction::WhileLoop(
                    self.build_while_loop(pair, private_access_typeid)?,
                )),
                Rule::for_loop => body.push(Instruction::ForLoop(
                    self.build_for_loop(pair, private_access_typeid)?,
                )),
                Rule::function_return => body.push(Instruction::Return(
                    self.build_function_return(pair, private_access_typeid)?,
                )),
                Rule::var_init => body.push(Instruction::VariableInit(
                    self.build_variable_init(pair, private_access_typeid)?,
                )),
                Rule::var_assign => body.push(Instruction::VariableAssign(
                    self.build_variable_assign(pair, private_access_typeid)?,
                )),
                Rule::loop_break => {
                    let context = self.get_context(&pair);
                    return Err(Error::new(context, 0, ErrorKind::SyntaxError));
                }
                _ => {
                    unreachable!();
                }
            }
        }

        Ok((
            name,
            name_pos,
            uses_self,
            UserFunction::new(arguments, return_typeid, body, context),
        ))
    }

    fn build_function_return(
        &self,
        function_return: Pair<Rule>,
        private_access_typeid: Option<usize>,
    ) -> Result<Return, Error> {
        let context = self.get_context(&function_return);

        let expr = match function_return.into_inner().nth(1) {
            Some(pair) => Some(self.build_expression(pair, private_access_typeid)?),
            None => None,
        };

        Ok(Return::new(expr, context))
    }
    fn build_branch(
        &self,
        branch: Pair<Rule>,
        private_access_typeid: Option<usize>,
    ) -> Result<Branch, Error> {
        let context = self.get_context(&branch);

        let mut branch = branch.into_inner();

        let main_condition =
            self.build_expression(branch.next().unwrap(), private_access_typeid)?;
        let mut main_body: Vec<Instruction> = Vec::new();
        let mut else_if_branches: Vec<BranchBody> = Vec::new();
        let mut else_body: Vec<Instruction> = Vec::new();

        for pair in branch {
            match pair.as_rule() {
                Rule::branch => main_body.push(Instruction::Branch(
                    self.build_branch(pair, private_access_typeid)?,
                )),
                Rule::branch_else_if => {
                    let mut else_if_branch = pair.into_inner();
                    let condition = self
                        .build_expression(else_if_branch.next().unwrap(), private_access_typeid)?;
                    let mut body: Vec<Instruction> = Vec::new();
                    self.build_body(else_if_branch, private_access_typeid, &mut body)?;
                    else_if_branches.push(BranchBody::new(condition, body));
                }
                Rule::branch_else => {
                    self.build_body(pair.into_inner(), private_access_typeid, &mut else_body)?;
                }
                Rule::while_loop => main_body.push(Instruction::WhileLoop(
                    self.build_while_loop(pair, private_access_typeid)?,
                )),
                Rule::for_loop => main_body.push(Instruction::ForLoop(
                    self.build_for_loop(pair, private_access_typeid)?,
                )),
                Rule::loop_break => main_body.push(Instruction::Break(Break::new(context))),
                Rule::function_return => main_body.push(Instruction::Return(
                    self.build_function_return(pair, private_access_typeid)?,
                )),
                Rule::var_init => main_body.push(Instruction::VariableInit(
                    self.build_variable_init(pair, private_access_typeid)?,
                )),
                Rule::var_assign => main_body.push(Instruction::VariableAssign(
                    self.build_variable_assign(pair, private_access_typeid)?,
                )),
                Rule::expression => main_body.push(Instruction::Expression(
                    self.build_expression(pair, private_access_typeid)?,
                )),
                _ => unreachable!(),
            }
        }

        let main_branch = BranchBody::new(main_condition, main_body);
        Ok(Branch::new(
            main_branch,
            else_if_branches,
            else_body,
            context,
        ))
    }

    fn build_while_loop(
        &self,
        while_loop: Pair<Rule>,
        private_access_typeid: Option<usize>,
    ) -> Result<WhileLoop, Error> {
        let context = self.get_context(&while_loop);
        let mut while_loop = while_loop.into_inner();

        let condition = self.build_expression(while_loop.next().unwrap(), private_access_typeid)?;

        let mut body: Vec<Instruction> = Vec::new();
        self.build_body(while_loop, private_access_typeid, &mut body)?;

        Ok(WhileLoop::new(condition, body, context))
    }

    fn build_for_loop(
        &self,
        for_loop: Pair<Rule>,
        private_access_typeid: Option<usize>,
    ) -> Result<ForLoop, Error> {
        let context = self.get_context(&for_loop);
        let mut for_loop = for_loop.into_inner();

        let alias = for_loop.next().unwrap().as_str().to_string();
        let operand_pair = for_loop.next().unwrap();
        let operand_pos = self.offset + operand_pair.as_span().start() - context.start;
        let operand = self.build_expression(operand_pair, private_access_typeid)?;

        let mut body: Vec<Instruction> = Vec::new();
        self.build_body(for_loop, private_access_typeid, &mut body)?;

        Ok(ForLoop::new(alias, operand, body, context, operand_pos))
    }

    fn build_loop_break(&self, loop_break: Pair<Rule>) -> Break {
        let context = self.get_context(&loop_break);
        Break::new(context)
    }

    fn build_body(
        &self,
        pairs: Pairs<Rule>,
        private_access_typeid: Option<usize>,
        output: &mut Vec<Instruction>,
    ) -> Result<(), Error> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::branch => output.push(Instruction::Branch(
                    self.build_branch(pair, private_access_typeid)?,
                )),
                Rule::while_loop => output.push(Instruction::WhileLoop(
                    self.build_while_loop(pair, private_access_typeid)?,
                )),
                Rule::for_loop => output.push(Instruction::ForLoop(
                    self.build_for_loop(pair, private_access_typeid)?,
                )),
                Rule::loop_break => output.push(Instruction::Break(self.build_loop_break(pair))),
                Rule::function_return => output.push(Instruction::Return(
                    self.build_function_return(pair, private_access_typeid)?,
                )),
                Rule::var_init => output.push(Instruction::VariableInit(
                    self.build_variable_init(pair, private_access_typeid)?,
                )),
                Rule::var_assign => output.push(Instruction::VariableAssign(
                    self.build_variable_assign(pair, private_access_typeid)?,
                )),
                Rule::expression => output.push(Instruction::Expression(
                    self.build_expression(pair, private_access_typeid)?,
                )),
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    fn build_variable_init(
        &self,
        var_init: Pair<Rule>,
        private_access_typeid: Option<usize>,
    ) -> Result<VariableInit, Error> {
        let context = self.get_context(&var_init);

        let var_init = var_init.into_inner();
        let mut expr = None;
        let mut identifiers: Vec<OptionallyAnnotatedIdentifier> = Vec::new();
        let mut assign_pos: usize = 0;

        for pair in var_init {
            match pair.as_rule() {
                Rule::var_id => {
                    identifiers.push(self.build_optionally_annotated_identifier(pair, context)?)
                }
                Rule::assign => assign_pos = self.offset + pair.as_span().start() - context.start,
                Rule::expression => expr = Some(pair),
                _ => unreachable!(),
            }
        }

        let expr = expr.expect("A var_init rule should always contain an expression");
        let expr = self.build_expression(expr, private_access_typeid)?;
        Ok(VariableInit::new(identifiers, assign_pos, expr, context))
    }

    fn build_variable_assign(
        &self,
        var_assign: Pair<Rule>,
        private_access_typeid: Option<usize>,
    ) -> Result<VariableAssign, Error> {
        let context = self.get_context(&var_assign);

        let var_assign = var_assign.into_inner();
        let mut source_expr: Option<(Expression, usize)> = None;
        let mut identifier = String::new();
        let mut id_pos = 0;
        let mut assign_pos: usize = 0;
        let mut expr = None;

        for pair in var_assign {
            match pair.as_rule() {
                Rule::source_chain => {
                    let context = self.get_context(&pair);
                    let local_offset = pair.as_span().start();
                    let mut output = Vec::new();

                    self.tokenize_expression(
                        pair,
                        context,
                        local_offset,
                        &mut output,
                        private_access_typeid,
                    )?;
                    let dot_pos = output.pop().unwrap().pos();
                    let expr = Expression::compile(output, private_access_typeid, context);
                    source_expr = Some((expr, dot_pos));
                }
                Rule::identifier => {
                    id_pos = self.offset + pair.as_span().start() - context.start;
                    self.validate_identifier(pair.as_str(), context, id_pos)?;
                    identifier = pair.as_str().to_string();
                }
                Rule::assign => assign_pos = self.offset + pair.as_span().start() - context.start,
                Rule::expression => expr = Some(pair),
                _ => unreachable!(),
            }
        }

        let expr = expr.expect("A var_assign rule should always contain an expression");
        let expr = self.build_expression(expr, private_access_typeid)?;
        Ok(VariableAssign::new(
            source_expr,
            identifier,
            private_access_typeid,
            expr,
            context,
            id_pos,
            assign_pos,
        ))
    }

    fn build_annotated_identifier(
        &self,
        var_id: Pair<Rule>,
        context: Context,
        pos: usize,
    ) -> Result<AnnotatedIdentifier, Error> {
        let optional_id = self.build_optionally_annotated_identifier(var_id, context)?;

        let annotated_id = match AnnotatedIdentifier::from_optional(optional_id) {
            Some(id) => id,
            None => {
                return Err(Error::new(context, pos, ErrorKind::MissingAnnotation));
            }
        };

        Ok(annotated_id)
    }

    fn build_optionally_annotated_identifier(
        &self,
        opt_var_id: Pair<Rule>,
        context: Context,
    ) -> Result<OptionallyAnnotatedIdentifier, Error> {
        let mut name = String::new();
        let mut optional_typeid: Option<usize> = None;

        for pair in opt_var_id.into_inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    self.validate_identifier(
                        pair.as_str(),
                        context,
                        self.offset + pair.as_span().start() - context.start,
                    )?;
                    name = pair.as_str().to_string();
                }
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

    fn build_expression(
        &self,
        expression: Pair<Rule>,
        private_access_typeid: Option<usize>,
    ) -> Result<Expression, Error> {
        let context = self.get_context(&expression);

        let local_offset = expression.as_span().start();
        let mut output = Vec::new();

        self.tokenize_expression(
            expression,
            context,
            local_offset,
            &mut output,
            private_access_typeid,
        )?;
        Ok(Expression::compile(output, private_access_typeid, context))
    }

    fn tokenize_expression(
        &self,
        expression: Pair<Rule>,
        context: Context,
        local_offset: usize,
        output: &mut Vec<Token>,
        private_access_typeid: Option<usize>,
    ) -> Result<(), Error> {
        for pair in expression.into_inner() {
            let pos = pair.as_span().start() - local_offset;
            match pair.as_rule() {
                Rule::number => match pair.as_str().parse::<isize>() {
                    Ok(int) => output.push(Token::new_int(pos, int)),
                    Err(_) => output.push(Token::new_float(pos, pair.as_str().parse().unwrap())),
                },
                Rule::bool_true => output.push(Token::new_bool(pos, true)),
                Rule::bool_false => output.push(Token::new_bool(pos, false)),
                Rule::text => {
                    let result = pair.as_str()[1..pair.as_str().len() - 1].to_string();
                    let result = result.replace(r#"\""#, r#"""#);
                    let result = result.replace("\\n", "\n");
                    let result = result.replace("\\t", "\t");
                    let result = result.replace(r"\\", r"\");
                    output.push(Token::new_str(pos, result));
                }
                Rule::none => output.push(Token::new_none(pos)),
                Rule::identifier => {
                    if pair.as_str() != "self" {
                        self.validate_identifier(pair.as_str(), context, pos)?;
                    }
                    output.push(Token::new_identifier(pos, pair.as_str().to_string()));
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
                Rule::dot => output.push(Token::new_operator(pos, Operator::Dot)),
                Rule::gettype => output.push(Token::new_operator(pos, Operator::Typeof)),

                Rule::left_par => output.push(Token::new_parenthesis(pos, Parenthesis::Left)),
                Rule::right_par => output.push(Token::new_parenthesis(pos, Parenthesis::Right)),
                Rule::expression => self.tokenize_expression(
                    pair,
                    context,
                    local_offset,
                    output,
                    private_access_typeid,
                )?,
                Rule::function_call => {
                    output.push(self.build_function_call(pair, private_access_typeid, pos)?)
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    #[inline]
    fn build_function_call(
        &self,
        function_call: Pair<Rule>,
        private_access_typeid: Option<usize>,
        pos: usize,
    ) -> Result<Token, Error> {
        let context = self.get_context(&function_call);
        let global_pos = function_call.as_span().start() + self.offset;

        let mut function_call = function_call.into_inner();

        let name: String;
        let name_pos;
        let mut associated_type: Option<(usize, usize)> = None;

        let first_pair = function_call.next().unwrap();
        match first_pair.as_rule() {
            Rule::identifier => {
                name = first_pair.as_str().to_string();
                name_pos = 0;
            }
            Rule::scope => {
                let mut scope = first_pair.into_inner();
                let name_pair = scope.next().unwrap();
                let start_pos = name_pair.as_span().start();
                let typename = name_pair.as_str();
                let operator_pos =
                    self.offset + scope.next().unwrap().as_span().start() - context.start;
                let typeid = match self.get_typeid(typename) {
                    Some(id) => id,
                    None => {
                        return Err(Error::new(
                            context,
                            self.offset + start_pos - context.start,
                            ErrorKind::UnknownType(typename.to_string()),
                        ));
                    }
                };
                let name_pair = function_call.next().unwrap();
                name = name_pair.as_str().to_string();
                name_pos = self.offset + name_pair.as_span().start() - context.start;
                associated_type = Some((typeid, operator_pos));
            }
            _ => unreachable!(),
        }

        let mut arguments: Vec<Expression> = Vec::new();
        for pair in function_call {
            match pair.as_rule() {
                Rule::expression => {
                    arguments.push(self.build_expression(pair, private_access_typeid)?)
                }
                _ => {
                    unreachable!();
                }
            }
        }

        Ok(Token::new_function_call(
            pos,
            FunctionCall::new(
                associated_type,
                name,
                arguments,
                private_access_typeid,
                context,
                name_pos,
                get_line_column(global_pos, self.text),
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

    #[inline]
    fn validate_identifier(
        &self,
        identifier: &str,
        context: Context,
        pos: usize,
    ) -> Result<(), Error> {
        for item in KEYWORDS {
            if identifier == item {
                return Err(Error::new(context, pos, ErrorKind::IdentifierIsKeyword));
            }
        }
        match self.get_typeid(identifier) {
            Some(_) => Err(Error::new(context, pos, ErrorKind::IdentifierIsTypename)),
            None => Ok(()),
        }
    }
}

const KEYWORDS: [&'static str; 18] = [
    DYN_KEYWORD,
    "and",
    "break",
    "class",
    "else",
    "fn",
    "for",
    "if",
    "in",
    "let",
    "new",
    "none",
    "not",
    "or",
    "return",
    "self",
    "typeof",
    "while",
];

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
