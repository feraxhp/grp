#[macro_export]
macro_rules! empty_notes {
    () => { Vec::<String>::new() };
}

#[macro_export]
macro_rules! make_error {
    (
        $notes:ident, 
        $type:expr,
        $message:expr,
        $number:literal of
            $( $content:expr ),* $(,)?
    ) => {{
        let mut content_length: usize = $number;
        let notes_legth = $notes.len();
        if notes_legth > 0 { content_length += $notes.len() + 2; }
        let mut content: Vec<String> = Vec::with_capacity(content_length);
        
        $( content.push($content); )*
        
        if notes_legth > 0 { 
            content.push(cformat!(""));
            content.push(cformat!("<g># notes</>"));
            content.extend($crate::error::tools::Notes::as_notes($notes.iter()));
        }
        
        $crate::error::structs::Error {
            etype: $type.to_string(),
            message: $message.to_string(),
            content: content,
        }
    }};
    (
        $type:expr,
        $message:expr,
        $number:literal of
            $( $content:expr ),* $(,)?
    ) => {{
        let mut content: Vec<String> = Vec::with_capacity($number);
        
        $( content.push($content); )*
        
        $crate::error::structs::Error {
            etype: $type.to_string(),
            message: $message.to_string(),
            content: content,
        }
    }};
}
