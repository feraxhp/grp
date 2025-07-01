use color_print::cformat;

/// vec: [==1]
/// - 0 : path
pub(crate) fn content(vec: Vec<String>) -> Vec<String> {
    assert!(vec.len() == 1);
    vec![
        cformat!("<y>* Location: <m>{}</>", vec[0]),
        cformat!("  You may need to start a new repo"),
        cformat!("  •<g> git init </>")
    ]
}