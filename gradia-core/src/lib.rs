use std::{
    collections::HashMap,
    fmt::{self, Debug},
    io::{self, Write},
    process::exit,
};

pub fn builtin_function() -> HashMap<String, Type> {
    HashMap::from([
        (
            "+".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = *params.get(0)?;
                for i in params[1..params.len()].to_vec().iter() {
                    result += i;
                }
                Some(Type::Number(result))
            })),
        ),
        (
            "-".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = *params.get(0)?;
                for i in params[1..params.len()].to_vec().iter() {
                    result -= i;
                }
                Some(Type::Number(result))
            })),
        ),
        (
            "*".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = *params.get(0)?;
                for i in params[1..params.len()].to_vec().iter() {
                    result *= i;
                }
                Some(Type::Number(result))
            })),
        ),
        (
            "/".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = *params.get(0)?;
                for i in params[1..params.len()].to_vec().iter() {
                    result /= i;
                }
                Some(Type::Number(result))
            })),
        ),
        (
            "%".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = *params.get(0)?;
                for i in params[1..params.len()].to_vec().iter() {
                    result %= i;
                }
                Some(Type::Number(result))
            })),
        ),
        (
            "^".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = *params.get(0)?;
                for i in params[1..params.len()].to_vec().iter() {
                    result = result.powf(i.to_owned());
                }
                Some(Type::Number(result))
            })),
        ),
        (
            "concat".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::String(
                    params
                        .iter()
                        .map(|i| i.get_string())
                        .collect::<Vec<String>>()
                        .concat(),
                ))
            })),
        ),
        (
            "print".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                print!(
                    "{}",
                    params
                        .iter()
                        .map(|i| i.get_string())
                        .collect::<Vec<String>>()
                        .concat()
                );
                Some(Type::Null)
            })),
        ),
        (
            "input".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::String({
                    let mut input = String::new();
                    if let Some(prompt) = params.get(0) {
                        print!("{}", prompt.get_string());
                    }
                    io::stdout().flush().unwrap();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => input.trim().to_string(),
                        Err(_) => return None,
                    }
                }))
            })),
        ),
        (
            "=".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Bool({
                    let params: Vec<String> = params.iter().map(|i| format!("{i:?}")).collect();
                    params.windows(2).all(|window| window[0] == window[1])
                }))
            })),
        ),
        (
            "!=".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Bool({
                    let params: Vec<String> = params.iter().map(|i| format!("{i:?}")).collect();
                    params.windows(2).all(|window| window[0] != window[1])
                }))
            })),
        ),
        (
            ">".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Bool({
                    let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                    params.windows(2).all(|window| window[0] > window[1])
                }))
            })),
        ),
        (
            ">=".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Bool({
                    let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                    params.windows(2).all(|window| window[0] >= window[1])
                }))
            })),
        ),
        (
            "<".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Bool({
                    let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                    params.windows(2).all(|window| window[0] < window[1])
                }))
            })),
        ),
        (
            "<=".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Bool({
                    let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                    params.windows(2).all(|window| window[0] < window[1])
                }))
            })),
        ),
        (
            "&".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Bool({
                    let params: Vec<bool> = params.iter().map(|i| i.get_bool()).collect();
                    params.iter().all(|x| *x)
                }))
            })),
        ),
        (
            "|".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Bool({
                    let params: Vec<bool> = params.iter().map(|i| i.get_bool()).collect();
                    params.iter().any(|x| *x)
                }))
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
                    "list" => Some(Type::List(params.get(0)?.get_list())),
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
            "block".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let mut result: Vec<Expr> = vec![];
                for expr in params {
                    result.push(Expr {
                        expr,
                        annotate: None,
                    });
                }
                Some(Type::List(result))
            })),
        ),
        (
            "define".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                let value: Type;
                if let Type::List(args) = params.get(0)? {
                    value = Type::Function(Function::UserDefined(
                        args.get(1..)?.to_vec(),
                        params.get(1..)?.to_owned(),
                    ));
                    scope.insert(args.get(0)?.expr.get_string(), value.clone());
                } else {
                    value = params.get(1)?.to_owned();
                    scope.insert(params.get(0)?.get_string(), value.clone());
                }
                Some(value)
            })),
        ),
        (
            "lambda".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::Function(Function::UserDefined(
                    params.get(0)?.get_list(),
                    params.get(1..)?.to_vec(),
                )))
            })),
        ),
        (
            "if-else".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                if params.get(0)?.get_bool() {
                    Expr {
                        expr: Type::Expr(params.get(1)?.clone().get_list()),
                        annotate: None,
                    }
                    .eval(scope)
                } else {
                    Expr {
                        expr: Type::Expr(params.get(2)?.clone().get_list()),
                        annotate: None,
                    }
                    .eval(scope)
                }
            })),
        ),
        (
            "when".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                if params.get(0)?.get_bool() {
                    Expr {
                        expr: Type::Expr(params.get(1)?.clone().get_list()),
                        annotate: None,
                    }
                    .eval(scope)
                } else {
                    Some(Type::Null)
                }
            })),
        ),
        (
            "car".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(params.get(0)?.get_list().get(0)?.expr.clone())
            })),
        ),
        (
            "cdr".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let list = params.get(0)?;
                Some(Type::List(
                    list.get_list()[1..list.get_list().len()].to_vec(),
                ))
            })),
        ),
        (
            "range".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() == 1 {
                    let mut range: Vec<Expr> = vec![];
                    let mut current: f64 = 0.0;
                    while current < params[0].get_number() {
                        range.push(Expr {
                            expr: Type::Number(current),
                            annotate: None,
                        });
                        current += 1.0;
                    }
                    Some(Type::List(range))
                } else if params.len() == 2 {
                    let mut range: Vec<Expr> = vec![];
                    let mut current: f64 = params[0].get_number();
                    while current < params[1].get_number() {
                        range.push(Expr {
                            expr: Type::Number(current),
                            annotate: None,
                        });
                        current += 1.0;
                    }
                    Some(Type::List(range))
                } else if params.len() >= 3 {
                    let mut range: Vec<Expr> = vec![];
                    let mut current: f64 = params[0].get_number();
                    while current < params[1].get_number() {
                        range.push(Expr {
                            expr: Type::Number(current),
                            annotate: None,
                        });
                        current += params[2].get_number();
                    }
                    Some(Type::List(range))
                } else {
                    Some(Type::Null)
                }
            })),
        ),
        (
            "map".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                let mut result = vec![];
                let func = if let Type::Function(func) = params.get(1)? {
                    func
                } else {
                    return None;
                };
                for i in params.get(0)?.get_list() {
                    result.push(Expr {
                        expr: Expr {
                            expr: Type::Expr(vec![
                                Expr {
                                    expr: Type::Function(func.to_owned()),
                                    annotate: None,
                                },
                                i,
                            ]),
                            annotate: None,
                        }
                        .eval(scope)?,
                        annotate: None,
                    });
                }
                Some(Type::List(result))
            })),
        ),
        (
            "filter".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                let mut result = vec![];
                let func = if let Type::Function(func) = params.get(1)? {
                    func
                } else {
                    return None;
                };
                for i in params.get(0)?.get_list() {
                    if (Expr {
                        expr: Type::Expr(vec![
                            Expr {
                                expr: Type::Function(func.to_owned()),
                                annotate: None,
                            },
                            i.clone(),
                        ]),
                        annotate: None,
                    })
                    .eval(scope)?
                    .get_bool()
                    {
                        result.push(i)
                    }
                }
                Some(Type::List(result))
            })),
        ),
        (
            "reduce".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                let func = if let Type::Function(func) = params.get(2)? {
                    func
                } else {
                    return None;
                };
                let mut result = params.get(1)?.to_owned();
                let mut scope = scope.clone();

                for i in params.get(0)?.get_list() {
                    result = Expr {
                        expr: Type::Expr(vec![
                            Expr {
                                expr: Type::Function(func.to_owned()),
                                annotate: None,
                            },
                            i,
                            Expr {
                                expr: result,
                                annotate: None,
                            },
                        ]),
                        annotate: None,
                    }
                    .eval(&mut scope)?;
                }
                Some(result.to_owned())
            })),
        ),
        (
            "join".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::String(
                    params
                        .get(0)?
                        .get_list()
                        .iter()
                        .map(|i| i.expr.get_string())
                        .collect::<Vec<String>>()
                        .join(&params.get(1)?.get_string()),
                ))
            })),
        ),
        (
            "split".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Some(Type::List(
                    params
                        .get(0)?
                        .get_string()
                        .split(&params.get(1)?.get_string())
                        .map(|i| Expr {
                            expr: Type::String(i.to_string()),
                            annotate: None,
                        })
                        .collect::<Vec<Expr>>(),
                ))
            })),
        ),
        (
            "exit".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                exit(params.get(0).unwrap_or(&Type::Number(0.0)).get_number() as i32)
            })),
        ),
        ("new-line".to_string(), Type::String("\n".to_string())),
        ("double-quote".to_string(), Type::String("\"".to_string())),
        ("tab".to_string(), Type::String("\t".to_string())),
    ])
}

