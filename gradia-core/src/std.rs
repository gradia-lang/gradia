use crate::expr::{Expr, GradiaError};
use crate::fraction::Fraction;
use crate::types::{Function, Scope, Type};
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::exit;

pub fn builtin_function() -> Scope {
    HashMap::from([
        (
            "+".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 1 {
                    let params: Vec<Fraction> = params.iter().map(|i| i.get_number()).collect();
                    let mut result: Fraction = params[0];
                    for i in params[1..params.len()].to_vec().iter() {
                        result = result + i.clone();
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
                    let params: Vec<Fraction> = params.iter().map(|i| i.get_number()).collect();
                    if params.len() >= 2 {
                        let mut result: Fraction = params[0];
                        for i in params[1..params.len()].to_vec().iter() {
                            result = result - i.clone();
                        }
                        Ok(Type::Number(result))
                    } else {
                        Ok(Type::Number(Fraction::new(0.0) - params[0]))
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
                    let params: Vec<Fraction> = params.iter().map(|i| i.get_number()).collect();
                    let mut result: Fraction = params[0];
                    for i in params[1..params.len()].to_vec().iter() {
                        result = result * i.clone();
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
                    let params: Vec<Fraction> = params.iter().map(|i| i.get_number()).collect();
                    let mut result: Fraction = params[0];
                    for i in params[1..params.len()].to_vec().iter() {
                        result = result / i.clone();
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
                    let params: Vec<f64> = params.iter().map(|i| i.get_number().to_f64()).collect();
                    let mut result: f64 = params[0];
                    for i in params[1..params.len()].to_vec().iter() {
                        result %= i;
                    }
                    Ok(Type::Number(Fraction::new(result)))
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "^".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 1 {
                    let params: Vec<f64> = params.iter().map(|i| i.get_number().to_f64()).collect();
                    let mut result: f64 = params[0];
                    for i in params[1..params.len()].to_vec().iter() {
                        result = result.powf(i.to_owned());
                    }
                    Ok(Type::Number(Fraction::new(result)))
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
            "debug".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                for i in params {
                    println!("Debug: {:?}", i);
                }
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
                        let params: Vec<f64> =
                            params.iter().map(|i| i.get_number().to_f64()).collect();
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
                        let params: Vec<f64> =
                            params.iter().map(|i| i.get_number().to_f64()).collect();
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
                        let params: Vec<f64> =
                            params.iter().map(|i| i.get_number().to_f64()).collect();
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
                        let params: Vec<f64> =
                            params.iter().map(|i| i.get_number().to_f64()).collect();
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
                    match params[1].get_string().as_str() {
                        "number" => Ok(Type::Number(params[0].get_number())),
                        "string" => Ok(Type::String(params[0].get_string())),
                        "bool" => Ok(Type::Bool(params[0].get_bool())),
                        "list" => Ok(Type::List(params[0].get_list())),
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
                    Ok(Type::String(params[0].get_type()))
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
                    if let Type::List(args) = params[0].clone() {
                        value = Type::Function(Function::UserDefined(
                            args[1..].to_vec(),
                            params[1..].to_owned(),
                        ));
                        scope.insert(args[0].expr.get_string(), value.clone());
                    } else {
                        value = params[1].to_owned();
                        scope.insert(params[0].get_string(), value.clone());
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
                        if let Type::List(expr) = params[1].clone() {
                            Expr {
                                expr: Type::Expr(expr),
                                annotate: None,
                            }
                            .eval(scope)
                        } else {
                            Ok(params[1].clone())
                        }
                    } else {
                        if let Type::List(expr) = params[2].clone() {
                            Expr {
                                expr: Type::Expr(expr),
                                annotate: None,
                            }
                            .eval(scope)
                        } else {
                            Ok(params[2].clone())
                        }
                    }
                } else if params.len() == 2 {
                    if params[0].get_bool() {
                        if let Type::List(expr) = params[1].clone() {
                            Expr {
                                expr: Type::Expr(expr),
                                annotate: None,
                            }
                            .eval(scope)
                        } else {
                            Ok(params[1].clone())
                        }
                    } else {
                        Ok(Type::Null)
                    }
                } else {
                    Err(GradiaError::Function(params.len(), 3))
                }
            })),
        ),
        (
            "cond".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                for i in params {
                    if i.get_list()[0].eval(scope)?.get_bool() {
                        let code = i.get_list()[1].eval(scope)?;
                        return if let Type::List(expr) = code {
                            Expr {
                                expr: Type::Expr(expr),
                                annotate: None,
                            }
                            .eval(scope)
                        } else {
                            Ok(code.clone())
                        };
                    }
                }
                Ok(Type::Null)
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
                    while current < params[0].get_number().to_f64() {
                        range.push(Expr {
                            expr: Type::Number(Fraction::new(current)),
                            annotate: None,
                        });
                        current += 1.0;
                    }
                    Ok(Type::List(range))
                } else if params.len() == 2 {
                    let mut range: Vec<Expr> = vec![];
                    let mut current: f64 = params[0].get_number().to_f64();
                    while current < params[1].get_number().to_f64() {
                        range.push(Expr {
                            expr: Type::Number(Fraction::new(current)),
                            annotate: None,
                        });
                        current += 1.0;
                    }
                    Ok(Type::List(range))
                } else if params.len() == 3 {
                    let mut range: Vec<Expr> = vec![];
                    let mut current: f64 = params[0].get_number().to_f64();
                    while current < params[1].get_number().to_f64() {
                        range.push(Expr {
                            expr: Type::Number(Fraction::new(current)),
                            annotate: None,
                        });
                        current += params[2].get_number().to_f64();
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
                    Ok(Type::Number(Fraction::new(
                        params[0].get_list().len() as f64
                    )))
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
                            .repeat(params[1].get_number().to_f64() as usize),
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
            "error".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Err(GradiaError::Runtime(
                    params
                        .get(0)
                        .unwrap_or(&Type::String("Something went wrong".to_string()))
                        .get_string(),
                ))
            })),
        ),
        (
            "try".to_string(),
            Type::Function(Function::BuiltIn(|params, scope| {
                let tried = if let Type::List(expr) = params[0].clone() {
                    Expr {
                        expr: Type::Expr(expr),
                        annotate: None,
                    }
                    .eval(scope)
                } else {
                    Ok(params[0].clone())
                };
                if params.len() == 2 {
                    if let Ok(result) = tried {
                        Ok(result)
                    } else {
                        if let Type::List(expr) = params[1].clone() {
                            Expr {
                                expr: Type::Expr(expr),
                                annotate: None,
                            }
                            .eval(scope)
                        } else {
                            Ok(params[1].clone())
                        }
                    }
                } else {
                    Err(GradiaError::Function(params.len(), 2))
                }
            })),
        ),
        (
            "exit".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                exit(
                    params
                        .get(0)
                        .unwrap_or(&Type::Number(Fraction::new(0.0)))
                        .get_number()
                        .to_f64() as i32,
                )
            })),
        ),
        ("new-line".to_string(), Type::String("\n".to_string())),
        ("double-quote".to_string(), Type::String("\"".to_string())),
        ("tab".to_string(), Type::String("\t".to_string())),
    ])
}
