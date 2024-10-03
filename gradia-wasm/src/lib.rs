use gradia_core::{builtin_function, parse, tokenize, Function, Type};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(code: String) -> String {
    let scope = &mut builtin_function();
    scope.insert("stdout".to_string(), Type::String(String::new()));
    scope.insert(
        "print".to_string(),
        Type::Function(Function::BuiltIn(|params, scope| {
            scope.insert(
                "stdout".to_string(),
                Type::String(
                    scope.get("stdout")?.get_string()
                        + &params
                            .iter()
                            .map(|i| i.get_string())
                            .collect::<Vec<String>>()
                            .concat(),
                ),
            )
        })),
    );

    if let Some(lines) = tokenize(code) {
        for line in lines {
            if let Some(ast) = parse(line) {
                ast.eval(scope);
            }
        }
    }
    scope.get("stdout").unwrap().get_string()
}
