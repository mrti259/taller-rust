use crate::errors::*;
use crate::interpreter::BorthInterpreter;
use std::io::Read;

pub struct BorthRunner {
    path: String,
    stack_size: usize,
}

impl BorthRunner {
    pub fn from_args(args: &[String]) -> BorthResult<Self> {
        match parse_args(args) {
            Err(error) => Err(error),
            Ok((path, Some(stack_size))) => Ok(Self { path, stack_size }),
            Ok((path, None)) => Ok(Self {
                path,
                stack_size: 128,
            }),
        }
    }

    pub fn start(&self) -> BorthResult<()> {
        let mut code = String::new();
        match std::fs::File::open(&self.path) {
            Ok(mut file) => {
                if file.read_to_string(&mut code).is_err() {
                    return Err(BorthError::CanNotReadCode);
                }

                let mut interpreter =
                    BorthInterpreter::with_stack_size(self.stack_size, std::io::stdout());
                let code_result = interpreter.run_code(&code);
                let export_result = interpreter.export_stack_to("stack.fth");

                code_result.and(export_result)
            }
            _ => Err(BorthError::CanNotReadFile),
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

    #[test]
    fn test1_expect_filename_in_args() {
        let mut args = Vec::from(["forth".into()]);
        assert!(parse_args(&args).is_err());

        args.push("ruta/a/main.fth".into());
        assert!(parse_args(&args).is_ok());
    }

    #[test]
    fn test2_parse_stack_size() {
        let mut args = Vec::from(["forth".into()]);
        args.push("ruta/a/main.fth".into());
        args.push("--stack-size=10".into());

        assert!(parse_args(&args).is_ok_and(|(_, size)| size == Some(10)));
    }

    #[test]
    fn test3_stack_size_is_optional() {
        let mut args = Vec::from(["forth".into()]);
        args.push("ruta/a/main.fth".into());

        assert!(parse_args(&args).is_ok_and(|(_, size)| size.is_none()));
    }
}
