use gradia_core::{builtin_function, parse, tokenize, Function, Type};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Gradia {
    scope: HashMap<String, Type>,
}

#[wasm_bindgen]
impl Gradia {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Gradia {
        let mut scope = builtin_function();
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
        Gradia { scope }
    }

    pub fn run(&mut self, code: String) {
        if let Some(lines) = tokenize(code) {
            for line in lines {
                if let Some(ast) = parse(line) {
                    ast.eval(&mut self.scope);
                }
            }
        }
    }

    pub fn eval(&mut self, code: String) -> String {
        let mut result = String::new();
        if let Some(lines) = tokenize(code) {
            for line in lines {
                if let Some(ast) = parse(line) {
                    result = format!("{:?}", ast.eval(&mut self.scope));
                }
            }
        }
        result
    }

    pub fn get_stdout(&self) -> String {
        self.scope.get("stdout").unwrap().get_string()
    }
}

#[wasm_bindgen]
pub fn run_gradia(code: String) -> String {
    let mut gradia = Gradia::new();
    gradia.run(code);
    gradia.get_stdout()
}
