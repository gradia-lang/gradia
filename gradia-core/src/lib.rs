use std::{
    collections::HashMap,
    fmt::{self, Debug},
    io::{self, Write},
    process::exit,
};
use thiserror::Error;

pub type Scope = HashMap<String, Type>;
pub fn builtin_function() -> Scope {
    HashMap::from([
        (
            "+".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 1 {
                    let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                    let mut result: f64 = params.get(0).cloned().unwrap_or_default();
                    for i in params[1..params.len()].to_vec().iter() {
                        result += i;
                    }
                    Ok(Type::Number(result))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "-".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 1 {
                    let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                    if params.len() >= 2 {
                        let mut result: f64 = params.get(0).cloned().unwrap_or_default();
                        for i in params[1..params.len()].to_vec().iter() {
                            result -= i;
                        }
                        Ok(Type::Number(result))
                    } else {
                        Ok(Type::Number(-params.get(0).cloned().unwrap_or_default()))
                    }
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "*".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 1 {
                    let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                    let mut result: f64 = params.get(0).cloned().unwrap_or_default();
                    for i in params[1..params.len()].to_vec().iter() {
                        result *= i;
                    }
                    Ok(Type::Number(result))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "/".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 1 {
                    let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                    let mut result: f64 = params.get(0).cloned().unwrap_or_default();
                    for i in params[1..params.len()].to_vec().iter() {
                        result /= i;
                    }
                    Ok(Type::Number(result))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "%".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 1 {
                    let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                    let mut result: f64 = params[0];
                    for i in params[1..params.len()].to_vec().iter() {
                        result %= i;
                    }
                    Ok(Type::Number(result))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "^".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 1 {
                    let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                    let mut result: f64 = params[0];
                    for i in params[1..params.len()].to_vec().iter() {
                        result = result.powf(i.to_owned());
                    }
                    Ok(Type::Number(result))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "concat".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Ok(Type::String(
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
                Ok(Type::Null)
            })),
        ),
        (
            "input".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() <= 1 {
                    Ok(Type::String({
                        let mut input = String::new();
                        if let Some(prompt) = params.get(0) {
                            print!("{}", prompt.get_string());
                        }
                        io::stdout().flush().unwrap_or_default();
                        match io::stdin().read_line(&mut input) {
                            Ok(_) => input.trim().to_string(),
                            Err(_) => {
                                return Err(GradiaError::Runtime(
                                    "reading line was fault".to_string(),
                                ))
                            }
                        }
                    }))
                } else {
                    Err(GradiaError::Function(params.len(), 1))
                }
            })),
        ),
        (
            "=".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 2 {
                    Ok(Type::Bool({
                        let params: Vec<String> = params.iter().map(|i| format!("{i:?}")).collect();
                        params.windows(2).all(|window| window[0] == window[1])
                    }))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "!=".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 2 {
                    Ok(Type::Bool({
                        let params: Vec<String> = params.iter().map(|i| format!("{i:?}")).collect();
                        params.windows(2).all(|window| window[0] != window[1])
                    }))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            ">".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 2 {
                    Ok(Type::Bool({
                        let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                        params.windows(2).all(|window| window[0] > window[1])
                    }))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            ">=".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 2 {
                    Ok(Type::Bool({
                        let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                        params.windows(2).all(|window| window[0] >= window[1])
                    }))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "<".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 2 {
                    Ok(Type::Bool({
                        let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                        params.windows(2).all(|window| window[0] < window[1])
                    }))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "<=".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 2 {
                    Ok(Type::Bool({
                        let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                        params.windows(2).all(|window| window[0] < window[1])
                    }))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "&".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 2 {
                    Ok(Type::Bool({
                        let params: Vec<bool> = params.iter().map(|i| i.get_bool()).collect();
                        params.iter().all(|x| *x)
                    }))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "|".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 2 {
                    Ok(Type::Bool({
                        let params: Vec<bool> = params.iter().map(|i| i.get_bool()).collect();
                        params.iter().any(|x| *x)
                    }))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "!".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() == 1 {
                    Ok(Type::Bool(!params[0].get_bool()))
                } else {
                    Err(GradiaError::Function(params.len(), 1))
                }
            })),
        ),
        (
            "cast".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() == 2 {
                    match params
                        .get(1)
                        .cloned()
                        .unwrap_or_default()
                        .get_string()
                        .as_str()
                    {
                        "number" => Ok(Type::Number(
                            params.get(0).cloned().unwrap_or_default().get_number(),
                        )),
                        "string" => Ok(Type::String(
                            params.get(0).cloned().unwrap_or_default().get_string(),
                        )),
                        "bool" => Ok(Type::Bool(
                            params.get(0).cloned().unwrap_or_default().get_bool(),
                        )),
                        "list" => Ok(Type::List(
                            params.get(0).cloned().unwrap_or_default().get_list(),
                        )),
                        other => Err(GradiaError::Runtime(format!("unknown type name `{other}`"))),
                    }
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "type".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() == 1 {
                    Ok(Type::String(
                        params.get(0).cloned().unwrap_or_default().get_type(),
                    ))
                } else {
                    Err(GradiaError::Function(params.len(), 1))
                }
            })),
        ),
        (
            "eval".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                let mut result = Type::Null;
                for expr in params {
                    result = Expr {
                        expr: Type::Expr(expr.get_list()),
                        annotate: None,
                    }
                    .eval(scope)?;
                }
                Ok(result)
            })),
        ),
        (
            "define".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                let value: Type;
                if params.len() >= 2 {
                    if let Type::List(args) = params.get(0).cloned().unwrap_or_default() {
                        value = Type::Function(Function::UserDefined(
                            args.get(1..).unwrap_or_default().to_vec(),
                            params.get(1..).unwrap_or_default().to_owned(),
                        ));
                        scope.insert(
                            args.get(0).cloned().unwrap_or_default().expr.get_string(),
                            value.clone(),
                        );
                    } else {
                        value = params.get(1).cloned().unwrap_or_default().to_owned();
                        scope.insert(
                            params.get(0).cloned().unwrap_or_default().get_string(),
                            value.clone(),
                        );
                    }
                } else {
                    return Err(GradiaError::Function(params.len(), 2));
                }
                Ok(value)
            })),
        ),
        (
            "lambda".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 2 {
                    Ok(Type::Function(Function::UserDefined(
                        params[0].get_list(),
                        params[1..].to_vec(),
                    )))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "if".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                if params.len() == 3 {
                    if params[0].get_bool() {
                        Expr {
                            expr: Type::Expr(params[1].get_list()),
                            annotate: None,
                        }
                        .eval(scope)
                    } else {
                        Expr {
                            expr: Type::Expr(params[2].get_list()),
                            annotate: None,
                        }
                        .eval(scope)
                    }
                } else if params.len() == 2 {
                    if params[0].get_bool() {
                        Expr {
                            expr: Type::Expr(params[1].get_list()),
                            annotate: None,
                        }
                        .eval(scope)
                    } else {
                        Ok(Type::Null)
                    }
                } else {
                    Err(GradiaError::Function(params.len(), 3))
                }
            })),
        ),
        (
            "car".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() == 1 {
                    Ok(params[0]
                        .get_list()
                        .get(0)
                        .cloned()
                        .unwrap_or_default()
                        .expr
                        .clone())
                } else {
                    Err(GradiaError::Function(params.len(), 1))
                }
            })),
        ),
        (
            "cdr".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() == 1 {
                    let list = params[0].get_list();
                    Ok(Type::List(
                        list.get(1..list.len()).unwrap_or_default().to_vec(),
                    ))
                } else {
                    Err(GradiaError::Function(params.len(), 1))
                }
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
                    Ok(Type::List(range))
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
                    Ok(Type::List(range))
                } else if params.len() == 3 {
                    let mut range: Vec<Expr> = vec![];
                    let mut current: f64 = params[0].get_number();
                    while current < params[1].get_number() {
                        range.push(Expr {
                            expr: Type::Number(current),
                            annotate: None,
                        });
                        current += params[2].get_number();
                    }
                    Ok(Type::List(range))
                } else {
                    Err(GradiaError::Function(params.len(), 3))
                }
            })),
        ),
        (
            "map".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                if params.len() == 2 {
                    let mut result = vec![];
                    let func = params[1].clone();
                    for i in params[0].get_list() {
                        result.push(Expr {
                            expr: Expr {
                                expr: Type::Expr(vec![
                                    Expr {
                                        expr: func.clone(),
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
                    Ok(Type::List(result))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "filter".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                if params.len() == 2 {
                    let mut result = vec![];
                    let func = params[1].clone();
                    for i in params[0].get_list() {
                        if (Expr {
                            expr: Type::Expr(vec![
                                Expr {
                                    expr: func.to_owned(),
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
                    Ok(Type::List(result))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "reduce".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                if params.len() == 2 {
                    let func = params[1].clone();
                    let list = params[0].get_list();
                    let mut result = if let Some(first) = list.get(0) {
                        first.expr.clone()
                    } else {
                        return Err(GradiaError::Runtime("passed list is empty".to_string()));
                    };
                    let mut scope = scope.clone();

                    for i in list.get(1..).unwrap_or_default() {
                        result = Expr {
                            expr: Type::Expr(vec![
                                Expr {
                                    expr: func.clone(),
                                    annotate: None,
                                },
                                Expr {
                                    expr: result,
                                    annotate: None,
                                },
                                i.clone(),
                            ]),
                            annotate: None,
                        }
                        .eval(&mut scope)?
                    }
                    Ok(result.to_owned())
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "reverse".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() == 1 {
                    let mut list = params[0].get_list();
                    list.reverse();
                    Ok(Type::List(list))
                } else {
                    Err(GradiaError::Function(params.len(), 1))
                }
            })),
        ),
        (
            "len".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() == 1 {
                    Ok(Type::Number(params[0].get_list().len() as f64))
                } else {
                    Err(GradiaError::Function(params.len(), 1))
                }
            })),
        ),
        (
            "repeat".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() == 2 {
                    Ok(Type::String(
                        params[0]
                            .get_string()
                            .repeat(params[1].get_number() as usize),
                    ))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "join".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() == 2 {
                    Ok(Type::String(
                        params[0]
                            .get_list()
                            .iter()
                            .map(|i| i.expr.get_string())
                            .collect::<Vec<String>>()
                            .join(&params[1].get_string()),
                    ))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "split".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() == 2 {
                    Ok(Type::List(
                        params[0]
                            .get_string()
                            .split(&params[1].get_string())
                            .map(|i| Expr {
                                expr: Type::String(i.to_string()),
                                annotate: None,
                            })
                            .collect::<Vec<Expr>>(),
                    ))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
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

#[derive(Debug, Error)]
pub enum GradiaError {
    #[error("Runtime Error! {0}")]
    Runtime(String),

    #[error("Runtime Error! the passed arguments length {0} is different to expected length {1} of the function's arguments")]
    Function(usize, usize),

    #[error("Syntax Error! {0}")]
    Syntax(String),
}

pub fn parse(token: (String, Option<String>)) -> Result<Expr, GradiaError> {
    // Setting type annotation
    let annotate = if let Some(annotate) = token.1 {
        match annotate.as_str() {
            "function" => Some(Type::Function(Function::BuiltIn(|_, _| Ok(Type::Null)))),
            "list" => Some(Type::List(vec![])),
            "symbol" => Some(Type::Symbol(String::new())),
            "string" => Some(Type::String(String::new())),
            "number" => Some(Type::Number(0.0)),
            "bool" => Some(Type::Bool(false)),
            "null" => Some(Type::Null),
            "any" => None,
            other => {
                return Err(GradiaError::Syntax(format!(
                    "unknown type annotation `{other}`"
                )))
            }
        }
    } else {
        None
    };

    let mut token = token.0.trim().to_string();
    Ok(
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
        // Symbol that explicitly
        } else if token.starts_with("'") {
            token.remove(0); // Removing outer syntax
            Expr {
                expr: Type::Symbol(token),
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

pub fn tokenize(input: String) -> Result<Vec<(String, Option<String>)>, GradiaError> {
    let mut tokens: Vec<(String, Option<String>)> = Vec::new();
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
                    return Err(GradiaError::Syntax(
                        "there's duplicate end of the parentheses".to_string(),
                    ));
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
                        tokens.push((current_token.clone(), Some(after_colon.clone())));
                        current_token.clear();
                        after_colon.clear();
                    } else {
                        tokens.push((current_token.clone(), None));
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
        return Err(GradiaError::Syntax(
            "there's not end of the quote".to_string(),
        ));
    }
    if in_parentheses != 0 {
        return Err(GradiaError::Syntax(
            "there's not end of the parentheses".to_string(),
        ));
    }

    if in_parentheses == 0 && !current_token.is_empty() {
        if is_colon {
            tokens.push((current_token.clone(), Some(after_colon.clone())));
            current_token.clear();
        } else {
            tokens.push((current_token.clone(), None));
            current_token.clear();
        }
    }
    Ok(tokens)
}

#[derive(Clone)]
pub struct Expr {
    expr: Type,
    annotate: Option<Type>,
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
                    return
                    Err(GradiaError::Runtime(format!(
                        "the passed arguments length {} is different to expected length {} of the function's arguments",
                        expr.get(1..).unwrap_or_default().len(), args.len()
                    )));
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
                            return
                            Err(GradiaError::Runtime(format!(
                                "the passed argument value `{:?}` is different to expected type `{}` of the function",
                                v, annotate.get_type()
                            )));
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
                return Err(GradiaError::Runtime(format!(
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
                return Err(GradiaError::Runtime(format!(
                    "the result value `{:?}` is different to expected type `{}`",
                    result,
                    annotate.get_type()
                )));
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
    BuiltIn(fn(Vec<Type>, &mut Scope) -> Result<Type, GradiaError>),
    UserDefined(Vec<Expr>, Vec<Type>),
}

impl Type {
    pub fn get_number(&self) -> f64 {
        match &self {
            Type::Number(n) => n.to_owned(),
            Type::String(s) | Type::Symbol(s) => s.trim().parse().unwrap_or(0.0),
            Type::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            Type::Expr(x) | Type::List(x) => {
                x.get(0).cloned().unwrap_or_default().expr.get_number()
            }
            Type::Function(_) | Type::Null => 0.0,
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
        write!(f, "{fmt}")
    }
}

impl Default for Type {
    fn default() -> Self {
        Type::Null
    }
}
