use crate::expr::{Expr, GradiaError};
use crate::fraction::Fraction;
use crate::types::{Class, Type};

pub fn parse(token: (String, Option<String>)) -> Result<Expr, GradiaError> {
    // Setting type annotation
    let annotate = if let Some(annotate) = token.1 {
        Class::from(annotate)?
    } else {
        None
    };

    let mut token = token.0.trim().to_string();
    Ok(
        // Number case
        if let Ok(n) = token.parse::<f64>() {
            Expr {
                expr: Type::Number(Fraction::new(n)),
                annotate,
            }
        // Fraction case
        } else if let Some(n) = Fraction::from(token.clone()) {
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
