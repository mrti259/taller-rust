use crate::{
    context::*,
    errors::*,
    expression::*,
    expression::{function::*, operation::*},
    stack::*,
};
use std::{collections::HashMap, rc::Rc};

pub struct BorthDict {
    words: HashMap<String, Rc<BorthExpression>>,
}

impl BorthDict {
    pub fn new() -> Self {
        let mut this = Self {
            words: HashMap::new(),
        };
        this.init_words();
        this
    }

    // initialization

    fn init_words(&mut self) {
        self.add("+", BorthExpression::Operation(add::call));
        self.add("-", BorthExpression::Operation(sub::call));
        self.add("*", BorthExpression::Operation(mul::call));
        self.add("/", BorthExpression::Operation(div::call));
        self.add("dup", BorthExpression::Operation(dup::call));
        self.add("drop", BorthExpression::Operation(drop::call));
        self.add("swap", BorthExpression::Operation(swap::call));
        self.add("over", BorthExpression::Operation(over::call));
        self.add("rot", BorthExpression::Operation(rot::call));
        self.add("=", BorthExpression::Operation(eq::call));
        self.add("<", BorthExpression::Operation(lt::call));
        self.add(">", BorthExpression::Operation(gt::call));
        self.add("and", BorthExpression::Operation(and::call));
        self.add("or", BorthExpression::Operation(or::call));
        self.add("not", BorthExpression::Operation(not::call));
        self.add(".", BorthExpression::Operation(dot::call));
        self.add("emit", BorthExpression::Operation(emit::call));
        self.add("cr", BorthExpression::Operation(cr::call));
        self.add(
            ".\"",
            BorthExpression::FunctionWithWhiteSpace(dot_quote::call),
        );
        self.add("if", BorthExpression::FunctionWithDict(if_then::call));
        self.add(":", BorthExpression::FunctionWithMutDict(word_def::call));
    }

    // word definition

    pub fn add(&mut self, token: &str, exp: BorthExpression) {
        self.words.insert(token.to_uppercase(), Rc::new(exp));
    }

    pub fn add_word(&mut self, token: &str, body: Vec<Rc<BorthExpression>>) {
        self.add(token, BorthExpression::Word(body));
    }

    // evaluation

    pub fn eval(&self, ctx: &mut BorthContext, token: &str) -> BorthResult<()> {
        if token.is_empty() {
            return Ok(());
        }
        match token.parse::<BorthItem>() {
            Ok(value) => ctx.push_value(value),
            _ => self.detect_and_eval_word(ctx, token),
        }
    }

    pub fn detect_word(&self, token: &str) -> BorthResult<Rc<BorthExpression>> {
        let key = &token.to_uppercase();
        if !self.words.contains_key(key) {
            return Err(BorthError::UnknownWord(token.to_string()));
        }
        let word = &self.words[key];
        Ok(Rc::clone(word))
    }

    fn detect_and_eval_word(&self, ctx: &mut BorthContext, token: &str) -> BorthResult<()> {
        let key = &token.to_uppercase();
        if !self.words.contains_key(key) {
            return Err(BorthError::UnknownWord(token.to_string()));
        }
        let word = &self.words[key];
        Self::eval_word(ctx, token, &word)
    }

