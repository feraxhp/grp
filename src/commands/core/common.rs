
pub fn invalid() {
    color_print::cprintln!("<r>Command not found!</>")
}

#[macro_export]
macro_rules! fail {
    ($e: expr) => {{
        color_print::ceprintln!("<r>◉ {}</>", &$e.message);
        $e.show();
        std::process::exit(1)
    }};
}

#[macro_export]
macro_rules! success {
    ($name: literal) => {{
        color_print::ceprintln!("<g>✓ <y,i>{}</y,i> succeeded!</>", $name);
    }};
}
