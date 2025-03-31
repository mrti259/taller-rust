use std::rc::Rc;

use crate::{context::*, dict::BorthDict, errors::*, expression::BorthExpression, stack::*};

pub fn call(ctx: &mut BorthContext, dict: &mut BorthDict, token: &str) -> BorthResult<()> {
    match &mut ctx.new_word {
        None => {
            if token.parse::<BorthItem>().is_ok() {
                return Err(BorthError::InvalidWord);
            }
            ctx.new_word = Some((token.to_string(), Vec::new()));
            Ok(())
        }
        Some((name, body)) => {
            if token == ";" {
                if body.is_empty() {
                    return Err(BorthError::InvalidWord);
                }
                dict.add_word(name, body.to_vec());
                ctx.pop_expression();
                ctx.new_word = None;
                return Ok(());
            }
            let expression = if let Ok(value) = token.parse::<BorthItem>() {
                Rc::new(BorthExpression::Number(value))
            } else {
                dict.detect_word(token)?
            };
            body.push(expression);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_context() -> BorthContext {
        BorthContext::with_stack_size(10)
    }

    fn create_dict() -> BorthDict {
        BorthDict::new()
    }

    #[test]
    fn test1_word_def() {
        let mut ctx = create_context();
        let mut dict = create_dict();
        assert_eq!(call(&mut ctx, &mut dict, "foo"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, "1"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, ";"), Ok(()));
        ctx.test(&[], "", &[]);

        assert_eq!(dict.eval(&mut ctx, "foo"), Ok(()));
        ctx.test(&[1], "", &[]);
    }

    #[test]
    fn test2_invalid_word() {
        let mut ctx = create_context();
        let mut dict = create_dict();
        assert_eq!(call(&mut ctx, &mut dict, "1"), Err(BorthError::InvalidWord));
    }

    #[test]
    fn test3_invalid_word() {
        let mut ctx = create_context();
        let mut dict = create_dict();
        assert_eq!(call(&mut ctx, &mut dict, "foo"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, ";"), Err(BorthError::InvalidWord));
    }

    #[test]
    fn test4_word_def() {
        let mut ctx = create_context();
        let mut dict = create_dict();
        assert_eq!(call(&mut ctx, &mut dict, "dup-twice"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, "dup"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, "dup"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, ";"), Ok(()));

        let _ = ctx.push_value(1);
        assert_eq!(dict.eval(&mut ctx, "dup-twice"), Ok(()));
        ctx.test(&[1, 1, 1], "", &[]);
    }

    #[test]
    fn test5_word_def() {
        let mut ctx = create_context();
        let mut dict = create_dict();
        assert_eq!(call(&mut ctx, &mut dict, "countup"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, "1"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, "2"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, "3"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, ";"), Ok(()));

        assert_eq!(dict.eval(&mut ctx, "countup"), Ok(()));
        ctx.test(&[1, 2, 3], "", &[]);
    }

    #[test]
    fn test6_word_redefinition() {
        let mut ctx = create_context();
        let mut dict = create_dict();
        assert_eq!(call(&mut ctx, &mut dict, "foo"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, "dup"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, ";"), Ok(()));

        assert_eq!(call(&mut ctx, &mut dict, "foo"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, "dup"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, "dup"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, ";"), Ok(()));

        let _ = ctx.push_value(1);
        assert_eq!(dict.eval(&mut ctx, "foo"), Ok(()));
        ctx.test(&[1, 1, 1], "", &[]);
    }

    #[test]
    fn test7_shadowing() {
        let mut ctx = create_context();
        let mut dict = create_dict();
        assert_eq!(call(&mut ctx, &mut dict, "swap"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, "dup"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, ";"), Ok(()));

        let _ = ctx.push_value(1);
        assert_eq!(dict.eval(&mut ctx, "swap"), Ok(()));
        ctx.test(&[1, 1], "", &[]);
    }

    #[test]
    fn test8_shadowing_symbol() {
        let mut ctx = create_context();
        let mut dict = create_dict();
        assert_eq!(call(&mut ctx, &mut dict, "+"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, "*"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, ";"), Ok(()));

        let _ = ctx.push_value(3);
        let _ = ctx.push_value(4);
        assert_eq!(dict.eval(&mut ctx, "+"), Ok(()));
        ctx.test(&[12], "", &[]);
    }

    #[test]
    fn test_hello_world() {
        let mut ctx = create_context();
        let mut dict = create_dict();
        assert_eq!(call(&mut ctx, &mut dict, "hello"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, ".\""), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, "hello"), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, "world\""), Ok(()));
        assert_eq!(call(&mut ctx, &mut dict, ";"), Ok(()));
        ctx.test(&[], "", &[]);

        assert_eq!(dict.eval(&mut ctx, "hello"), Ok(()));
        ctx.test(&[], "hello world", &[]);
    }
}
