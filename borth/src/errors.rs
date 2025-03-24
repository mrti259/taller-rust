use std::fmt::Display;

#[derive(Debug)]
pub enum BorthError {
    MissingArguments,
    TooManyArguments,
    BadArguments,
    CanNotReadFile,
    CanNotReadCode,
    CanNotWriteFile,
    RuntimeError,
}

impl Display for BorthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:?}", self)
        } else {
            let name = format!("{:#}", self);
            f.write_str(&to_kebabcase(name))
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
