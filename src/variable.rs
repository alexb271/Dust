use std::fmt;

#[derive(Debug, Clone)]
pub struct Variable {
    value: Value,
    is_dynamic: bool,
}

impl Variable {
    #[inline]
    pub fn new(value: Value, is_dynamic: bool) -> Self {
        Variable { value, is_dynamic }
    }

    #[inline]
    pub fn set_value(&mut self, value: Value) {
        self.value = value;
    }

    #[inline]
    pub fn get_value(&self) -> &Value {
        &self.value
    }

    #[inline]
    pub fn get_value_mut(&mut self) -> &mut Value {
        &mut self.value
    }

    #[inline]
    pub fn is_dynamic(&self) -> bool {
        self.is_dynamic
    }

    #[inline]
    pub fn typeid(&self) -> usize {
        self.value.typeid()
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    None,
    Number(f64),
    Str(Box<String>),
    Bool(bool),
    // TODO User Defined Types
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(x) => write!(f, "{}", format_float(*x)),
            Value::Str(x) => write!(f, "{x}"),
            Value::Bool(x) => {
                if *x {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Value::None => {
                write!(f, "none")
            }
        }
    }
}

#[inline]
fn format_float(number: f64) -> String {
    let mut string = format!("{:.6}", number);
    while string.ends_with('0') {
        string.pop();
    }
    if string.ends_with('.') {
        string.pop();
    }
    string
}

impl Value {
    #[inline]
    pub fn typeid(&self) -> usize {
        match self {
            Value::None => crate::TYPEID_NONE,
            Value::Number(_) => crate::TYPEID_NUMBER,
            Value::Str(_) => crate::TYPEID_STRING,
            Value::Bool(_) => crate::TYPEID_BOOL,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OptionallyAnnotatedIdentifier {
    name: String,
    typeid: Option<usize>,
}

impl OptionallyAnnotatedIdentifier {
    #[inline]
    pub fn new(name: String, typeid: Option<usize>) -> Self {
        OptionallyAnnotatedIdentifier { name, typeid }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn typeid(&self) -> Option<usize> {
        self.typeid
    }
}

#[derive(Debug, Clone)]
pub struct AnnotatedIdentifier {
    name: String,
    typeid: usize,
}

impl AnnotatedIdentifier {
    #[inline]
    pub fn new(name: String, typeid: usize) -> Self {
        AnnotatedIdentifier { name, typeid }
    }

    pub fn from_optional(input: OptionallyAnnotatedIdentifier) -> Option<Self> {
        if let Some(typeid) = input.typeid {
            Some(AnnotatedIdentifier {
                name: input.name,
                typeid,
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn typeid(&self) -> usize {
        self.typeid
    }
}
