use crate::{context::*, errors::BorthResult};

pub fn call(ctx: &mut BorthContext, token: &str, whitespace: &str) -> BorthResult<()> {
    if token.ends_with("\"") {
        ctx.pop_expression();
        ctx.print(token.trim_end_matches("\""));
    } else {
        let mut str = token.to_string();
        str.push_str(whitespace);
        ctx.print(&str);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_context() -> BorthContext {
        BorthContext::with_stack_size(10)
    }

    #[test]
    fn test1_dot_quote_hello_world() {
        let mut ctx = create_context();
        assert_eq!(call(&mut ctx, "hello", " "), Ok(()));
        assert_eq!(call(&mut ctx, "world\"", ""), Ok(()));
        ctx.test(&[], "hello world", &[]);
    }

    #[test]
    fn test2_dot_quote_multiple_whitespace() {
        let mut ctx = create_context();
        assert_eq!(call(&mut ctx, "hello", " "), Ok(()));
        assert_eq!(call(&mut ctx, "", " "), Ok(()));
        assert_eq!(call(&mut ctx, "", " "), Ok(()));
        assert_eq!(call(&mut ctx, "", " "), Ok(()));
        assert_eq!(call(&mut ctx, "", " "), Ok(()));
        assert_eq!(call(&mut ctx, "world\"", ""), Ok(()));
        ctx.test(&[], "hello     world", &[]);
    }

    #[test]
    fn test3_dot_quote_multiples() {
        let mut ctx = create_context();
        assert_eq!(call(&mut ctx, "hello\"", " "), Ok(()));
        assert_eq!(call(&mut ctx, "world\"", ""), Ok(()));
        ctx.test(&[], "hello world", &[]);
    }
}
