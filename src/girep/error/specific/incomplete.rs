use color_print::cformat;

/// vec: [==1]
///  - 0 : file-path
pub(crate) fn content(vec: Vec<String>) -> Vec<String> {
    debug_assert_eq!(vec.len(), 1);
    vec![
        cformat!("<r>* Bad amount of arguments</>"),
        cformat!("  -> file: <b,i>{}</>", vec[0]),
        cformat!("<m>This is for debug purposes, if u see this please report it</>"),
    ]
}