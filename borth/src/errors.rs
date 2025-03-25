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
}

impl Display for BorthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:?}", self)
        } else {
            let name = match self {
                Self::UnknownWord => "?".into(),
                _ => to_kebabcase(format!("{:#?}", self)),
            };
            f.write_str(&name)
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