pub fn parse(token: Vec<String>) -> Option<Expr> {
    // Setting type annotation
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
            "null" => Some(Type::Null),
            _ => None,
        }
    } else {
        None
    };

    let mut token = token[0].trim().to_string();
    Some(
        // Number case
        if let Ok(n) = token.parse::<f64>() {
            Expr {
                expr: Type::Number(n),
                annotate,
            }
        // Bool calse
        } else if let Ok(b) = token.parse::<bool>() {
            Expr {
                expr: Type::Bool(b),
                annotate,
            }
        // Null calse
        } else if token == "null".to_string() {
            Expr {
                expr: Type::Null,
                annotate,
            }
        // String calse
        } else if token.starts_with('"') && token.ends_with('"') {
            token.remove(0); // Removing outer syntax
            token.remove(token.len() - 1);
            Expr {
                expr: Type::String(token),
                annotate,
            }
        // Expression case
        } else if token.starts_with('(') && token.ends_with(')') {
            token.remove(0); // Removing outer syntax
            token.remove(token.len() - 1);
            Expr {
                expr: {
                    let mut list = vec![];
                    for i in tokenize(token)? {
                        list.push(parse(i)?)
                    }
                    Type::Expr(list)
                },
                annotate,
            }
        // List case
        } else if token.starts_with("'(") && token.ends_with(')') {
            token.remove(0); // Removing outer syntax
            token.remove(0);
            token.remove(token.len() - 1);
            Expr {
                expr: {
                    let mut list = vec![];
                    for i in tokenize(token)? {
                        list.push(parse(i)?)
                    }
                    Type::List(list)
                },
                annotate,
            }
        // Other case will be symbol
        } else {
            Expr {
                expr: Type::Symbol(token.clone()),
                annotate,
            }
        },
    )
}