    fn eval_word(ctx: &mut BorthContext, _token: &str, word: &BorthExpression) -> BorthResult<()> {
        match word {
            BorthExpression::Operation(cb) => cb(ctx),
            BorthExpression::FunctionWithWhiteSpace(cb) => {
                ctx.push_expression(BorthExpression::FunctionWithWhiteSpace(*cb));
                Ok(())
            }
            BorthExpression::FunctionWithDict(cb) => {
                ctx.push_expression(BorthExpression::FunctionWithDict(*cb));
                Ok(())
            }
            BorthExpression::FunctionWithMutDict(cb) => {
                ctx.push_expression(BorthExpression::FunctionWithMutDict(*cb));
                Ok(())
            }
            BorthExpression::Number(value) => ctx.push_value(*value),
            BorthExpression::Word(body) => {
                for exp in body.iter() {
                    Self::eval_word(ctx, _token, exp)?;
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::BorthItem;

    fn create_dict() -> BorthDict {
        BorthDict::new()
    }

    fn create_context() -> BorthContext {
        BorthContext::with_stack_size(10)
    }

    fn push_to_stack(ctx: &mut BorthContext, items: &[BorthItem]) {
        for item in items {
            let _ = ctx.push_value(*item);
        }
    }

    fn assert_context_equals(
        ctx: &BorthContext,
        stack: &[BorthItem],
        output: &str,
        words_stack: &[BorthExpression],
    ) {
        ctx.test(stack, output, words_stack);
    }

    #[test]
    fn test_stack_underflow() {
        let mut ctx = create_context();
        let dict = create_dict();
        assert_eq!(dict.eval(&mut ctx, "+"), Err(BorthError::StackUnderflow));
    }

    //arithmetic

    #[test]
    fn test_add() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[1, 2]);
        assert_eq!(dict.eval(&mut ctx, "+"), Ok(()));
        assert_context_equals(&ctx, &[3], "", &[]);
    }

    #[test]
    fn test_sub() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[3, 4]);
        assert_eq!(dict.eval(&mut ctx, "-"), Ok(()));
        assert_context_equals(&ctx, &[-1], "", &[]);
    }

