use clap::Parser;
use gradia_core::{
    parser::{parse, tokenize},
    std::builtin_function,
    types::Scope,
};
use rustyline::DefaultEditor;
use std::fs::read_to_string;

const VERSION: &str = "0.1.0";

#[derive(Parser, Debug)]
#[command(
    name = "Gradia",
    version = VERSION,
    author = "梶塚太智, kajizukataichi@outlook.jp",
    about = "Lisp like programming language that can give type annotation for gradual typing",
)]
struct Cli {
    /// Script file to be running
    #[arg(index = 1)]
    file: Option<String>,

    /// Run code quickly
    #[arg(short = 'l', long, name = "CODE")]
    one_liner: Option<String>,
}

fn main() {
    let mut scope: Scope = builtin_function();
    let args = Cli::parse();

    if let Some(path) = args.file {
        if let Ok(code) = read_to_string(path) {
            if let Ok(lines) = tokenize(code) {
                for line in lines {
                    if let Ok(ast) = parse(line) {
                        ast.eval(&mut scope).unwrap();
                    }
                }
            }
        } else {
            eprintln!("Error! opening file is fault");
        }
    } else if let Some(code) = args.one_liner {
        if let Ok(lines) = tokenize(code) {
            for line in lines {
                if let Ok(ast) = parse(line) {
                    ast.eval(&mut scope).unwrap();
                }
            }
        }
    } else {
        println!("Gradia {VERSION}");
        if let Ok(mut rl) = DefaultEditor::new() {
            loop {
                match rl.readline("> ") {
                    Ok(code) => {
                        rl.add_history_entry(&code).unwrap_or_default();
                        match tokenize(code) {
                            Ok(lines) => {
                                for line in lines {
                                    match parse(line) {
                                        Ok(ast) => match ast.eval(&mut scope) {
                                            Ok(result) => println!("{:?}", result),
                                            Err(err) => println!("{err}"),
                                        },
                                        Err(err) => println!("{err}"),
                                    }
                                }
                            }
                            Err(err) => println!("{err}"),
                        }
                    }
                    Err(err) => println!("{err}"),
                }
            }
        }
    }
}
