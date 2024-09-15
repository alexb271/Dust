use crate::builtin::{
    TYPEID_BOOL, TYPEID_FLOAT, TYPEID_INT, TYPEID_NONE, TYPEID_STRING, TYPEID_VEC,
};
use crate::class::ClassInstance;
use crate::session::ParseSession;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

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
    pub fn get_value_clone(&self) -> Value {
        self.value.clone()
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

pub type StringValue = Rc<RefCell<String>>;
pub type ClassValue = Rc<RefCell<ClassInstance>>;
pub type VecValue = Rc<RefCell<Vec<Value>>>;

#[derive(Debug, Clone)]
pub enum Value {
    None,
    Int(isize),
    Float(f64),
    Str(StringValue),
    Bool(bool),
    Class(ClassValue),
    Vector(VecValue),
}

impl Value {
    #[inline]
    pub fn default(typeid: usize) -> Option<Value> {
        match typeid {
            TYPEID_NONE => Some(Value::None),
            TYPEID_INT => Some(Value::Int(0)),
            TYPEID_FLOAT => Some(Value::Float(0.0)),
            TYPEID_STRING => Some(Value::new_string("".to_string())),
            TYPEID_BOOL => Some(Value::Bool(false)),
            _ => None,
        }
    }

    #[inline]
    pub fn new_string(value: String) -> Value {
        Value::Str(Rc::new(RefCell::new(value)))
    }

    #[inline]
    pub fn new_class_instance(value: ClassInstance) -> Value {
        Value::Class(Rc::new(RefCell::new(value)))
    }

    #[inline]
    pub fn new_vec_instance() -> Value {
        Value::Vector(Rc::new(RefCell::new(Vec::new())))
    }

    #[inline]
    pub fn new_vec_instance_from(vec: Vec<Value>) -> Value {
        Value::Vector(Rc::new(RefCell::new(vec)))
    }

    #[inline]
    pub fn to_string(&self, parse_session: &ParseSession) -> String {
        match self {
            Value::None => "none".to_string(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => format_float(*f),
            Value::Str(s) => s.borrow().clone(),
            Value::Bool(b) => match b {
                true => "true".to_string(),
                false => "false".to_string(),
            },
            Value::Class(c) => parse_session.get_typename(c.borrow().typeid()),
            Value::Vector(l) => {
                let mut result = String::from("(");
                let vec: &Vec<Value> = &l.borrow();
                for item in vec {
                    match item {
                        Value::Vector(_) => {
                            result.push_str("[...]");
                        }
                        Value::Str(_) => {
                            result.push_str("\"");
                            result.push_str(&item.to_string(parse_session));
                            result.push_str("\"");
                        }
                        _ => {
                            result.push_str(&item.to_string(parse_session));
                        }
                    }
                    result.push_str(", ");
                }
                if !vec.is_empty() {
                    result.pop();
                    result.pop();
                }
                result.push_str(")");
                result
            }
        }
    }

    #[inline]
    pub fn typeid(&self) -> usize {
        match self {
            Value::None => TYPEID_NONE,
            Value::Int(_) => TYPEID_INT,
            Value::Float(_) => TYPEID_FLOAT,
            Value::Str(_) => TYPEID_STRING,
            Value::Bool(_) => TYPEID_BOOL,
            Value::Class(c) => c.borrow().typeid(),
            Value::Vector(_) => TYPEID_VEC,
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

impl fmt::Display for AnnotatedIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.typeid)
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
