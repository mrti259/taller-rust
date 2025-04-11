use crate::{context::*, errors::*};

pub fn call(ctx: &mut BorthContext) -> BorthResult<()> {
    let item1 = ctx.pop_value()?;
    let ascii = char::from_u32(item1 as u32).ok_or(BorthError::RuntimeError)?;
    ctx.print(&ascii.to_string());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::BorthItem;

    fn create_context() -> BorthContext {
        BorthContext::with_stack_size(10)
    }

    fn push_to_stack(ctx: &mut BorthContext, items: &[BorthItem]) {
        for item in items {
            let _ = ctx.push_value(*item);
        }
    }

    #[test]
    fn test1_emit_uppercase() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[65]);
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[], "A");
    }

    #[test]
    fn test2_emit_lowercase() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[97]);
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[], "a");
    }

    #[test]
    fn test3_emit_multiple() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[68, 67, 66, 65]);
        assert_eq!(call(&mut ctx), Ok(()));
        assert_eq!(call(&mut ctx), Ok(()));
        assert_eq!(call(&mut ctx), Ok(()));
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[], "A B C D");
    }

    #[test]
    fn test4_stack_underflow() {
        let mut ctx = create_context();
        assert_eq!(call(&mut ctx), Err(BorthError::StackUnderflow));
    }
}
