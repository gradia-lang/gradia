use std::{
    collections::HashMap,
    io::{self, Write},
    mem::discriminant,
    process::exit,
};
fn main() {
    let scope = HashMap::from([
        (
            "+".to_string(),
            Type::Function(|params| {
                Some(Type::Number(
                    params.get(0)?.get_number() + params.get(1)?.get_number(),
                ))
            }),
        ),
        (
            "-".to_string(),
            Type::Function(|params| {
                Some(Type::Number(
                    params.get(0)?.get_number() - params.get(1)?.get_number(),
                ))
            }),
        ),
        (
            "*".to_string(),
            Type::Function(|params| {
                Some(Type::Number(
                    params.get(0)?.get_number() * params.get(1)?.get_number(),
                ))
            }),
        ),
        (
            "/".to_string(),
            Type::Function(|params| {
                Some(Type::Number(
                    params.get(0)?.get_number() / params.get(1)?.get_number(),
                ))
            }),
        ),
        (
            "concat".to_string(),
            Type::Function(|params| {
                Some(Type::String(
                    params.get(0)?.get_string() + &params.get(1)?.get_string(),
                ))
            }),
        ),
        (
            "print".to_string(),
            Type::Function(|params| {
                println!("{}", params.get(0)?.get_string());
                Some(Type::Null)
            }),
        ),
        (
            "&".to_string(),
            Type::Function(|params| {
                Some(Type::Bool(
                    params.get(0)?.get_bool() & params.get(1)?.get_bool(),
                ))
            }),
        ),
        (
            "|".to_string(),
            Type::Function(|params| {
                Some(Type::Bool(
                    params.get(0)?.get_bool() | params.get(1)?.get_bool(),
                ))
            }),
        ),
        (
            "!".to_string(),
            Type::Function(|params| Some(Type::Bool(!params.get(0)?.get_bool()))),
        ),
        ("exit".to_string(), Type::Function(|_| exit(0))),
    ]);

    println!("Statia");
    loop {
        let program = parse_expr(input("> "));
        if let Some(result) = program.eval(scope.clone()) {
            println!("{result:?}",);
        }
    }
}

/// Get standard input from user
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    result.trim().to_string()
}

fn parse_expr(source: String) -> Expr {
    let tokens = tokenize_expr(source);
    let mut expr: Vec<Expr> = vec![];
    for token in tokens {
        let annotate = if token.len() == 2 {
            match token[1].as_str() {
                "function" => Some(Type::Function(|x| Some(x[0].clone()))),
                "string" => Some(Type::String(String::new())),
                "number" => Some(Type::Number(0.0)),
                "bool" => Some(Type::Bool(false)),
                _ => None,
            }
        } else {
            None
        };

        if let Ok(n) = token[0].parse::<f64>() {
            expr.push(Expr {
                expr: Type::Number(n),
                annotate,
            });
        } else if let Ok(b) = token[0].parse::<bool>() {
            expr.push(Expr {
                expr: Type::Bool(b),
                annotate,
            });
        } else if token[0].starts_with('"') && token[0].ends_with('"') {
            let mut string = token[0].clone();
            string.remove(0);
            string.remove(string.len() - 1);
            expr.push(Expr {
                expr: Type::String(string),
                annotate,
            });
        } else if token[0].starts_with('(') && token[0].ends_with(')') {
            let mut string = token[0].clone();
            string.remove(0);
            string.remove(string.len() - 1);
            expr.push(Expr {
                expr: parse_expr(string).expr,
                annotate,
            });
        } else {
            expr.push(Expr {
                expr: Type::Variable(token[0].clone()),
                annotate,
            });
        }
    }

    if expr.len() == 1 {
        expr[0].clone()
    } else {
        Expr {
            expr: Type::Expr(expr),
            annotate: None,
        }
    }
}

