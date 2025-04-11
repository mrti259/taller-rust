use std::{rc::Rc, slice::Iter};

use crate::{dict::BorthDict, expression::BorthExpression};

pub type BorthTokens<'a> = Vec<(&'a str, &'a str)>;
pub type BorthIterator<'a> = Iter<'a, (&'a str, &'a str)>;

pub fn parse_tokens(code: &str) -> BorthTokens<'_> {
    let mut tokens = vec![];
    let mut whitespaces = code.match_indices(char::is_whitespace);
    let mut offset = 0;
    while offset < code.len() {
        let (stop, whitespace) = match whitespaces.next() {
            Some(result) => result,
            None => (code.len(), ""),
        };
        let word = match code.get(offset..stop) {
            Some(token) => token,
            None => code,
        };
        offset = stop + 1;
        tokens.push((word, whitespace));
    }
    tokens
}

pub fn parse_expressions(tokens: BorthTokens, dict: &mut BorthDict) -> Vec<Rc<BorthExpression>> {
    let mut expressions = vec![];
    let mut iterator = tokens.iter();
    while let Some(expression) = dict.detect_next(&mut iterator) {
        expressions.push(expression);
    }
    expressions
}
