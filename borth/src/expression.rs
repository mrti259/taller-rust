pub mod arithmetic;
pub mod booleans;
pub mod conditionals;
pub mod output;
pub mod stack;
pub mod word_def;

use crate::{context::*, dict::BorthDict, errors::*, stack::*};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum BorthExpression {
    Operation(fn(&mut BorthContext) -> BorthResult<()>),
    FunctionWithWhiteSpace(fn(&mut BorthContext, &str, &str) -> BorthResult<()>),
    FunctionWithDict(fn(&mut BorthContext, &BorthDict, &str) -> BorthResult<()>),
    FunctionWithMutDict(fn(&mut BorthContext, &mut BorthDict, &str) -> BorthResult<()>),
    Number(BorthItem),
    Word(Vec<Rc<Self>>),
}
