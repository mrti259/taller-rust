mod errors;
mod interpreter;
mod runner;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = runner::Runner::from_args(&args).and_then(|runner| runner.start());
    if let Err(error) = &result {
        print!("{}", error);
    }
}
