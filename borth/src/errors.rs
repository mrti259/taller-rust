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
