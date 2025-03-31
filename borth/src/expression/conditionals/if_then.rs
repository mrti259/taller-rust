use super::else_then;
use crate::{context::*, dict::BorthDict, errors::*, expression::*};

pub fn call(ctx: &mut BorthContext, dict: &BorthDict, token: &str) -> BorthResult<()> {
    match token.to_uppercase().as_str() {
        "THEN" => {
            ctx.pop_value()?;
            ctx.pop_expression();
            Ok(())
        }
        "ELSE" => {
            ctx.push_expression(BorthExpression::FunctionWithDict(else_then::call));
            Ok(())
        }
        _ => {
            let value = ctx.pop_value()?;
            if value != 0 {
                dict.eval(ctx, token)?;
            }
            ctx.push_value(value)
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
    fn test1_if_false() {
        let mut ctx = create_context();
        let dict = create_dict();
        let _ = ctx.push_value(0);
        let _ = ctx.push_expression(BorthExpression::FunctionWithDict(call));
        assert_eq!(call(&mut ctx, &dict, "1"), Ok(()));
        assert_eq!(call(&mut ctx, &dict, "THEN"), Ok(()));
        ctx.test(&[], "", &[]);
    }

    #[test]
    fn test2_if_true() {
        let mut ctx = create_context();
        let dict = create_dict();
        let _ = ctx.push_value(-1);
        let _ = ctx.push_expression(BorthExpression::FunctionWithDict(call));
        assert_eq!(call(&mut ctx, &dict, "1"), Ok(()));
        assert_eq!(call(&mut ctx, &dict, "THEN"), Ok(()));
        ctx.test(&[1], "", &[]);
    }

    #[test]
    fn test3_if_false_open() {
        let mut ctx = create_context();
        let dict = create_dict();
        let _ = ctx.push_value(0);
        let _ = ctx.push_expression(BorthExpression::FunctionWithDict(call));
        assert_eq!(call(&mut ctx, &dict, "1"), Ok(()));
        ctx.test(&[0], "", &[BorthExpression::FunctionWithDict(call)]);
    }

    #[test]
    fn test4_if_true_open() {
        let mut ctx = create_context();
        let dict = create_dict();
        let _ = ctx.push_value(-1);
        let _ = ctx.push_expression(BorthExpression::FunctionWithDict(call));
        assert_eq!(call(&mut ctx, &dict, "1"), Ok(()));
        ctx.test(&[1, -1], "", &[BorthExpression::FunctionWithDict(call)]);
    }
}
