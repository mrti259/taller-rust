pub mod arithmetic;
pub mod booleans;
pub mod output;
pub mod specials;
pub mod stack;

use super::{context::*, errors::*};
use specials::*;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
/// Posible expressions in Borth
pub enum BorthExpression {
    Number(BorthItem),
    Operation(fn(&mut BorthContext) -> BorthResult<()>),
    DotQuote(String),
    IfElseThen(Vec<Rc<Self>>, Vec<Rc<Self>>),
    Word(Vec<Rc<Self>>),
    UnknownWord(String),
    IncompleteStatement,
    InvalidWord,
    WordCreated,
}

impl BorthExpression {
    /// Handle the evaluation of the expression
    pub fn eval(&self, ctx: &mut BorthContext) -> BorthResult<()> {
        match self {
            BorthExpression::Number(value) => ctx.push_value(*value),
            BorthExpression::Operation(cb) => cb(ctx),
            BorthExpression::DotQuote(str) => dot_quote::call(ctx, str),
            BorthExpression::IfElseThen(if_block, else_block) => {
                if_else_then::call(ctx, if_block, else_block)
            }
            BorthExpression::Word(body) => word_def::call(ctx, body),
            BorthExpression::UnknownWord(word) => Err(BorthError::UnknownWord(word.into())),
            BorthExpression::IncompleteStatement => Err(BorthError::IncompleteStatement),
            BorthExpression::InvalidWord => Err(BorthError::InvalidWord),
            BorthExpression::WordCreated => Ok(()),
        }
    }
}
