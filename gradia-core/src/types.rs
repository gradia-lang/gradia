use crate::expr::{Expr, GradiaError};
use crate::fraction::Fraction;
use std::collections::HashMap;
use std::fmt::{self, Debug};

pub type Scope = HashMap<String, Type>;

#[derive(Clone)]
pub enum Type {
    Function(Function),
    Expr(Vec<Expr>),
    List(Vec<Expr>),
    Symbol(String),
    Number(Fraction),
    String(String),
    Bool(bool),
    Null,
}

#[derive(Clone, Debug)]
pub enum Function {
    BuiltIn(fn(Vec<Type>, &mut Scope) -> Result<Type, GradiaError>),
    UserDefined(Vec<Expr>, Vec<Type>),
}

impl Type {
    pub fn get_number(&self) -> Fraction {
        match &self {
            Type::Number(n) => n.to_owned(),
            Type::String(s) | Type::Symbol(s) => {
                Fraction::from(s.to_string()).unwrap_or(Fraction::new(0.0))
            }
            Type::Bool(b) => {
                if *b {
                    Fraction::new(1.0)
                } else {
                    Fraction::new(0.0)
                }
            }
            Type::Expr(x) | Type::List(x) => {
                x.get(0).cloned().unwrap_or_default().expr.get_number()
            }
            Type::Function(_) | Type::Null => Fraction::new(0.0),
        }
    }

    pub fn get_string(&self) -> String {
        match &self {
            Type::Number(n) => n.display(),
            Type::String(s) => s.to_owned(),
            Type::Bool(b) => b.to_string(),
            Type::Symbol(v) => v.to_owned(),
            other => format!("{other:?}"),
        }
    }

    pub fn get_bool(&self) -> bool {
        match &self {
            Type::Number(n) => *n != Fraction::new(0.0),
            Type::String(s) | Type::Symbol(s) => !s.is_empty(),
            Type::Expr(s) | Type::List(s) => !s.is_empty(),
            Type::Bool(b) => *b,
            Type::Function(_) | Type::Null => false,
        }
    }

    pub fn get_type(&self) -> String {
        match &self {
            Type::Number(_) => "number".to_string(),
            Type::String(_) => "string".to_string(),
            Type::Bool(_) => "bool".to_string(),
            Type::Expr(_) => "expr".to_string(),
            Type::Symbol(_) => "symbol".to_string(),
            Type::List(_) => "list".to_string(),
            Type::Null => "null".to_string(),
            Type::Function(_) => "function".to_string(),
        }
    }

    pub fn get_list(&self) -> Vec<Expr> {
        match &self {
            Type::Expr(e) => e.to_owned(),
            Type::List(l) => l.to_owned(),
            other => vec![Expr {
                expr: other.to_owned().to_owned(),
                annotate: None,
            }],
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt = match &self {
            Type::String(s) => format!("\"{s}\""),
            Type::Number(n) => n.display(),
            Type::Bool(b) => b.to_string(),
            Type::Function(Function::UserDefined(args, code)) => {
                format!(
                    "(lambda '({}) {})",
                    args.iter()
                        .map(|i| format!("{i:?}"))
                        .collect::<Vec<String>>()
                        .join(" "),
                    code.iter()
                        .map(|i| format!("{i:?}"))
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            Type::Function(Function::BuiltIn(n)) => format!("function({n:?})"),
            Type::Symbol(v) => v.to_owned(),
            Type::List(l) => format!(
                "'({})",
                l.iter()
                    .map(|x| format!("{x:?}"))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Type::Expr(l) => format!(
                "({})",
                l.iter()
                    .map(|x| format!("{x:?}"))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Type::Null => "null".to_string(),
        };
        write!(f, "{fmt}")
    }
}

impl Default for Type {
    fn default() -> Self {
        Type::Null
    }
}
