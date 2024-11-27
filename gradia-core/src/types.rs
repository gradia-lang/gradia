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

#[derive(Copy, Clone, Debug)]
pub enum Class {
    Function,
    List,
    Symbol,
    Number,
    String,
    Bool,
    Null,
}

impl Type {
    pub fn get_number(&self) -> Fraction {
        match &self {
            Type::Number(n) => n.to_owned(),
            Type::String(s) | Type::Symbol(s) => Fraction::from(s.to_string())
                .unwrap_or(Fraction::new(s.trim().parse().unwrap_or(0.0))),
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
            Type::Number(_) => "number",
            Type::String(_) => "string",
            Type::Bool(_) => "bool",
            Type::Expr(_) => "expr",
            Type::Symbol(_) => "symbol",
            Type::List(_) => "list",
            Type::Null => "null",
            Type::Function(_) => "function",
        }
        .to_string()
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

impl Class {
    pub fn from(source: String) -> Result<Option<Class>, GradiaError> {
        Ok(match source.as_str() {
            "function" => Some(Class::Function),
            "list" => Some(Class::List),
            "symbol" => Some(Class::Symbol),
            "number" => Some(Class::Number),
            "string" => Some(Class::String),
            "bool" => Some(Class::Bool),
            "null" => Some(Class::Null),
            "any" => None,
            other => {
                return Err(GradiaError::Syntax(format!(
                    "unknown type annotation `{other}`"
                )))
            }
        })
    }

    pub fn parse(&self, value: Type) -> Type {
        match self {
            Class::Symbol => Type::Symbol(value.get_string()),
            Class::Bool => Type::Bool(value.get_bool()),
            Class::Number => Type::Number(value.get_number()),
            Class::String => Type::String(value.get_string()),
            Class::List => Type::List(value.get_list()),
            Class::Null => Type::Null,
            Class::Function => Type::Function(Function::BuiltIn(|params, _| {
                if let Some(val) = params.get(0) {
                    Ok(val.clone())
                } else {
                    Err(GradiaError::Function(params.len(), 1))
                }
            })),
        }
    }

    pub fn get_type(&self) -> String {
        format!("{self:?}").to_lowercase()
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
