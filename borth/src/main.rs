mod context;
mod dict;
mod errors;
mod expression;
mod interpreter;
mod parser;
mod runner;
mod stack;

use runner::BorthRunner;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = BorthRunner::from_args(&args)
        .and_then(|runner| runner.start("stack.fth", &mut std::io::stdout()));

    if let Err(error) = result {
        print!("{}", error);
    }
}
