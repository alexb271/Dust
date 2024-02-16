use crate::builtin;
use crate::error::{Context, Error, ErrorKind};
use crate::function::{Function, FunctionCall};
use crate::instruction;
use crate::variable::Variable;
use std::collections::HashMap;

// Two structs are needed in order to be able to
// assign different mutability modifiers to them
// and satisfy the borrow checker

#[derive(Debug, Clone)]
pub struct Session {
    pub exec_session: ExecSession,
    pub parsed_session: ParsedSession,
}

impl Session {
    #[inline]
    pub fn new() -> Self {
        Session {
            exec_session: ExecSession::new(),
            parsed_session: ParsedSession::new(),
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.exec_session.clear();
        self.parsed_session.clear();
    }

    #[inline]
    pub fn process(&mut self, input: &str) {
        crate::process(input, self)
    }
}

// Contains data that cannot be modified after parsing
// such as function definitions

#[derive(Debug, Clone)]
pub struct ParsedSession {
    function_store: HashMap<String, Function>,
    type_names: Vec<String>,
    source_code: String,
    offset: usize,
}

impl ParsedSession {
    #[inline]
    pub fn new() -> Self {
        let mut function_store = HashMap::new();
        builtin::load_builtin(&mut function_store);

        let mut type_names = Vec::new();
        type_names.push(crate::DYN_KEYWORD.to_string());
        type_names.push("none".to_string());
        type_names.push("number".to_string());
        type_names.push("string".to_string());
        type_names.push("bool".to_string());

        ParsedSession {
            function_store,
            type_names,
            source_code: String::new(),
            offset: 0,
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.function_store.clear();
        builtin::load_builtin(&mut self.function_store);
        self.source_code.clear();
        self.offset = 0;
        self.type_names.clear();
        self.type_names.push(crate::DYN_KEYWORD.to_string());
        self.type_names.push("none".to_string());
        self.type_names.push("number".to_string());
        self.type_names.push("string".to_string());
        self.type_names.push("bool".to_string());
    }

    #[inline]
    pub fn get_typename(&self, typeid: usize) -> String {
        if let Some(typename) = self.type_names.get(typeid) {
            typename.clone()
        } else {
            String::new()
        }
    }

    #[inline]
    pub fn add_function(&mut self, name: String, value: Function) {
        self.function_store.insert(name, value);
    }

    #[inline]
    pub fn append_source_code(&mut self, input: &str) {
        if !self.source_code.is_empty() {
            self.offset = self.source_code.len();
        }
        if input.ends_with('\n') {
            self.source_code.push_str(input);
        } else {
            self.source_code.push_str(format!("{}\n", input).as_str());
        }
    }

    #[inline]
    pub fn get_source_code(&self) -> &str {
        &self.source_code
    }

    #[inline]
    pub fn get_source_code_offset(&self) -> usize {
        self.offset
    }

    #[inline]
    pub fn get_source_code_len(&self) -> usize {
        self.source_code.len()
    }
}

#[derive(Debug, Clone)]
pub struct BacktraceItem {
    pub name: String,
    pub line: usize,
    pub col: usize,
}

impl BacktraceItem {
    #[inline]
    pub fn new(name: &str, line_col: (usize, usize)) -> Self {
        let (line, col) = line_col;
        BacktraceItem {
            name: name.to_string(),
            line,
            col,
        }
    }
}

// Contains data that can be modified even after
// parsing, during execution

#[derive(Debug, Clone)]
pub struct ExecSession {
    #[cfg(test)]
    pub output_stream: String,

