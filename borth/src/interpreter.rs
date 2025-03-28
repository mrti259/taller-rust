use crate::{errors::*, stack::*};
use std::{fmt::Display, fs::File, io::Write};

type BorthItem = i16;

pub struct BorthInterpreter<Output: Write> {
    stack: BorthStack<BorthItem>,
    output: Output,
}

impl<Writer: Write> BorthInterpreter<Writer> {
    pub fn with_stack_size(size: usize, output: Writer) -> Self {
        Self {
            stack: BorthStack::with_capacity(size),
            output,
        }
    }

    pub fn export_stack_to(&mut self, path_to_file: &str) -> BorthResult<()> {
        if let Ok(mut file) = File::create(path_to_file) {
            self.stack.reverse();
            while let Ok(item) = self.stack.pop() {
                let mut value = item.to_string();
                if self.stack.len() > 0 {
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
                let item = self.stack.pop()?;
                self.write(item)
            }
            "EMIT" => {
                let item = self.stack.pop()?;
                let ascii = char::from_u32(item as u32).ok_or(BorthError::RuntimeError)?;
                self.write(ascii)
            }
            "CR" => self.write("\n"),
            ".\"" => {
                op_stack.push(token.to_string());
                Ok(())
            }
            "IF" => todo!(),
            "ELSE" => todo!(),
            "THEN" => todo!(),
            _ => self.push(token),
        }
    }

    fn write<T: Display>(&mut self, sth: T) -> BorthResult<()> {
        if write!(self.output, "{}", sth).is_err() {
            return Err(BorthError::CanNotWriteToOutput);
        }
        Ok(())
    }

    fn push(&mut self, token: &str) -> BorthResult<()> {
        match token.parse::<BorthItem>() {
            Ok(item) => self.stack.push(item),
            _ => Err(BorthError::UnknownWord),
        }
    }

    fn sum(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = item1 + item2;
        self.stack.push(result)
    }

    fn sub(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = item1 - item2;
        self.stack.push(result)
    }

    fn prod(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = item1 * item2;
        self.stack.push(result)
    }

    fn div(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = item1 / item2;
        self.stack.push(result)
    }

    fn dup(&mut self) -> BorthResult<()> {
        let item = self.stack.pop()?;
        self.stack.push(item)?;
        self.stack.push(item)
    }

    fn drop(&mut self) -> BorthResult<()> {
        self.stack.pop()?;
        Ok(())
    }

    fn swap(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        self.stack.push(item1)?;
        self.stack.push(item2)
    }

    fn over(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        self.stack.push(item2)?;
        self.stack.push(item1)?;
        self.stack.push(item2)
    }

    fn rot(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let item3 = self.stack.pop()?;
        self.stack.push(item2)?;
        self.stack.push(item1)?;
        self.stack.push(item3)
    }

    fn eq(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = if item1 == item2 { -1 } else { 0 };
        self.stack.push(result)
    }

    fn lt(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = if item1 < item2 { -1 } else { 0 };
        self.stack.push(result)
    }

    fn gt(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = if item1 > item2 { -1 } else { 0 };
        self.stack.push(result)
    }

    fn and(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = item1 & item2;
        self.stack.push(result)
    }

    fn or(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = item1 | item2;
        self.stack.push(result)
    }

    fn not(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let result = !item1;
        self.stack.push(result)
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

    type Output = Cursor<Vec<u8>>;

    fn create_interpreter() -> BorthInterpreter<Output> {
        BorthInterpreter::with_stack_size(10, Cursor::new(Vec::new()))
    }

    fn delete_file_if_exists(path_to_file: &str) {
        if Path::new(path_to_file).exists() {
            let _ = remove_file(path_to_file);
        }
    }

    fn assert_stack_equals(interpreter: BorthInterpreter<Output>, items: &[BorthItem]) {
        let mut stack = interpreter.stack;
        stack.reverse();
        for item in items {
            match stack.pop() {
                Ok(value) => assert_eq!(&value, item),
                _ => assert!(false),
            }
        }
    }

    fn run_code_and_assert_stack_equals(code: &str, stack: &[BorthItem]) {
        let mut interpreter = create_interpreter();
        interpreter.run_code(code).expect("Valid testing code");
        assert_stack_equals(interpreter, stack);
    }

    fn run_code_and_assert_output_equals(code: &str, output: &str) {
        let mut interpreter = create_interpreter();
        interpreter.run_code(code).expect("Valid testing code");
        let mut buf = Default::default();
        interpreter.output.set_position(0);
        assert!(interpreter.output.read_to_string(&mut buf).is_ok());
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

        assert!(interpreter.export_stack_to(path_to_file).is_ok());

        let mut file = File::open(path_to_file).expect("File exists.");
        let mut buf = String::default();
        assert!(file.read_to_string(&mut buf).is_ok());
        let stack_read = buf
            .split_whitespace()
            .filter_map(|item| item.parse::<BorthItem>().ok())
            .collect::<Vec<BorthItem>>();
        assert_eq!(stack_read, [1, 5, 1, 2]);
    }

    #[test]
    fn test04_stop_at_error() {
        let mut interpreter = create_interpreter();
        assert!(interpreter.run_code("1 2 3 UNKNOWN + 4 5 6 + ").is_err());
        assert_stack_equals(interpreter, &[1, 2, 3]);
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
