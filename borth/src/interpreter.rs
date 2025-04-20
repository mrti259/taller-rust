use super::{context::*, dict::*, errors::*};
use crate::{parser, stack::BorthItem};

pub struct BorthInterpreter {
    ctx: BorthContext,
    dict: BorthDict,
}

impl BorthInterpreter {
    pub fn with_stack_size(stack_size: usize) -> Self {
        Self {
            ctx: BorthContext::with_stack_size(stack_size),
            dict: BorthDict::new(),
        }
    }

    pub fn run_code(&mut self, code: &str) -> (&[BorthItem], &str) {
        if let Err(err) = self.eval(code) {
            self.ctx.print(&format!("{}", err));
        }
        (self.ctx.stack_items(), self.ctx.output())
    }

    fn eval(&mut self, code: &str) -> BorthResult<()> {
        let tokens = parser::parse_tokens(code);
        let expressions = parser::parse_expressions(tokens, &mut self.dict);
        for exp in expressions {
            exp.eval(&mut self.ctx)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::BorthItem;

    fn create_interpreter() -> BorthInterpreter {
        BorthInterpreter::with_stack_size(20)
    }

    fn run_code_and_assert_stack_equals(code: &str, expected: &[BorthItem]) {
        let mut interpreter = create_interpreter();
        let (stack, _) = interpreter.run_code(code);
        assert_eq!(stack, expected);
    }

    fn run_code_and_assert_output_equals(code: &str, expected: &str) {
        let mut interpreter = create_interpreter();
        let (_, output) = interpreter.run_code(code);
        assert_eq!(output, expected);
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
    fn test03_stop_at_error_and_output_error() {
        let mut interpreter = create_interpreter();
        let (stack, output) = interpreter.run_code("1 2 3 UNKNOWN + 4 5 6 + ");
        assert_eq!(stack, &[1, 2, 3]);
        assert_eq!(output, "?");
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

    #[test]
    fn test_non_transitive() {
        run_code_and_assert_stack_equals(
            " : foo 5 ;
            : bar foo ;
            : foo 6 ;
            bar foo ",
            &[5, 6],
        );
    }

    #[test]
    fn test_heavy() {
        run_code_and_assert_stack_equals(
            " : word1 1 ;
            : word2 word1 word1 ;
            : word4 word2 word2 ;
            : word8 word4 word4 ;
            : word16 word8 word8 ;
            : word32 word16 word16 ;
            : word64 word32 word32 ;
            : word128 word64 word64 ;
            : word256 word128 word128 ;
            : word512 word256 word256 ;
            : word1024 word512 word512 ;
            : word2048 word1024 word1024 ;
            : word4096 word2048 word2048 ;
            : word8192 word4096 word4096 ;
            : word16384 word8192 word8192 ;
            : word32768 word16384 word16384 ;
            : word65536 word32768 word32768 ;
            : word131072 word65536 word65536 ;
            : word262144 word131072 word131072 ;
            : word524288 word262144 word262144 ;
            : word1048576 word524288 word524288 ;
            : word2097152 word1048576 word1048576 ;
            : word4194304 word2097152 word2097152 ;
            : word8388608 word4194304 word4194304 ;
            : word16777216 word8388608 word8388608 ;
            : word33554432 word16777216 word16777216 ;
            : word67108864 word33554432 word33554432 ;
            : word134217728 word67108864 word67108864 ;
            ",
            &[],
        );
    }

    #[test]
    fn test_nested_if() {
        run_code_and_assert_stack_equals(
            " : f
                if
                    if 1 else 2 then
                else
                    drop 3
                then ;
            -1 -1 f
            0 -1 f
            0 0 f ",
            &[1, 2, 3],
        );
    }

    #[test]
    fn test_nested_if_else() {
        run_code_and_assert_stack_equals(
            "
            : f
            dup 0 = if
                drop 2
            else dup 1 = if
                drop 3
            else
                drop 4
            then then ;
            0 f
            1 f
            2 f
            ",
            &[2, 3, 4],
        );
    }
}
