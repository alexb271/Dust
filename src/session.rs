use crate::builtin::{self, DYN_KEYWORD};
use crate::class::ClassDefinition;
use crate::error::{Context, Error, ErrorKind};
use crate::function::Function;
use crate::variable::Variable;
use std::collections::HashMap;

// Two structs are needed in order to be able to
// assign different mutability modifiers to them

#[derive(Debug, Clone)]
pub struct Session {
    pub exec_session: ExecSession,
    pub parse_session: ParseSession,
}

impl Session {
    #[inline]
    pub fn new() -> Self {
        Session {
            exec_session: ExecSession::new(),
            parse_session: ParseSession::new(),
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.exec_session.clear();
        self.parse_session.clear();
    }

    #[inline]
    pub fn process(&mut self, input: &str) {
        crate::process(input, self)
    }
}

// Contains data that cannot be modified after parsing
// such as function definitions

#[derive(Debug, Clone)]
pub struct ParseSession {
    function_store: HashMap<String, Function>,
    class_definitions: Vec<ClassDefinition>,
    default_type_names: [&'static str; 11],
    type_names: Vec<String>,
    source_code: String,
    offset: usize,
}

impl ParseSession {
    #[inline]
    pub fn new() -> Self {
        let mut function_store = HashMap::new();
        builtin::load_builtin_functions(&mut function_store);
        let mut class_definitions = Vec::new();
        builtin::load_builtin_class_definitions(&mut class_definitions);
        let type_names = Vec::new();

        ParseSession {
            function_store,
            class_definitions,
            default_type_names: [
                DYN_KEYWORD,
                "none",
                "int",
                "float",
                "string",
                "bool",
                "Vec",
                "Result",
                "File",
                "Fs",
                "Math",
            ],
            type_names,
            source_code: String::new(),
            offset: 0,
        }
    }

    const CLASSLESS_TYPES_COUNT: usize = 2; // dyn and none

    #[inline]
    pub fn clear(&mut self) {
        self.function_store.clear();
        builtin::load_builtin_functions(&mut self.function_store);
        self.class_definitions.clear();
        builtin::load_builtin_class_definitions(&mut self.class_definitions);
        self.type_names.clear();
        self.source_code.clear();
        self.offset = 0;
    }

    #[inline]
    pub fn add_typename(&mut self, typename: &str) {
        self.type_names.push(typename.to_string());
    }

    #[inline]
    pub fn get_typename(&self, typeid: usize) -> String {
        if typeid < self.default_type_names.len() {
            self.default_type_names[typeid].to_string()
        } else {
            self.type_names[typeid - self.default_type_names.len()].clone()
        }
    }

    #[inline]
    pub fn create_typemap(&self) -> HashMap<String, usize> {
        let mut result: HashMap<String, usize> = HashMap::new();
        let mut typeid: usize = 0;
        for item in self.default_type_names {
            result.insert(item.to_string(), typeid);
            typeid += 1;
        }
        for item in &self.type_names {
            result.insert(item.clone(), typeid);
            typeid += 1;
        }
        result
    }

    #[inline]
    pub fn add_function(&mut self, name: String, value: Function) -> bool {
        if self.function_store.contains_key(&name) {
            false
        } else {
            self.function_store.insert(name, value);
            true
        }
    }

    #[inline]
    pub fn get_function(
        &self,
        name: &str,
        query_otpions: Option<FnQueryOptions>,
    ) -> Result<&Function, ErrorKind> {
        match query_otpions {
            Some(options) => {
                if options.associated_typeid >= Self::CLASSLESS_TYPES_COUNT {
                    self.class_definitions[options.associated_typeid - Self::CLASSLESS_TYPES_COUNT]
                        .get_function(name, options.member_only, options.private_access)
                } else {
                    Err(ErrorKind::FunctionNotFound)
                }
            }
            None => match self.function_store.get(name) {
                Some(f) => Ok(f),
                None => Err(ErrorKind::FunctionNotFound),
            },
        }
    }

    #[inline]
    pub fn get_next_typeid(&self) -> usize {
        self.default_type_names.len() + self.type_names.len()
    }

    #[inline]
    pub fn add_class_definition(&mut self, class_name: String, class_definition: ClassDefinition) {
        if class_definition.typeid() != self.get_next_typeid() {
            panic!("Attempting to add class definition with invalid typeid");
        }
        self.type_names.push(class_name);
        self.class_definitions.push(class_definition);
    }

    #[inline]
    pub fn get_class_definition(&self, typeid: usize) -> Option<&ClassDefinition> {
        self.class_definitions
            .get(typeid - Self::CLASSLESS_TYPES_COUNT)
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

// Contains data that can be modified even after
// parsing, during execution

#[derive(Debug, Clone)]
pub struct ExecSession {
    #[cfg(test)]
    pub output_stream: String,

    global_scope: HashMap<String, Variable>,
    local_scopes: Vec<HashMap<String, Variable>>,
    call_count: usize,
    backtrace: Vec<BacktraceItem>,
}

impl ExecSession {
    #[inline]
    pub fn new() -> Self {
        ExecSession {
            #[cfg(test)]
            output_stream: String::new(),

            global_scope: HashMap::new(),
            local_scopes: Vec::new(),
            call_count: 0,
            backtrace: Vec::new(),
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        #[cfg(test)]
        self.output_stream.clear();

        self.global_scope.clear();
        self.local_scopes.clear();
        self.call_count = 0;
        self.backtrace.clear();
    }

    #[inline]
    pub fn get_variable(&self, var_name: &str) -> Option<&Variable> {
        let len = self.local_scopes.len();
        if len > 0 {
            self.local_scopes[len - 1].get(var_name)
        } else {
            self.global_scope.get(var_name)
        }
    }

    #[inline]
    pub fn get_variable_mut(&mut self, var_name: &str) -> Option<&mut Variable> {
        let len = self.local_scopes.len();
        if len > 0 {
            self.local_scopes[len - 1].get_mut(var_name)
        } else {
            self.global_scope.get_mut(var_name)
        }
    }

    #[inline]
    pub fn add_variable(&mut self, var_name: &str, var: Variable) {
        let len = self.local_scopes.len();
        if len > 0 {
            self.local_scopes[len - 1].insert(var_name.to_string(), var);
        } else {
            self.global_scope.insert(var_name.to_string(), var);
        }
    }

    #[inline]
    pub fn variable_exists(&self, var_name: &str) -> bool {
        let len = self.local_scopes.len();
        if len > 0 {
            self.local_scopes[len - 1].contains_key(var_name)
        } else {
            self.global_scope.contains_key(var_name)
        }
    }

    #[inline]
    pub fn add_backtrace(&mut self, backtrace_item: BacktraceItem) {
        self.backtrace.push(backtrace_item)
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
    pub fn add_scope(&mut self, scope: HashMap<String, Variable>) {
        self.local_scopes.push(scope);
    }

    #[inline]
    pub fn pop_scope(&mut self) {
        self.local_scopes.pop();
    }

    #[inline]
    pub fn increment_call_count(&mut self) -> Result<(), Error> {
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
    pub fn decrement_call_count(&mut self) {
        self.call_count -= 1;
    }
}

#[derive(Debug, Clone)]
pub struct FnQueryOptions {
    associated_typeid: usize,
    member_only: bool,
    private_access: bool,
}

impl FnQueryOptions {
    #[inline]
    pub fn new(
        associated_typeid: usize,
        member_only: bool,
        private_access: bool,
    ) -> FnQueryOptions {
        FnQueryOptions {
            associated_typeid,
            member_only,
            private_access,
        }
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
