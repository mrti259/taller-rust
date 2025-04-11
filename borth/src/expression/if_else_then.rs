use super::BorthExpression;
use crate::{context::*, dict::BorthDict, errors::*, parser::*};
use std::rc::Rc;

pub fn create(iterator: &mut BorthIterator, dict: &BorthDict) -> BorthExpression {
    let mut if_block = vec![];
    let mut else_block = vec![];

    let mut in_else_block = false;
    for (token, _) in iterator.by_ref() {
        match token.to_lowercase().as_str() {
            "then" => return BorthExpression::IfElseThen(if_block, else_block),
            "else" => {
                in_else_block = true;
            }
            _ => {
                let exp = dict.detect(token);
                if in_else_block {
                    else_block.push(exp);
                } else {
                    if_block.push(exp);
                }
            }
        }
    }
    BorthExpression::IncompleteStatement
}

pub fn call(
    ctx: &mut BorthContext,
    if_block: &Vec<Rc<BorthExpression>>,
    else_block: &Vec<Rc<BorthExpression>>,
) -> BorthResult<()> {
    let block_to_eval = if ctx.pop_value()? != 0 {
        if_block
    } else {
        else_block
    };
    for exp in block_to_eval {
        exp.eval(ctx)?
    }
    Ok(())
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

    fn assert_create_and_call(ctx: &mut BorthContext, tokens: Vec<(&str, &str)>) {
        let dict = create_dict();
        assert!(match create(&mut tokens.iter(), &dict) {
            BorthExpression::IfElseThen(if_block, else_block) =>
                call(ctx, &if_block, &else_block).is_ok(),
            _ => false,
        });
    }

    fn assert_incomplete_statement(tokens: Vec<(&str, &str)>) {
        let dict = create_dict();
        assert!(match create(&mut tokens.iter(), &dict) {
            BorthExpression::IncompleteStatement => true,
            _ => false,
        });
    }

    #[test]
    fn test1_if_then_false() {
        let tokens = parse_tokens("1 then");
        let mut ctx = create_context();
        let _ = ctx.push_value(0);
        assert_create_and_call(&mut ctx, tokens);
        ctx.test(&[], "");
    }

    #[test]
    fn test2_if_then_true() {
        let tokens = parse_tokens("1 then");
        let mut ctx = create_context();
        let _ = ctx.push_value(-1);
        assert_create_and_call(&mut ctx, tokens);
        ctx.test(&[1], "");
    }

    #[test]
    fn test3_if_then_open() {
        let tokens = parse_tokens("1");
        assert_incomplete_statement(tokens);
    }

    #[test]
    fn test4_if_else_then_false() {
        let tokens = parse_tokens("0 else 1 then");
        let mut ctx = create_context();
        let _ = ctx.push_value(0);
        assert_create_and_call(&mut ctx, tokens);
        ctx.test(&[1], "");
    }

    #[test]
    fn test5_if_else_then_true() {
        let tokens = parse_tokens("0 else 1 then");
        let mut ctx = create_context();
        let _ = ctx.push_value(-1);
        assert_create_and_call(&mut ctx, tokens);
        ctx.test(&[0], "");
    }

    #[test]
    fn test6_if_else_then_open() {
        let tokens = parse_tokens("0 else 1 ");
        assert_incomplete_statement(tokens);
    }
}
