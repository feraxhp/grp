use color_print::cformat;

/// vec: [>=1]
/// - 0+: error message
pub(crate) fn content(vec: Vec<String>) -> Vec<String> {
    let mut vect = vec![
        cformat!("<y>* You must not see this error</>"),
    ];

    for item in vec {
        vect.push(
            cformat!("  <m>-></> {}", item)
        );
    };

    vect
}