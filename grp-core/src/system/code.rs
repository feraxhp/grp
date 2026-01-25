
#[macro_export]
/// Returns a string with the current source code location
/// 
/// > Returns something like `"src/main.rs:10:5"`
/// 
/// This macro formats the current file name, 
/// line number and column number
/// into a single string in the 
/// format *file:line:column*
///
/// # Examples
/// 
/// ```rust
/// let loc = location!();
/// ```
macro_rules! location {
    () => {
        format!("{}:{}:{}", file!(), line!(), column!())
    };
}
