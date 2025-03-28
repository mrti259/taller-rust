mod errors;
mod interpreter;
mod runner;
mod stack;

use runner::BorthRunner;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = BorthRunner::from_args(&args).and_then(|runner| runner.start());
    if let Err(error) = result {
        print!("{}", error);
    }
}
