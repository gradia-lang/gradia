use clap::Parser;
use gradia::core::{builtin_function, parse, tokenize};
use rustyline::DefaultEditor;
use std::fs::read_to_string;

const VERSION: &str = "0.1.0";

#[derive(Parser, Debug)]
#[command(
    name = "Gradia",
    version = VERSION,
    author = "梶塚太智, kajizukataichi@outlook.jp",
    about = "Lisp like programming language that can add type annotation",
)]
struct Cli {
    /// Script file to be running
    #[arg(index = 1)]
    file: Option<String>,
}

fn main() {
    let scope = &mut builtin_function();
    let args = Cli::parse();

    if let Some(path) = args.file {
        if let Ok(code) = read_to_string(path) {
            if let Some(lines) = tokenize(code) {
                for line in lines {
                    if let Some(ast) = parse(line) {
                        ast.eval(scope);
                    }
                }
            }
        } else {
            eprintln!("Error! opening file is fault");
        }
    } else {
        println!("Gradia {VERSION}");
        if let Ok(mut rl) = DefaultEditor::new() {
            loop {
                if let Ok(code) = rl.readline("> ") {
                    rl.add_history_entry(&code).unwrap_or_default();
                    if let Some(lines) = tokenize(code) {
                        for line in lines {
                            if let Some(ast) = parse(line) {
                                if let Some(result) = ast.eval(scope) {
                                    println!("{:?}", result);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
