use crate::errors::*;
use crate::interpreter::*;
use std::io::Read;

pub struct Runner {
    path: String,
    stack_size: usize,
}

impl Runner {
    pub fn from_args(args: &[String]) -> Result<Self, BorthError> {
        match parse_args(args) {
            Err(error) => Err(error),
            Ok((path, Some(stack_size))) => Ok(Self { path, stack_size }),
            Ok((path, None)) => Ok(Self {
                path,
                stack_size: 128,
            }),
        }
    }

    pub fn start(&self) -> Result<(), BorthError> {
        let mut code = String::new();
        match std::fs::File::open(&self.path) {
            Ok(mut file) => {
                if file.read_to_string(&mut code).is_err() {
                    return Err(BorthError::CanNotReadCode);
                }

                let mut interpreter = Interpreter::with_stack_size(self.stack_size);
                interpreter.run_code(&code)?;
                interpreter.export_stack_to("stack.fth")?;
                Ok(())
            }
            _ => Err(BorthError::CanNotReadFile),
        }
    }
}

fn parse_args(args: &[String]) -> Result<(String, Option<usize>), BorthError> {
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
    fn expect_filename_in_args() {
        let mut args = Vec::from(["forth".into()]);
        assert!(parse_args(&args).is_err());

        args.push("ruta/a/main.fth".into());
        assert!(parse_args(&args).is_ok());
    }

    #[test]
    fn parse_stack_size() {
        let mut args = Vec::from(["forth".into()]);
        args.push("ruta/a/main.fth".into());
        args.push("--stack-size=10".into());

        assert!(parse_args(&args).is_ok_and(|(_, size)| size == Some(10)));
    }

    #[test]
    fn stack_size_is_optional() {
        let mut args = Vec::from(["forth".into()]);
        args.push("ruta/a/main.fth".into());

        assert!(parse_args(&args).is_ok_and(|(_, size)| size.is_none()));
    }
}