    #[test]
    fn test_mul() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[3, 4]);
        assert_eq!(dict.eval(&mut ctx, "*"), Ok(()));
        assert_context_equals(&ctx, &[12], "", &[]);
    }

    #[test]
    fn test_div() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[8, 2]);
        assert_eq!(dict.eval(&mut ctx, "/"), Ok(()));
        assert_context_equals(&ctx, &[4], "", &[]);
    }

    //stack manipulation

    #[test]
    fn test_dup() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[2]);
        assert_eq!(dict.eval(&mut ctx, "dup"), Ok(()));
        assert_context_equals(&ctx, &[2, 2], "", &[]);
    }

    #[test]
    fn test_drop() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[1, 2, 3]);
        assert_eq!(dict.eval(&mut ctx, "drop"), Ok(()));
        assert_context_equals(&ctx, &[1, 2], "", &[]);
    }

    #[test]
    fn test_swap() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[8, 2]);
        assert_eq!(dict.eval(&mut ctx, "swap"), Ok(()));
        assert_context_equals(&ctx, &[2, 8], "", &[]);
    }

    #[test]
    fn test_over() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[8, 2]);
        assert_eq!(dict.eval(&mut ctx, "over"), Ok(()));
        assert_context_equals(&ctx, &[8, 2, 8], "", &[]);
    }

    #[test]
    fn test_rot() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[8, 2, 3]);
        assert_eq!(dict.eval(&mut ctx, "rot"), Ok(()));
        assert_context_equals(&ctx, &[2, 3, 8], "", &[]);
    }

    //booleans

    #[test]
    fn test_eq() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[1, 1]);
        assert_eq!(dict.eval(&mut ctx, "="), Ok(()));
        assert_context_equals(&ctx, &[-1], "", &[]);
    }

    #[test]
    fn test_lt() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[1, 2]);
        assert_eq!(dict.eval(&mut ctx, "<"), Ok(()));
        assert_context_equals(&ctx, &[-1], "", &[]);
    }

    #[test]
    fn test_gt() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[2, 3]);
        assert_eq!(dict.eval(&mut ctx, ">"), Ok(()));
        assert_context_equals(&ctx, &[0], "", &[]);
    }

    #[test]
    fn test_and() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[-1, -1]);
        assert_eq!(dict.eval(&mut ctx, "and"), Ok(()));
        assert_context_equals(&ctx, &[-1], "", &[]);
    }

    #[test]
    fn test_or() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[0, 0]);
        assert_eq!(dict.eval(&mut ctx, "or"), Ok(()));
        assert_context_equals(&ctx, &[0], "", &[]);
    }

    #[test]
    fn test_not() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[0]);
        assert_eq!(dict.eval(&mut ctx, "not"), Ok(()));
        assert_context_equals(&ctx, &[-1], "", &[]);
    }

    //output

    #[test]
    fn test_dot() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[8, 2, 3]);
        assert_eq!(dict.eval(&mut ctx, "."), Ok(()));
        assert_context_equals(&ctx, &[8, 2], "3", &[]);
    }

    #[test]
    fn test_emit() {
        let mut ctx = create_context();
        let dict = create_dict();
        push_to_stack(&mut ctx, &[97]);
        assert_eq!(dict.eval(&mut ctx, "emit"), Ok(()));
        assert_context_equals(&ctx, &[], "a", &[]);
    }

    #[test]
    fn test_cr() {
        let mut ctx = create_context();
        let dict = create_dict();
        assert_eq!(dict.eval(&mut ctx, "cr"), Ok(()));
        assert_context_equals(&ctx, &[], "\n", &[]);
    }

    #[test]
    fn test_dot_quote() {
        let mut ctx = create_context();
        let dict = create_dict();
        assert_eq!(dict.eval(&mut ctx, ".\""), Ok(()));
        assert_context_equals(
            &ctx,
            &[],
            "",
            &[BorthExpression::FunctionWithWhiteSpace(dot_quote::call)],
        );
    }

    // conditional

    #[test]
    fn test_if_else_then() {
        let mut ctx = create_context();
        let dict = create_dict();
        assert_eq!(dict.eval(&mut ctx, ".\""), Ok(()));
        assert_context_equals(
            &ctx,
            &[],
            "",
            &[BorthExpression::FunctionWithWhiteSpace(dot_quote::call)],
        );
    }

    // word definition

    #[test]
    fn test_add_word() {
        let mut ctx = BorthContext::with_stack_size(10);
        let mut dict = create_dict();
        dict.add_word(
            "foo",
            vec![
                Rc::new(BorthExpression::Number(1)),
                Rc::new(BorthExpression::Number(9)),
                Rc::new(BorthExpression::Operation(add::call)),
                Rc::new(BorthExpression::Number(5)),
            ],
        );

        //before call
        assert_context_equals(&ctx, &[], "", &[]);

        // after call
        assert_eq!(dict.eval(&mut ctx, "foo"), Ok(()));
        assert_context_equals(&ctx, &[10, 5], "", &[]);
    }

    #[test]
    fn test_add_word_with_conditional() {
        let mut ctx = BorthContext::with_stack_size(10);
        let mut dict = create_dict();
        dict.add_word(
            "foo",
            vec![
                Rc::new(BorthExpression::Number(1)),
                Rc::new(BorthExpression::Number(9)),
                Rc::new(BorthExpression::Operation(add::call)),
                Rc::new(BorthExpression::Number(5)),
            ],
        );

        //before call
        assert_context_equals(&ctx, &[], "", &[]);

        // after call
        assert_eq!(dict.eval(&mut ctx, "foo"), Ok(()));
        assert_context_equals(&ctx, &[10, 5], "", &[]);
    }

    #[test]
    fn test_case_insensitive() {
        let mut ctx = BorthContext::with_stack_size(10);
        let mut dict = create_dict();
        dict.add_word(
            "foo",
            vec![
                Rc::new(BorthExpression::Number(1)),
                Rc::new(BorthExpression::Number(9)),
                Rc::new(BorthExpression::Operation(add::call)),
                Rc::new(BorthExpression::Number(5)),
            ],
        );
        assert_eq!(dict.eval(&mut ctx, "FoO"), Ok(()));
        assert_context_equals(&ctx, &[10, 5], "", &[]);
    }
}
