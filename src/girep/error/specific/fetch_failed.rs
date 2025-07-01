use color_print::cformat;


/// vec: [==2]
/// - 0 : message
/// - 1 : context
pub(crate) fn content(vec: Vec<String>) -> Vec<String> {
    debug_assert_eq!(vec.len(), 2);
    vec![
        cformat!("<r>* message: <m,i>{}</>", vec[0]),
        cformat!("<r>* context: <m,i>{}</>", vec[1]),
    ]
}
