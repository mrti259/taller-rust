use crate::errors::*;
use std::{fmt::Display, fs::File, io::Write};

type StackElementType = i16;
type Stack = Vec<StackElementType>;

pub struct Interpreter<Writer: Write> {
    stack: Stack,
    writer: Writer,
}

impl<Writer: Write> Interpreter<Writer> {
    pub fn with_stack_size(size: usize, writer: Writer) -> Self {
        Self {
            stack: Vec::with_capacity(size),
            writer,
        }
    }

    pub fn export_stack_to(&self, path_to_file: &str) -> BorthResult<()> {
        if let Ok(mut file) = File::create(path_to_file) {
            for i in 0..self.stack.len() {
                let mut value = self.stack[i].to_string();
                if i < self.stack.len() - 1 {
                    value.push(' ');
                }
                if file.write(value.as_bytes()).is_err() {
                    return Err(BorthError::CanNotWriteFile);
                }
            }
            Ok(())
        } else {
            Err(BorthError::CanNotWriteFile)
        }
    }

    pub fn run_code(&mut self, code: &str) -> BorthResult<()> {
        let mut op_stack = Vec::<String>::new();
        let mut stops = code.match_indices(char::is_whitespace);
        let mut offset = 0;
        while offset < code.len() {
            let stop = stops.next();
            let pos = stop.map_or(code.len(), |s| s.0);
            let whitespace = stop.map_or("", |s| s.1);
            if let Some(token) = code.get(offset..pos) {
                self.detect_operation(token, whitespace, &mut op_stack)?;
            }
            offset = pos + 1;
        }
        Ok(())
    }

    fn detect_operation(
        &mut self,
        token: &str,
        whitespace: &str,
        op_stack: &mut Vec<String>,
    ) -> BorthResult<()> {
        if let Some(last_op) = op_stack.last() {
            if last_op == ".\"" {
                if token.ends_with("\"") {
                    op_stack.pop();
                    self.write(token.trim_end_matches("\""))?;
                } else {
                    self.write(token)?;
                    self.write(whitespace)?;
                }
                return Ok(());
            }
        }
        match token.trim().to_uppercase().as_str() {
            "" => Ok(()),
            "+" => self.sum(),
            "-" => self.sub(),
            "*" => self.prod(),
            "/" => self.div(),
            "DUP" => self.dup(),
            "DROP" => self.drop(),
            "SWAP" => self.swap(),
            "OVER" => self.over(),
            "ROT" => self.rot(),
            "=" => self.eq(),
            "<" => self.lt(),
            ">" => self.gt(),
            "AND" => self.and(),
            "OR" => self.or(),
            "NOT" => self.not(),
            "." => {
                let element = self.pop()?;
                self.write(element)
            }
            "EMIT" => {
                let element = self.pop()?;
                let ascii = char::from_u32(element as u32).ok_or(BorthError::RuntimeError)?;
                self.write(ascii)
            }
            "CR" => self.write("\n"),
            ".\"" => {
                op_stack.push(token.to_string());
                Ok(())
            }
            "\"" => todo!(),
            "IF" => todo!(),
            "ELSE" => todo!(),
            "THEN" => todo!(),
            _ => self.push(token),
        }
    }

    fn write<T: Display>(&mut self, sth: T) -> BorthResult<()> {
        if write!(self.writer, "{}", sth).is_err() {
            return Err(BorthError::CanNotWriteToOutput);
        }
        Ok(())
    }

    fn push(&mut self, token: &str) -> BorthResult<()> {
        match token.parse::<StackElementType>() {
            Ok(element) => {
                self.stack.push(element);
                Ok(())
            }
            _ => Err(BorthError::UnknownWord),
        }
    }

    fn pop(&mut self) -> BorthResult<StackElementType> {
        self.stack.pop().ok_or(BorthError::StackUnderflow)
    }

    fn sum(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let result = element1 + element2;
        self.stack.push(result);
        Ok(())
    }

    fn sub(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let result = element1 - element2;
        self.stack.push(result);
        Ok(())
    }

    fn prod(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let result = element1 * element2;
        self.stack.push(result);
        Ok(())
    }

    fn div(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let result = element1 / element2;
        self.stack.push(result);
        Ok(())
    }

    fn dup(&mut self) -> BorthResult<()> {
        let element = self.pop()?;
        self.stack.push(element);
        self.stack.push(element);
        Ok(())
    }

    fn drop(&mut self) -> BorthResult<()> {
        self.pop()?;
        Ok(())
    }

    fn swap(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        self.stack.push(element1);
        self.stack.push(element2);
        Ok(())
    }

    fn over(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        self.stack.push(element2);
        self.stack.push(element1);
        self.stack.push(element2);
        Ok(())
    }

    fn rot(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let element3 = self.pop()?;
        self.stack.push(element2);
        self.stack.push(element1);
        self.stack.push(element3);
        Ok(())
    }

    fn eq(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let result = if element1 == element2 { -1 } else { 0 };
        self.stack.push(result);
        Ok(())
    }

    fn lt(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let result = if element1 < element2 { -1 } else { 0 };
        self.stack.push(result);
        Ok(())
    }

    fn gt(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let result = if element1 > element2 { -1 } else { 0 };
        self.stack.push(result);
        Ok(())
    }

    fn and(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let result = element1 & element2;
        self.stack.push(result);
        Ok(())
    }

    fn or(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let result = element1 | element2;
        self.stack.push(result);
        Ok(())
    }

    fn not(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let result = !element1;
        self.stack.push(result);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::{File, remove_file},
        io::{Cursor, Read},
        path::Path,
    };

