use crate::types::{Class, Function, Scope, Type};
use std::fmt::{self, Debug};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GradiaError {
    #[error("Runtime Error! {0}")]
    Runtime(String),

    #[error("Function Error! the passed arguments length {0} is different to expected length {1} of the function's arguments")]
    Function(usize, usize),

    #[error("Type Error! the result value `{0:?}` is different to expected type `{1}`")]
    Type(Type, String),

    #[error("Syntax Error! {0}")]
    Syntax(String),
}

#[derive(Clone)]
pub struct Expr {
    pub expr: Type,
    pub annotate: Option<Class>,
}

impl Expr {
    pub fn eval(&self, scope: &mut Scope) -> Result<Type, GradiaError> {
        let result = if let Type::Expr(expr) = &self.expr {
            // Prepare expression
            let expr = {
                let mut new = vec![];
                for i in expr {
                    new.push(i.eval(scope)?)
                }
                new
            };

            if let Some(Type::Function(Function::BuiltIn(func))) = expr.get(0).cloned() {
                func(expr[1..].to_vec(), scope)?
            } else if let Some(Type::Function(Function::UserDefined(args, code))) =
                expr.get(0).cloned()
            {
                // Check arguments length
                if args.len() != expr.get(1..).unwrap_or_default().len() {
                    return Err(GradiaError::Function(
                        expr.get(1..).unwrap_or_default().len(),
                        args.len(),
                    ));
                }

                // Setting arguemnt and its value
                let mut func_scope = scope.clone();
                for (k, v) in args.iter().zip(expr.get(1..).unwrap_or_default().to_vec()) {
                    if let Some(annotate) = k.annotate.clone() {
                        // Type check between arguments and expects
                        if annotate.get_type() == v.get_type() {
                            // Setting argument by passed value
                            func_scope.insert(k.expr.get_string(), v);
                        } else {
                            return Err(GradiaError::Type(v, annotate.get_type()));
                        }
                    } else {
                        // Setting argument by passed value
                        func_scope.insert(k.expr.get_string(), v);
                    }
                }

                // Execution of function's code
                let mut result = Type::Null;
                for line in code {
                    result = Expr {
                        // Convert list to as expression
                        expr: if let Type::List(expr) = line.to_owned() {
                            Type::Expr(expr)
                        } else {
                            line.to_owned()
                        },
                        annotate: None,
                    }
                    .eval(&mut func_scope)?
                }
                result
            } else {
                return Err(GradiaError::Syntax(format!(
                    "first atom in expression should be function, but provided `{:?}` is not function",
                    expr.get(0).cloned().unwrap_or_default()
                )));
            }
        } else {
            let expr = self.expr.clone();
            if let Type::Symbol(name) = expr.clone() {
                // Loading variable from scope
                if let Some(value) = scope.get(&name).to_owned() {
                    value.to_owned()
                } else {
                    expr
                }
            } else {
                expr
            }
        };

        // Type check between result value and except type
        if let Some(annotate) = self.annotate.clone() {
            if &result.get_type() == &annotate.get_type() {
                Ok(result)
            } else {
                return Err(GradiaError::Type(result, annotate.get_type()));
            }
        } else {
            Ok(result)
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(annotate) = self.annotate.clone() {
            write!(f, "{:?}:{}", self.expr, annotate.get_type())
        } else {
            write!(f, "{:?}", self.expr)
        }
    }
}

impl Default for Expr {
    fn default() -> Self {
        Expr {
            expr: Type::Null,
            annotate: None,
        }
    }
}
