use crate::{context::*, errors::BorthResult, expression::BorthExpression, parser::*};

pub fn create(iterator: &mut BorthIterator) -> BorthExpression {
    let mut str = String::new();
    for (token, whitespace) in iterator.by_ref() {
        if token.ends_with("\"") {
            str.push_str(token.trim_end_matches("\""));
            return BorthExpression::DotQuote(str);
        }
        str.push_str(token);
        str.push_str(whitespace);
    }
    BorthExpression::IncompleteStatement
}

pub fn call(ctx: &mut BorthContext, str: &str) -> BorthResult<()> {
    ctx.print(str);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_context() -> BorthContext {
        BorthContext::with_stack_size(10)
    }

    fn assert_create_and_call(ctx: &mut BorthContext, tokens: Vec<(&str, &str)>, output: &str) {
        assert!(match create(&mut tokens.iter()) {
            BorthExpression::DotQuote(str) => call(ctx, &str).is_ok(),
            _ => false,
        });
        ctx.test(&[], output);
    }

    #[test]
    fn test1_dot_quote_hello() {
        let tokens = parse_tokens("hello\"");
        assert_create_and_call(&mut create_context(), tokens, "hello");
    }

    #[test]
    fn test2_dot_quote_hello_world() {
        let tokens = parse_tokens("hello world\"");
        assert_create_and_call(&mut create_context(), tokens, "hello world");
    }

    #[test]
    fn test3_dot_quote_multiple_whitespace() {
        let tokens = parse_tokens("hello     world\"");
        assert_create_and_call(&mut create_context(), tokens, "hello     world");
    }

    #[test]
    fn test4_dot_quote_multiples() {
        let mut ctx = create_context();
        let tokens = parse_tokens("hello\"");
        assert_create_and_call(&mut ctx, tokens, "hello");
        let tokens = parse_tokens("world\"");
        assert_create_and_call(&mut ctx, tokens, "hello world");
    }
}
