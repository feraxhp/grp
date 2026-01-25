use color_print::cformat;

/// vec: [==2]
/// - 0 : pconf
/// - 1 : user
pub(crate) fn content(vec: Vec<String>) -> Vec<String> {
    debug_assert_eq!(vec.len(), 2);
    vec![
        cformat!("<y>* Please check your token.</>"),
        cformat!("  <g>» pconf : <m>{}</>", vec[0]),
        cformat!("  <g>» user  : <m>{}</>", vec[1]),
    ]
}