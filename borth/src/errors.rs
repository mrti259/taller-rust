use std::fmt::Display;

pub type BorthResult<T> = Result<T, BorthError>;

#[derive(Debug)]
pub enum BorthError {
    // Common errors:
    StackUnderflow,
    // StackOverflow,
    // InvalidWord,
    // DivisionByZero,
    UnknownWord,

    // Custom errors:
    MissingArguments,
    TooManyArguments,
    BadArguments,
    CanNotReadFile,
    CanNotReadCode,
    CanNotWriteFile,
    CanNotWriteToOutput,
    RuntimeError,
}

impl Display for BorthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:?}", self)
        } else {
            let name = match self {
                Self::UnknownWord => "?",
                _ => &to_kebabcase(format!("{:#?}", self)),
            };
            f.write_str(name)
        }
    }
}

fn to_kebabcase(str: String) -> String {
    str.chars()
        .map(|c| {
            if c.is_uppercase() {
                format!("-{}", c.to_lowercase())
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
        .trim_start_matches("-")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_display_error_in_kebabcase() {
        assert_eq!(BorthError::StackUnderflow.to_string(), "stack-underflow");
    }

    #[test]
    fn test2_display_unknown_word_as_question_mark() {
        assert_eq!(BorthError::UnknownWord.to_string(), "?");
    }
}