pub fn tokenize(input: String) -> Option<Vec<Vec<String>>> {
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
                if in_parentheses > 0 {
                    in_parentheses -= 1;
                } else {
                    eprintln!("Error! there's duplicate end of the parentheses");
                    return None;
                }
            }
            ' ' | '　' | '\n' | '\t' | '\r' if !in_quote => {
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

    // Syntax error check
    if in_quote {
        eprintln!("Error! there's not end of the quote");
        return None;
    }
    if in_parentheses != 0 {
        eprintln!("Error! there's not end of the parentheses");
        return None;
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
    Some(tokens)
}

#[derive(Clone)]
pub struct Expr {
    expr: Type,
    annotate: Option<Type>,
}

impl Expr {
    pub fn eval(&self, scope: &mut HashMap<String, Type>) -> Option<Type> {
        let result = if let Type::Expr(expr) = &self.expr {
            // Prepare expression
            let expr = {
                let mut new = vec![];
                for i in expr {
                    new.push(i.eval(scope)?)
                }
                new
            };

            if let Type::Function(Function::BuiltIn(func)) = expr.get(0)? {
                func(expr.get(1..)?.to_vec(), scope)?
            } else if let Type::Function(Function::UserDefined(args, code)) = expr.get(0)? {
                if args.len() != expr.get(1..)?.len() {
                    eprintln!(
                        "Error! the passed arguments length {} is different to expected length {} of the function's arguments",
                        expr.get(1..)?.len(), args.len()
                    );
                    return None;
                }

                let mut func_scope = scope.clone();
                for (k, v) in args.iter().zip(expr.get(1..)?.to_vec()) {
                    if let Some(annotate) = k.annotate.clone() {
                        // Type check between arguments and expects
                        if annotate.get_type() == v.get_type() {
                            // Setting argument by passed value
                            func_scope.insert(k.expr.get_string(), v);
                        } else {
                            eprintln!(
                                "Error! the passed argument value `{:?}` is different to expected type `{}` of the function",
                                v, annotate.get_type()
                            );
                            return None;
                        }
                    } else {
                        // Setting argument by passed value
                        func_scope.insert(k.expr.get_string(), v);
                    }
                }

                // Execution of function's code
                let mut result = None;
                for line in code {
                    result = Expr {
                        expr: if let Type::List(expr) = line.to_owned() {
                            Type::Expr(expr)
                        } else {
                            line.to_owned()
                        },
                        annotate: None,
                    }
                    .eval(&mut func_scope);
                }
                result?
            } else {
                if expr.len() == 1 {
                    expr[0].clone()
                } else {
                    Type::List(
                        expr.iter()
                            .map(|i| Expr {
                                expr: i.to_owned(),
                                annotate: None,
                            })
                            .collect(),
                    )
                }
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
pub enum Type {
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
pub enum Function {
    BuiltIn(fn(Vec<Type>, &mut HashMap<String, Type>) -> Option<Type>),
    UserDefined(Vec<Expr>, Vec<Type>),
}

impl Type {
    pub fn get_number(&self) -> f64 {
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

    pub fn get_string(&self) -> String {
        match &self {
            Type::Number(n) => n.to_string(),
            Type::String(s) => s.to_owned(),
            Type::Bool(b) => b.to_string(),
            Type::Symbol(v) => v.to_owned(),
            other => format!("{other:?}"),
        }
    }

    pub fn get_bool(&self) -> bool {
        match &self {
            Type::Number(n) => *n != 0.0,
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
            Type::Number(n) => n.to_string(),
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
        write!(f, "{fmt}",)
    }
}