    global_namespace: HashMap<String, Variable>,
    local_namespaces: Vec<HashMap<String, Variable>>,
    call_count: usize,
    backtrace: Vec<BacktraceItem>,
}

impl ExecSession {
    #[inline]
    pub fn new() -> Self {
        ExecSession {
            #[cfg(test)]
            output_stream: String::new(),

            global_namespace: HashMap::new(),
            local_namespaces: Vec::new(),
            call_count: 0,
            backtrace: Vec::new(),
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        #[cfg(test)]
        self.output_stream.clear();

        self.global_namespace.clear();
        self.local_namespaces.clear();
        self.call_count = 0;
        self.backtrace.clear();
    }

    #[inline]
    pub fn get_variable(&self, var_name: &str) -> Option<&Variable> {
        let len = self.local_namespaces.len();
        if len > 0 {
            self.local_namespaces[len - 1].get(var_name)
        } else {
            self.global_namespace.get(var_name)
        }
    }

    #[inline]
    pub fn get_variable_mut(&mut self, var_name: &str) -> Option<&mut Variable> {
        let len = self.local_namespaces.len();
        if len > 0 {
            self.local_namespaces[len - 1].get_mut(var_name)
        } else {
            self.global_namespace.get_mut(var_name)
        }
    }

    #[inline]
    pub fn add_variable(&mut self, var_name: &str, var: Variable) {
        let len = self.local_namespaces.len();
        if len > 0 {
            self.local_namespaces[len - 1].insert(var_name.to_string(), var);
        } else {
            self.global_namespace.insert(var_name.to_string(), var);
        }
    }

    #[inline]
    pub fn variable_exists(&self, var_name: &str) -> bool {
        let len = self.local_namespaces.len();
        if len > 0 {
            self.local_namespaces[len - 1].contains_key(var_name)
        } else {
            self.global_namespace.contains_key(var_name)
        }
    }

    #[inline]
    pub fn get_backtrace(&self) -> &Vec<BacktraceItem> {
        &self.backtrace
    }

    #[inline]
    pub fn clear_backtrace(&mut self) {
        self.backtrace.clear();
    }

    #[inline]
    fn add_namespace(&mut self, namespace: HashMap<String, Variable>) {
        self.local_namespaces.push(namespace);
    }

    #[inline]
    fn pop_namespace(&mut self) {
        self.local_namespaces.pop();
    }

    #[inline]
    fn increment_call_count(&mut self) -> Result<(), Error> {
        if self.call_count >= crate::FUNCTION_CALL_LIMIT {
            Err(Error::new(
                Context { start: 0, end: 0 },
                0,
                ErrorKind::IterationLimitReached,
            ))
        } else {
            self.call_count += 1;
            Ok(())
        }
    }

    #[inline]
    fn decrement_call_count(&mut self) {
        self.call_count -= 1;
    }
}

pub fn call_function(
    parsed_session: &ParsedSession,
    exec_session: &mut ExecSession,
    fncall: &FunctionCall,
    context: Context,
    pos: usize,
) -> instruction::Result {
    let function = match parsed_session.function_store.get(fncall.name()) {
        Some(f) => f,
        None => {
            return Err(Error::new(context, pos, ErrorKind::IdentifierNotFound));
        }
    };

    if function.arguments().len() != fncall.arguments().len() {
        return Err(Error::new(
            context,
            pos,
            ErrorKind::InvalidNumberOfArguments,
        ));
    }

    let mut fn_namespace: HashMap<String, Variable> = HashMap::new();
    for i in 0..function.arguments().len() {
        let expr = &fncall.arguments()[i];
        let value = expr
            .exec(exec_session, parsed_session)?
            .expect("Expressions should always return a value on success");

        let result;
        let expected_type_id = function.arguments()[i].typeid();

        if expected_type_id != 0 {
            if value.typeid() != expected_type_id {
                let pos = expr.context().start - fncall.context().start;
                let typename = parsed_session.get_typename(value.typeid());
                let expected_type_name = parsed_session.get_typename(expected_type_id);
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

        fn_namespace.insert(function.arguments()[i].name().to_string(), result);
    }

    exec_session.increment_call_count()?;
    exec_session.add_namespace(fn_namespace);

    let result = function.exec(exec_session, parsed_session);
    if let Err(_) = result {
        let backtrace_item = BacktraceItem::new(fncall.name(), fncall.get_line_col());
        exec_session.backtrace.push(backtrace_item)
    }

    exec_session.pop_namespace();
    exec_session.decrement_call_count();
    return result;
}
