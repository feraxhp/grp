use color_print::cformat;

/// vec: [>=1]
/// - 0+: error message
pub(crate) fn content(vec: Vec<String>) -> Vec<String> {
    let mut vect = Vec::new();

    for (index, item ) in vec.iter().enumerate() {
        vect.push(
            cformat!("  <r>* ({}) Error:</> <m,i>{}</>", index, item)
        );
    };

    vect
}