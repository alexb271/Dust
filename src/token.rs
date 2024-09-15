use crate::function::FunctionCall;
use crate::variable::Value;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Pow,
    Mod,
    Neg,
    And,
    Or,
    Not,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    Typeof,
    Dot,
}

impl Operator {
    #[inline]
    pub fn precedence(&self) -> i8 {
        match self {
            Operator::Add => 5,
            Operator::Sub => 5,
            Operator::Mult => 6,
            Operator::Div => 6,
            Operator::Mod => 6,
            Operator::Pow => 7,
            Operator::Neg => 7,
            Operator::And => 2,
            Operator::Or => 1,
            Operator::Not => 6,
            Operator::LessThan => 4,
            Operator::GreaterThan => 4,
            Operator::Equal => 3,
            Operator::NotEqual => 3,
            Operator::Typeof => 3,
            Operator::Dot => 8,
        }
    }

    #[inline]
    pub fn is_left_associative(&self) -> bool {
        match self {
            Operator::Add => true,
            Operator::Sub => true,
            Operator::Mult => true,
            Operator::Div => true,
            Operator::Mod => true,
            Operator::Pow => true,
            Operator::And => true,
            Operator::Or => true,
            Operator::LessThan => true,
            Operator::GreaterThan => true,
            Operator::Equal => true,
            Operator::NotEqual => true,
            Operator::Dot => true,

            Operator::Neg => false,
            Operator::Not => false,
            Operator::Typeof => false,
        }
    }

    #[inline]
    pub fn is_unary(&self) -> bool {
        match self {
            Operator::Neg => true,
            Operator::Not => true,
            Operator::Typeof => true,

            _ => false,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mult => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Mod => write!(f, "%"),
            Operator::Pow => write!(f, "^"),
            Operator::Neg => write!(f, "-"),
            Operator::And => write!(f, "and"),
            Operator::Or => write!(f, "or"),
            Operator::Not => write!(f, "not"),
            Operator::LessThan => write!(f, "<"),
            Operator::GreaterThan => write!(f, ">"),
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::Typeof => write!(f, "typeof"),
            Operator::Dot => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Parenthesis {
    Left,
    Right,
}

impl fmt::Display for Parenthesis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Parenthesis::Left => write!(f, "("),
            Parenthesis::Right => write!(f, ")"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Value(Value),
    Identifier(Box<String>),
    FunctionCall(Box<FunctionCall>),
    Operator(Operator),
    Parenthesis(Parenthesis),
}

#[derive(Debug, Clone)]
pub struct Token {
    pos: usize,
    kind: TokenKind,
}

impl Token {
    #[inline]
    pub fn new_int(pos: usize, value: isize) -> Token {
        Token {
            pos,
            kind: TokenKind::Value(Value::Int(value)),
        }
    }

    #[inline]
    pub fn new_float(pos: usize, value: f64) -> Token {
        Token {
            pos,
            kind: TokenKind::Value(Value::Float(value)),
        }
    }

    #[inline]
    pub fn new_str(pos: usize, value: String) -> Token {
        Token {
            pos,
            kind: TokenKind::Value(Value::new_string(value)),
        }
    }

    #[inline]
    pub fn new_bool(pos: usize, value: bool) -> Token {
        Token {
            pos,
            kind: TokenKind::Value(Value::Bool(value)),
        }
    }

    #[inline]
    pub fn new_none(pos: usize) -> Token {
        Token {
            pos,
            kind: TokenKind::Value(Value::None),
        }
    }

    #[inline]
    pub fn new_identifier(pos: usize, value: String) -> Token {
        Token {
            pos,
            kind: TokenKind::Identifier(Box::new(value)),
        }
    }

    #[inline]
    pub fn new_function_call(pos: usize, value: FunctionCall) -> Token {
        Token {
            pos,
            kind: TokenKind::FunctionCall(Box::new(value)),
        }
    }

    #[inline]
    pub fn new_operator(pos: usize, value: Operator) -> Token {
        Token {
            pos,
            kind: TokenKind::Operator(value),
        }
    }

    #[inline]
    pub fn new_parenthesis(pos: usize, value: Parenthesis) -> Token {
        Token {
            pos,
            kind: TokenKind::Parenthesis(value),
        }
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}
