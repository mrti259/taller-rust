use crate::errors::*;
use crate::interpreter::BorthInterpreter;
use std::fs::File;
use std::io::{Read, Stdout};

pub struct BorthRunner {
    path: String,
    stack_size: usize,
}

impl BorthRunner {
    pub fn from_args(args: &[String]) -> BorthResult<Self> {
        let (path, stack_size) = parse_args(args)?;
        let stack_size = stack_size.unwrap_or(128_000);
        Ok(Self { path, stack_size })
    }

    pub fn start(&self) -> BorthResult<()> {
        let code = self.get_code_from_file()?;
        let mut interpreter = self.create_interpreter();
        let code_result = interpreter.run_code(&code);
        let export_result = self.save_stack_to_file(&interpreter, "stack.fth");
        code_result.and(export_result)
    }

    fn get_code_from_file(&self) -> BorthResult<String> {
        match File::open(&self.path) {
            Ok(mut file) => {
                let mut code = Default::default();
                if file.read_to_string(&mut code).is_err() {
                    return Err(BorthError::CanNotReadCode);
                }
                Ok(code)
            }
            _ => Err(BorthError::CanNotReadFile),
        }
    }

    fn create_interpreter(&self) -> BorthInterpreter<Stdout> {
        BorthInterpreter::with_stack_size(self.stack_size, std::io::stdout())
    }

    fn save_stack_to_file(
        &self,
        interpreter: &BorthInterpreter<Stdout>,
        path_to_file: &str,
    ) -> BorthResult<()> {
        match File::create(path_to_file) {
            Ok(mut file) => interpreter.export_stack_to(&mut file),
            _ => Err(BorthError::CanNotWriteFile),
        }
    }
}

fn parse_args(args: &[String]) -> BorthResult<(String, Option<usize>)> {
    let len = args.len();
    if len < 2 {
        return Err(BorthError::MissingArguments);
    }
    if len > 3 {
        return Err(BorthError::TooManyArguments);
    }
    let path = args[1].to_string();
    if args.len() <= 2 {
        return Ok((path, None));
    }
    if !args[2].contains("--stack-size=") {
        return Err(BorthError::BadArguments);
    }
    match args[2].replace("--stack-size=", "").parse::<usize>() {
        Ok(value) => Ok((path, Some(value))),
        _ => Err(BorthError::BadArguments),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::remove_file, path::Path};

    fn create_args() -> Vec<String> {
        vec!["forth".into()]
    }

    #[test]
    fn test1_expect_filename_in_args() {
        let mut args = create_args();
        assert!(parse_args(&args).is_err());

        args.push("ruta/a/main.fth".into());
        assert!(parse_args(&args).is_ok());
    }

    #[test]
    fn test2_parse_stack_size() {
        let mut args = create_args();
        args.push("ruta/a/main.fth".into());
        args.push("--stack-size=10".into());

        assert!(parse_args(&args).is_ok_and(|(_, size)| size == Some(10)));
    }

    #[test]
    fn test3_stack_size_is_optional() {
        let mut args = create_args();
        args.push("ruta/a/main.fth".into());

        assert!(parse_args(&args).is_ok_and(|(_, size)| size.is_none()));
    }

    #[test]
    fn test4_runner_run_ok() {
        let mut args = create_args();
        args.push("./fth-examples/3.fth".into());
        let runner = BorthRunner::from_args(&args);
        assert!(runner.is_ok());
        assert!(runner.is_ok_and(|r| r.start().is_ok()));
    }

    #[test]
    fn test5_runner_can_not_read_file() {
        let mut args = create_args();
        args.push("./fth-examples/0.fth".into());
        let runner = BorthRunner::from_args(&args);
        assert!(runner.is_ok());
        assert!(match runner.and_then(|r| r.start()) {
            Err(BorthError::CanNotReadFile) => true,
            _ => false,
        });
    }

    #[test]
    fn test6_runner_handle_interpreter_error_and_export_stack() {
        let mut args = create_args();
        args.push("./fth-examples/stack_underflow.fth".into());
        let runner = BorthRunner::from_args(&args);
        assert!(runner.is_ok());

        let export_path = Path::new("stack.fth");
        if export_path.exists() {
            let _ = remove_file(export_path);
        }
        assert!(!export_path.exists());
        assert!(match runner.and_then(|r| r.start()) {
            Err(BorthError::StackUnderflow) => true,
            _ => false,
        });
        assert!(export_path.exists());
    }
}
