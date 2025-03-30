use crate::{context::*, dict::BorthDict, errors::*};

pub fn call(ctx: &mut BorthContext, dict: &BorthDict, token: &str) -> BorthResult<()> {
    match token.to_uppercase().as_str() {
        "THEN" => {
            ctx.pop_value()?;
            ctx.pop_expression();
            ctx.pop_expression();
            Ok(())
        }
        _ => {
            let value = ctx.pop_value()?;
            if value == 0 {
                dict.eval(ctx, token)?;
            }
            ctx.push_value(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::{function::*, *};

    fn create_context() -> BorthContext {
        BorthContext::with_stack_size(10)
    }

    fn create_dict() -> BorthDict {
        BorthDict::new()
    }

    #[test]
    fn test1_if_else_false() {
        let mut ctx = create_context();
        let dict = create_dict();
        let _ = ctx.push_value(0);
        let _ = ctx.push_expression(BorthExpression::FunctionWithDict(if_then::call));
        let _ = ctx.push_expression(BorthExpression::FunctionWithDict(call));
        assert_eq!(call(&mut ctx, &dict, "1"), Ok(()));
        assert_eq!(call(&mut ctx, &dict, "THEN"), Ok(()));
        ctx.test(&[1], "", &[]);
    }

    #[test]
    fn test2_if_else_true() {
        let mut ctx = create_context();
        let dict = create_dict();
        let _ = ctx.push_value(-1);
        let _ = ctx.push_expression(BorthExpression::FunctionWithDict(if_then::call));
        let _ = ctx.push_expression(BorthExpression::FunctionWithDict(call));
        assert_eq!(call(&mut ctx, &dict, "1"), Ok(()));
        assert_eq!(call(&mut ctx, &dict, "THEN"), Ok(()));
        ctx.test(&[], "", &[]);
    }

    #[test]
    fn test3_if_else_false_open() {
        let mut ctx = create_context();
        let dict = create_dict();
        let _ = ctx.push_value(0);
        let _ = ctx.push_expression(BorthExpression::FunctionWithDict(if_then::call));
        let _ = ctx.push_expression(BorthExpression::FunctionWithDict(call));
        assert_eq!(call(&mut ctx, &dict, "1"), Ok(()));
        ctx.test(
            &[1, 0],
            "",
            &[
                BorthExpression::FunctionWithDict(if_then::call),
                BorthExpression::FunctionWithDict(call),
            ],
        );
    }

    #[test]
    fn test4_if_else_true_open() {
        let mut ctx = create_context();
        let dict = create_dict();
        let _ = ctx.push_value(-1);
        let _ = ctx.push_expression(BorthExpression::FunctionWithDict(if_then::call));
        let _ = ctx.push_expression(BorthExpression::FunctionWithDict(call));
        assert_eq!(call(&mut ctx, &dict, "1"), Ok(()));
        ctx.test(
            &[-1],
            "",
            &[
                BorthExpression::FunctionWithDict(if_then::call),
                BorthExpression::FunctionWithDict(call),
            ],
        );
    }
}
