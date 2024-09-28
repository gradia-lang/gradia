use std::{
    collections::HashMap,
    fmt::{self, Debug},
    io::{self, Write},
    mem::discriminant,
    process::exit,
};
fn main() {
    let scope = &mut HashMap::from([
        (
            "+".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Number(
                    params.get(0)?.get_number() + params.get(1)?.get_number(),
                ))
            })),
        ),
        (
            "-".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Number(
                    params.get(0)?.get_number() - params.get(1)?.get_number(),
                ))
            })),
        ),
        (
            "*".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Number(
                    params.get(0)?.get_number() * params.get(1)?.get_number(),
                ))
            })),
        ),
        (
            "/".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Number(
                    params.get(0)?.get_number() / params.get(1)?.get_number(),
                ))
            })),
        ),
        (
            "concat".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::String(
                    params.get(0)?.get_string() + &params.get(1)?.get_string(),
                ))
            })),
        ),
        (
            "print".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                println!("{}", params.get(0)?.get_string());
                Some(Type::Null)
            })),
        ),
        (
            "&".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Bool(
                    params.get(0)?.get_bool() & params.get(1)?.get_bool(),
                ))
            })),
        ),
        (
            "|".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Bool(
                    params.get(0)?.get_bool() | params.get(1)?.get_bool(),
                ))
            })),
        ),
        (
            "!".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Bool(!params.get(0)?.get_bool()))
            })),
        ),
        (
            "cast".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                match params.get(1)?.get_string().as_str() {
                    "number" => Some(Type::Number(params.get(0)?.get_number())),
                    "string" => Some(Type::String(params.get(0)?.get_string())),
                    "bool" => Some(Type::Bool(params.get(0)?.get_bool())),
                    _ => Some(params.get(0)?.clone()),
                }
            })),
        ),
        (
            "type".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::String(params.get(0)?.get_type()))
            })),
        ),
        (
            "input".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::String(input(&params.get(0)?.get_string())))
            })),
        ),
        (
            "eval".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                let mut result = None;
                for expr in params {
                    result = Expr {
                        expr: Type::Expr(expr.get_list()),
                        annotate: None,
                    }
                    .eval(scope);
                }
                result
            })),
        ),
        (
            "var".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                scope.insert(params.get(0)?.get_string(), params.get(1)?.to_owned());
                Some(Type::Null)
            })),
        ),
        (
            "func".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                scope.insert(
                    params.get(0)?.get_string(),
                    Type::Function(Function::UserDefined(
                        params
                            .get(1)?
                            .get_list()
                            .iter()
                            .map(|i| i.expr.get_string())
                            .collect::<Vec<String>>(),
                        params.get(2..)?.to_vec(),
                    )),
                );
                Some(Type::Null)
            })),
        ),
        (
            "lambda".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Function(Function::UserDefined(
                    params
                        .get(0)?
                        .get_list()
                        .iter()
                        .map(|i| i.expr.get_string())
                        .collect::<Vec<String>>(),
                    params.get(1..)?.to_vec(),
                )))
            })),
        ),
        (
            "exit".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                exit(params.get(0)?.get_number() as i32)
            })),
        ),
    ]);

    println!("Statia");
    loop {
        let program = parse_expr(input("> "));
        if let Some(result) = program.eval(scope) {
            println!("{:?}", result);
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
                "function" => Some(Type::Function(Function::BuiltIn(|x, _| {
                    Some(x.get(0)?.clone())
                }))),
                "list" => Some(Type::List(vec![])),
                "symbol" => Some(Type::Symbol(String::new())),
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
        } else if token[0] == "null".to_string() {
            expr.push(Expr {
                expr: Type::Null,
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
        } else if token[0].starts_with("'(") && token[0].ends_with(')') {
            let mut string = token[0].clone();
            string.remove(0);
            expr.push(Expr {
                expr: Type::List(parse_expr(string).expr.get_list()),
                annotate,
            });
        } else {
            expr.push(Expr {
                expr: Type::Symbol(token[0].clone()),
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

#[derive(Clone)]
struct Expr {
    expr: Type,
    annotate: Option<Type>,
}

impl Expr {
    fn eval(&self, scope: &mut HashMap<String, Type>) -> Option<Type> {
        let result = if let Type::Expr(expr) = &self.expr {
            let expr = {
                let mut new = vec![];
                for i in expr {
                    let temp = i.eval(scope)?;
                    new.push(if let Type::Symbol(name) = temp.clone() {
                        if let Some(value) = scope.get(&name).to_owned() {
                            value.to_owned()
                        } else {
                            temp
                        }
                    } else {
                        temp
                    });
                }
                new
            };
            if let Type::Function(Function::BuiltIn(func)) = expr.get(0)? {
                func(expr.get(1..)?.to_vec(), scope)?
            } else if let Type::Function(Function::UserDefined(args, code)) = expr.get(0)? {
                let mut scope = scope.clone();
                for (k, v) in args.iter().zip(expr.get(1..)?.to_vec()) {
                    scope.insert(k.to_owned(), v);
                }
                let code: Vec<Expr> = code.get(0)?.get_list();
                Expr {
                    expr: Type::Expr(code),
                    annotate: self.annotate.clone(),
                }
                .eval(&mut scope)?
            } else {
                return None;
            }
        } else {
            let temp = self.expr.clone();
            if let Type::Symbol(name) = temp.clone() {
                if let Some(value) = scope.get(&name).to_owned() {
                    value.to_owned()
                } else {
                    temp
                }
            } else {
                temp
            }
        };

        // Type check between except type and annotate value
        if let Some(annotate) = self.annotate.clone() {
            if discriminant(&result) == discriminant(&annotate) {
                Some(result)
            } else {
                eprintln!(
                    "Error! the result value `{:?}` is different to expected type `{}` ",
                    result,
                    annotate.get_type()
                );
                None
            }
        } else {
            Some(result)
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

#[derive(Clone)]
enum Type {
    Function(Function),
    Expr(Vec<Expr>),
    List(Vec<Expr>),
    Symbol(String),
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

#[derive(Clone, Debug)]
enum Function {
    BuiltIn(fn(Vec<Type>, &mut HashMap<String, Type>) -> Option<Type>),
    UserDefined(Vec<String>, Vec<Type>),
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
            Type::Expr(x) | Type::List(x) => x.len() as f64,
            Type::Function(_) | Type::Null => 0.0,
            Type::Symbol(v) => v.len() as f64,
        }
    }

    fn get_string(&self) -> String {
        match &self {
            Type::Number(n) => n.to_string(),
            Type::String(s) => s.to_owned(),
            Type::Bool(b) => b.to_string(),
            Type::Function(_) | Type::Expr(_) | Type::List(_) | Type::Null => String::new(),
            Type::Symbol(v) => v.to_owned(),
        }
    }

    fn get_bool(&self) -> bool {
        match &self {
            Type::Number(n) => *n != 0.0,
            Type::String(s) | Type::Symbol(s) => !s.is_empty(),
            Type::Expr(s) | Type::List(s) => !s.is_empty(),
            Type::Bool(b) => *b,
            Type::Function(_) | Type::Null => false,
        }
    }

    fn get_type(&self) -> String {
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

    fn get_list(&self) -> Vec<Expr> {
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
            Type::Number(n) => n.to_string(),
            Type::Bool(b) => b.to_string(),
            Type::Function(Function::UserDefined(args, code)) => {
                format!("(lambda '({}) {:?})", args.join(" "), code[0])
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
        write!(f, "{fmt}",)
    }
}