    fn create_interpreter() -> Interpreter<Cursor<Vec<u8>>> {
        Interpreter::with_stack_size(10, Cursor::new(Vec::new()))
    }

    fn delete_file_if_exists(path_to_file: &str) {
        if Path::new(path_to_file).exists() {
            let _ = remove_file(path_to_file);
        }
    }

    fn run_code_and_assert_stack_equals(code: &str, stack: &[StackElementType]) {
        let mut interpreter = create_interpreter();
        interpreter.run_code(code).expect("Valid testing code");
        assert_eq!(&interpreter.stack, stack);
    }

    fn run_code_and_assert_output_equals(code: &str, output: &str) {
        let mut interpreter = create_interpreter();
        interpreter.run_code(code).expect("Valid testing code");
        let mut buf = Default::default();
        interpreter.writer.set_position(0);
        assert!(interpreter.writer.read_to_string(&mut buf).is_ok());
        assert_eq!(buf, output);
    }

    #[test]
    fn test01_push_to_stack() {
        run_code_and_assert_stack_equals("1 2 3", &[1, 2, 3]);
    }

    #[test]
    fn test02_perform_operation() {
        run_code_and_assert_stack_equals("1 2 +", &[3]);
    }

    #[test]
    fn test03_export_stack() {
        let path_to_file = "stack.fth";
        delete_file_if_exists(path_to_file);

        let mut interpreter = create_interpreter();
        assert!(interpreter.run_code("1 2 3 + 1 2").is_ok());
        assert_eq!(interpreter.stack, [1, 5, 1, 2]);

        assert!(interpreter.export_stack_to(path_to_file).is_ok());

        let mut file = File::open(path_to_file).expect("File exists.");
        let mut content = String::new();
        assert!(file.read_to_string(&mut content).is_ok());
        let stack_read = content
            .split(" ")
            .filter_map(|element| element.parse::<StackElementType>().ok())
            .collect::<Stack>();
        assert_eq!(stack_read, interpreter.stack);
    }

    #[test]
    fn test04_stop_at_error() {
        let mut forth = create_interpreter();
        assert!(forth.run_code("1 2 3 UNKNOWN + 4 5 6 + ").is_err());
        assert_eq!(forth.stack, [1, 2, 3]);
    }

    #[test]
    fn test05_ignore_whitespaces() {
        run_code_and_assert_stack_equals("1 2\n\n 3\n \n4\n 5            6", &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test06_sum() {
        run_code_and_assert_stack_equals("4 5 +", &[9]);
    }

    #[test]
    fn test07_sub() {
        run_code_and_assert_stack_equals("4 5 -", &[1]);
    }

    #[test]
    fn test08_prod() {
        run_code_and_assert_stack_equals("4 5 *", &[20]);
    }

    #[test]
    fn test09_div() {
        run_code_and_assert_stack_equals("4 9 /", &[2]);
    }

    #[test]
    fn test10_dup() {
        run_code_and_assert_stack_equals("5 DUP", &[5, 5]);
    }

    #[test]
    fn test11_drop() {
        run_code_and_assert_stack_equals("1 2 3 DROP", &[1, 2]);
    }

    #[test]
    fn test12_swap() {
        run_code_and_assert_stack_equals("1 2 3 SWAP", &[1, 3, 2]);
    }

    #[test]
    fn test13_over() {
        run_code_and_assert_stack_equals("1 2 3 OVER", &[1, 2, 3, 2]);
    }

    #[test]
    fn test14_rot() {
        run_code_and_assert_stack_equals("1 2 3 ROT", &[2, 3, 1]);
    }

    #[test]
    fn test15_eq() {
        run_code_and_assert_stack_equals("1 2 =", &[0]);
        run_code_and_assert_stack_equals("2 2 =", &[-1]);
    }

    #[test]
    fn test16_lt() {
        run_code_and_assert_stack_equals("1 2 <", &[0]);
        run_code_and_assert_stack_equals("2 1 <", &[-1]);
    }

    #[test]
    fn test17_gt() {
        run_code_and_assert_stack_equals("2 1 >", &[0]);
        run_code_and_assert_stack_equals("1 2 >", &[-1]);
    }

    #[test]
    fn test18_and() {
        run_code_and_assert_stack_equals("0 0 AND", &[0]);
        run_code_and_assert_stack_equals("0 -1 AND", &[0]);
        run_code_and_assert_stack_equals("-1 0 AND", &[0]);
        run_code_and_assert_stack_equals("-1 -1 AND", &[-1]);
    }

    #[test]
    fn test19_or() {
        run_code_and_assert_stack_equals("0 0 OR", &[0]);
        run_code_and_assert_stack_equals("0 -1 OR", &[-1]);
        run_code_and_assert_stack_equals("-1 0 OR", &[-1]);
        run_code_and_assert_stack_equals("-1 -1 OR", &[-1]);
    }

    #[test]
    fn test19_not() {
        run_code_and_assert_stack_equals("0 NOT", &[-1]);
        run_code_and_assert_stack_equals("-1 NOT", &[0]);
    }

    #[test]
    fn test20_dot() {
        let code = "0 .";
        run_code_and_assert_stack_equals(code, &[]);
        run_code_and_assert_output_equals(code, "0");
    }

    #[test]
    fn test21_emit() {
        let code = "33 119 111 87 emit emit emit emit";
        run_code_and_assert_stack_equals(code, &[]);
        run_code_and_assert_output_equals(code, "Wow!");
    }

    #[test]
    fn test22_emit() {
        run_code_and_assert_output_equals("CR", "\n");
    }

    #[test]
    fn test23_output_string() {
        run_code_and_assert_output_equals(".\" Hello World!\"", "Hello World!");
    }
}
