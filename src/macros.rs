#[macro_export]
/// Unwraps a Result.
/// If it's an error, it'll be printed to stderr and the program execution will be ended
macro_rules! unwrap_or_return {
    ($ result:expr) => {
        match $result {
            Ok(ok) => ok,
            Err(err) => {
                eprintln!("{}: {}", "error:".red(), err);
                return Ok(());
            }
        }
    };
}
