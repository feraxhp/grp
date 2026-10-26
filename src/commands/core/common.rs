
pub fn invalid() {
    color_print::cprintln!("<r>Command not found!</>")
}

#[macro_export]
macro_rules! fail {
    ($e: expr) => {{
        color_print::ceprintln!("<r>◉ {}</>", $e.message());
        $e.eprint();
        std::process::exit(1)
    }};
}

#[macro_export]
macro_rules! success {
    ($name: literal) => { success!($name "succeeded!") };
    ($name: literal $desc:literal) => {{
        color_print::ceprintln!("<g>✓ <y,i>{}</y,i> {}</>", $name, $desc);
    }};
    
    (np $name: literal) => { success!(np $name "succeeded!") };
    (np $name: literal $desc:literal) => {{
        color_print::cformat!("<g><y,i>{}</y,i> {}</>", $name, $desc)
    }};
}

#[macro_export]
macro_rules! warning {
    ($name: literal) => { success!($name "finish with errors!") };
    ($name: literal $desc:literal) => {{
        color_print::ceprintln!("<y>◎ <m,i>{}</m,i> {}</>", $name, $desc);
    }};
    
    (np $name: literal) => { success!(np $name "finish with errors!") };
    (np $name: literal $desc:literal) => {{
        color_print::cformat!("<y><m,i>{}</m,i> {}</>", $name, $desc)
    }};
}