fn tokenize_expr(input: String) -> Vec<Vec<String>> {
    let mut tokens: Vec<Vec<String>> = Vec::new();
    let mut current_token = String::new();
    let mut after_colon = String::new();
    let mut is_colon = false;
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '(' if !in_quote => {
                if is_colon {
                    after_colon.push(c);
                } else {
                    current_token.push(c);
                }
                in_parentheses += 1;
            }
            ')' if !in_quote => {
                if is_colon {
                    after_colon.push(c);
                } else {
                    current_token.push(c);
                }
                in_parentheses -= 1;
            }
            ' ' if !in_quote => {
                if in_parentheses != 0 {
                    if is_colon {
                        after_colon.push(c);
                    } else {
                        current_token.push(c);
                    }
                } else if !current_token.is_empty() {
                    if is_colon {
                        is_colon = false;
                        tokens.push(vec![current_token.clone(), after_colon.clone()]);
                        current_token.clear();
                        after_colon.clear();
                    } else {
                        tokens.push(vec![current_token.clone()]);
                        current_token.clear();
                    }
                }
            }
            ':' if !in_quote => {
                if in_parentheses != 0 {
                    if is_colon {
                        after_colon.push(c);
                    } else {
                        current_token.push(c);
                    }
                } else {
                    is_colon = true;
                }
            }
            '"' => {
                in_quote = !in_quote;
                if is_colon {
                    after_colon.push(c);
                } else {
                    current_token.push(c);
                }
            }
            _ => {
                if is_colon {
                    after_colon.push(c);
                } else {
                    current_token.push(c);
                }
            }
        }
    }

    if in_parentheses == 0 && !current_token.is_empty() {
        if is_colon {
            tokens.push(vec![current_token.clone(), after_colon]);
            current_token.clear();
        } else {
            tokens.push(vec![current_token.clone()]);
            current_token.clear();
        }
    }
    tokens
}

#[derive(Clone, Debug)]
struct Expr {
    expr: Type,
    annotate: Option<Type>,
}

impl Expr {
    fn eval(&self, scope: HashMap<String, Type>) -> Option<Type> {
        let result = if let Type::Expr(expr) = &self.expr {
            let expr = {
                let mut new = vec![];
                for i in expr {
                    let temp = i.eval(scope.clone())?;
                    new.push(if let Type::Variable(name) = temp {
                        scope.get(&name)?.to_owned()
                    } else {
                        temp
                    });
                }
                new
            };
            if let Type::Function(func) = expr.get(0)? {
                func(expr.get(1..)?.to_vec())?
            } else {
                return None;
            }
        } else {
            self.expr.clone()
        };

        // Type check between except type and annotate value
        if let Some(annotate) = self.annotate.clone() {
            if discriminant(&result) == discriminant(&annotate) {
                Some(result)
            } else {
                None
            }
        } else {
            Some(result)
        }
    }
}

#[derive(Clone, Debug)]
enum Type {
    Function(fn(Vec<Type>) -> Option<Type>),
    Expr(Vec<Expr>),
    Variable(String),
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

impl Type {
    fn get_number(&self) -> f64 {
        match &self {
            Type::Number(n) => n.to_owned(),
            Type::String(s) => s.trim().parse().unwrap_or(0.0),
            Type::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            Type::Expr(x) => x.len() as f64,
            Type::Function(_) | Type::Null => 0.0,
            Type::Variable(v) => v.len() as f64,
        }
    }

    fn get_string(&self) -> String {
        match &self {
            Type::Number(n) => n.to_string(),
            Type::String(s) => s.to_owned(),
            Type::Bool(b) => b.to_string(),
            Type::Function(_) | Type::Expr(_) | Type::Null => String::new(),
            Type::Variable(v) => v.to_owned(),
        }
    }

    fn get_bool(&self) -> bool {
        match &self {
            Type::Number(n) => *n != 0.0,
            Type::String(s) | Type::Variable(s) => !s.is_empty(),
            Type::Expr(s) => !s.is_empty(),
            Type::Bool(b) => *b,
            Type::Function(_) | Type::Null => false,
        }
    }
}
