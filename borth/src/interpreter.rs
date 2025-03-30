pub use crate::{context::*, dict::*, errors::*, expression::*, stack::*};
use std::io::Write;

pub struct BorthInterpreter {
    dict: BorthDict,
    ctx: BorthContext,
}

impl BorthInterpreter {
    pub fn with_stack_size(stack_size: usize) -> Self {
        Self {
            dict: BorthDict::new(),
            ctx: BorthContext::with_stack_size(stack_size),
        }
    }

    pub fn export_stack_to(&self, file: &mut impl Write) -> BorthResult<()> {
        let items = self.ctx.stack_items();
        let len = items.len();
        for (i, item) in items.iter().enumerate() {
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

    pub fn eval(&mut self, code: &str, writer: &mut impl Write) -> BorthResult<()> {
        let run_result = self.run_code(code);
        let writer_result = self.ctx.write_output(writer);
        run_result.and(writer_result)
    }

    fn run_code(&mut self, code: &str) -> BorthResult<()> {
        let mut whitespaces = code.match_indices(char::is_whitespace);
        let mut offset = 0;
        while offset < code.len() {
            let (stop, whitespace) = match whitespaces.next() {
                Some(result) => result,
                None => (code.len(), ""),
            };
            let token = match code.get(offset..stop) {
                Some(token) => token,
                None => code,
            };
            self.process_token(token, whitespace)?;
            offset = stop + 1;
        }
        Ok(())
    }

    fn process_token(&mut self, token: &str, whitespace: &str) -> BorthResult<()> {
        match self.ctx.last_expression() {
            Some(BorthExpression::FunctionWithWhiteSpace(callback)) => {
                callback(&mut self.ctx, token, whitespace)
            }
            Some(BorthExpression::FunctionWithDict(callback)) => {
                callback(&mut self.ctx, &self.dict, token)
            }
            Some(BorthExpression::FunctionWithMutDict(callback)) => {
                callback(&mut self.ctx, &mut self.dict, token)
            }
            None => self.detect_operation(token),
            _ => Err(BorthError::RuntimeError),
        }
    }

    fn detect_operation(&mut self, token: &str) -> BorthResult<()> {
        if token.is_empty() {
            return Ok(());
        }
        match token.parse::<BorthItem>() {
            Ok(item) => self.ctx.push_value(item),
            _ => self.dict.eval(&mut self.ctx, token),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    fn create_interpreter() -> BorthInterpreter {
        BorthInterpreter::with_stack_size(20)
    }

    fn assert_stack_equals(interpreter: &BorthInterpreter, items: &[BorthItem]) {
        assert_eq!(interpreter.ctx.stack_items(), items);
    }

    fn run_code_and_assert_stack_equals(code: &str, stack: &[BorthItem]) {
        let mut interpreter = create_interpreter();
        interpreter.run_code(code).expect("Valid testing code");
        assert_stack_equals(&interpreter, stack);
    }

    fn run_code_and_assert_output_equals(code: &str, content: &str) {
        let mut output = Cursor::new(Vec::<u8>::new());
        let mut interpreter = create_interpreter();
        interpreter
            .eval(code, &mut output)
            .expect("Valid testing code");
        let mut buf = Default::default();
        output.set_position(0);
        let _ = output.read_to_string(&mut buf);
        assert_eq!(buf, content);
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
        let mut interpreter = create_interpreter();
        assert!(interpreter.run_code("1 2 3 + 1 2").is_ok());

        let mut file = Cursor::new(Vec::new());
        assert!(interpreter.export_stack_to(&mut file).is_ok());

        let mut buf = Default::default();
        let _ = file.set_position(0);
        let _ = file.read_to_string(&mut buf);
        assert_eq!(buf, "1 5 1 2");
    }

    #[test]
    fn test04_stop_at_error() {
        let mut interpreter = create_interpreter();
        assert!(interpreter.run_code("1 2 3 UNKNOWN + 4 5 6 + ").is_err());
        assert_stack_equals(&interpreter, &[1, 2, 3]);
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
        run_code_and_assert_stack_equals("4 5 -", &[-1]);
    }

    #[test]
    fn test08_prod() {
        run_code_and_assert_stack_equals("4 5 *", &[20]);
    }

    #[test]
    fn test09_div() {
        run_code_and_assert_stack_equals("12 4 /", &[3]);
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
        run_code_and_assert_stack_equals("2 1 <", &[0]);
        run_code_and_assert_stack_equals("1 2 <", &[-1]);
    }

    #[test]
    fn test17_gt() {
        run_code_and_assert_stack_equals("1 2 >", &[0]);
        run_code_and_assert_stack_equals("2 1 >", &[-1]);
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
        run_code_and_assert_stack_equals("10 NOT NOT", &[-1]);
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
        run_code_and_assert_output_equals(code, "W o w !");
    }

    #[test]
    fn test22_emit() {
        run_code_and_assert_output_equals("CR", "\n");
    }

    #[test]
    fn test23_output_string() {
        run_code_and_assert_output_equals(".\" Hello World!\"", "Hello World!");
    }

    #[test]
    fn test24_if_then() {
        run_code_and_assert_stack_equals("0 IF 1 THEN", &[]);
        run_code_and_assert_stack_equals("0 IF 1 2 + THEN", &[]);
        run_code_and_assert_stack_equals("-1 IF 2 THEN", &[2]);
        run_code_and_assert_stack_equals("-1 IF 1 2 + THEN", &[3]);
    }

    #[test]
    fn test25_if_else_then() {
        run_code_and_assert_stack_equals("0 IF 1 ELSE 2 THEN", &[2]);
        run_code_and_assert_stack_equals("0 IF 1 ELSE 1 2 + THEN", &[3]);
        run_code_and_assert_stack_equals("-1 IF ELSE 3 THEN", &[]);
        run_code_and_assert_stack_equals("-1 IF ELSE 1 2 3 + THEN", &[]);
    }

    #[test]
    fn test26_define_word() {
        run_code_and_assert_stack_equals(": foo 1 ;", &[]);
        run_code_and_assert_stack_equals(": foo 1 ; foo", &[1]);
    }

    #[test]
    fn test27_define_word() {
        run_code_and_assert_stack_equals(
            " : MAX OVER OVER < IF SWAP THEN DROP ;\n10 20 MAX ",
            &[20],
        );
    }
}
