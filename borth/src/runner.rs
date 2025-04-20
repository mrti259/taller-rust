use crate::{errors::*, interpreter::*, stack::BorthItem};
use std::{
    fs::File,
    io::{Read, Write},
};

pub struct BorthRunner {
    code_path: String,
    stack_size: usize,
}

impl BorthRunner {
    pub fn from_args(args: &[String]) -> BorthResult<Self> {
        let (code_path, stack_size) = parse_args(args)?;
        let stack_size = stack_size.unwrap_or(128_000);
        Ok(Self {
            code_path,
            stack_size,
        })
    }

    pub fn start(&self, stack_file: &str, writer: &mut impl Write) -> BorthResult<()> {
        let code = get_code_from_file(&self.code_path)?;
        let mut interpreter = BorthInterpreter::with_stack_size(self.stack_size);
        let (stack, output) = interpreter.run_code(&code);
        let save_result = save_stack_to_file(stack, stack_file);
        let write_result = write_output(writer, output);
        save_result.and(write_result)
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

fn get_code_from_file(path: &str) -> BorthResult<String> {
    match File::open(path) {
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

fn save_stack_to_file(stack: &[BorthItem], path_to_file: &str) -> BorthResult<()> {
    match File::create(path_to_file) {
        Ok(mut file) => {
            let len = stack.len();
            for (i, item) in stack.iter().enumerate() {
                let mut buf = item.to_string();
                if i < len - 1 {
                    buf.push(' ');
                }
                if file.write(buf.as_bytes()).is_err() {
                    return Err(BorthError::CanNotWriteFile);
                }
            }
            Ok(())
        }
        _ => Err(BorthError::CanNotWriteFile),
    }
}

fn write_output(writer: &mut impl Write, output: &str) -> BorthResult<()> {
    write!(writer, "{}", output).or(Err(BorthError::CanNotWriteToOutput))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn create_args() -> Vec<String> {
        vec!["forth".into()]
    }

    fn create_runner(args: &Vec<String>) -> BorthResult<BorthRunner> {
        BorthRunner::from_args(args)
    }

    fn create_writer() -> impl Write {
        Vec::new()
    }

    fn run(runner: BorthRunner) -> BorthResult<()> {
        runner.start("/tmp/borth-test-stack.fth", &mut create_writer())
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
        let runner = create_runner(&args);

        assert!(runner.is_ok());
        assert!(runner.is_ok_and(|r| run(r).is_ok()));
    }

    #[test]
    fn test5_runner_can_not_read_file() {
        let mut args = create_args();
        args.push("./fth-examples/0.fth".into());
        let runner = create_runner(&args);

        assert!(runner.is_ok());
        assert_eq!(runner.and_then(|r| run(r)), Err(BorthError::CanNotReadFile));
    }

    #[test]
    fn test6_runner_can_not_write_stack() {
        let mut args = create_args();
        args.push("./fth-examples/3.fth".into());
        let runner = create_runner(&args);

        assert!(runner.is_ok());
        assert_eq!(
            runner.and_then(|r| r.start("/tmp/borth/test-stack.fth", &mut create_writer())),
            Err(BorthError::CanNotWriteFile)
        );
    }

    #[test]
    fn test7_runner_can_not_write_output() {
        let mut args = create_args();
        args.push("./fth-examples/3.fth".into());
        let runner = create_runner(&args);
        let mut writer = Cursor::new([]);

        assert!(runner.is_ok());
        assert_eq!(
            runner.and_then(|r| r.start("/tmp/borth-test-stack.fth", &mut writer)),
            Err(BorthError::CanNotWriteToOutput)
        );
    }
}
